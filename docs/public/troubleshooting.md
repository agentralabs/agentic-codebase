---
status: stable
---

# Troubleshooting

Common issues and solutions for AgenticCodebase.

## Installation Issues

### Binary not found after install

Ensure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
# Add to ~/.bashrc or ~/.zshrc for persistence
```

### Install script fails with "jq not found"

The installer needs `jq` or `python3` for MCP config merging:

```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt install jq

# Or use python3 (usually pre-installed)
python3 --version
```

### Cargo build fails

Ensure you have the latest stable Rust toolchain. Tree-sitter grammars require a C compiler:

```bash
rustup update stable

# macOS: ensure Xcode command-line tools are installed
xcode-select --install

# Linux: ensure build-essential is installed
sudo apt install build-essential
```

## MCP Server Issues

### Server not appearing in MCP client

1. Verify the binary exists: `ls ~/.local/bin/agentic-codebase-mcp`
2. Check config was merged: look for `agentic-codebase` in your MCP client config
3. Restart your MCP client completely (not just reload)
4. Run manually to check for errors: `agentic-codebase-mcp serve`

### "No graphs loaded" error when calling tools

The server could not auto-resolve a graph. Fix by either:

```bash
# Option 1: Pre-load a graph explicitly
agentic-codebase-mcp --graph /path/to/project.acb serve

# Option 2: Set the workspace root
export AGENTRA_WORKSPACE_ROOT=/path/to/your/repo

# Option 3: Set the graph path directly
export ACB_GRAPH=/path/to/project.acb

# Option 4: Run from within a git repository
cd /path/to/your/repo && agentic-codebase-mcp serve
```

### Auto-compilation fails silently

The MCP server auto-compiles graphs when starting in a repository. If it fails:

1. Check stderr output: `agentic-codebase-mcp serve 2>/tmp/acb-err.log`
2. Verify the repo root is not `/`, `$HOME`, or `$HOME/Documents`
3. Check disk space in the cache directory: `ls -la ~/.codex/graphs/`
4. Try manual compilation: `acb compile /path/to/repo -o test.acb`

### Server crashes on startup

Check for stale lock files:

```bash
ls -la ~/.codex/graphs/*.lock
# Remove stale locks (check PID first)
rm -rf ~/.codex/graphs/*.lock
```

### SSE transport "AGENTIC_TOKEN required" error

Set the token for SSE authentication:

```bash
export AGENTIC_TOKEN="$(openssl rand -hex 32)"
```

## File Format Issues

### "Invalid magic bytes" error

The file is not a valid `.acb` file. Check:

```bash
xxd -l 4 project.acb
# Should show: 4143 4442 (ACDB)
```

### "Unsupported version" error

The file was created with a newer version of AgenticCodebase. Update your installation:

```bash
curl -fsSL https://agentralabs.tech/install/codebase | bash
```

### Stale graph (missing new symbols)

The cached graph may be outdated. Force a recompile:

```bash
# Delete the cached graph
rm ~/.codex/graphs/*.acb

# Or recompile manually
acb compile /path/to/repo -o project.acb
```

## Query Issues

### "Unit not found" for a known symbol

Symbol lookup uses the short name, not the fully qualified path:

```bash
# Use the symbol name, not the file path
acb query project.acb symbol --name "authenticate"

# Try contains mode for partial matches
# (In MCP, use mode: "contains" in symbol_lookup)
```

### Impact analysis returns empty results

Ensure the unit ID is valid:

```bash
# First find the unit ID
acb query project.acb symbol --name "MyFunction"
# Note the ID, then run impact analysis
acb query project.acb impact --unit-id 42
```

### Query results seem incomplete

Increase the limit and depth:

```bash
acb query project.acb deps --unit-id 42 --depth 10 --limit 100
```

## Compilation Issues

### Large repository takes too long to compile

Exclude unnecessary directories:

```bash
acb compile ./src --exclude="*test*" --exclude="vendor" --exclude="node_modules"
```

The compiler automatically skips `.git`, `target`, `node_modules`, `.venv`, `venv`, `dist`, `build`, `.next`, and `.cache` directories.

### Unsupported language files are ignored

AgenticCodebase supports: Python (`.py`), Rust (`.rs`), TypeScript (`.ts`, `.tsx`), JavaScript (`.js`, `.jsx`), Go (`.go`), C++ (`.c`, `.cc`, `.cpp`, `.h`, `.hpp`), and Java (`.java`). Other file types are silently skipped during compilation.

## Performance Issues

### High memory usage during compilation

Large codebases with many symbols consume more memory. Consider:

1. Compiling subdirectories separately
2. Using `--exclude` to skip generated code
3. Ensuring LZ4 compression is working (default)

### Slow startup with large .acb files

For graphs with many thousands of units:

1. Ensure the `.acb` file is on a fast local disk (not network mount)
2. Memory mapping (`memmap2`) handles large files efficiently but requires sufficient virtual address space
3. Consider splitting into per-service graphs and using workspaces

## Getting Help

- GitHub Issues: https://github.com/agentralabs/agentic-codebase/issues
- Documentation: https://agentralabs.tech/docs/codebase
