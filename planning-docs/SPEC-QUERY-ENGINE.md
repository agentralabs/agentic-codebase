# SPEC-QUERY-ENGINE.md

> Navigation, not search. The agent walks concept graphs, traces impacts, and predicts the future.

---

## QueryEngine

```rust
// src/engine/query.rs

pub struct QueryEngine;

impl QueryEngine {
    pub fn new() -> Self;
    
    // === Core Queries (1-8) ===
    pub fn symbol_lookup(&self, graph: &CodeGraph, params: SymbolLookupParams) -> AcbResult<Vec<&CodeUnit>>;
    pub fn dependency_graph(&self, graph: &CodeGraph, params: DependencyParams) -> AcbResult<DependencyResult>;
    pub fn reverse_dependency(&self, graph: &CodeGraph, params: DependencyParams) -> AcbResult<DependencyResult>;
    pub fn call_graph(&self, graph: &CodeGraph, params: CallGraphParams) -> AcbResult<CallGraphResult>;
    pub fn type_hierarchy(&self, graph: &CodeGraph, params: HierarchyParams) -> AcbResult<HierarchyResult>;
    pub fn containment(&self, graph: &CodeGraph, unit_id: u64) -> AcbResult<Vec<&CodeUnit>>;
    pub fn pattern_match(&self, graph: &CodeGraph, params: PatternParams) -> AcbResult<Vec<PatternMatch>>;
    pub fn semantic_search(&self, graph: &CodeGraph, params: SemanticParams) -> AcbResult<Vec<SemanticMatch>>;
    
    // === Built Queries (9-11, 22-23) ===
    pub fn impact_analysis(&self, graph: &CodeGraph, params: ImpactParams) -> AcbResult<ImpactResult>;
    pub fn test_coverage(&self, graph: &CodeGraph, unit_id: u64) -> AcbResult<CoverageResult>;
    pub fn cross_language_trace(&self, graph: &CodeGraph, params: TraceParams) -> AcbResult<TraceResult>;
    pub fn similarity(&self, graph: &CodeGraph, params: SimilarityParams) -> AcbResult<Vec<SimilarityMatch>>;
    pub fn shortest_path(&self, graph: &CodeGraph, from: u64, to: u64) -> AcbResult<PathResult>;
    
    // === Novel Queries (12-21, 24) ===
    pub fn collective_patterns(&self, graph: &CodeGraph, params: CollectiveParams) -> AcbResult<CollectiveResult>;
    pub fn temporal_evolution(&self, graph: &CodeGraph, unit_id: u64) -> AcbResult<EvolutionResult>;
    pub fn stability_analysis(&self, graph: &CodeGraph, unit_id: u64) -> AcbResult<StabilityResult>;
    pub fn coupling_detection(&self, graph: &CodeGraph, params: CouplingParams) -> AcbResult<Vec<Coupling>>;
    pub fn dead_code(&self, graph: &CodeGraph, params: DeadCodeParams) -> AcbResult<Vec<&CodeUnit>>;
    pub fn prophecy(&self, graph: &CodeGraph, params: ProphecyParams) -> AcbResult<ProphecyResult>;
    pub fn concept_mapping(&self, graph: &CodeGraph, concept: &str) -> AcbResult<ConceptMap>;
    pub fn migration_path(&self, graph: &CodeGraph, params: MigrationParams) -> AcbResult<MigrationPlan>;
    pub fn test_gap(&self, graph: &CodeGraph, params: TestGapParams) -> AcbResult<Vec<TestGap>>;
    pub fn architectural_drift(&self, graph: &CodeGraph, params: DriftParams) -> AcbResult<DriftReport>;
    pub fn hotspot_detection(&self, graph: &CodeGraph, params: HotspotParams) -> AcbResult<Vec<Hotspot>>;
}
```

---

## Query 1: Symbol Lookup

Find code units by name.

