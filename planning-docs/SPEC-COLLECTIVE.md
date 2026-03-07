# SPEC-COLLECTIVE.md

> A million agents analyzing `express.js` independently is insane. Share patterns, not code.

---

## Overview

The Collective Intelligence layer enables:
1. **Pattern sharing**: Common usage patterns for open-source libraries
2. **Mistake aggregation**: Frequent errors and their fixes
3. **Performance knowledge**: Hidden performance characteristics
4. **Migration paths**: What broke during upgrades

**Critical constraint**: Private code NEVER leaves the local machine. Only patterns from open-source dependencies are shared.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Local Machine                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │ Your Code   │    │ Collective  │    │ .acb file   │     │
│  │ (Private)   │───▶│ Extractor   │───▶│ + metadata  │     │
│  └─────────────┘    └──────┬──────┘    └─────────────┘     │
│                            │                                 │
│                    Only open-source                          │
│                    library patterns                          │
│                            │                                 │
└────────────────────────────┼────────────────────────────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │   Collective    │
                    │   Registry      │
                    │   (Cloud/P2P)   │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
         ┌────────┐    ┌────────┐    ┌────────┐
         │Agent A │    │Agent B │    │Agent C │
         │pulls   │    │pulls   │    │pulls   │
         │patterns│    │patterns│    │patterns│
         └────────┘    └────────┘    └────────┘
```

---

## Privacy Model

```rust
// src/collective/privacy.rs

/// What CAN be shared
pub enum Shareable {
    /// Usage patterns of open-source libraries
    LibraryUsagePattern {
        library: String,
        version: String,
        pattern: UsagePattern,
    },
    /// Common mistakes with open-source APIs
    LibraryMistake {
        library: String,
        mistake_hash: [u8; 32],  // Hash, not actual code
        fix_category: String,
    },
    /// Performance characteristics
    PerformanceNote {
        library: String,
        symbol: String,
        characteristic: String,
    },
    /// Upgrade impact
    MigrationImpact {
        library: String,
        from_version: String,
        to_version: String,
        impact_category: String,
        affected_pattern_hash: [u8; 32],
    },
}

/// What CANNOT be shared
pub enum NonShareable {
    /// Any code from private repositories
    PrivateCode,
    /// Symbol names from private code
    PrivateSymbols,
    /// File paths from private code
    PrivatePaths,
    /// Business logic patterns
    BusinessPatterns,
    /// Anything not explicitly open-source
    Unknown,
}

/// Determine if a code unit's patterns can be shared
pub fn is_shareable(unit: &CodeUnit, graph: &CodeGraph) -> bool {
    // Only share if:
    // 1. Unit is from an open-source dependency (node_modules, site-packages, etc.)
    // 2. OR unit is using open-source library symbols
    // 3. AND unit's own code is not exposed (only the pattern of usage)
    
    // Check if path indicates dependency
    let path_str = unit.file_path.to_string_lossy();
    let is_dependency = 
        path_str.contains("node_modules") ||
        path_str.contains("site-packages") ||
        path_str.contains(".cargo/registry") ||
        path_str.contains("vendor/");
    
    if is_dependency {
        return false; // We share patterns of USAGE, not the libraries themselves
    }
    
    // Check if unit uses open-source libraries
    let edges = graph.edges_from(unit.id);
    edges.iter().any(|e| {
        if let Some(target) = graph.get_unit(e.target_id) {
            let target_path = target.file_path.to_string_lossy();
            target_path.contains("node_modules") ||
            target_path.contains("site-packages") ||
            target_path.contains(".cargo/registry")
        } else {
            false
        }
    })
}
```

---

## Pattern Extraction

```rust
// src/collective/patterns.rs

