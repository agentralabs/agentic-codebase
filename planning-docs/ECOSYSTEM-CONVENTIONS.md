# Agentic Ecosystem Conventions

> Canonical reference extracted from AgenticMemory and AgenticVision.
> Every future Agentic-family repository MUST follow these conventions.
> Generated 2026-02-19, updated 2026-02-20.
> Includes shell completions, REPL, SSE transport, multi-tenant, config, FFI, CI/CD, and install conventions.
> GitHub org: `agentralabs` | Domain: `agentralabs.tech`

---

## Table of Contents

1. [Repository Root Files](#1-repository-root-files)
2. [Git Conventions](#2-git-conventions)
3. [Cargo.toml Conventions](#3-cargotoml-conventions)
4. [Workspace Structure](#4-workspace-structure)
5. [CLI Conventions](#5-cli-conventions)
6. [Command Conventions](#6-command-conventions)
7. [Error Handling](#7-error-handling)
8. [Module Organization](#8-module-organization)
9. [Testing Conventions](#9-testing-conventions)
10. [Documentation Conventions](#10-documentation-conventions)
11. [MCP Server Conventions](#11-mcp-server-conventions)
12. [Logging and Tracing](#12-logging-and-tracing)
13. [CI/CD Conventions](#13-cicd-conventions)
14. [Release Conventions](#14-release-conventions)
15. [Benchmark Conventions](#15-benchmark-conventions)
16. [Configuration Conventions](#16-configuration-conventions)
17. [Edge Case Handling](#17-edge-case-handling)
18. [Code Style](#18-code-style)
19. [Community Files](#19-community-files)
20. [Paper and Publication](#20-paper-and-publication)
21. [Shell Completions](#21-shell-completions)
22. [Interactive REPL](#22-interactive-repl)
23. [FFI Bindings](#23-ffi-bindings)
24. [CI/CD Pipeline (canonical)](#24-cicd-pipeline-canonical)
25. [Install Scripts (canonical)](#25-install-scripts-canonical)
26. [Publishing](#26-publishing)
27. [Standalone Sister Independence (canonical)](#27-standalone-sister-independence-canonical)
28. [Autonomic Operations Defaults (canonical)](#28-autonomic-operations-defaults-canonical)

---

## 1. Repository Root Files

Every repo MUST have these files at root:

| File | Required | Notes |
|:-----|:--------:|:------|
| `README.md` | YES | Badges, description, quickstart, architecture, benchmarks |
| `LICENSE` | YES | MIT license text (matches `license = "MIT"` in Cargo.toml) |
| `CHANGELOG.md` | YES | Keep a Changelog format, SemVer sections |
| `CONTRIBUTING.md` | YES | Development setup, PR process, code style |
| `CODE_OF_CONDUCT.md` | YES | Contributor Covenant v2.1 |
| `SECURITY.md` | YES | Vulnerability reporting instructions |
| `Cargo.toml` | YES | Workspace root |
| `Cargo.lock` | YES | Committed (required for binary crates) |
| `.gitignore` | YES | See Section 2 |
| `.gitattributes` | RECOMMENDED | LF line endings for `.rs`, binary for `.amem`/`.avis`/`.acb` |
| `Makefile` | OPTIONAL | AgenticVision has one; AgenticMemory uses scripts/ |
| `rust-toolchain.toml` | OPTIONAL | Pin Rust version if needed |

### README.md Structure (from Memory/Vision)

```markdown
# ProjectName

<p align="center">
  <img src="assets/logo.svg" width="120" alt="Logo">
</p>

<p align="center">
  <a href="..."><img src="badge" alt="CI"></a>
  <a href="..."><img src="badge" alt="crates.io"></a>
  <a href="..."><img src="badge" alt="License"></a>
</p>

> One-line tagline.

## Features
## Quick Start
## Architecture
## Benchmarks
## Documentation
## Integration with Agentic Ecosystem
## Contributing
## License
```

---

## 2. Git Conventions

### .gitignore (canonical template)

```gitignore
# Build
/target/
**/*.rs.bk

# OS
.DS_Store
Thumbs.db

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# Environment
.env
.env.local

# LaTeX build artifacts
*.aux
*.bbl
*.blg
*.log
*.out
*.toc
*.fls
*.fdb_latexmk
*.synctex.gz

# Python
__pycache__/
*.pyc
.pytest_cache/
*.egg-info/
dist/
build/
.venv/

# Test artifacts
*.tmp
*.bak
```

### Commit Message Convention

Format: `Verb: description` (sentence case, no period)

Examples from siblings:
- `Add: BM25 text search and hybrid query`
- `Fix: stability score clamping edge case`
- `Update: README with benchmark results`
- `Refactor: extract pattern detector into own module`

### Branch Naming

- `main` -- default branch
- Feature branches: `feature/description` or `add/description`
- Fix branches: `fix/description`

---

## 3. Cargo.toml Conventions

### Required Package Fields

```toml
[package]
name = "agentic-{name}"
version = "0.1.0"
edition = "2021"
license = "MIT"
repository = "https://github.com/agentralabs/agentic-{name}"
authors = ["Omoshola Owolabi"]
description = "One-line description"
readme = "README.md"
keywords = ["ai", "agents", ...]        # max 5
categories = ["development-tools", ...]   # from crates.io taxonomy
```

### Recommended Fields

```toml
homepage = "https://github.com/agentralabs/agentic-{name}"
documentation = "https://docs.rs/agentic-{name}"
exclude = [".DS_Store", "planning-docs/", "paper/"]
```

### Required Dependencies (ecosystem standard)

```toml
clap = { version = "4", features = ["derive"] }
clap_complete = "4"       # Shell completion generation
rustyline = "14"           # Interactive REPL
toml = "0.8"               # Config file support
```

### Required crate-type for FFI

```toml
[lib]
name = "agentic_{name}"
path = "src/lib.rs"
crate-type = ["lib", "cdylib", "staticlib"]
```

### Dependency Conventions

- Pin to `major.minor` (e.g., `"1.0"`, `"0.22"`)
- Features inline: `serde = { version = "1", features = ["derive"] }`
- Internal deps use BOTH version and path: `agentic-x = { version = "0.1.0", path = "../agentic-x" }`
- Common dependencies across ecosystem:
  - `thiserror` for library error types
  - `anyhow` for binary main() only
  - `clap = { version = "4", features = ["derive"] }` for CLI
  - `serde = { version = "1", features = ["derive"] }`
  - `serde_json = "1"`
  - `tracing = "0.1"` for instrumentation
  - `tracing-subscriber = { version = "0.3", features = ["env-filter"] }` for binary
  - `tokio = { version = "1", features = ["full"] }` for MCP server
  - `criterion = { version = "0.5", features = ["html_reports"] }` for benchmarks
  - `tempfile = "3"` for test isolation

### Binary Targets

```toml
[[bin]]
name = "agentic-{name}-mcp"
path = "src/main.rs"

[lib]
name = "agentic_{name}"
path = "src/lib.rs"

# OR for single-crate with two binaries:
[[bin]]
name = "{short}"           # e.g., "acb", "amem"
path = "src/bin/{short}.rs"

[[bin]]
name = "{short}-mcp"       # e.g., "acb-mcp", "amem-mcp"
path = "src/bin/{short}-mcp.rs"
```

### Profile Settings

Both siblings currently have NO profile settings. For future repos, consider:

```toml
[profile.release]
lto = "thin"
strip = true
codegen-units = 1
```

---

## 4. Workspace Structure

### AgenticMemory Pattern (monorepo with workspace)

```
Cargo.toml              # [workspace] root
crates/
  agentic-memory/       # Core library + CLI binary
    Cargo.toml
    src/
      lib.rs
      bin/amem.rs
  agentic-memory-mcp/   # MCP server library + binary
    Cargo.toml
    src/
      lib.rs
      main.rs
    tests/
      edge_cases.rs
      common/
tests/bridge/           # Cross-crate integration tests
  Cargo.toml
```

### AgenticCodebase Pattern (single crate, two binaries)

```
Cargo.toml              # Single [package], no workspace
src/
  lib.rs
  bin/
    acb.rs              # CLI binary
    acb-mcp.rs          # MCP server binary
tests/
  phase1_foundation.rs
  ...
benches/
  benchmarks.rs
```

Both patterns are acceptable. The workspace pattern is preferred for larger projects with distinct library/server boundaries. The single-crate pattern works for tightly coupled code.

### Workspace Cargo.toml (if using workspace)

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
repository = "https://github.com/agentralabs/agentic-{name}"
authors = ["Omoshola Owolabi"]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
# ... shared deps
```

---

## 5. CLI Conventions

### Structure

- Use `clap` v4 with derive API
- `#[derive(Parser)]` for top-level struct
- `#[derive(Subcommand)]` for command enum
- `#[derive(ValueEnum)]` for flag values (e.g., OutputFormat)

### Required Global Flags

```rust
#[derive(Parser)]
#[command(
    name = "{binary-name}",
    about = "{ProjectName} -- one-line description",
    version,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,  // None => launch REPL

    /// Output format
    #[arg(long, default_value = "text", global = true)]
    format: OutputFormat,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Text,
    Json,
}
```

### Help Text Style

- Doc comments (`///`) on struct fields serve as help text
- Sentence-cased, period-terminated: `"Path to .acb file."`
- Short flags only for common args: `-o` for output, `-v` for verbose
- Long flags for everything else

### Error Reporting

```rust
fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
```

AgenticMemory maps error types to exit codes (3 for invalid input, 4 for file not found, etc.). This is recommended but not required.

---

## 6. Command Conventions

### Naming Pattern

- Verb-based, lowercase
- Multi-word: kebab-case (e.g., `serve-http`)
- Core commands that both siblings share:
  - **Memory**: `create`, `add`, `query`, `traverse`, `stats`, `export`, `import`, `merge`
  - **Vision**: `serve`, `validate`, `info`
  - **Codebase**: `compile`, `info`, `query`, `get`

### Output Format

Every command MUST support `--format text|json`:
- **Text**: Human-readable, aligned key-value pairs for terminals
- **JSON**: Machine-readable via `serde_json::to_string_pretty()`

### Path Validation Pattern

```rust
fn validate_file_path(path: &Path, extension: &str) -> AcbResult<()> {
    if !path.exists() {
        return Err(/* PathNotFound */);
    }
    if !path.is_file() {
        return Err(/* not a file */);
    }
    if path.extension().and_then(|e| e.to_str()) != Some(extension) {
        // warn but continue
    }
    Ok(())
}
```

---

## 7. Error Handling

### Library Error Type

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum {Project}Error {
    #[error("Human-readable message")]
    VariantName,

    #[error("Message with data: {0}")]
    VariantWithData(u64),

    #[error("Structured: expected {expected}, got {got}")]
    StructuredVariant { expected: usize, got: usize },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type {Project}Result<T> = Result<T, {Project}Error>;
```

### Conventions

- Human-readable error messages (sentences)
- `#[from]` for automatic conversion from std errors
- Named fields for structured errors
- Type alias: `AcbResult<T>`, `AmemResult<T>`, `VisionResult<T>`
- Clamping for bounded values (confidence, weight): `.clamp(0.0, 1.0)`
- `?` operator for propagation
- Library uses typed errors; CLI can use `Box<dyn Error>` or `anyhow`

### MCP Error Codes

```rust
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
}

pub mod mcp_error_codes {
    pub const RESOURCE_NOT_FOUND: i32 = -32802;
    pub const TOOL_NOT_FOUND: i32 = -32803;
    // project-specific: -32850 to -32899
}
```

---

## 8. Module Organization

### Standard Module Layout

```
src/
  lib.rs              # Module declarations + pub use re-exports
  bin/
    {short}.rs        # CLI entry point
    {short}-mcp.rs    # MCP server entry point
  types/
    mod.rs            # Constants, re-exports
    error.rs          # Error enum + Result alias
    {domain}.rs       # Domain-specific types
  {domain}/
    mod.rs            # pub mod + pub use re-exports
    {feature}.rs      # One file per feature
```

### lib.rs Pattern

```rust
//! {ProjectName} -- one-line description.
//!
//! Multi-line overview of architecture and key modules.

pub mod types;
pub mod graph;
pub mod format;
pub mod engine;
pub mod index;
// ...

// Re-export key types at crate root
pub use types::{MainType, Error, Result};
pub use graph::Graph;
pub use format::{Reader, Writer};
```

### mod.rs Pattern

```rust
//! Brief module description.
//!
//! Optional details about what this module provides.

pub mod feature_a;
pub mod feature_b;

pub use feature_a::{TypeA, FunctionA};
pub use feature_b::{TypeB, FunctionB};
```

### Visibility

- `pub` for all public API items
- `pub(crate)` for internal-only fields on public structs
- Private for implementation details

---

## 9. Testing Conventions

### Test Organization

Both siblings use **external integration tests** in `tests/` as the primary testing strategy:

| Pattern | Description | Used By |
|:--------|:-----------|:--------|
| Phase-numbered files | `phase{N}_{area}.rs` | Memory, Codebase |
| Category files | `edge_cases.rs` | Memory, Vision |
| Numbered tests | `test_{NN}_{name}()` | Vision |
| Common utilities | `tests/common/` | Memory |

### Phase-Numbered Test Files (ecosystem standard)

```
tests/
  phase1_types.rs          # Data structures, serialization
  phase2_protocol.rs       # Protocol handling
  phase3_tools.rs          # Tool implementations
  phase4_session.rs        # Session management
  phase5_integration.rs    # End-to-end flows
  edge_cases.rs            # Comprehensive edge cases
  common/
    mod.rs
    fixtures.rs
    mock_client.rs
```

### Test Function Naming

```rust
#[test]
fn test_what_is_being_tested() { ... }

// OR with category prefix:
#[test]
fn test_01_path_traversal() { ... }
```

### Edge Case Test Categories (REQUIRED)

Both siblings have dedicated edge case test files covering:

1. **Protocol edge cases**: Malformed JSON-RPC, wrong version, empty method, string IDs
2. **Input validation**: Missing params, invalid types, empty inputs, boundary values
3. **Security**: Path traversal, malicious input
4. **Concurrency**: Rapid reconnect, parallel access
5. **Boundary values**: Min/max integers, empty collections, huge payloads
6. **Unicode**: Full Unicode support in names, descriptions, content
7. **File format**: Magic validation, version check, truncation, corruption

### Test Helpers

```rust
// In tests/common/fixtures.rs
pub fn create_test_session() -> ... { ... }

// In tests/common/mock_client.rs
pub struct MockClient { ... }
impl MockClient {
    pub fn new(handler: ProtocolHandler) -> Self { ... }
    pub async fn initialize(&mut self) { ... }
    pub async fn call_tool(&self, name: &str, args: Value) -> Value { ... }
}
```

### Test Fixtures

- Use `tempfile::tempdir()` for file I/O tests
- Keep fixture files in `testdata/` (Codebase) or `tests/mapping-suite/` (Vision)
- Never hardcode paths -- use `env!("CARGO_MANIFEST_DIR")`

---

## 10. Documentation Conventions

### Rustdoc Style

```rust
//! Module-level doc (inner doc comment).
//!
//! Detailed description of what this module provides.

/// Item-level doc (outer doc comment).
///
/// Longer description if needed.
///
/// # Examples
///
/// ```
/// let builder = GraphBuilder::new();
/// ```
pub fn function() { ... }
```

### Documentation Files

Both siblings maintain a `docs/` directory:

```
docs/
  quickstart.md
  concepts.md
  api-reference.md
  benchmarks.md
  faq.md
  integration-guide.md
  file-format.md      # Binary format specification
```

### Assets

```
assets/
  logo.svg
  logo-icon.svg
  benchmark-chart.svg
  architecture.svg
```

---

## 11. MCP Server Conventions

### Protocol

- JSON-RPC 2.0 over stdio (default transport)
- Newline-delimited messages
- Protocol version: `"2024-11-05"`
- Optional SSE transport behind `#[cfg(feature = "sse")]`

### Server Info

```rust
const SERVER_NAME: &str = "agentic-{name}";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
const MCP_VERSION: &str = "2024-11-05";
```

### Tool Pattern (one file per tool)

```rust
// tools/tool_name.rs

pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "tool_name".to_string(),
        description: Some("Description".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": { ... },
            "required": [...]
        }),
    }
}

pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: ToolParams = serde_json::from_value(args)?;
    // validate, execute, return
    Ok(ToolCallResult::json(&result))
}
```

### Registry Pattern (dispatch)

```rust
// tools/registry.rs
pub fn list() -> Vec<ToolDefinition> { ... }

pub async fn call(
    name: &str,
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    match name {
        "tool_a" => tool_a::execute(args, session).await,
        "tool_b" => tool_b::execute(args, session).await,
        _ => Err(McpError::ToolNotFound(name.to_string())),
    }
}
```

### MCP Module Layout

```
mcp/ (or protocol/)
  mod.rs
  handler.rs         # Main JSON-RPC dispatch
  negotiation.rs     # Capability negotiation
  validator.rs       # Request validation
tools/
  mod.rs
  registry.rs        # List + dispatch
  tool_a.rs          # definition() + execute()
resources/
  mod.rs
  registry.rs
  resource_a.rs
prompts/
  mod.rs
  registry.rs
  prompt_a.rs
transport/
  mod.rs
  framing.rs         # Newline-delimited JSON
  stdio.rs           # Stdin/stdout
  sse.rs             # HTTP/SSE (feature-gated)
types/
  mod.rs
  capabilities.rs    # Initialization, server info
  error.rs           # McpError + error codes
  message.rs         # JsonRpcRequest, JsonRpcResponse
  request.rs
  response.rs        # ToolCallResult, ToolDefinition, etc.
```

### Feature Flags for Transport

```toml
[features]
default = ["stdio"]
stdio = []
sse = ["axum", "tower", "tower-http"]
all-transports = ["stdio", "sse"]
```

---

## 12. Logging and Tracing

### Framework

- Library: `tracing` crate (NOT `log`)
- Binary: `tracing-subscriber` with `env-filter` feature

### Initialization Pattern (in binary main())

```rust
let filter = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&log_level));

tracing_subscriber::fmt()
    .with_env_filter(filter)
    .with_writer(std::io::stderr)  // CRITICAL: stderr only
    .init();
```

### CRITICAL Rule

**stdout is ALWAYS reserved for MCP JSON-RPC protocol messages.**
All logging, tracing, debug output, and error messages MUST go to **stderr**.

### Log Level Usage

```rust
tracing::info!("Starting server on {}", addr);
tracing::warn!("Model not found, using fallback");
tracing::debug!("Processing query: {:?}", params);
tracing::error!("Failed to save: {e}");
```

---

## 13. CI/CD Conventions

### GitHub Actions CI (`.github/workflows/ci.yml`)

All three repos MUST use this identical CI pattern:

```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -D warnings

jobs:
  rust:
    name: Rust (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Clippy
        run: cargo clippy --workspace --all-targets -- -D warnings

      - name: Test
        run: cargo test --workspace

      - name: Build release
        run: cargo build --workspace --release
```

### Required CI Checks

1. `RUSTFLAGS: -D warnings` env var -- compiler warnings are errors
2. `Swatinem/rust-cache@v2` -- NOT `actions/cache@v4`
3. `cargo fmt --all -- --check` -- formatting (with `--all` for workspaces)
4. `cargo clippy --workspace --all-targets -- -D warnings` -- full clippy
5. `cargo test --workspace` -- all tests pass
6. `cargo build --workspace --release` -- release build succeeds

Additional repo-specific jobs (Python SDK tests, installer tests) are fine to keep alongside the canonical Rust job.

---

## 14. Release Conventions

### GitHub Actions Release (`.github/workflows/release.yml`)

All three repos MUST use this canonical release pattern with 4 targets, tar.gz packaging, stripping, and crates.io publishing:

```yaml
name: Release
on:
  push:
    tags: ["v*"]

permissions:
  contents: write

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build (${{ matrix.target }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            triple: linux-x86_64
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
            triple: linux-aarch64
          - target: x86_64-apple-darwin
            os: macos-latest
            triple: darwin-x86_64
          - target: aarch64-apple-darwin
            os: macos-latest
            triple: darwin-aarch64

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross-compilation tools
        if: matrix.target == 'aarch64-unknown-linux-gnu'
        run: |
          sudo apt-get update
          sudo apt-get install -y gcc-aarch64-linux-gnu

      - name: Build
        run: cargo build --workspace --release --target ${{ matrix.target }}

      - name: Strip binary
        run: strip target/${{ matrix.target }}/release/{binary} || true

      - name: Package
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          ASSET="agentic-{name}-${VERSION}-${{ matrix.triple }}"
          mkdir -p "${ASSET}"
          cp target/${{ matrix.target }}/release/{binary} "${ASSET}/"
          tar czf "${ASSET}.tar.gz" "${ASSET}"

      - uses: actions/upload-artifact@v4

  release:
    needs: build
    steps:
      - uses: softprops/action-gh-release@v2
        with:
          files: artifacts/**/*.tar.gz
          generate_release_notes: true

  publish-crate:
    needs: release
    steps:
      - run: cargo publish -p {crate} --token "${{ secrets.CARGO_REGISTRY_TOKEN }}"
        continue-on-error: true
```

### Asset Naming Convention (canonical)

`agentic-{name}-{version}-{triple}.tar.gz`

Where `{triple}` is one of:
- `linux-x86_64`
- `linux-aarch64`
- `darwin-x86_64`
- `darwin-aarch64`

Examples:
- `agentic-memory-0.2.0-darwin-aarch64.tar.gz`
- `agentic-vision-mcp-0.1.1-linux-x86_64.tar.gz`
- `agentic-codebase-0.1.0-darwin-x86_64.tar.gz`

### Release Requirements

1. 4 platform targets (linux x86_64, linux aarch64, darwin x86_64, darwin aarch64)
2. tar.gz packaging (not raw binaries)
3. Binary stripping (`strip` command)
4. Cross-compilation via `gcc-aarch64-linux-gnu` for ARM Linux
5. Auto crates.io publish with `CARGO_REGISTRY_TOKEN` secret
6. `continue-on-error: true` on publish (idempotent re-publish)

---

## 15. Benchmark Conventions

### Setup

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmarks"
harness = false
```

### File Organization

```rust
// benches/benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn graph_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph");
    group.bench_function("build_1k", |b| { ... });
    group.bench_function("build_10k", |b| { ... });
    group.finish();
}

fn io_benches(c: &mut Criterion) { ... }
fn query_benches(c: &mut Criterion) { ... }

criterion_group!(benches, graph_benches, io_benches, query_benches);
criterion_main!(benches);
```

### Naming Convention

`bench_{operation}_{scale}` -- e.g., `bench_bm25_fast_100k`, `bench_add_node_10k`

### Helper Functions

```rust
fn make_large_graph(n: usize) -> Graph { ... }
fn make_small_graph() -> Graph { ... }
```

---

## 16. Configuration Conventions

### Path Resolution Priority

1. Explicit CLI flag (`--file`)
2. Environment variable (`{PROJECT}_FILE`)
3. CWD convention (`.{ext}/default.{ext}`)
4. Home directory default (`~/default.{ext}`)

### Environment Variables

| Variable | Purpose |
|:---------|:--------|
| `{PREFIX}_FILE` | Override file path |
| `{PREFIX}_TOKEN` | Auth token for SSE |
| `RUST_LOG` | Log level override |

---

## 17. Edge Case Handling

### Mandatory Boundary Checks

- Self-edges: REJECT with specific error
- Invalid IDs: Validate source/target exist before adding edge
- Value clamping: All bounded values use `.clamp(min, max)`
- String limits: Max content size, max name length
- File format: Magic byte validation, version check, truncation detection
- Empty inputs: Handle gracefully (return empty results, not errors)
- Unicode: Full UTF-8 support in all string fields

### FFI Safety (if applicable)

- `std::panic::catch_unwind` on all `extern "C"` functions
- Null pointer checks before dereferencing
- Error codes as negative integers: `OK = 0`, `ERR_IO = -1`, etc.
- `#[no_mangle]` on all exported functions

---

## 18. Code Style

### Naming Conventions

| Entity | Style | Example |
|:-------|:------|:--------|
| Types | PascalCase | `CodeGraph`, `AcbError` |
| Functions | snake_case | `add_node`, `build_graph` |
| Constants | SCREAMING_SNAKE_CASE | `MAGIC_BYTES`, `MAX_SIZE` |
| Modules | snake_case | `code_graph`, `symbol_index` |
| Enum variants | PascalCase | `EventType::Fact` |
| CLI binary | kebab-case | `acb`, `acb-mcp` |
| Crate names | kebab-case | `agentic-codebase` |
| Library names | snake_case | `agentic_codebase` |

### Import Ordering

1. `std` imports
2. External crate imports
3. Internal crate imports (`crate::`, `super::`)
4. Blank lines between groups

### Enum Representation

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[repr(u8)]
pub enum TypeName {
    Variant = 0,
    ...
}
```

Each enum provides: `from_u8()`, `from_name()`, `name()`, `Display` impl.

### Type Aliases

```rust
pub type AcbResult<T> = Result<T, AcbError>;
```

---

## 19. Community Files

### GitHub Issue Templates

`.github/ISSUE_TEMPLATE/bug_report.md`:
```yaml
name: Bug Report
about: Report a bug
title: "[BUG] "
labels: bug
```
Fields: Description, Steps to Reproduce, Expected, Actual, Environment.

`.github/ISSUE_TEMPLATE/feature_request.md`:
```yaml
name: Feature Request
about: Suggest a feature
title: "[FEATURE] "
labels: enhancement
```
Fields: Problem, Proposed Solution.

### Pull Request Template

`.github/PULL_REQUEST_TEMPLATE.md`:
```markdown
## What
Brief description.

## Type
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation

## Testing
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] Lints pass
```

### Funding

`.github/FUNDING.yml`:
```yaml
github: [agentralabs]
```

---

## 20. Paper and Publication

### Directory Structure

```
paper/
  paper-i-{name}/
    {name}-paper.tex
    references.bib
    {name}-paper.pdf
  paper-ii-{name}/
    ...
```

### LaTeX Conventions (from all three papers)

- Two-column, 10pt article
- 0.75in margins
- Color palette: `amblue`, `amteal`, `amorange`, `amgray`, `amgreen`, `amred`, `ampurple`
- ORCID icon macro (inline TikZ)
- fancyhdr with project name + page numbers
- BibTeX with `references.bib`
- All figures in TikZ/pgfplots (no external images)
- All tables with booktabs (`\toprule`, `\midrule`, `\bottomrule`)

---

## 21. Shell Completions

Every CLI binary MUST support shell completion generation via `clap_complete`.

### Required Subcommand

```rust
/// Generate shell completions.
Completions {
    /// Shell type (bash, zsh, fish, powershell, elvish).
    shell: Shell,
},
```

### Implementation Pattern

```rust
use clap::CommandFactory;
use clap_complete::Shell;

Some(Command::Completions { shell }) => {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "{binary}", &mut std::io::stdout());
}
```

### Usage

```bash
# One-time setup (example for zsh)
{binary} completions zsh >> ~/.zshrc

# Or pipe to a completions directory
{binary} completions bash > /etc/bash_completion.d/{binary}
```

---

## 22. Interactive REPL

Every CLI binary SHOULD launch an interactive REPL when invoked with no subcommand.

### Dependencies

```toml
rustyline = "14"
```

### Module Structure

```
src/cli/
  repl.rs              # Main REPL loop (welcome banner, readline, history)
  repl_commands.rs     # Slash command dispatch + ReplState
  repl_complete.rs     # Tab completion helper (commands, files, types)
```

### REPL Entry Point Pattern

```rust
// In command dispatch:
None => crate::cli::repl::run(),  // No subcommand => launch REPL
```

### Required Features

- **Tab completion**: Commands, file paths (`.acb`, `.amem`, `.avis`), query types
- **Ghost-text hinting**: Inline suggestion of matching commands
- **History**: Persistent across sessions (`~/.{binary}_history`)
- **Colored prompt**: Respects `NO_COLOR` env var
- **Slash commands**: `/help`, `/exit`, `/clear`, plus domain-specific commands
- **Fuzzy suggestion**: Levenshtein distance "did you mean?" on misspelled commands

### Tab Completion Helper

```rust
struct {Project}Helper;

impl Completer for {Project}Helper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, _ctx: &Context) -> Result<(usize, Vec<Pair>)> {
        // 1. Command completion (if no space in input)
        // 2. Context-aware completion (files, types, etc.)
    }
}

impl Hinter for {Project}Helper { /* ghost text */ }
impl Highlighter for {Project}Helper {}
impl Validator for {Project}Helper {}
impl Helper for {Project}Helper {}
```

### Smart Tab Event Handler

```rust
struct TabCompleteOrAcceptHint;

impl ConditionalEventHandler for TabCompleteOrAcceptHint {
    fn handle(&self, _evt: &Event, _n: RepeatCount, _positive: bool, ctx: &EventContext) -> Option<Cmd> {
        if ctx.has_hint() {
            Some(Cmd::CompleteHint)  // Accept ghost text
        } else {
            Some(Cmd::Complete)      // Show completion list
        }
    }
}
```

### REPL State

```rust
struct ReplState {
    // Domain-specific state, e.g.:
    graph: Option<CodeGraph>,     // For ACB
    file_path: Option<PathBuf>,   // For amem/vision
}
```

---

## 23. FFI Bindings

Every library crate SHOULD provide C-compatible FFI bindings for cross-language integration.

### Cargo.toml

```toml
[lib]
crate-type = ["lib", "cdylib", "staticlib"]
```

### Module Structure

```
src/ffi/
  mod.rs       # pub mod c_api;
  c_api.rs     # All #[no_mangle] extern "C" functions
```

### Error Codes (ecosystem standard)

```rust
pub const {PREFIX}_OK: i32 = 0;
pub const {PREFIX}_ERR_IO: i32 = -1;
pub const {PREFIX}_ERR_INVALID: i32 = -2;
pub const {PREFIX}_ERR_NOT_FOUND: i32 = -3;
pub const {PREFIX}_ERR_OVERFLOW: i32 = -4;
pub const {PREFIX}_ERR_NULL_PTR: i32 = -5;
```

### Safety Pattern

Every FFI function MUST:

1. Use `#[no_mangle]` and `extern "C"`
2. Wrap all logic in `std::panic::catch_unwind()`
3. Check all pointers for null before dereferencing
4. Include `/// # Safety` doc section describing invariants
5. Return error codes (never panic across FFI boundary)
6. Use opaque `*mut c_void` handles (never expose Rust types)

### Function Naming

`{prefix}_graph_{operation}` -- e.g., `acb_graph_open`, `amem_graph_new`, `acb_graph_free`.

---

## 24. CI/CD Pipeline (canonical)

### Canonical CI Job Structure

The `rust` job is required and must be identical across all repos. Additional repo-specific jobs (Python SDK, installer tests) are optional:

```
ci.yml
├── rust (REQUIRED — identical across all repos)
│   ├── fmt check
│   ├── clippy (workspace + all-targets)
│   ├── test (workspace)
│   └── build release (workspace)
├── python-sdk-tests (OPTIONAL — AMem only)
└── installer-tests (OPTIONAL — AMem only)
```

### Key Requirements

- `RUSTFLAGS: -D warnings` as global env var
- `Swatinem/rust-cache@v2` for caching (NOT `actions/cache@v4`)
- Rust toolchain components: `clippy, rustfmt`
- Matrix: `[ubuntu-latest, macos-latest]`

### Makefile (canonical)

Every repo MUST have a Makefile with these targets:

```makefile
.PHONY: all build build-debug test test-unit test-integration lint lint-fmt lint-clippy bench clean install

all: build
build:            cargo build --workspace --release
build-debug:      cargo build --workspace
test:             test-unit test-integration
test-unit:        cargo test --lib
test-integration: cargo test --tests
lint:             lint-fmt lint-clippy
lint-fmt:         cargo fmt --all -- --check
lint-clippy:      cargo clippy --workspace --all-targets -- -D warnings
bench:            cargo bench
clean:            cargo clean
install:          cargo install --path .
```

---

## 25. Install Scripts (canonical)

### Canonical Pattern

All install scripts MUST follow this structure:

```bash
#!/bin/bash
# Agentic{Product} — one-liner install script
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/agentralabs/{repo}/main/scripts/install.sh | bash
# Future:
#   curl -fsSL https://agentralabs.tech/install/{product} | sh
```

### Required Features

| Feature | Implementation |
|:--------|:---------------|
| Named args | `--version=X.Y.Z`, `--dir=/path`, `--dry-run`, `--help` |
| Platform detection | `darwin-x86_64`, `darwin-aarch64`, `linux-x86_64`, `linux-aarch64` |
| Dependencies | `curl`, `jq` (checked with helpful install hints) |
| Download format | tar.gz extraction (matching release workflow) |
| Config merge | jq-based, non-destructive merge into `mcpServers` |
| Claude Desktop | `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) |
| Claude Desktop (Linux) | `${XDG_CONFIG_HOME:-$HOME/.config}/Claude/claude_desktop_config.json` |
| Claude Code | `$HOME/.claude/mcp.json` |
| PATH check | Warn if `~/.local/bin` not in PATH |
| Install dir | `$HOME/.local/bin` (default) |

### Config Merge Pattern

```bash
merge_config() {
    local config_file="$1"
    if [ -f "$config_file" ] && [ -s "$config_file" ]; then
        jq --arg key "$SERVER_KEY" --arg cmd "${INSTALL_DIR}/${BINARY_NAME}" \
           '.mcpServers //= {} | .mcpServers[$key] = {"command": $cmd, "args": ["serve"]}' \
           "$config_file" > "$config_file.tmp" && mv "$config_file.tmp" "$config_file"
    else
        jq -n --arg key "$SERVER_KEY" --arg cmd "${INSTALL_DIR}/${BINARY_NAME}" \
           '{ "mcpServers": { ($key): { "command": $cmd, "args": ["serve"] } } }' \
           > "$config_file"
    fi
}
```

### Future Domain

When `agentralabs.tech` is live, install URLs will be:
- `curl -fsSL https://agentralabs.tech/install/memory | sh`
- `curl -fsSL https://agentralabs.tech/install/vision | sh`
- `curl -fsSL https://agentralabs.tech/install/codebase | sh`

---

## 26. Publishing

### scripts/publish.sh (canonical)

Every repo MUST have `scripts/publish.sh` with pre-publish checks:

```bash
#!/bin/bash
set -euo pipefail

echo "1. Running tests..."
cargo test --workspace

echo "2. Checking formatting..."
cargo fmt --all -- --check

echo "3. Running clippy..."
cargo clippy --workspace -- -D warnings

echo "4. Dry-run publish..."
# For single-crate repos:
cargo publish --dry-run
# For workspace repos:
cargo publish -p {core-crate} --dry-run
cargo publish -p {mcp-crate} --dry-run

echo "All checks passed!"
echo "To publish:"
echo "  cargo publish -p {core-crate}"
echo "  cargo publish -p {mcp-crate}"
```

### Publish Order (workspace repos)

1. Core library first (`agentic-memory`, `agentic-vision`)
2. Wait for crates.io index to update
3. MCP server second (`agentic-memory-mcp`, `agentic-vision-mcp`)

### Automated Publishing

Release workflows include a `publish-crate` job that runs after the GitHub release is created, using `CARGO_REGISTRY_TOKEN` secret with `continue-on-error: true`.

### Cargo.toml URLs (canonical)

```toml
repository = "https://github.com/agentralabs/agentic-{name}"
homepage = "https://agentralabs.tech"
documentation = "https://docs.rs/agentic-{name}"
```

---

## Gap Analysis: AgenticCodebase vs Siblings

> **Status: ALL GAPS CLOSED** (as of 2026-02-20)

### CRITICAL (Tier 1) -- All Closed

| # | Gap | Status |
|:-:|:----|:------:|
| 1 | `.gitignore` | CLOSED |
| 2 | `LICENSE` file | CLOSED |
| 3 | `README.md` (badges, TOC, quickstart) | CLOSED |
| 4 | Git initialization | CLOSED |
| 5 | `.github/workflows/ci.yml` | CLOSED |
| 6 | `.github/workflows/release.yml` | CLOSED |
| 7 | `CHANGELOG.md` | CLOSED |
| 8 | Tracing (not log) | CLOSED |
| 9 | Log subscriber init | CLOSED |

### HIGH (Tier 1) -- All Closed

| # | Gap | Status |
|:-:|:----|:------:|
| 10 | `CONTRIBUTING.md` | CLOSED |
| 11 | `CODE_OF_CONDUCT.md` | CLOSED |
| 12 | `SECURITY.md` | CLOSED |
| 13 | `.github/ISSUE_TEMPLATE/` | CLOSED |
| 14 | `.github/PULL_REQUEST_TEMPLATE.md` | CLOSED |
| 15 | `.github/FUNDING.yml` | CLOSED |
| 16 | `scripts/install.sh` | CLOSED |
| 17 | `scripts/publish.sh` | CLOSED |
| 18 | Edge case integration tests | CLOSED (46 tests) |
| 19 | MCP edge case tests | CLOSED (28 tests) |
| 20 | Test common utilities | CLOSED |
| 21 | `docs/` directory | CLOSED (7 files) |
| 22 | `assets/` directory | CLOSED |
| 23 | Unused dev-deps cleanup | CLOSED |
| 24 | `homepage`/`documentation` in Cargo.toml | CLOSED |
| 25 | `exclude` in Cargo.toml | CLOSED |

### MEDIUM (Tier 2) -- All Closed

| # | Gap | Status |
|:-:|:----|:------:|
| 26 | `examples/` directory | CLOSED (7 examples) |
| 27 | Python integration tests | N/A (Rust-only) |
| 28 | `Makefile` | CLOSED |
| 29 | Feature flags in Cargo.toml | CLOSED (`stdio`, `sse`, `all-transports`) |
| 30 | SSE transport for MCP | CLOSED (`src/mcp/sse.rs`) |
| 31 | Multi-tenant MCP support | CLOSED (`src/mcp/tenant.rs`) |
| 32 | Config file support (TOML) | CLOSED (`src/config/loader.rs`) |
| 33 | Environment variable path resolution | CLOSED (`ACB_GRAPH` env var) |
| 34 | `#[repr(u8)]` on enums | CLOSED |
| 35 | Stub modules (compile, incremental, c_api) | CLOSED (all implemented) |
| 36 | `[profile.release]` settings | CLOSED |

### Additional Conventions (added 2026-02-20)

| # | Convention | Status |
|:-:|:-----------|:------:|
| 37 | Shell completions (`clap_complete`) | CLOSED (all 3 repos) |
| 38 | Interactive REPL (`rustyline`) | CLOSED (all 3 repos) |
| 39 | C FFI bindings (`src/ffi/c_api.rs`) | CLOSED |

---

## 27. Standalone Sister Independence (canonical)

This rule is canonical for all current and future Agentic-family repos.

### Core Requirement

Each sister project MUST remain independently installable and operable:

- AgenticMemory alone must work.
- AgenticVision alone must work.
- AgenticCodebase alone must work.

### Non-negotiable Constraints

1. A sister MUST NOT require another sister as a hard runtime dependency.
2. Cross-sister behavior MUST be optional adapter-based integration.
3. If another sister is missing, behavior MUST degrade gracefully, not fail startup.
4. Autonomic operations (tiering, sleep-cycle maintenance, backup, migration) MUST run local-first inside the current repo.
5. Any future unified OS layer MUST preserve standalone mode.

### Documentation Contract

Public docs (README, install docs, integration docs) MUST preserve a clear standalone guarantee statement.

### PR Conformance Questions

- Can a user install and run this repo alone after this change?
- Does this change introduce hard runtime dependency on another sister?
- Does absence of another sister break core workflows?
- Are lifecycle tasks still valid in standalone mode?

If any answer is "no", the change is non-conformant and must be revised.

---

## 28. Autonomic Operations Defaults (canonical)

This section defines the minimum canonical autonomic behavior for all Agentic-family repos.

### Objective

Users should not manage day-to-day storage and lifecycle hygiene manually.

### Required Baseline

Each repo MUST provide, by default (without required user config):

1. Periodic background maintenance hooks.
2. Automatic preservation safeguards before destructive overwrite-style mutations.
3. Automatic cleanup/retention pruning for maintenance artifacts.
4. Environment-variable overrides for operators, while keeping safe defaults.
5. Profile-based operations (`desktop|cloud|aggressive`) and policy-gated storage migration (`auto-safe|strict|off`).

### Independence Constraint

Autonomic behavior MUST execute local-first inside the current repo and MUST NOT require another sister to be installed or running.

### Phase 4 Canonical Mapping

- AgenticMemory:
  - Autonomous maintenance loop in MCP runtime.
  - Periodic save checks plus rolling backup snapshots with retention pruning.
  - Sleep-cycle maintenance (decay refresh, tier balancing, completed-session auto-archive).
  - Profile-driven default posture.
  - Migration policy gate: auto-safe, strict, off.
  - Legacy `.amem` checkpoint + auto-migration path.
  - SLA-aware maintenance throttling under sustained mutation pressure.
  - Health-ledger snapshot output with shared directory contract.
- AgenticVision:
  - Daemon maintenance loop for expired cache cleanup and registry delta GC.
  - Profile-driven default posture.
  - Migration policy gate: auto-safe, strict, off.
  - Legacy `.ctx` cache auto-migration on load.
  - SLA-aware GC throttling under sustained cache pressure.
  - Health-ledger snapshot output with shared directory contract.
- AgenticCodebase:
  - Automatic pre-write backup on `.acb` overwrite paths.
  - Periodic collective cache eviction maintenance.
  - Profile-driven default posture.
  - Migration policy gate: auto-safe, strict, off.
  - Legacy `.acb` checkpoint + auto-migration on read.
  - SLA-aware maintenance throttling under sustained registry operation load.
  - Health-ledger snapshot output for both CLI and collective maintenance.

### PR Conformance Questions

- Does the change reduce routine user maintenance burden?
- Is there a safe automatic fallback path if user configuration is absent?
- Are retention and backup policies bounded (no unbounded growth)?
- Does the repo remain fully functional standalone after this change?

If any answer is "no", revise before merge.