```rust
pub struct SymbolLookupParams {
    /// Name to search for (exact or prefix)
    pub name: String,
    /// Match mode
    pub mode: MatchMode,
    /// Filter by unit type
    pub unit_types: Vec<CodeUnitType>,
    /// Filter by language
    pub languages: Vec<Language>,
    /// Max results
    pub limit: usize,
}

pub enum MatchMode {
    Exact,
    Prefix,
    Contains,
    Fuzzy,
}
```

**Algorithm:**
1. Use symbol name index for Exact/Prefix
2. Use linear scan with string matching for Contains
3. Use Levenshtein distance for Fuzzy
4. Apply filters, return up to limit

---

## Query 2: Dependency Graph

What does this unit depend on?

```rust
pub struct DependencyParams {
    pub unit_id: u64,
    pub depth: u32,
    pub edge_types: Vec<EdgeType>,  // Default: [Imports, Calls, UsesType]
    pub include_transitive: bool,
}

pub struct DependencyResult {
    pub root_id: u64,
    pub dependencies: Vec<DependencyNode>,
    pub total_count: usize,
}

pub struct DependencyNode {
    pub unit_id: u64,
    pub depth: u32,
    pub edge_type: EdgeType,
    pub path: Vec<u64>,  // How we got here
}
```

**Algorithm:**
1. BFS from unit_id following outgoing edges of specified types
2. Track depth and path to each discovered node
3. Stop at max depth or when no new nodes found

---

## Query 3: Reverse Dependency

What depends on this unit?

Same structure as Query 2, but follows **incoming** edges instead of outgoing.

---

## Query 4: Call Graph

Trace function calls.

```rust
pub struct CallGraphParams {
    pub unit_id: u64,
    pub direction: Direction,
    pub depth: u32,
    pub include_indirect: bool,  // Include calls through function pointers/callbacks
}

pub enum Direction {
    Callers,    // Who calls this?
    Callees,    // What does this call?
    Both,
}

pub struct CallGraphResult {
    pub root_id: u64,
    pub callers: Vec<CallNode>,
    pub callees: Vec<CallNode>,
}

pub struct CallNode {
    pub unit_id: u64,
    pub call_sites: Vec<Span>,  // Where the calls happen
    pub depth: u32,
}
```

---

## Query 5: Type Hierarchy

Navigate inheritance and implementation relationships.

```rust
pub struct HierarchyParams {
    pub unit_id: u64,
    pub direction: HierarchyDirection,
    pub include_interfaces: bool,
}

pub enum HierarchyDirection {
    Ancestors,    // What does this inherit from?
    Descendants,  // What inherits from this?
    Both,
}

pub struct HierarchyResult {
    pub root_id: u64,
    pub ancestors: Vec<HierarchyNode>,
    pub descendants: Vec<HierarchyNode>,
}

pub struct HierarchyNode {
    pub unit_id: u64,
    pub relationship: EdgeType,  // Inherits or Implements
    pub depth: u32,
}
```

---

## Query 6: Containment

What's inside this module/class?

**Algorithm:**
1. Find all units where unit has Contains edge to target
2. Recursively find nested containment
3. Return flattened or tree structure

---

## Query 7: Pattern Match

Find code matching a structural pattern.

```rust
pub struct PatternParams {
    /// Pattern specification (custom DSL)
    pub pattern: String,
    /// Languages to search
    pub languages: Vec<Language>,
    /// Max results
    pub limit: usize,
}

pub struct PatternMatch {
    pub unit_id: u64,
    pub match_score: f32,
    pub bindings: HashMap<String, u64>,  // Pattern variables → matched units
}
```

**Pattern DSL Examples:**
```
# Find all functions that call both A and B
function { calls: [A, B] }

# Find all classes that inherit from Base and implement Interface
class { inherits: Base, implements: Interface }

# Find all async functions with complexity > 10
async function { complexity: >10 }
```

---

## Query 8: Semantic Search

Find code by description using vector similarity.

```rust
pub struct SemanticParams {
    /// Natural language query
    pub query: String,
    /// Or direct vector
    pub query_vec: Option<Vec<f32>>,
    /// Number of results
    pub top_k: usize,
    /// Minimum similarity threshold
    pub min_similarity: f32,
    /// Filter by type
    pub unit_types: Vec<CodeUnitType>,
}

pub struct SemanticMatch {
    pub unit_id: u64,
    pub similarity: f32,
}
```

