# SPEC-TESTS.md

> Every test that must pass. Organized by phase. 300+ tests minimum. Do not proceed to the next phase until all tests in the current phase pass.

---

## Test Philosophy

1. **Tests prove the spec works**, not that the code runs
2. **Test behavior, not implementation**
3. **Every query type has at least 5 tests**
4. **Every edge case from SPEC-EDGE-CASES must have a test**
5. **Snapshot tests for parse output stability**
6. **Benchmark tests run with `--release`**

---

## Phase 1 Tests: Foundation

File: `tests/phase1_foundation.rs`

### Data Structure Tests

```
test_code_unit_type_roundtrip
  - Create each CodeUnitType variant
  - Convert to u8, convert back
  - Assert equality

test_code_unit_type_invalid
  - CodeUnitType::from_u8(255) returns None
  - CodeUnitType::from_u8(13) returns None

test_edge_type_roundtrip
  - All 18 edge types round-trip correctly

test_language_detection_python
  - Language::from_extension("py") == Python
  - Language::from_extension("pyi") == Python

test_language_detection_all
  - Test all supported extensions

test_span_creation
  - Span::new creates valid span
  - Span::point creates single-point span

test_span_contains
  - Point inside span returns true
  - Point outside returns false
  - Edge cases: start/end boundaries

test_code_unit_creation
  - Create CodeUnit with all fields
  - Verify all fields accessible

test_code_unit_builder
  - Builder pattern works
  - Optional fields have defaults

test_edge_creation
  - Create edge with all fields
  - Weight clamping works

test_self_edge_detection
  - Utility to detect self-edges
```

### File Header Tests

```
test_header_write_read_roundtrip
  - Create header with known values
  - Write to Vec<u8>
  - Read back
  - Assert all fields match

test_header_size_is_128_bytes
  - Write header
  - Assert length == 128

test_header_magic_validation
  - Corrupt magic bytes
  - Read returns InvalidMagic

test_header_version_validation
  - Write header with version 99
  - Read returns UnsupportedVersion(99)

test_header_little_endian
  - Write known values
  - Verify byte order
```

### CodeGraph Basic Tests

```
test_empty_graph
  - New graph has 0 units, 0 edges

test_add_single_unit
  - Add one unit
  - unit_count() == 1
  - get_unit(0) returns unit

test_add_multiple_units
  - Add 100 units
  - All accessible by ID

test_add_edge
  - Add 2 units, add edge
  - edge_count() == 1
  - edges_from(source) returns edge

test_edge_validation
  - Edge to non-existent unit fails
  - Self-edge fails

test_duplicate_edge
  - Adding same edge twice: define behavior
```

### File I/O Tests

```
test_write_empty_graph
  - Write empty graph
  - File is valid, header correct

test_write_read_single_unit
  - Write graph with 1 unit
  - Read back
  - Unit matches

test_write_read_with_edges
  - Write graph with units and edges
  - Read back
  - All data matches

test_write_read_all_unit_types
  - One unit of each type
  - All survive roundtrip

test_write_read_all_edge_types
  - One edge of each type
  - All survive roundtrip

test_string_pool_compression
  - Write graph with long strings
  - Verify compression happened
  - Read back, strings intact

test_unicode_strings
  - Names with Unicode
  - Paths with Unicode
  - All survive roundtrip

test_large_graph_write_read
  - 10K units, 50K edges
  - Write, read, verify

test_corrupted_file_detection
  - Truncate file
  - Read returns Truncated

test_wrong_magic_detection
  - Modify magic bytes
  - Read returns InvalidMagic
```

**Phase 1 Total: ~40 tests**

---

## Phase 2 Tests: Parsing

File: `tests/phase2_parsing.rs`

### Python Parsing

```
test_parse_python_function
  - Parse simple function
  - Verify name, span, params detected

test_parse_python_class
  - Parse class with methods
  - Verify hierarchy

test_parse_python_async
  - Parse async function
  - is_async flag set

test_parse_python_decorator
  - Parse decorated function
  - Decorator info captured

test_parse_python_import
  - Parse import statements
  - Import units created

test_parse_python_docstring
  - Parse docstring
  - doc_summary extracted

test_parse_python_type_hints
  - Parse type annotations
  - signature captured

test_parse_python_nested
  - Nested functions/classes
  - Contains edges correct
```

### Rust Parsing

```
test_parse_rust_function
test_parse_rust_struct
test_parse_rust_impl
test_parse_rust_trait
test_parse_rust_macro
test_parse_rust_async
test_parse_rust_generics
test_parse_rust_lifetime (captured in signature)
```

