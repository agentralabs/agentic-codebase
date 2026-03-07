# SPEC-INDEXES.md

> Fast lookup structures for every query pattern. O(1) where possible, O(log n) otherwise.

---

## Overview

Indexes accelerate queries by pre-computing access paths. Each index is:
- **Independent**: Knows nothing about other indexes
- **Incrementally updateable**: Add/remove without full rebuild
- **Serializable**: Stored in the .acb file
- **Memory-mappable**: Can be accessed directly from disk

---

## Index Types

| Index | Purpose | Lookup Complexity |
|-------|---------|-------------------|
| Symbol Name Index | Find by name | O(1) hash lookup |
| Type Index | Filter by CodeUnitType | O(1) per type |
| Path Index | Filter by file path | O(log n) prefix |
| Language Index | Filter by language | O(1) per language |
| Qualified Name Index | Autocomplete, prefix match | O(log n) |
| Embedding Index | Similarity search | O(n) brute force |

---

## Symbol Name Index

```rust
// src/index/symbol_index.rs

/// Hash-based index for symbol name lookup
pub struct SymbolIndex {
    /// Number of hash buckets
    bucket_count: usize,
    /// Bucket array: bucket[hash % bucket_count] = (offset, count)
    buckets: Vec<BucketEntry>,
    /// Sorted entries by hash for each bucket
    entries: Vec<SymbolEntry>,
}

#[derive(Clone, Copy)]
struct BucketEntry {
    /// Offset into entries array
    offset: u32,
    /// Number of entries in this bucket
    count: u32,
}

#[derive(Clone, Copy)]
struct SymbolEntry {
    /// Hash of the symbol name (lowercase)
    name_hash: u64,
    /// Code unit ID
    unit_id: u64,
}

impl SymbolIndex {
    /// Create a new index with the given bucket count
    pub fn new(bucket_count: usize) -> Self {
        Self {
            bucket_count,
            buckets: vec![BucketEntry { offset: 0, count: 0 }; bucket_count],
            entries: Vec::new(),
        }
    }
    
    /// Build index from code units
    pub fn build(units: &[CodeUnit]) -> Self {
        // Choose bucket count based on unit count (load factor ~0.7)
        let bucket_count = ((units.len() as f64 * 1.4) as usize).next_power_of_two();
        let mut index = Self::new(bucket_count);
        
        // Group units by bucket
        let mut by_bucket: Vec<Vec<SymbolEntry>> = vec![Vec::new(); bucket_count];
        
        for unit in units {
            let hash = Self::hash_name(&unit.name);
            let bucket = (hash as usize) % bucket_count;
            by_bucket[bucket].push(SymbolEntry {
                name_hash: hash,
                unit_id: unit.id,
            });
        }
        
        // Sort each bucket by hash and flatten
        let mut offset = 0u32;
        for (i, bucket_entries) in by_bucket.iter_mut().enumerate() {
            bucket_entries.sort_by_key(|e| e.name_hash);
            
            index.buckets[i] = BucketEntry {
                offset,
                count: bucket_entries.len() as u32,
            };
            
            index.entries.extend(bucket_entries.iter());
            offset += bucket_entries.len() as u32;
        }
        
        index
    }
    
    /// Look up units by exact name
    pub fn lookup(&self, name: &str) -> Vec<u64> {
        let hash = Self::hash_name(name);
        let bucket_idx = (hash as usize) % self.bucket_count;
        let bucket = &self.buckets[bucket_idx];
        
        let start = bucket.offset as usize;
        let end = start + bucket.count as usize;
        
        // Binary search within bucket
        let entries = &self.entries[start..end];
        let mut results = Vec::new();
        
        // Find first match
        if let Ok(pos) = entries.binary_search_by_key(&hash, |e| e.name_hash) {
            // Collect all matches (there may be collisions)
            let mut i = pos;
            while i < entries.len() && entries[i].name_hash == hash {
                results.push(entries[i].unit_id);
                i += 1;
            }
            // Check backwards too
            let mut i = pos;
            while i > 0 {
                i -= 1;
                if entries[i].name_hash == hash {
                    results.push(entries[i].unit_id);
                } else {
                    break;
                }
            }
        }
        
        results
    }
    
    /// Look up by prefix (returns candidates, caller must filter)
    pub fn lookup_prefix(&self, prefix: &str) -> Vec<u64> {
        // For prefix matching, we need to scan more broadly
        // This is less efficient but still faster than full scan
        let prefix_lower = prefix.to_lowercase();
        
        // We'd need a separate trie for efficient prefix matching
        // For now, fall back to type index + filter
        Vec::new()
    }
    
    fn hash_name(name: &str) -> u64 {
        let lower = name.to_lowercase();
        let hash = blake3::hash(lower.as_bytes());
        u64::from_le_bytes(hash.as_bytes()[0..8].try_into().unwrap())
    }
    
    /// Serialize to bytes for .acb file
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Bucket count
        bytes.extend(&(self.bucket_count as u32).to_le_bytes());
        
        // Buckets
        for bucket in &self.buckets {
            bytes.extend(&bucket.offset.to_le_bytes());
            bytes.extend(&bucket.count.to_le_bytes());
        }
        
        // Entries
        for entry in &self.entries {
            bytes.extend(&entry.name_hash.to_le_bytes());
            bytes.extend(&entry.unit_id.to_le_bytes());
        }
        
        bytes
    }
    
    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> AcbResult<Self> {
        let mut offset = 0;
        
        let bucket_count = u32::from_le_bytes(
            bytes[offset..offset+4].try_into().unwrap()
        ) as usize;
        offset += 4;
        
        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            let bucket_offset = u32::from_le_bytes(
                bytes[offset..offset+4].try_into().unwrap()
            );
            offset += 4;
            let count = u32::from_le_bytes(
                bytes[offset..offset+4].try_into().unwrap()
            );
            offset += 4;
            buckets.push(BucketEntry { offset: bucket_offset, count });
        }
        
        let entry_count = (bytes.len() - offset) / 16;
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            let name_hash = u64::from_le_bytes(
                bytes[offset..offset+8].try_into().unwrap()
            );
            offset += 8;
            let unit_id = u64::from_le_bytes(
                bytes[offset..offset+8].try_into().unwrap()
            );
            offset += 8;
            entries.push(SymbolEntry { name_hash, unit_id });
        }
        
        Ok(Self { bucket_count, buckets, entries })
    }
}
```