**Algorithm:**
1. If query string provided, generate embedding (pluggable)
2. Brute-force cosine similarity against all feature vectors
3. Apply filters, return top_k

---

## Query 9: Impact Analysis ⭐

**The killer query.** What breaks if I change this?

```rust
pub struct ImpactParams {
    pub unit_id: u64,
    pub depth: u32,
    pub include_tests: bool,
    pub include_temporal: bool,  // Include historically coupled units
}

pub struct ImpactResult {
    pub root_id: u64,
    pub direct_dependents: Vec<ImpactNode>,
    pub transitive_dependents: Vec<ImpactNode>,
    pub affected_tests: Vec<u64>,
    pub historical_breakage: Vec<HistoricalBreak>,
    pub risk_score: f32,
    pub recommendation: String,
}

pub struct ImpactNode {
    pub unit_id: u64,
    pub risk_level: RiskLevel,
    pub reason: String,
    pub path: Vec<u64>,
}

pub enum RiskLevel {
    High,    // Direct dependency, no test coverage
    Medium,  // Transitive dependency or has tests
    Low,     // Distant or well-tested
}
```

**Algorithm:**
1. Run reverse_dependency to find all dependents
2. For each dependent, check test coverage (Tests edges)
3. Check temporal coupling (CouplesWith, BreaksWith edges)
4. Score risk: no_test_coverage + historical_breakage + coupling_strength
5. Generate recommendation based on findings

---

## Query 10: Test Coverage

What tests cover this code?

```rust
pub struct CoverageResult {
    pub unit_id: u64,
    pub direct_tests: Vec<u64>,
    pub indirect_tests: Vec<u64>,  // Tests of callers
    pub coverage_ratio: f32,       // Estimated based on call graph
    pub untested_paths: Vec<Vec<u64>>,  // Paths without test coverage
}
```

---

## Query 11: Cross-Language Trace

Trace calls across FFI boundaries.

```rust
pub struct TraceParams {
    pub unit_id: u64,
    pub direction: Direction,
    pub max_hops: u32,
}

pub struct TraceResult {
    pub path: Vec<TraceHop>,
}

pub struct TraceHop {
    pub unit_id: u64,
    pub language: Language,
    pub ffi_type: Option<FfiType>,  // FFI, WASM, HTTP, etc.
}
```

---

## Query 12: Collective Patterns 🆕

How do experts use this library?

```rust
pub struct CollectiveParams {
    pub library: String,
    pub symbol: Option<String>,
    pub pattern_type: PatternType,
}

pub enum PatternType {
    Usage,       // Common usage patterns
    Mistakes,    // Frequent errors
    Performance, // Performance characteristics
    Migration,   // Upgrade paths
}

pub struct CollectiveResult {
    pub library: String,
    pub analysis_count: u64,
    pub patterns: Vec<CollectivePattern>,
    pub mistakes: Vec<CollectiveMistake>,
    pub your_score: f32,
    pub suggestions: Vec<String>,
}
```

**Algorithm:**
1. Query collective registry for library data
2. Compare user's code patterns against collective
3. Identify deviations and improvements

---

## Query 13: Temporal Evolution 🆕

How has this code changed over time?

```rust
pub struct EvolutionResult {
    pub unit_id: u64,
    pub timeline: Vec<EvolutionEvent>,
    pub change_velocity: f32,  // Changes per month
    pub trend: Trend,
}

pub struct EvolutionEvent {
    pub timestamp: u64,
    pub commit_hash: String,
    pub change_type: ChangeType,
    pub author: Option<String>,
    pub message: Option<String>,
}

pub enum Trend {
    Stabilizing,  // Fewer changes over time
    Accelerating, // More changes over time
    Stable,       // Consistent low changes
    Volatile,     // Erratic changes
}
```

---

## Query 14: Stability Analysis 🆕

