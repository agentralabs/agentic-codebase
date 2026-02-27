---
status: stable
---

# Configuration

AgenticCodebase configuration options for all runtime modes.

## Environment Variables

| Variable | Default | Allowed Values | Effect |
|----------|---------|----------------|--------|
| `ACB_GRAPH` | Auto-detected | Path to `.acb` file | Explicit graph file path |
| `ACB_LOG` | `warn` | `trace`, `debug`, `info`, `warn`, `error` | Logging verbosity (via `RUST_LOG`) |
| `AGENTRA_WORKSPACE_ROOT` | Auto-detected | Directory path | Override workspace root detection |
| `AGENTRA_PROJECT_ROOT` | Auto-detected | Directory path | Override project root detection |
| `AGENTRA_GRAPH_CACHE_DIR` | `~/.codex/graphs` | Directory path | Override graph cache directory |
| `AGENTRA_GRAPH_LOCK_WAIT_SECS` | `90` | Positive integer | Max seconds to wait for graph build lock |
| `AGENTRA_GRAPH_LOCK_STALE_SECS` | `300` | Positive integer | Seconds before a lock is considered stale |
| `AGENTIC_TOKEN` | None | String | Bearer token for SSE transport authentication |
| `NO_COLOR` | None | `1` | Disable colored terminal output |

## MCP Server Configuration

The MCP server (`agentic-codebase-mcp`) accepts the following arguments:

```json
{
  "mcpServers": {
    "agentic-codebase": {
      "command": "~/.local/bin/agentic-codebase-mcp",
      "args": ["serve"]
    }
  }
}
```

### Pre-loading a Graph

```json
{
  "mcpServers": {
    "agentic-codebase": {
      "command": "~/.local/bin/agentic-codebase-mcp",
      "args": ["serve", "--graph", "/path/to/project.acb", "--name", "myproject"]
    }
  }
}
```

### Using a TOML Config File

```json
{
  "mcpServers": {
    "agentic-codebase": {
      "command": "~/.local/bin/agentic-codebase-mcp",
      "args": ["--config", "/path/to/config.toml", "serve"]
    }
  }
}
```

### TOML Config Format

```toml
graph_path = "/path/to/project.acb"
transport = "stdio"          # "stdio" or "sse"
sse_addr = "127.0.0.1:3000"  # only used with transport = "sse"
log_level = "warn"           # trace, debug, info, warn, error
```

## MCP Server Arguments

| Argument | Description |
|----------|-------------|
| `--config <path>` | Path to a TOML config file (global) |
| `--graph <path>` | Path to an `.acb` graph file to pre-load (global) |
| `--name <name>` | Name for the pre-loaded graph (global, default: filename stem) |
| `serve` | Run the MCP server on stdin/stdout (default) |
| `serve-http` | Run the MCP server over HTTP/SSE (requires `sse` feature) |

### SSE Transport Arguments

| Argument | Description |
|----------|-------------|
| `--addr <host:port>` | Listen address (default: `127.0.0.1:3000`) |
| `--multi-tenant` | Enable multi-tenant mode (routes by X-User-ID header) |
| `--data-dir <path>` | Data directory for multi-tenant graph files (required with `--multi-tenant`) |
| `--token <token>` | Bearer token for authentication (or use `AGENTIC_TOKEN` env var) |

## Graph File Resolution

AgenticCodebase resolves the `.acb` file in this order:

1. Explicit `--graph` CLI argument
2. `ACB_GRAPH` environment variable
3. `.acb/graph.acb` in the current directory
4. `~/.agentic-codebase/graph.acb` (global default)

## Auto-Graph Resolution (MCP Server)

When the MCP server starts without an explicit graph path, it auto-resolves the repository root and compiles a graph:

1. Check `AGENTRA_WORKSPACE_ROOT` environment variable
2. Check `AGENTRA_PROJECT_ROOT` environment variable
3. Run `git rev-parse --show-toplevel` to find the git root
4. Fall back to the current working directory
5. Skip if the resolved path is `/`, `$HOME`, `$HOME/Documents`, or `$HOME/Desktop`
6. Compute a deterministic cache key: `<slug>-<sha256-hash-12chars>.acb`
7. Store the compiled graph in `AGENTRA_GRAPH_CACHE_DIR` (default: `~/.codex/graphs`)
8. Recompile only if any source file is newer than the cached graph

## Storage Budget

The `acb budget` command projects long-horizon storage usage. Configuration:

| Variable | Default | Effect |
|----------|---------|--------|
| `ACB_STORAGE_BUDGET_MODE` | `auto-rollup` | `auto-rollup`, `warn`, or `off` |

Modes:
- **auto-rollup**: Automatically compact older graph snapshots when budget is exceeded
- **warn**: Log a warning but take no action
- **off**: Disable budget tracking entirely