---

## Type Index

```rust
// src/index/type_index.rs

/// Index for filtering by CodeUnitType
pub struct TypeIndex {
    /// For each type, list of unit IDs
    by_type: [Vec<u64>; 13], // 13 CodeUnitType variants
}

impl TypeIndex {
    pub fn new() -> Self {
        Self {
            by_type: Default::default(),
        }
    }
    
    pub fn build(units: &[CodeUnit]) -> Self {
        let mut index = Self::new();
        
        for unit in units {
            let type_idx = unit.unit_type as usize;
            index.by_type[type_idx].push(unit.id);
        }
        
        // Sort each list for binary search
        for list in &mut index.by_type {
            list.sort_unstable();
        }
        
        index
    }
    
    /// Get all units of a given type
    pub fn get(&self, unit_type: CodeUnitType) -> &[u64] {
        &self.by_type[unit_type as usize]
    }
    
    /// Get units matching any of the given types
    pub fn get_any(&self, types: &[CodeUnitType]) -> Vec<u64> {
        let mut result = Vec::new();
        for t in types {
            result.extend(self.get(*t));
        }
        result.sort_unstable();
        result.dedup();
        result
    }
    
    /// Check if a unit is of a given type (O(log n))
    pub fn contains(&self, unit_type: CodeUnitType, unit_id: u64) -> bool {
        self.by_type[unit_type as usize]
            .binary_search(&unit_id)
            .is_ok()
    }
    
    /// Count units of each type
    pub fn counts(&self) -> [(CodeUnitType, usize); 13] {
        [
            (CodeUnitType::Module, self.by_type[0].len()),
            (CodeUnitType::Symbol, self.by_type[1].len()),
            (CodeUnitType::Type, self.by_type[2].len()),
            (CodeUnitType::Function, self.by_type[3].len()),
            (CodeUnitType::Parameter, self.by_type[4].len()),
            (CodeUnitType::Import, self.by_type[5].len()),
            (CodeUnitType::Test, self.by_type[6].len()),
            (CodeUnitType::Doc, self.by_type[7].len()),
            (CodeUnitType::Config, self.by_type[8].len()),
            (CodeUnitType::Pattern, self.by_type[9].len()),
            (CodeUnitType::Trait, self.by_type[10].len()),
            (CodeUnitType::Impl, self.by_type[11].len()),
            (CodeUnitType::Macro, self.by_type[12].len()),
        ]
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        for list in &self.by_type {
            bytes.extend(&(list.len() as u64).to_le_bytes());
            for &id in list {
                bytes.extend(&id.to_le_bytes());
            }
        }
        
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> AcbResult<Self> {
        let mut index = Self::new();
        let mut offset = 0;
        
        for i in 0..13 {
            let count = u64::from_le_bytes(
                bytes[offset..offset+8].try_into().unwrap()
            ) as usize;
            offset += 8;
            
            index.by_type[i] = Vec::with_capacity(count);
            for _ in 0..count {
                let id = u64::from_le_bytes(
                    bytes[offset..offset+8].try_into().unwrap()
                );
                offset += 8;
                index.by_type[i].push(id);
            }
        }
        
        Ok(index)
    }
}
```

