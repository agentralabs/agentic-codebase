---
status: stable
---

# Architecture

AgenticCodebase is a 4-crate Rust workspace with additional language bindings.

## Workspace Structure

```
agentic-codebase/
  Cargo.toml                    (workspace root)
  src/                          (core library + binaries)
    lib.rs                      (crate root)
    bin/acb.rs                  (CLI binary: acb)
    bin/agentic-codebase-mcp.rs (MCP server binary)
    types/                      (data types, file header, error types)
    parse/                      (tree-sitter language parsers)
    semantic/                   (cross-file resolution, pattern detection)
    format/                     (binary .acb reader/writer, compression)
    graph/                      (in-memory code graph, traversal)
    engine/                     (compilation pipeline, query executor)
    index/                      (symbol, type, path, embedding indexes)
    temporal/                   (change history, stability, prophecy)
    collective/                 (collective intelligence, pattern sync)
    grounding/                  (anti-hallucination verification)
    workspace/                  (multi-context cross-codebase queries)
    ffi/                        (C-compatible FFI bindings)
    config/                     (configuration loading, path resolution)
    cli/                        (CLI commands, REPL, output formatting)
    mcp/                        (MCP server, protocol, SSE transport)
  crates/
    agentic-codebase-cli/       (standalone CLI crate)
    agentic-codebase-mcp/       (standalone MCP crate)
    agentic-codebase-ffi/       (C FFI shared library)
  npm/wasm/                     (npm WASM package)
```

## Crate Responsibilities

### agentic-codebase (core)

The core library. All semantic code analysis logic lives here.

- Language parsing: Python, Rust, TypeScript, JavaScript, Go, C++, Java, C# via tree-sitter
- Semantic analysis: cross-file resolution, FFI tracing, pattern detection, architecture inference
- Code graph: units (modules, symbols, types, functions, imports, tests, docs, configs, patterns, traits, impls, macros) and typed edges
- File format: `.acb` binary format (magic `ACDB`, version 1, 128-byte header)
- Query engine: symbol lookup, impact analysis, dependency traversal, prophecy, stability scoring
- Grounding engine: claim verification, citation, hallucination detection, truth maintenance
- Indexes: symbol, type, path, language, embedding, and semantic search indexes
- Temporal analysis: change history, stability, coupling, code archaeology
- Workspaces: multi-codebase comparison, translation tracking, migration planning
- Compression: LZ4 for compact binary storage
- No async runtime required for core operations

### agentic-codebase-mcp

The MCP server binary (`agentic-codebase-mcp`).

- JSON-RPC 2.0 over stdio (default) or HTTP/SSE (with `sse` feature)
- 60+ MCP tools across core, grounding, workspace, translation, and invention categories
- MCP resources via `acb://` URI scheme
- 2 MCP prompts (analyse_unit, explain_coupling)
- Auto-graph resolution: detects repository root, compiles graph on first use
- Lazy graph loading with deferred path support
- Content-Length framing with header-based message parsing
- Multi-tenant mode for SSE transport (routes by X-User-ID header)
- Auto-logging of tool calls with operation records

### agentic-codebase-cli

The command-line interface binary (`acb`).

- Human-friendly terminal output with styled formatting
- Subcommands: init, compile, info, query, get, health, gate, budget, export, ground, evidence, suggest, workspace, completions
- Interactive REPL when launched without a subcommand
- Text and JSON output formats
- Tab completion for bash, zsh, fish, powershell, elvish
- 12 query types: symbol, deps, rdeps, impact, calls, similar, prophecy, stability, coupling, test-gap, hotspots, dead-code

### agentic-codebase-ffi

C-compatible shared library for cross-language integration.

- Opaque handle pattern for graph instances (`acb_graph_open` / `acb_graph_free`)
- Buffer-based string exchange for unit names and file paths
- Direct accessors for unit count, edge count, dimension, complexity, stability, language
- Edge traversal via output arrays (target IDs, edge types, weights)
- Error codes: `ACB_OK` (0), `ACB_ERR_IO` (-1), `ACB_ERR_INVALID` (-2), `ACB_ERR_NOT_FOUND` (-3), `ACB_ERR_OVERFLOW` (-4), `ACB_ERR_NULL_PTR` (-5)
- All functions use `panic::catch_unwind` for safety

## Data Flow

```
Agent (Claude/GPT/etc.)
  |
  | MCP protocol (JSON-RPC 2.0 over stdio)
  v
agentic-codebase-mcp
  |
  | Rust function calls
  v
agentic-codebase (core)
  |
  | Binary I/O (memory-mapped)
  v
project.acb (file)
```

## File Format

The `.acb` binary format has a fixed 128-byte header:

| Offset | Size | Field |
|--------|------|-------|
| 0x00 | 4 | Magic bytes: `ACDB` (0x41 0x43 0x44 0x42) |
| 0x04 | 4 | Version: `0x00000001` |
| 0x08 | 4 | Feature vector dimension |
| 0x0C | 4 | Language count |
| 0x10 | 8 | Unit count |
| 0x18 | 8 | Edge count |
| 0x20 | 8 | Unit table offset |
| 0x28 | 8 | Edge table offset |
| 0x30 | 8 | String pool offset |
| 0x38 | 8 | Feature vector offset |
| 0x40 | 8 | Temporal block offset |
| 0x48 | 8 | Index block offset |
| 0x50 | 32 | Repository path hash (SHA-256) |
| 0x70 | 8 | Compiled-at timestamp (Unix epoch microseconds) |
| 0x78 | 8 | Reserved |

All fields are little-endian. The body contains LZ4-compressed unit tables, edge tables, string pools, feature vectors, temporal data, and indexes.

## Supported Languages

| Language | Parser | Extensions |
|----------|--------|------------|
| Python | tree-sitter-python | `.py` |
| Rust | tree-sitter-rust | `.rs` |
| TypeScript | tree-sitter-typescript | `.ts`, `.tsx` |
| JavaScript | tree-sitter-javascript | `.js`, `.jsx` |
| Go | tree-sitter-go | `.go` |
| C++ | tree-sitter-cpp | `.c`, `.cc`, `.cpp`, `.h`, `.hpp` |
| Java | tree-sitter-java | `.java` |
| C# | tree-sitter-c-sharp | (via tree-sitter-c-sharp) |

## Cross-Sister Integration

AgenticCodebase integrates with other Agentra sisters:

- **AgenticMemory**: Grounding claims link to memory nodes. Code archaeology informs memory freshness.
- **AgenticVision**: Visual captures can reference code units. Architecture diagrams link to inferred patterns.
- **AgenticTime**: Sequences model deployment pipelines. Duration estimates track refactoring effort.
- **AgenticIdentity**: Code analysis operations are signed with identity receipts for audit trails.

## Runtime Isolation

Each repository gets its own `.acb` file, resolved by deterministic path hashing (SHA-256 of canonical path). Same-name folders in different locations never share graph state. Directory-based locking with stale lock recovery ensures safe concurrent compilation.
