# SPEC-FILE-FORMAT.md

> The exact binary layout of an `.acb` file. Every byte accounted for.

---

## File Layout Overview

```
Offset 0x0000:  ┌─────────────────────────────────┐
                │  FILE HEADER (128 bytes)         │
                │  Magic, version, counts, offsets │
                ├─────────────────────────────────┤
                │  CODE UNIT TABLE                 │
                │  (unit_count × 96 bytes)         │
                ├─────────────────────────────────┤
                │  EDGE TABLE                      │
                │  (edge_count × 40 bytes)         │
                ├─────────────────────────────────┤
                │  STRING POOL                     │
                │  (LZ4-compressed, variable)      │
                ├─────────────────────────────────┤
                │  FEATURE VECTOR BLOCK            │
                │  (unit_count × dim × 4 bytes)    │
                ├─────────────────────────────────┤
                │  TEMPORAL BLOCK                  │
                │  (change history, compressed)    │
                ├─────────────────────────────────┤
                │  INDEX BLOCK                     │
                │  (multiple indexes, variable)    │
                └─────────────────────────────────┘
```

---

## Section 1: File Header (128 bytes)

```
Offset  Size   Type       Field                    Description
------  ----   ----       -----                    -----------
0x00    4      [u8;4]     magic                    Must be [0x41,0x43,0x44,0x42] ("ACDB")
0x04    4      u32        version                  Format version (currently 1)
0x08    4      u32        dimension                Feature vector dimensionality (default 256)
0x0C    4      u32        language_count           Number of languages in this file
0x10    8      u64        unit_count               Number of code units
0x18    8      u64        edge_count               Number of edges
0x20    8      u64        unit_table_offset        Byte offset to code unit table
0x28    8      u64        edge_table_offset        Byte offset to edge table
0x30    8      u64        string_pool_offset       Byte offset to string pool
0x38    8      u64        feature_vec_offset       Byte offset to feature vectors
0x40    8      u64        temporal_offset          Byte offset to temporal block
0x48    8      u64        index_offset             Byte offset to index block
0x50    32     [u8;32]    repo_hash                Blake3 hash of repo root path
0x70    8      u64        compiled_at              Compilation timestamp (microseconds)
0x78    16     [u8;16]    _reserved                Reserved for future use (zeros)
                                                    Total: 128 bytes
```

### Validation on Read

1. Check magic bytes. If not "ACDB", return `AcbError::InvalidMagic`.
2. Check version. If not 1, return `AcbError::UnsupportedVersion`.
3. Check all offsets are within file size. If not, return `AcbError::Truncated`.
4. Check unit_table_offset == 128 (immediately after header).

---

## Section 2: Code Unit Table

Starts at `unit_table_offset` (always byte 128).

Each code unit record is 96 bytes:

```
Offset  Size   Type       Field
------  ----   ----       -----
+0x00   8      u64        id
+0x08   1      u8         unit_type
+0x09   1      u8         language
+0x0A   1      u8         visibility
+0x0B   1      u8         flags (bit 0: is_async, bit 1: is_generator)
+0x0C   4      u32        complexity
+0x10   8      u64        name_offset (relative to string_pool_offset)
+0x18   4      u32        name_len
+0x1C   4      u32        _pad1
+0x20   8      u64        qualified_name_offset
+0x28   4      u32        qualified_name_len
+0x2C   4      u32        _pad2
+0x30   8      u64        path_offset
+0x38   4      u32        path_len
+0x3C   4      u32        span_start_line
+0x40   4      u32        span_start_col
+0x44   4      u32        span_end_line
+0x48   4      u32        span_end_col
+0x4C   4      u32        change_count
+0x50   8      u64        created_at
+0x58   8      u64        last_modified
+0x60   4      u32        stability_score (f32 as bits)
+0x64   4      u32        edge_count
+0x68   8      u64        edge_offset (relative to edge_table_offset)
                          Total: 96 bytes (0x70 - 0x00 = 112... recalculate)
```

**Corrected 96-byte layout:**

```
Offset  Size   Type       Field
------  ----   ----       -----
+0x00   8      u64        id
+0x08   1      u8         unit_type
+0x09   1      u8         language
+0x0A   1      u8         visibility
+0x0B   1      u8         flags
+0x0C   4      u32        complexity
+0x10   4      u32        name_offset_lo       (offset split for alignment)
+0x14   4      u32        name_offset_hi
+0x18   2      u16        name_len
+0x1A   2      u16        qname_len
+0x1C   4      u32        qname_offset_lo
+0x20   4      u32        qname_offset_hi
+0x24   4      u32        path_offset_lo
+0x28   4      u32        path_offset_hi
+0x2C   2      u16        path_len
+0x2E   2      u16        _pad1
+0x30   4      u32        span_start_line
+0x34   4      u32        span_start_col
+0x38   4      u32        span_end_line
+0x3C   4      u32        span_end_col
+0x40   8      u64        created_at
+0x48   8      u64        last_modified
+0x50   4      u32        change_count
+0x54   4      u32        stability_score_bits
+0x58   4      u32        edge_count
+0x5C   4      u32        edge_offset_lo
+0x60   4      u32        edge_offset_hi
+0x64   4      u32        _pad2
                          Total: 104 bytes
```