/// A usage pattern extracted from code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePattern {
    /// Library being used
    pub library: String,
    /// Library version
    pub version: Option<String>,
    /// Pattern category
    pub category: PatternCategory,
    /// Pattern signature (anonymized)
    pub signature: PatternSignature,
    /// Frequency this pattern was seen
    pub frequency: u32,
    /// Quality indicators
    pub quality: PatternQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternCategory {
    /// How to initialize/configure
    Initialization,
    /// Common API usage sequence
    ApiSequence,
    /// Error handling pattern
    ErrorHandling,
    /// Resource management
    ResourceManagement,
    /// Testing pattern
    Testing,
    /// Performance optimization
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSignature {
    /// Sequence of API calls (by symbol name only)
    pub call_sequence: Vec<String>,
    /// Structure hash (not actual code)
    pub structure_hash: [u8; 32],
    /// Complexity bucket
    pub complexity: ComplexityBucket,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComplexityBucket {
    Simple,   // 1-5 statements
    Medium,   // 6-20 statements
    Complex,  // 21-50 statements
    VeryComplex, // 50+ statements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternQuality {
    /// Does it have error handling?
    pub has_error_handling: bool,
    /// Is it tested?
    pub is_tested: bool,
    /// Has it been stable (not changed frequently)?
    pub is_stable: bool,
}

/// Extract patterns from a codebase
pub struct PatternExtractor;

impl PatternExtractor {
    pub fn extract(graph: &CodeGraph) -> Vec<UsagePattern> {
        let mut patterns = Vec::new();
        
        // Find all units that use open-source libraries
        for unit in graph.all_units() {
            if !is_shareable(unit, graph) {
                continue;
            }
            
            // Extract usage patterns for each library used
            let library_usages = Self::extract_library_usages(unit, graph);
            
            for (library, usage) in library_usages {
                let pattern = UsagePattern {
                    library,
                    version: Self::detect_library_version(graph),
                    category: Self::categorize_usage(&usage),
                    signature: Self::anonymize_to_signature(&usage),
                    frequency: 1,
                    quality: Self::assess_quality(unit, graph),
                };
                patterns.push(pattern);
            }
        }
        
        // Deduplicate and count frequencies
        Self::deduplicate_and_count(&mut patterns);
        
        patterns
    }
    
    fn extract_library_usages(
        unit: &CodeUnit,
        graph: &CodeGraph,
    ) -> Vec<(String, LibraryUsage)> {
        // Find all calls to library functions
        // Group by library
        // Extract call sequence
        todo!()
    }
    
    fn anonymize_to_signature(usage: &LibraryUsage) -> PatternSignature {
        // Convert actual code to anonymized signature
        // Keep: API call names, structure
        // Remove: variable names, literals, business logic
        todo!()
    }
    
    fn categorize_usage(usage: &LibraryUsage) -> PatternCategory {
        // Heuristics to determine pattern category
        todo!()
    }
    
    fn assess_quality(unit: &CodeUnit, graph: &CodeGraph) -> PatternQuality {
        // Check for error handling, tests, stability
        todo!()
    }
}

struct LibraryUsage {
    calls: Vec<String>,
    structure: Vec<u8>,
}
```

---

## Delta Compression

```rust
// src/collective/delta.rs

/// Compressed delta for efficient sync
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveDelta {
    /// Unique identifier for this delta
    pub id: [u8; 16],
    /// Timestamp
    pub timestamp: u64,
    /// Source identifier (anonymized)
    pub source_hash: [u8; 32],
    /// Patterns in this delta
    pub patterns: Vec<UsagePattern>,
    /// Mistakes discovered
    pub mistakes: Vec<MistakeReport>,
    /// Compressed size
    pub compressed_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MistakeReport {
    pub library: String,
    pub mistake_signature: [u8; 32],
    pub category: MistakeCategory,
    pub fix_hint: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MistakeCategory {
    TypeMismatch,
    MissingErrorHandling,
    ResourceLeak,
    RaceCondition,
    PerformanceAntipattern,
    DeprecatedApi,
    SecurityIssue,
}

impl CollectiveDelta {
    /// Create a delta from extracted patterns
    pub fn create(
        patterns: Vec<UsagePattern>,
        mistakes: Vec<MistakeReport>,
    ) -> Self {
        Self {
            id: rand::random(),
            timestamp: crate::types::now_micros(),
            source_hash: blake3::hash(&rand::random::<[u8; 32]>()).into(),
            patterns,
            mistakes,
            compressed_size: 0,
        }
    }
    
    /// Compress for transmission
    pub fn compress(&self) -> AcbResult<Vec<u8>> {
        let json = serde_json::to_vec(self)?;
        let compressed = lz4_flex::compress_prepend_size(&json);
        Ok(compressed)
    }
    
    /// Decompress received delta
    pub fn decompress(data: &[u8]) -> AcbResult<Self> {
        let json = lz4_flex::decompress_size_prepended(data)
            .map_err(|e| AcbError::Compression(e.to_string()))?;
        let delta = serde_json::from_slice(&json)?;
        Ok(delta)
    }
}
```

---

## Registry Client

```rust
// src/collective/registry.rs

/// Client for interacting with collective registry
pub struct RegistryClient {
    endpoint: String,
    enabled: bool,
    cache: CollectiveCache,
}

impl RegistryClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            enabled: true,
            cache: CollectiveCache::new(),
        }
    }
    
    /// Push patterns to collective (async, non-blocking)
    pub async fn push_delta(&self, delta: CollectiveDelta) -> AcbResult<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let compressed = delta.compress()?;
        
        // Fire and forget - don't block on upload
        tokio::spawn(async move {
            // HTTP POST to registry
            // On failure, queue for retry
        });
        
        Ok(())
    }
    
    /// Query collective for library patterns
    pub async fn query_patterns(&self, library: &str) -> AcbResult<Vec<UsagePattern>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        // Check cache first
        if let Some(cached) = self.cache.get_patterns(library) {
            return Ok(cached);
        }
        
        // Query registry
        let patterns = self.fetch_patterns(library).await?;
        
        // Cache result
        self.cache.set_patterns(library, patterns.clone());
        
        Ok(patterns)
    }
    
    /// Query for common mistakes
    pub async fn query_mistakes(&self, library: &str) -> AcbResult<Vec<AggregatedMistake>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        // Similar to query_patterns
        todo!()
    }
    
    async fn fetch_patterns(&self, library: &str) -> AcbResult<Vec<UsagePattern>> {
        // HTTP GET from registry
        // Parse response
        todo!()
    }
}

