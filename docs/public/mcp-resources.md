---
status: stable
---

# MCP Resources

AgenticCodebase exposes code graph data through the `acb://` URI scheme in the MCP Resources API.

## URI Scheme

All resources use the `acb://` prefix. Resources are dynamically generated based on loaded graphs.

## Available Resources

### `acb://graphs/{name}/stats`

Return statistics for a loaded code graph.

**Format:** JSON object.

```json
{
  "unit_count": 1842,
  "edge_count": 7653,
  "dimension": 256
}
```

**MIME type:** `application/json`

### `acb://graphs/{name}/units`

Return all code units in a loaded graph.

**Format:** JSON array of unit objects.

```json
[
  {
    "id": 0,
    "name": "UserService",
    "type": "function",
    "file": "src/services/user.rs"
  },
  {
    "id": 1,
    "name": "authenticate",
    "type": "function",
    "file": "src/auth/mod.rs"
  }
]
```

**MIME type:** `application/json`

## Resource Discovery

Resources are listed dynamically based on which graphs are loaded. Each loaded graph produces two resources:

| URI Pattern | Description |
|-------------|-------------|
| `acb://graphs/{name}/stats` | Statistics for the named graph |
| `acb://graphs/{name}/units` | All code units in the named graph |

For example, if a graph named `myproject` is loaded, the server exposes:
- `acb://graphs/myproject/stats`
- `acb://graphs/myproject/units`

## Error Handling

| Condition | Response |
|-----------|----------|
| Missing `uri` field | JSON-RPC invalid params error |
| Unknown graph name | JSON-RPC invalid params error: "Graph not found: {name}" |
| Unknown resource type | JSON-RPC invalid params error: "Unknown resource type: {type}" |
| Malformed URI | JSON-RPC invalid params error: "Invalid resource URI: {uri}" |

## Cross-Sister Resources

When running alongside other Agentra sisters, AgenticCodebase resources can be referenced in memory nodes and temporal data:

- Memory nodes can link to `acb://graphs/{name}/units` for code context
- Time sequences can reference code unit IDs from graph stats
- Vision captures can link to code units for visual documentation of architecture