### TypeScript Parsing

```
test_parse_ts_function
test_parse_ts_class
test_parse_ts_interface
test_parse_ts_async
test_parse_ts_decorator
test_parse_ts_generic
test_parse_tsx_component
test_parse_ts_type_alias
```

### Cross-Language

```
test_parse_mixed_repo
  - Repo with Python + TypeScript
  - Both languages parsed correctly
  - Stats accurate

test_language_detection_by_path
  - File without extension
  - Shebang detection

test_ignore_patterns
  - .gitignore respected
  - Custom excludes work
```

### Snapshot Tests

```
test_parse_python_snapshot
  - Parse testdata/python/simple_module.py
  - Compare output to snapshot
  - Ensures parse stability

(Similar for each language and fixture)
```

**Phase 2 Total: ~50 tests**

---

## Phase 3 Tests: Semantic Analysis

File: `tests/phase3_semantic.rs`

### Resolution

```
test_resolve_local_reference
  - Function calls another in same file
  - CALLS edge created

test_resolve_cross_file_reference
  - Import from another file
  - IMPORTS edge created, target resolved

test_resolve_external_import
  - Import from stdlib/external
  - External import marked

test_resolve_qualified_name
  - Build correct qualified names
  - a.b.c.func → full path

test_resolve_circular_import
  - A imports B, B imports A
  - No infinite loop, both resolved
```

### FFI Tracing

```
test_ffi_python_rust
  - Python calls Rust via PyO3
  - FFI_BINDS edge created

test_ffi_detection
  - Detect FFI patterns automatically
```

### Pattern Detection

```
test_detect_singleton
test_detect_factory
test_detect_repository
test_detect_decorator_pattern
```

### Concept Extraction

```
test_extract_concept_user
  - Code related to "User"
  - Grouped correctly

test_extract_concept_auth
  - Authentication-related code
  - Detected and grouped
```

**Phase 3 Total: ~40 tests**

---

## Phase 4 Tests: Query Engine

File: `tests/phase4_queries.rs`

### Core Queries (1-8)

```
# Symbol Lookup
test_symbol_lookup_exact
test_symbol_lookup_prefix
test_symbol_lookup_contains
test_symbol_lookup_fuzzy
test_symbol_lookup_no_results
test_symbol_lookup_with_filters

# Dependency Graph
test_dependency_direct
test_dependency_transitive
test_dependency_depth_limit
test_dependency_cycle_handling
test_dependency_edge_type_filter

# Reverse Dependency
test_reverse_dep_direct
test_reverse_dep_transitive
test_reverse_dep_depth_limit

# Call Graph
test_call_graph_callers
test_call_graph_callees
test_call_graph_both
test_call_graph_depth

# Type Hierarchy
test_hierarchy_ancestors
test_hierarchy_descendants
test_hierarchy_both
test_hierarchy_interface

# Containment
test_containment_module
test_containment_class
test_containment_nested

# Pattern Match
test_pattern_function_calls
test_pattern_class_inherits
test_pattern_async_complexity

# Semantic Search
test_semantic_search_basic
test_semantic_search_filters
test_semantic_search_threshold
```

### Built Queries (9-11, 22-23)

```
# Impact Analysis
test_impact_direct
test_impact_transitive
test_impact_with_tests
test_impact_risk_scoring

# Test Coverage
test_coverage_direct
test_coverage_indirect
test_coverage_none

# Cross-Language Trace
test_trace_python_to_rust
test_trace_full_chain

# Similarity
test_similarity_same_file
test_similarity_cross_file
test_similarity_threshold

# Shortest Path
test_shortest_path_direct
test_shortest_path_indirect
test_shortest_path_no_path
```

### Novel Queries (12-21, 24)

```
# Collective Patterns (mock data)
test_collective_patterns_usage
test_collective_patterns_mistakes

# Temporal Evolution
test_evolution_timeline
test_evolution_trend

# Stability Analysis
test_stability_stable
test_stability_volatile
test_stability_factors

# Coupling Detection
test_coupling_explicit
test_coupling_temporal
test_coupling_hidden

# Dead Code
test_dead_code_unreachable
test_dead_code_entry_points

# Prophecy
test_prophecy_likely_break
test_prophecy_tech_debt

# Concept Mapping
test_concept_mapping_user
test_concept_mapping_auth

# Migration Path
test_migration_simple
test_migration_complex

# Test Gap
test_gap_recent_changes
test_gap_complexity_filter

# Architectural Drift
test_drift_layer_violation
test_drift_cycle

# Hotspot Detection
test_hotspot_buggy_file
test_hotspot_clean_file
```