#[derive(Debug)]
pub struct AggregatedMistake {
    pub library: String,
    pub category: MistakeCategory,
    pub frequency: u64,
    pub description: String,
    pub fix_suggestion: String,
}
```

---

## Local Cache

```rust
// src/collective/cache.rs

use std::time::{Duration, Instant};

/// Local cache for collective data
pub struct CollectiveCache {
    patterns: HashMap<String, CacheEntry<Vec<UsagePattern>>>,
    mistakes: HashMap<String, CacheEntry<Vec<AggregatedMistake>>>,
    ttl: Duration,
}

struct CacheEntry<T> {
    data: T,
    cached_at: Instant,
}

impl CollectiveCache {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            mistakes: HashMap::new(),
            ttl: Duration::from_secs(3600), // 1 hour default
        }
    }
    
    pub fn get_patterns(&self, library: &str) -> Option<Vec<UsagePattern>> {
        self.patterns.get(library).and_then(|entry| {
            if entry.cached_at.elapsed() < self.ttl {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }
    
    pub fn set_patterns(&mut self, library: &str, patterns: Vec<UsagePattern>) {
        self.patterns.insert(library.to_string(), CacheEntry {
            data: patterns,
            cached_at: Instant::now(),
        });
    }
    
    pub fn clear(&mut self) {
        self.patterns.clear();
        self.mistakes.clear();
    }
}
```

---

## Collective Query Integration

```rust
// src/engine/query.rs

impl QueryEngine {
    /// Query 12: Collective Patterns
    pub async fn collective_patterns(
        &self,
        graph: &CodeGraph,
        params: CollectiveParams,
    ) -> AcbResult<CollectiveResult> {
        let client = RegistryClient::new(&self.collective_endpoint);
        
        // Get patterns from collective
        let patterns = client.query_patterns(&params.library).await?;
        
        // Get mistakes
        let mistakes = client.query_mistakes(&params.library).await?;
        
        // Score user's code against patterns
        let your_patterns = PatternExtractor::extract_for_library(graph, &params.library);
        let score = Self::score_against_collective(&your_patterns, &patterns);
        
        // Generate suggestions
        let suggestions = Self::generate_suggestions(&your_patterns, &patterns, &mistakes);
        
        Ok(CollectiveResult {
            library: params.library,
            analysis_count: patterns.iter().map(|p| p.frequency as u64).sum(),
            patterns: Self::top_patterns(patterns, 10),
            mistakes: mistakes.into_iter().take(5).collect(),
            your_score: score,
            suggestions,
        })
    }
    
    fn score_against_collective(
        yours: &[UsagePattern],
        collective: &[UsagePattern],
    ) -> f32 {
        // Compare your patterns against collective best practices
        // Higher score = more aligned with collective wisdom
        todo!()
    }
    
    fn generate_suggestions(
        yours: &[UsagePattern],
        collective: &[UsagePattern],
        mistakes: &[AggregatedMistake],
    ) -> Vec<String> {
        // Find where your code differs from collective patterns
        // Check if you're making common mistakes
        todo!()
    }
}
```

---

## Offline Mode

```rust
// src/collective/mod.rs

/// Collective works fully offline - just without shared intelligence
pub struct CollectiveManager {
    client: Option<RegistryClient>,
    local_patterns: HashMap<String, Vec<UsagePattern>>,
}

impl CollectiveManager {
    pub fn offline() -> Self {
        Self {
            client: None,
            local_patterns: HashMap::new(),
        }
    }
    
    pub fn online(endpoint: &str) -> Self {
        Self {
            client: Some(RegistryClient::new(endpoint)),
            local_patterns: HashMap::new(),
        }
    }
    
    /// Query works in both modes
    pub async fn query(&self, library: &str) -> Vec<UsagePattern> {
        // Try online first
        if let Some(client) = &self.client {
            if let Ok(patterns) = client.query_patterns(library).await {
                return patterns;
            }
        }
        
        // Fall back to local
        self.local_patterns.get(library).cloned().unwrap_or_default()
    }
}
```

---

## Configuration

```toml
# ~/.config/acb/config.toml

[collective]
# Enable collective intelligence
enabled = true

# Registry endpoint
endpoint = "https://collective.agenticcodebase.dev"

# Alternative: use local/private registry
# endpoint = "http://localhost:8080"

# Cache TTL in seconds
cache_ttl = 3600

# Opt-in to sharing patterns (default: true if enabled)
share_patterns = true

# Rate limit for pushes (per hour)
push_rate_limit = 10
```

---

## Security Considerations

1. **No code transmission**: Only pattern signatures, never actual code
2. **Anonymized source**: Source hash is randomized, not traceable
3. **Library patterns only**: Private code usage is never extracted
4. **Local validation**: Patterns are validated locally before push
5. **Opt-in sharing**: Users explicitly enable pattern sharing
6. **HTTPS only**: All registry communication is encrypted
7. **Rate limiting**: Prevent abuse of registry