How risky is touching this code?

```rust
pub struct StabilityResult {
    pub unit_id: u64,
    pub stability_score: f32,      // 0.0 = very unstable, 1.0 = rock solid
    pub factors: Vec<StabilityFactor>,
    pub recommendation: StabilityRecommendation,
}

pub struct StabilityFactor {
    pub name: String,
    pub impact: f32,  // Contribution to score
    pub detail: String,
}

pub enum StabilityRecommendation {
    SafeToModify,
    ProceedWithCaution { reason: String },
    ConsiderRefactoring { suggestion: String },
    HighRisk { mitigation: String },
}
```

---

## Query 15: Coupling Detection 🆕

Find secretly coupled code.

```rust
pub struct CouplingParams {
    pub unit_id: Option<u64>,  // None = scan entire graph
    pub min_coupling: f32,     // Minimum coupling strength
    pub include_explicit: bool, // Include explicit dependencies
}

pub struct Coupling {
    pub unit_a: u64,
    pub unit_b: u64,
    pub coupling_type: CouplingType,
    pub strength: f32,  // How often they change together
    pub evidence: String,
}

pub enum CouplingType {
    Explicit,   // Direct call/import
    Temporal,   // Change together historically
    Hidden,     // No explicit link but change together
}
```

**Algorithm:**
1. Load change history for units
2. Calculate co-change frequency: times_changed_together / min(changes_a, changes_b)
3. Filter by threshold
4. Cross-reference with explicit edges to identify hidden couplings

---

## Query 16: Dead Code 🆕

Find unreachable code.

```rust
pub struct DeadCodeParams {
    pub scope: Scope,
    pub include_tests: bool,
    pub entry_points: Vec<u64>,  // Known entry points (main, exports)
}

pub enum Scope {
    All,
    Module(String),
    Language(Language),
}
```

**Algorithm:**
1. Identify entry points (exports, main functions, test entry)
2. Run reachability analysis from entry points
3. Any unit not reachable is potentially dead
4. Score by confidence (public but unreachable = suspicious)

---

## Query 17: Prophecy 🆕

Predict what will break based on patterns.

```rust
pub struct ProphecyParams {
    pub scope: Scope,
    pub time_horizon_days: u32,
    pub include_ecosystem: bool,  // Include library update predictions
}

pub struct ProphecyResult {
    pub predictions: Vec<Prediction>,
    pub ecosystem_alerts: Vec<EcosystemAlert>,
}

pub struct Prediction {
    pub unit_id: u64,
    pub prediction_type: PredictionType,
    pub confidence: f32,
    pub estimated_days: u32,
    pub reasoning: String,
    pub recommendation: String,
}

pub enum PredictionType {
    LikelyToBreak,
    NeedsRefactoring,
    TechDebtAccumulating,
    TestCoverageDecaying,
}

pub struct EcosystemAlert {
    pub library: String,
    pub current_version: String,
    pub alert_type: AlertType,
    pub recommendation: String,
}
```

**Algorithm:**
1. Analyze change patterns and identify unstable units
2. Apply pattern matching to historical incident data
3. Query collective for ecosystem alerts
4. Generate predictions with confidence scores

---

## Query 18: Concept Mapping 🆕

Where is a concept implemented across the codebase?

```rust
pub struct ConceptMap {
    pub concept: String,
    pub implementations: Vec<ConceptImpl>,
    pub related_concepts: Vec<String>,
}

pub struct ConceptImpl {
    pub unit_id: u64,
    pub role: ConceptRole,
    pub confidence: f32,
}

pub enum ConceptRole {
    Definition,    // The main definition
    Usage,         // Uses the concept
    Extension,     // Extends/customizes
    Test,          // Tests the concept
}
```

**Example:** "User authentication" → finds login routes, auth middleware, session handlers, user model, auth tests.

---

## Query 19: Migration Path 🆕

How to safely change X to Y?