**Final corrected 96-byte layout (compact):**

```
struct CodeUnitRecord {        // 96 bytes total
    id: u64,                   // 0x00: 8
    unit_type: u8,             // 0x08: 1
    language: u8,              // 0x09: 1  
    visibility: u8,            // 0x0A: 1
    flags: u8,                 // 0x0B: 1
    complexity: u16,           // 0x0C: 2
    name_len: u16,             // 0x0E: 2
    name_offset: u32,          // 0x10: 4
    qname_len: u16,            // 0x14: 2
    path_len: u16,             // 0x16: 2
    qname_offset: u32,         // 0x18: 4
    path_offset: u32,          // 0x1C: 4
    span_start_line: u32,      // 0x20: 4
    span_start_col: u16,       // 0x24: 2
    span_end_col: u16,         // 0x26: 2
    span_end_line: u32,        // 0x28: 4
    change_count: u16,         // 0x2C: 2
    stability_x100: u16,       // 0x2E: 2 (stability * 100, 0-10000)
    created_at: u64,           // 0x30: 8
    last_modified: u64,        // 0x38: 8
    edge_count: u32,           // 0x40: 4
    edge_offset: u32,          // 0x44: 4
    content_hash: [u8; 32],    // 0x48: 32
}                              // Total: 0x68 = 104 bytes
```

Let me redo this properly as exactly 96 bytes:

```
struct CodeUnitRecord {        // 96 bytes
    // Identity: 16 bytes
    id: u64,                   // 8
    unit_type: u8,             // 1
    language: u8,              // 1  
    visibility: u8,            // 1
    flags: u8,                 // 1
    complexity: u16,           // 2
    _pad1: u16,                // 2
    
    // String references: 24 bytes
    name_offset: u32,          // 4 (into string pool)
    name_len: u16,             // 2
    qname_offset: u32,         // 4
    qname_len: u16,            // 2
    path_offset: u32,          // 4
    path_len: u16,             // 2
    _pad2: [u8; 6],            // 6
    
    // Source location: 16 bytes
    span_start_line: u32,      // 4
    span_start_col: u16,       // 2
    span_end_line: u32,        // 4
    span_end_col: u16,         // 2
    _pad3: u32,                // 4
    
    // Temporal: 24 bytes
    created_at: u64,           // 8
    last_modified: u64,        // 8
    change_count: u32,         // 4
    stability_x100: u16,       // 2 (0-10000 = 0.00-100.00%)
    _pad4: u16,                // 2
    
    // Graph: 16 bytes
    edge_offset: u64,          // 8
    edge_count: u32,           // 4
    _pad5: u32,                // 4
}
// 16 + 24 + 16 + 24 + 16 = 96 bytes ✓
```

Code units are stored in order of their `id`. Unit with id=0 is at position 0 in the table. This enables O(1) access: `unit_table_offset + (id × 96)`.

---

## Section 3: Edge Table

Starts at `edge_table_offset`.

Edges are grouped by source unit. All edges from the same source are contiguous.

Each edge record is 40 bytes:

```
struct EdgeRecord {            // 40 bytes
    source_id: u64,            // 8
    target_id: u64,            // 8
    edge_type: u8,             // 1
    _pad1: [u8; 3],            // 3
    weight_bits: u32,          // 4 (f32 as bits)
    created_at: u64,           // 8
    context: u32,              // 4
    _pad2: u32,                // 4
}
// Total: 40 bytes
```

A unit's edges start at `edge_table_offset + unit.edge_offset` and there are `unit.edge_count` of them.

---

## Section 4: String Pool

Starts at `string_pool_offset`.

Variable-length section containing all strings (names, qualified names, paths, signatures, docs).

Format:
```
[uncompressed_size: u64]
[compressed_data: LZ4 block]
```

The compressed data, when decompressed, is a sequence of UTF-8 strings. Each string referenced by (offset, len) from code unit records.

### Compression
- Use LZ4 block compression
- `lz4_flex::compress_prepend_size()` / `decompress_size_prepended()`
- Typical compression ratio: 3-5x for code identifiers

### String Access
1. Decompress string pool into memory (or mmap the decompressed view)
2. For unit N: `&pool[unit.name_offset..unit.name_offset + unit.name_len]`

---

## Section 5: Feature Vector Block

Starts at `feature_vec_offset`.