---

## Path Index

```rust
// src/index/path_index.rs

use std::collections::BTreeMap;

/// Index for filtering by file path with prefix matching
pub struct PathIndex {
    /// Sorted map from path string to unit IDs
    by_path: BTreeMap<String, Vec<u64>>,
    /// Reverse index: unit_id -> path
    unit_to_path: HashMap<u64, String>,
}

impl PathIndex {
    pub fn new() -> Self {
        Self {
            by_path: BTreeMap::new(),
            unit_to_path: HashMap::new(),
        }
    }
    
    pub fn build(units: &[CodeUnit]) -> Self {
        let mut index = Self::new();
        
        for unit in units {
            let path = unit.file_path.to_string_lossy().to_string();
            index.by_path
                .entry(path.clone())
                .or_default()
                .push(unit.id);
            index.unit_to_path.insert(unit.id, path);
        }
        
        index
    }
    
    /// Get all units in a specific file
    pub fn get_file(&self, path: &str) -> &[u64] {
        self.by_path.get(path).map(|v| v.as_slice()).unwrap_or(&[])
    }
    
    /// Get all units in files matching a prefix (e.g., "src/payments/")
    pub fn get_prefix(&self, prefix: &str) -> Vec<u64> {
        let mut result = Vec::new();
        
        for (path, ids) in self.by_path.range(prefix.to_string()..) {
            if path.starts_with(prefix) {
                result.extend(ids);
            } else {
                break;
            }
        }
        
        result
    }
    
    /// Get the path for a unit
    pub fn get_path(&self, unit_id: u64) -> Option<&str> {
        self.unit_to_path.get(&unit_id).map(|s| s.as_str())
    }
    
    /// List all unique paths
    pub fn all_paths(&self) -> impl Iterator<Item = &str> {
        self.by_path.keys().map(|s| s.as_str())
    }
    
    /// Count units per path
    pub fn path_counts(&self) -> impl Iterator<Item = (&str, usize)> {
        self.by_path.iter().map(|(k, v)| (k.as_str(), v.len()))
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Number of paths
        bytes.extend(&(self.by_path.len() as u64).to_le_bytes());
        
        for (path, ids) in &self.by_path {
            // Path length and bytes
            let path_bytes = path.as_bytes();
            bytes.extend(&(path_bytes.len() as u32).to_le_bytes());
            bytes.extend(path_bytes);
            
            // ID count and IDs
            bytes.extend(&(ids.len() as u32).to_le_bytes());
            for &id in ids {
                bytes.extend(&id.to_le_bytes());
            }
        }
        
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> AcbResult<Self> {
        let mut index = Self::new();
        let mut offset = 0;
        
        let path_count = u64::from_le_bytes(
            bytes[offset..offset+8].try_into().unwrap()
        ) as usize;
        offset += 8;
        
        for _ in 0..path_count {
            let path_len = u32::from_le_bytes(
                bytes[offset..offset+4].try_into().unwrap()
            ) as usize;
            offset += 4;
            
            let path = String::from_utf8_lossy(&bytes[offset..offset+path_len]).to_string();
            offset += path_len;
            
            let id_count = u32::from_le_bytes(
                bytes[offset..offset+4].try_into().unwrap()
            ) as usize;
            offset += 4;
            
            let mut ids = Vec::with_capacity(id_count);
            for _ in 0..id_count {
                let id = u64::from_le_bytes(
                    bytes[offset..offset+8].try_into().unwrap()
                );
                offset += 8;
                ids.push(id);
            }
            
            for &id in &ids {
                index.unit_to_path.insert(id, path.clone());
            }
            index.by_path.insert(path, ids);
        }
        
        Ok(index)
    }
}
```

