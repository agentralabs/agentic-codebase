# SPEC-DEPENDENCIES.md

> These are the ONLY allowed dependencies. Do not add anything else.

---

## Cargo.toml (Main Crate)

```toml
[package]
name = "agentic-codebase"
version = "0.1.0"
edition = "2021"
description = "Semantic code compiler for AI agents - transforms codebases into navigable concept graphs"
license = "MIT"
readme = "README.md"
repository = "https://github.com/agentic-revolution/agentic-codebase"
keywords = ["ai", "agents", "code-analysis", "semantic-graph", "mcp"]
categories = ["development-tools", "parsing"]

[lib]
name = "agentic_codebase"
path = "src/lib.rs"

[[bin]]
name = "acb"
path = "src/bin/acb.rs"

[dependencies]
# Parsing - tree-sitter ecosystem
tree-sitter = "0.22"
tree-sitter-python = "0.21"
tree-sitter-rust = "0.21"
tree-sitter-typescript = "0.21"
tree-sitter-javascript = "0.21"
tree-sitter-go = "0.21"

# Compression
lz4_flex = "0.11"                   # Pure Rust LZ4 (no C dependency)

# Error handling  
thiserror = "2"                     # Derive macro for error types

# Logging
log = "0.4"                         # Logging facade

# CLI
clap = { version = "4", features = ["derive"] }

# Serialization (for CLI/MCP JSON output, NOT for .acb format)
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Memory mapping
memmap2 = "0.9"                     # Cross-platform mmap

# Timestamp
chrono = { version = "0.4", default-features = false, features = ["std", "clock"] }

# File system traversal
walkdir = "2"                       # Recursive directory walking
ignore = "0.4"                      # .gitignore parsing

# Hashing (for content-addressed storage)
blake3 = "1"                        # Fast cryptographic hash

# Git integration (for temporal analysis)
gix = { version = "0.63", default-features = false, features = ["basic", "blob-diff"] }

[dev-dependencies]
# Testing
criterion = { version = "0.5", features = ["html_reports"] }
tempfile = "3"                      # Temporary files for tests
rand = "0.8"                        # Random data generation
env_logger = "0.11"                 # Logger for tests
insta = "1.39"                      # Snapshot testing

[[bench]]
name = "benchmarks"
harness = false
```

---

## Cargo.toml (MCP Server Crate)

```toml
# mcp/Cargo.toml
[package]
name = "agentic-codebase-mcp"
version = "0.1.0"
edition = "2021"
description = "MCP server for AgenticCodebase"
license = "MIT"

[[bin]]
name = "acb-mcp-server"
path = "src/main.rs"

[dependencies]
# The core library
agentic-codebase = { path = ".." }

# MCP protocol
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"

# JSON-RPC
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Logging
log = "0.4"
env_logger = "0.11"

# Error handling
thiserror = "2"
anyhow = "1"
```

---

## Why Each Dependency

### Core Library

| Dependency | Purpose | Justification |
|------------|---------|---------------|
| tree-sitter | Multi-language parsing | Industry standard, incremental, error-tolerant. No alternative matches its language coverage. |
| tree-sitter-{lang} | Language grammars | Official grammars for each supported language |
| lz4_flex | String pool compression | Fast decompression, reasonable ratio, pure Rust |
| thiserror | Error type derives | Zero runtime cost, eliminates boilerplate |
| log | Logging facade | Standard Rust logging, zero cost if unused |
| clap | CLI argument parsing | Standard, derive-based, excellent help generation |
| serde + serde_json | JSON I/O | CLI and MCP need JSON. NOT used in .acb format. |
| memmap2 | Memory-mapped files | Cross-platform mmap for zero-copy file access |
| chrono | Timestamps | Clock access only; internal storage is raw u64 |
| walkdir | Directory traversal | Battle-tested recursive walker |
| ignore | Gitignore parsing | Respects .gitignore, .ignore files |
| blake3 | Content hashing | Fast, cryptographic, for cache keys and dedup |
| gix | Git integration | Pure Rust git implementation for history analysis |

### MCP Server

| Dependency | Purpose | Justification |
|------------|---------|---------------|
| tokio | Async runtime | MCP uses stdio which benefits from async I/O |
| async-trait | Async trait methods | Required for async MCP handlers |
| anyhow | Error handling in binary | Simpler error handling for the server binary |

### Dev Dependencies

| Dependency | Purpose | Justification |
|------------|---------|---------------|
| criterion | Benchmarks | Industry standard for Rust benchmarking |
| tempfile | Test temp files | Cleanup-safe temporary directories |
| rand | Test data generation | Random graph generation for stress tests |
| env_logger | Test logging | See logs during test failures |
| insta | Snapshot testing | Parse output stability tests |

---

## What We Do NOT Use

### No General-Purpose Databases
- No SQLite, RocksDB, sled
- The `.acb` file IS the database
- Memory-mapped, single-file, zero external processes

### No Async in Core Library
- No tokio in the library crate (only in MCP server)
- mmap is already non-blocking at OS level
- Simpler API, easier testing

### No Heavy Parser Frameworks
- No syn (too Rust-specific)
- No pest, nom, lalrpop (tree-sitter is sufficient)
- tree-sitter gives us all languages with one abstraction

### No Vector Databases
- No pgvector, qdrant, pinecone
- Brute-force cosine similarity is fast enough at our scale
- 100K vectors × 256 dims = <1ms on modern hardware

### No ML Frameworks
- No PyTorch, ONNX in the core
- Embedding generation is pluggable/external
- Core library works without any ML runtime

### No Network Libraries in Core
- No reqwest, hyper in the library
- Collective sync is optional and in a separate module
- Core functionality works 100% offline

---

## Dependency Audit Requirements

Before adding ANY dependency:

1. **Is it in SPEC-DEPENDENCIES.md?** If no, implement it yourself.
2. **Does it have active maintenance?** Check last commit, issue response time.
3. **Does it have unsafe code?** If yes, is it audited?
4. **Does it pull in a dependency tree?** Audit transitive dependencies.
5. **Does it work on all platforms?** Linux, macOS, Windows.

---

## Version Pinning Strategy

- **Exact versions** for tree-sitter grammars (parsing output must be reproducible)
- **Minor version ranges** (`"1"` not `"1.2.3"`) for stable crates
- **Run `cargo update` monthly** and verify tests pass
- **Document breaking changes** when upgrading

---

## Build Verification

After setting up dependencies:

```bash
# Must succeed with no warnings
cargo build --release 2>&1 | grep -i warning && exit 1

# Must have no unused dependencies
cargo machete  # Install: cargo install cargo-machete

# Must have no security advisories
cargo audit    # Install: cargo install cargo-audit
```