**Phase 4 Total: ~80 tests**

---

## Phase 5 Tests: Indexes & Performance

File: `tests/phase5_indexes.rs`

```
test_symbol_index_build
test_symbol_index_lookup
test_symbol_index_prefix

test_type_index_build
test_type_index_filter

test_path_index_build
test_path_index_lookup

test_language_index_build
test_language_index_filter

test_mmap_read_header
test_mmap_read_unit
test_mmap_read_edges
test_mmap_feature_vectors

test_concurrent_mmap_read
  - Multiple threads reading same file
  - No corruption
```

**Phase 5 Total: ~20 tests**

---

## Phase 6 Tests: CLI

File: `tests/phase6_cli.rs`

```
test_cli_compile
test_cli_compile_with_exclude
test_cli_info
test_cli_query_symbol
test_cli_query_impact
test_cli_json_output
test_cli_help
test_cli_version
test_cli_invalid_path
test_cli_invalid_acb
```

**Phase 6 Total: ~15 tests**

---

## Phase 7 Tests: MCP

File: `tests/phase7_mcp.rs`

### Protocol Compliance

```
test_mcp_initialize
test_mcp_list_tools
test_mcp_list_resources
test_mcp_list_prompts
test_mcp_invalid_method
test_mcp_invalid_params
```

### Tool Tests

```
test_mcp_tool_compile
test_mcp_tool_symbol_lookup
test_mcp_tool_impact_analysis
test_mcp_tool_prophecy
(One test per tool)
```

### Resource Tests

```
test_mcp_resource_stats
test_mcp_resource_unit
test_mcp_resource_404
```

**Phase 7 Total: ~40 tests**

---

## Phase 8 Tests: Collective

File: `tests/phase8_collective.rs`

```
test_delta_compression
test_delta_apply
test_pattern_extraction
test_privacy_filter
test_offline_mode
```

**Phase 8 Total: ~10 tests**

---

## Phase 9 Tests: Temporal

File: `tests/phase9_temporal.rs`

```
test_history_extraction
test_stability_calculation
test_coupling_calculation
test_prophecy_prediction
test_no_git_fallback
```

**Phase 9 Total: ~10 tests**

---

## Phase 10 Tests: Integration

File: `tests/phase10_integration.rs`

```
test_full_lifecycle
  - Compile → Query → Modify → Recompile → Query
  - Everything works

test_large_repo
  - 100K LOC fixture
  - Compile in <30s
  - Queries in <10ms

test_cross_language_complete
  - Python + Rust + TypeScript
  - All edges resolved

test_mcp_full_workflow
  - Initialize → Compile → Multiple queries → Cleanup
```

**Phase 10 Total: ~10 tests**

---

## Benchmarks

File: `benches/benchmarks.rs`

```rust
// Compilation benchmarks
bench_compile_1k_loc
bench_compile_10k_loc
bench_compile_100k_loc

// Query benchmarks
bench_symbol_lookup_100k
bench_dependency_depth5_100k
bench_impact_analysis_100k
bench_semantic_search_100k
bench_prophecy_100k

// I/O benchmarks
bench_write_100k_graph
bench_read_100k_graph
bench_mmap_100k_graph

// String pool benchmarks
bench_string_compression
bench_string_decompression
```

---

## Test Data Generation

Script: `scripts/generate_fixtures.rs`

```rust
/// Generate a deterministic large codebase for testing
fn generate_large_fixture(path: &Path, config: FixtureConfig) {
    // Generate Python modules
    // Generate Rust crates
    // Generate TypeScript packages
    // Create realistic cross-references
    // Create test files
}
```

---

## Total Test Count

| Phase | Tests |
|-------|-------|
| Phase 1 | ~40 |
| Phase 2 | ~50 |
| Phase 3 | ~40 |
| Phase 4 | ~80 |
| Phase 5 | ~20 |
| Phase 6 | ~15 |
| Phase 7 | ~40 |
| Phase 8 | ~10 |
| Phase 9 | ~10 |
| Phase 10 | ~10 |
| **Total** | **~315** |

---

## Running Tests

```bash
# Run all tests
cargo test

# Run specific phase
cargo test phase1

# Run with logging
RUST_LOG=debug cargo test

# Run benchmarks
cargo bench

# Run with coverage
cargo tarpaulin --out Html
```