---

## Language Index

```rust
// src/index/language_index.rs

/// Index for filtering by programming language
pub struct LanguageIndex {
    /// For each language, list of unit IDs
    by_language: [Vec<u64>; 6], // 5 languages + Unknown
}

impl LanguageIndex {
    pub fn new() -> Self {
        Self {
            by_language: Default::default(),
        }
    }
    
    pub fn build(units: &[CodeUnit]) -> Self {
        let mut index = Self::new();
        
        for unit in units {
            let lang_idx = unit.language as usize;
            if lang_idx < 6 {
                index.by_language[lang_idx].push(unit.id);
            }
        }
        
        for list in &mut index.by_language {
            list.sort_unstable();
        }
        
        index
    }
    
    pub fn get(&self, language: Language) -> &[u64] {
        let idx = language as usize;
        if idx < 6 {
            &self.by_language[idx]
        } else {
            &[]
        }
    }
    
    pub fn counts(&self) -> [(Language, usize); 6] {
        [
            (Language::Python, self.by_language[0].len()),
            (Language::Rust, self.by_language[1].len()),
            (Language::TypeScript, self.by_language[2].len()),
            (Language::JavaScript, self.by_language[3].len()),
            (Language::Go, self.by_language[4].len()),
            (Language::Unknown, self.by_language[5].len()),
        ]
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        for list in &self.by_language {
            bytes.extend(&(list.len() as u64).to_le_bytes());
            for &id in list {
                bytes.extend(&id.to_le_bytes());
            }
        }
        
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> AcbResult<Self> {
        let mut index = Self::new();
        let mut offset = 0;
        
        for i in 0..6 {
            let count = u64::from_le_bytes(
                bytes[offset..offset+8].try_into().unwrap()
            ) as usize;
            offset += 8;
            
            index.by_language[i] = Vec::with_capacity(count);
            for _ in 0..count {
                let id = u64::from_le_bytes(
                    bytes[offset..offset+8].try_into().unwrap()
                );
                offset += 8;
                index.by_language[i].push(id);
            }
        }
        
        Ok(index)
    }
}
```

---

## Embedding Index

```rust
// src/index/embedding_index.rs

/// Index for similarity search over feature vectors
/// Uses brute-force cosine similarity (fast enough for 100K vectors)
pub struct EmbeddingIndex {
    /// Dimension of vectors
    dimension: usize,
    /// Number of vectors
    count: usize,
    /// Precomputed norms for each vector
    norms: Vec<f32>,
}

impl EmbeddingIndex {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            count: 0,
            norms: Vec::new(),
        }
    }
    
    pub fn build(units: &[CodeUnit], dimension: usize) -> Self {
        let mut index = Self::new(dimension);
        index.count = units.len();
        index.norms = units.iter()
            .map(|u| Self::compute_norm(&u.feature_vec))
            .collect();
        index
    }
    
    /// Search for similar vectors
    /// Returns (unit_index, similarity) pairs sorted by similarity desc
    pub fn search(
        &self,
        query: &[f32],
        vectors: &[f32], // Flat array: count * dimension
        top_k: usize,
        min_similarity: f32,
    ) -> Vec<(usize, f32)> {
        let query_norm = Self::compute_norm(query);
        if query_norm == 0.0 {
            return Vec::new();
        }
        
        let mut results: Vec<(usize, f32)> = Vec::with_capacity(self.count);
        
        for i in 0..self.count {
            let vec_norm = self.norms[i];
            if vec_norm == 0.0 {
                continue;
            }
            
            // Compute dot product
            let offset = i * self.dimension;
            let vec = &vectors[offset..offset + self.dimension];
            
            let dot: f32 = query.iter()
                .zip(vec.iter())
                .map(|(a, b)| a * b)
                .sum();
            
            let similarity = dot / (query_norm * vec_norm);
            
            if similarity >= min_similarity {
                results.push((i, similarity));
            }
        }
        
        // Sort by similarity descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Take top_k
        results.truncate(top_k);
        results
    }
    
    fn compute_norm(vec: &[f32]) -> f32 {
        vec.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend(&(self.dimension as u32).to_le_bytes());
        bytes.extend(&(self.count as u64).to_le_bytes());
        
        for &norm in &self.norms {
            bytes.extend(&norm.to_le_bytes());
        }
        
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> AcbResult<Self> {
        let mut offset = 0;
        
        let dimension = u32::from_le_bytes(
            bytes[offset..offset+4].try_into().unwrap()
        ) as usize;
        offset += 4;
        
        let count = u64::from_le_bytes(
            bytes[offset..offset+8].try_into().unwrap()
        ) as usize;
        offset += 8;
        
        let mut norms = Vec::with_capacity(count);
        for _ in 0..count {
            let norm = f32::from_le_bytes(
                bytes[offset..offset+4].try_into().unwrap()
            );
            offset += 4;
            norms.push(norm);
        }
        
        Ok(Self { dimension, count, norms })
    }
}
```