```rust
pub struct MigrationParams {
    pub from_unit: u64,
    pub change_description: String,
    pub safety_level: SafetyLevel,
}

pub enum SafetyLevel {
    Conservative,  // Change nothing else
    Moderate,      // Update direct callers
    Aggressive,    // Update entire graph
}

pub struct MigrationPlan {
    pub steps: Vec<MigrationStep>,
    pub affected_units: Vec<u64>,
    pub risk_assessment: String,
    pub rollback_plan: String,
}

pub struct MigrationStep {
    pub order: u32,
    pub action: String,
    pub unit_id: u64,
    pub reason: String,
}
```

---

## Query 20: Test Gap 🆕

What recent changes lack test coverage?

```rust
pub struct TestGapParams {
    pub since: Option<u64>,     // Timestamp
    pub since_commit: Option<String>,
    pub min_complexity: u32,    // Only flag complex changes
}

pub struct TestGap {
    pub unit_id: u64,
    pub change_time: u64,
    pub complexity: u32,
    pub risk_score: f32,
    pub suggested_test: String,
}
```

---

## Query 21: Architectural Drift 🆕

Is the codebase diverging from its design?

```rust
pub struct DriftParams {
    pub baseline: Option<String>,  // Path to design doc or previous .acb
    pub rules: Vec<ArchRule>,      // Expected architectural rules
}

pub struct ArchRule {
    pub name: String,
    pub rule_type: RuleType,
    pub params: HashMap<String, String>,
}

pub enum RuleType {
    LayerDependency,   // Layer A should not depend on layer B
    ModuleBoundary,    // Module A should not access internals of B
    NamingConvention,  // Units matching X should follow pattern Y
    Cyclic,            // No cycles between these modules
}

pub struct DriftReport {
    pub violations: Vec<Violation>,
    pub drift_score: f32,
    pub trend: Trend,
    pub recommendations: Vec<String>,
}
```

---

## Query 22: Similarity

Find similar code (potential duplication or patterns).

```rust
pub struct SimilarityParams {
    pub unit_id: u64,
    pub top_k: usize,
    pub min_similarity: f32,
    pub same_repo_only: bool,
}

pub struct SimilarityMatch {
    pub unit_id: u64,
    pub similarity: f32,
    pub similarity_type: SimilarityType,
}

pub enum SimilarityType {
    Semantic,     // Similar meaning (vector)
    Structural,   // Similar structure (AST)
    Historical,   // Similar evolution pattern
}
```

---

## Query 23: Shortest Path

How are two symbols connected?

```rust
pub struct PathResult {
    pub from_id: u64,
    pub to_id: u64,
    pub path: Vec<PathHop>,
    pub distance: u32,
}

pub struct PathHop {
    pub unit_id: u64,
    pub edge_type: EdgeType,
}
```

**Algorithm:** Bidirectional BFS from both ends.

---

## Query 24: Hotspot Detection 🆕

Where do bugs cluster?

```rust
pub struct HotspotParams {
    pub scope: Scope,
    pub time_window_days: u32,
    pub min_incidents: u32,
}

pub struct Hotspot {
    pub unit_id: u64,
    pub incident_count: u32,
    pub fix_count: u32,
    pub churn_rate: f32,
    pub hotspot_score: f32,
    pub recommendation: String,
}
```

**Algorithm:**
1. Identify commits that are bug fixes (message analysis)
2. Count which units are touched by fix commits
3. Weight by recency and frequency
4. Rank by hotspot score

---

## Performance Requirements

All queries must complete within these bounds on a 100K-symbol graph:

| Query Type | Latency Target |
|------------|----------------|
| Symbol lookup | <1ms |
| Dependency/Call graph (depth 5) | <5ms |
| Impact analysis | <10ms |
| Semantic search | <50ms |
| Pattern match | <100ms |
| Prophecy | <200ms |
| Full graph scan queries | <500ms |

---

## Caching Strategy

- **Symbol index:** Always in memory after load
- **Edge traversal results:** LRU cache, 1000 entries
- **Feature vectors:** Memory-mapped, access on demand
- **Temporal data:** Loaded on first temporal query
- **Collective data:** Cached with 1-hour TTL
