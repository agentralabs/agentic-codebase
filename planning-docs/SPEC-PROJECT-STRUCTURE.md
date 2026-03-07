# SPEC-PROJECT-STRUCTURE.md

> Create this exact directory structure before writing any code.

```
agentic-codebase/
├── Cargo.toml
├── README.md
├── LICENSE                         # MIT
├── benches/
│   └── benchmarks.rs               # Criterion benchmarks
├── examples/
│   ├── basic_compile.rs            # Simple compile → query flow
│   ├── cross_language.rs           # Python + Rust FFI tracing
│   ├── impact_analysis.rs          # Change impact demonstration
│   └── large_repo.rs               # 100K LOC performance demo
├── src/
│   ├── lib.rs                      # Public API re-exports
│   ├── types/
│   │   ├── mod.rs
│   │   ├── code_unit.rs            # CodeUnit struct + CodeUnitType enum
│   │   ├── edge.rs                 # Edge struct + EdgeType enum
│   │   ├── language.rs             # Language enum + detection
│   │   ├── span.rs                 # Source location types
│   │   ├── header.rs               # FileHeader struct
│   │   └── error.rs                # AcbError enum + Result type
│   ├── parse/
│   │   ├── mod.rs
│   │   ├── parser.rs               # Main parser orchestrator
│   │   ├── python.rs               # Python-specific parsing
│   │   ├── rust.rs                 # Rust-specific parsing
│   │   ├── typescript.rs           # TypeScript/JavaScript parsing
│   │   ├── go.rs                   # Go parsing
│   │   └── treesitter.rs           # tree-sitter wrapper utilities
│   ├── semantic/
│   │   ├── mod.rs
│   │   ├── analyzer.rs             # Main semantic analyzer
│   │   ├── resolver.rs             # Cross-file symbol resolution
│   │   ├── ffi_tracer.rs           # Cross-language FFI tracing
│   │   ├── pattern_detector.rs     # Design pattern detection
│   │   └── concept_extractor.rs    # High-level concept extraction
│   ├── format/
│   │   ├── mod.rs
│   │   ├── writer.rs               # Writes .acb files from CodeGraph
│   │   ├── reader.rs               # Reads .acb files into CodeGraph
│   │   ├── mmap.rs                 # Memory-mapped file access
│   │   └── compression.rs          # LZ4 string pool compression
│   ├── graph/
│   │   ├── mod.rs
│   │   ├── code_graph.rs           # Core graph structure
│   │   ├── builder.rs              # Fluent API for building graphs
│   │   └── traversal.rs            # Graph traversal algorithms
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── compile.rs              # Main compilation pipeline
│   │   ├── query.rs                # Query executor (all 24 types)
│   │   └── incremental.rs          # Incremental recompilation
│   ├── index/
│   │   ├── mod.rs
│   │   ├── symbol_index.rs         # Index by symbol name
│   │   ├── type_index.rs           # Index by code unit type
│   │   ├── path_index.rs           # Index by file path
│   │   ├── language_index.rs       # Index by language
│   │   └── embedding_index.rs      # Vector similarity index
│   ├── temporal/
│   │   ├── mod.rs
│   │   ├── history.rs              # Change history tracking
│   │   ├── stability.rs            # Stability score calculation
│   │   ├── coupling.rs             # Coupling detection
│   │   └── prophecy.rs             # Predictive analysis
│   ├── collective/
│   │   ├── mod.rs
│   │   ├── delta.rs                # Delta compression for sync
│   │   ├── registry.rs             # Collective registry client
│   │   ├── patterns.rs             # Pattern aggregation
│   │   └── privacy.rs              # Privacy-preserving extraction
│   ├── ffi/
│   │   ├── mod.rs
│   │   └── c_api.rs                # C-compatible FFI bindings
│   ├── cli/
│   │   ├── mod.rs
│   │   └── commands.rs             # CLI command implementations
│   └── mcp/
│       ├── mod.rs
│       ├── server.rs               # MCP server implementation
│       ├── tools.rs                # MCP tool definitions
│       ├── resources.rs            # MCP resource handlers
│       └── prompts.rs              # MCP prompt templates
├── src/bin/
│   └── acb.rs                      # CLI entry point
├── mcp/
│   ├── src/
│   │   └── main.rs                 # MCP server binary
│   └── Cargo.toml                  # MCP server crate
├── tests/
│   ├── phase1_foundation.rs        # Data structures + file format
│   ├── phase2_parsing.rs           # Multi-language parsing
│   ├── phase3_semantic.rs          # Semantic analysis
│   ├── phase4_queries.rs           # All 24 query types
│   ├── phase5_indexes.rs           # Index + mmap tests
│   ├── phase6_cli.rs               # CLI integration
│   ├── phase7_mcp.rs               # MCP protocol compliance
│   ├── phase8_collective.rs        # Collective sync
│   ├── phase9_temporal.rs          # Temporal + prophecy
│   └── phase10_integration.rs      # End-to-end tests
├── ffi/
│   ├── agentic_codebase.h          # C header file
│   └── test_ffi.c                  # C FFI test program
└── testdata/
    ├── python/                     # Python test fixtures
    │   ├── simple_module.py
    │   ├── class_hierarchy.py
    │   └── async_patterns.py
    ├── rust/                       # Rust test fixtures
    │   ├── simple_lib.rs
    │   ├── traits_and_impls.rs
    │   └── ffi_exports.rs
    ├── typescript/                 # TypeScript test fixtures
    │   ├── simple_module.ts
    │   ├── react_component.tsx
    │   └── api_client.ts
    ├── mixed/                      # Multi-language fixtures
    │   ├── python_calls_rust/
    │   └── ts_calls_python/
    └── repos/                      # Full repo fixtures
        ├── small_python/           # ~1K LOC
        ├── medium_mixed/           # ~10K LOC
        └── large_monorepo/         # ~100K LOC (generated)
```