Dense array of feature vectors, one per code unit:

```
[vec_0: dim × f32][vec_1: dim × f32]...[vec_N: dim × f32]
```

Size: `unit_count × dimension × 4` bytes.

Vector for unit N starts at: `feature_vec_offset + (N × dimension × 4)`

All floats are little-endian IEEE 754.

---

## Section 6: Temporal Block

Starts at `temporal_offset`.

Contains compressed change history and coupling data.

Format:
```
[history_size: u64]
[history_data: LZ4 compressed]
[coupling_count: u64]
[coupling_data: variable]
```

### History Data (when decompressed)
```
For each unit with history:
  [unit_id: u64]
  [entry_count: u32]
  For each entry:
    [timestamp: u64]
    [commit_hash: [u8; 20]] (first 20 bytes of git SHA)
    [change_type: u8] (0=add, 1=modify, 2=delete, 3=rename)
```

### Coupling Data
```
For each coupling:
  [unit_a: u64]
  [unit_b: u64]
  [co_change_count: u32]
  [total_changes_a: u32]
  [total_changes_b: u32]
  [confidence: f32] (co_change_count / min(total_a, total_b))
```

---

## Section 7: Index Block

Starts at `index_offset`. Runs to end of file.

Contains multiple indexes, each prefixed with type and size:

```
[index_type: u32][index_size: u64][index_data: variable]
[index_type: u32][index_size: u64][index_data: variable]
...
[0xFFFFFFFF] (end marker)
```

### Index Types

| Type | Description |
|------|-------------|
| 0x01 | Symbol name index (name → unit_id) |
| 0x02 | Type index (CodeUnitType → [unit_ids]) |
| 0x03 | Path index (file_path → [unit_ids]) |
| 0x04 | Language index (Language → [unit_ids]) |
| 0x05 | Qualified name prefix index (for completion) |

### Symbol Name Index Format
```
[bucket_count: u32]
[buckets: bucket_count × (offset: u32, count: u32)]
[entries: sorted by hash]
  For each entry:
    [name_hash: u64]
    [unit_id: u64]
```

Hash function: `blake3::hash(name.to_lowercase().as_bytes())[0..8]`

### Type/Language Index Format
```
For each type/language:
  [type_or_lang: u8]
  [count: u64]
  [unit_ids: count × u64]
```

---

## Byte Order

**Everything is little-endian.** Use `to_le_bytes()` and `from_le_bytes()`.

---

## File Extension

`.acb` — AgenticCodeBase

---

## Writer Implementation

```rust
pub struct AcbWriter {
    dimension: usize,
}

impl AcbWriter {
    pub fn new(dimension: usize) -> Self;
    
    pub fn write_to_file(&self, graph: &CodeGraph, path: &Path) -> AcbResult<()>;
    
    pub fn write_to(&self, graph: &CodeGraph, writer: &mut impl Write) -> AcbResult<()>;
}
```

### Write Process
1. Collect all units, sorted by ID
2. Collect all edges, sorted by source_id then target_id
3. Build string pool, record offsets
4. Compress string pool
5. Calculate all section offsets
6. Write header
7. Write unit table
8. Write edge table
9. Write compressed string pool
10. Write feature vectors
11. Write temporal block
12. Build and write indexes

---

## Reader Implementation

```rust
pub struct AcbReader;

impl AcbReader {
    pub fn read_from_file(path: &Path) -> AcbResult<CodeGraph>;
    
    pub fn read_from(reader: &mut impl Read) -> AcbResult<CodeGraph>;
    
    /// Memory-map a file for zero-copy access
    pub fn mmap_file(path: &Path) -> AcbResult<MappedCodeGraph>;
}
```

### Read Process
1. Read and validate header
2. Read unit table → create CodeUnit structs
3. Read edge table → create Edge structs
4. Read and decompress string pool
5. Attach strings to units
6. Read feature vectors
7. Read temporal data (optional, on-demand)
8. Read indexes → populate lookup structures
9. Return CodeGraph

---

## Versioning

- Version 1: Initial release
- Version changes require new magic OR version bump
- Readers must reject unknown versions
- Writers must write current version only

---

## Size Estimates

For a 100K symbol codebase:

| Section | Calculation | Size |
|---------|-------------|------|
| Header | Fixed | 128 B |
| Unit table | 100K × 96 | ~9.6 MB |
| Edge table | 500K × 40 | ~20 MB |
| String pool | ~50 chars/symbol avg, 3x compression | ~1.7 MB |
| Feature vectors | 100K × 256 × 4 | ~100 MB |
| Temporal | Varies | ~5-20 MB |
| Indexes | ~10% of unit table | ~1 MB |
| **Total** | | **~130-140 MB** |

Without feature vectors: ~32-40 MB (vectors are optional/pluggable).