---

## Index Manager

```rust
// src/index/mod.rs

/// Manages all indexes and provides unified query interface
pub struct IndexManager {
    pub symbol_index: SymbolIndex,
    pub type_index: TypeIndex,
    pub path_index: PathIndex,
    pub language_index: LanguageIndex,
    pub embedding_index: EmbeddingIndex,
}

impl IndexManager {
    pub fn build(units: &[CodeUnit], dimension: usize) -> Self {
        Self {
            symbol_index: SymbolIndex::build(units),
            type_index: TypeIndex::build(units),
            path_index: PathIndex::build(units),
            language_index: LanguageIndex::build(units),
            embedding_index: EmbeddingIndex::build(units, dimension),
        }
    }
    
    /// Find units matching multiple criteria (intersection)
    pub fn find(
        &self,
        name: Option<&str>,
        types: &[CodeUnitType],
        languages: &[Language],
        path_prefix: Option<&str>,
    ) -> Vec<u64> {
        let mut candidates: Option<HashSet<u64>> = None;
        
        // Apply each filter
        if let Some(name) = name {
            let ids: HashSet<u64> = self.symbol_index.lookup(name).into_iter().collect();
            candidates = Some(match candidates {
                None => ids,
                Some(c) => c.intersection(&ids).copied().collect(),
            });
        }
        
        if !types.is_empty() {
            let ids: HashSet<u64> = self.type_index.get_any(types).into_iter().collect();
            candidates = Some(match candidates {
                None => ids,
                Some(c) => c.intersection(&ids).copied().collect(),
            });
        }
        
        if !languages.is_empty() {
            let mut ids = HashSet::new();
            for lang in languages {
                ids.extend(self.language_index.get(*lang));
            }
            candidates = Some(match candidates {
                None => ids,
                Some(c) => c.intersection(&ids).copied().collect(),
            });
        }
        
        if let Some(prefix) = path_prefix {
            let ids: HashSet<u64> = self.path_index.get_prefix(prefix).into_iter().collect();
            candidates = Some(match candidates {
                None => ids,
                Some(c) => c.intersection(&ids).copied().collect(),
            });
        }
        
        candidates.map(|c| c.into_iter().collect()).unwrap_or_default()
    }
}
```

---

## Memory-Mapped Index Access

```rust
// src/index/mmap.rs

use memmap2::Mmap;

/// Memory-mapped access to indexes without loading into heap
pub struct MappedIndexes {
    mmap: Mmap,
    symbol_offset: usize,
    symbol_len: usize,
    type_offset: usize,
    type_len: usize,
    // ... other offsets
}

impl MappedIndexes {
    pub fn open(mmap: Mmap, header: &FileHeader) -> AcbResult<Self> {
        // Parse index block header to find individual index offsets
        todo!()
    }
    
    /// Access symbol index directly from mmap
    pub fn symbol_lookup(&self, name: &str) -> Vec<u64> {
        // Direct binary search on mmap'd data
        todo!()
    }
}
```

---

## Performance Targets

| Operation | Target |
|-----------|--------|
| Symbol lookup (exact) | <100μs |
| Type index filter | <10μs |
| Path prefix filter | <1ms |
| Language filter | <10μs |
| Similarity search (100K × 256-dim) | <50ms |
| Index build (100K units) | <500ms |
| Index serialization | <100ms |