---

## Module Responsibilities

### `types/` — All data types. No logic. No I/O.
Pure struct definitions, enum definitions, trait implementations (Display, Debug, Clone, PartialEq, Serialize where needed). Error types with thiserror.

### `parse/` — Language-specific parsing.
Converts source code into raw syntax information using tree-sitter. One module per language. No semantic analysis here — just syntax extraction.

### `semantic/` — Semantic analysis layer.
Takes parsed syntax and extracts meaning: resolves references across files, traces FFI boundaries, detects patterns. This is where syntax becomes semantics.

### `format/` — Binary file I/O only.
Reads and writes `.acb` files. Handles compression. Handles mmap. Does NOT contain graph logic or query logic.

### `graph/` — In-memory graph operations.
The core data structure. Adding nodes, adding edges, traversing edges. No file I/O. No query planning. Pure graph operations.

### `engine/` — High-level operations.
The compilation pipeline and query engine. Orchestrates parsing → semantic → graph building. Orchestrates query execution across indexes.

### `index/` — Index structures for fast lookup.
Each index is independent. The engine uses them, but they don't know about each other. Incrementally updateable.

### `temporal/` — Time-based analysis.
Change history, stability scores, coupling detection, prophecy. Requires git integration for history extraction.

### `collective/` — Collective intelligence layer.
Delta sync, pattern aggregation, privacy-preserving extraction. Network-optional — works fully offline.

### `ffi/` — C-compatible bindings.
Thin wrapper around the public API. Opaque pointers. No Rust types exposed.

### `cli/` — Command-line interface.
Wraps the public API into CLI commands. Stateless — every command opens a file, does work, closes the file.

### `mcp/` — Model Context Protocol server.
The PRIMARY interface for LLM consumption. Implements MCP 2024-11-05 spec. Tools, Resources, Prompts.

---

## Crate Structure

```toml
# Main workspace
[workspace]
members = [".", "mcp"]

# Main library + CLI
[package]
name = "agentic-codebase"

[lib]
name = "agentic_codebase"
path = "src/lib.rs"

[[bin]]
name = "acb"
path = "src/bin/acb.rs"

# MCP server is separate crate
# mcp/Cargo.toml depends on agentic-codebase
```

---

## Test Data Organization

### Unit Test Fixtures (`testdata/python/`, `testdata/rust/`, etc.)
Small, focused files for testing specific parsing scenarios:
- Simple functions and classes
- Edge cases (async, decorators, macros)
- Cross-file references

### Integration Fixtures (`testdata/repos/`)
Complete mini-repositories:
- `small_python/` — ~1K LOC, single language, tests basic compilation
- `medium_mixed/` — ~10K LOC, Python + TypeScript, tests cross-language
- `large_monorepo/` — ~100K LOC, generated, tests performance

The large fixture is GENERATED by a script, not committed. Add `scripts/generate_large_fixture.rs` to create it deterministically.

---

## Naming Conventions

- **Types**: PascalCase (`CodeUnit`, `EdgeType`)
- **Functions**: snake_case (`compile_repo`, `query_impact`)
- **Constants**: SCREAMING_SNAKE_CASE (`MAX_SYMBOL_NAME`, `DEFAULT_DIMENSION`)
- **Modules**: snake_case, singular (`parse`, `semantic`, not `parsers`)
- **Test functions**: `test_<what>_<condition>` (`test_symbol_lookup_nonexistent`)

---

## Documentation Requirements

Every public item MUST have:
1. A one-line summary
2. A longer description if non-obvious
3. Examples for any function with non-trivial usage
4. Panic/error conditions documented

```rust
/// Compiles a repository into a semantic code graph.
///
/// This function recursively walks the given path, parses all supported
/// source files, performs semantic analysis, and produces a CodeGraph
/// that can be queried or saved to disk.
///
/// # Arguments
///
/// * `path` - Path to repository root
/// * `options` - Compilation options (languages, excludes, etc.)
///
/// # Returns
///
/// A `CodeGraph` containing all symbols and relationships.
///
/// # Errors
///
/// Returns `AcbError::PathNotFound` if path doesn't exist.
/// Returns `AcbError::ParseError` if a source file can't be parsed.
///
/// # Examples
///
/// ```
/// let graph = compile_repo("./my-project", CompileOptions::default())?;
/// println!("Found {} symbols", graph.unit_count());
/// ```
pub fn compile_repo(path: impl AsRef<Path>, options: CompileOptions) -> AcbResult<CodeGraph> {
    // ...
}
```
