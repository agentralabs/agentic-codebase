# SPEC-MCP.md

> **MCP is the primary interface.** The library exists to serve the MCP server.

---

## MCP Server Overview

AgenticCodebase exposes a Model Context Protocol server that allows LLMs to:
1. **Compile** repositories into semantic graphs
2. **Query** the graph using 24 query types
3. **Navigate** concepts, not files
4. **Predict** impact of changes
5. **Access** collective intelligence about libraries

---

## Protocol Compliance

- **MCP Version**: 2024-11-05
- **Transport**: stdio (JSON-RPC 2.0 over stdin/stdout)
- **Server Name**: `agentic-codebase`
- **Server Version**: Matches crate version

---

## Tools (Actions the LLM Can Take)

### Core Compilation Tools

| Tool | Purpose | Required Params |
|------|---------|-----------------|
| `compile` | Compile a repository into .acb | `path` |
| `compile_incremental` | Update existing .acb with changes | `acb_path`, `changes` |
| `load` | Load an existing .acb file | `acb_path` |
| `unload` | Unload a graph from memory | `graph_id` |

### Query Tools (All 24 Query Types)

| Tool | Purpose | Required Params |
|------|---------|-----------------|
| `symbol_lookup` | Find a symbol by name | `name` |
| `dependency_graph` | What does this depend on? | `unit_id`, `depth` |
| `reverse_dependency` | What depends on this? | `unit_id`, `depth` |
| `call_graph` | Call relationships | `unit_id`, `direction` |
| `type_hierarchy` | Inheritance tree | `unit_id`, `direction` |
| `containment` | What's inside this module? | `unit_id` |
| `pattern_match` | Find code matching structure | `pattern` |
| `semantic_search` | Find code by description | `query`, `top_k` |
| `impact_analysis` | What breaks if I change this? | `unit_id` |
| `test_coverage` | What tests cover this? | `unit_id` |
| `cross_language_trace` | Trace across FFI | `unit_id` |
| `collective_patterns` | How do experts use this? | `library`, `symbol` |
| `temporal_evolution` | How has this changed? | `unit_id` |
| `stability_analysis` | How risky is changing this? | `unit_id` |
| `coupling_detection` | What's secretly coupled? | `unit_id` or `all` |
| `dead_code` | What's never executed? | `scope` |
| `prophecy` | What will break next? | `scope` |
| `concept_mapping` | Where is X implemented? | `concept` |
| `migration_path` | How to safely change X to Y? | `from`, `to` |
| `test_gap` | What changes lack tests? | `since` |
| `architectural_drift` | Is codebase diverging from design? | `baseline` |
| `similarity` | What code is similar? | `unit_id`, `top_k` |
| `shortest_path` | How are two symbols connected? | `from_id`, `to_id` |
| `hotspot_detection` | Where do bugs cluster? | `scope` |

### Navigation Tools

| Tool | Purpose | Required Params |
|------|---------|-----------------|
| `goto_definition` | Jump to symbol definition | `file`, `line`, `col` |
| `find_references` | All references to a symbol | `unit_id` |
| `get_context` | Full context around a symbol | `unit_id`, `depth` |

---

## Tool Specifications

### `compile`

Compile a repository into a semantic code graph.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "path": {
      "type": "string",
      "description": "Path to repository root"
    },
    "output": {
      "type": "string",
      "description": "Path for .acb output file (optional, defaults to <repo>/.acb)"
    },
    "languages": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Languages to include (optional, defaults to all supported)"
    },
    "exclude": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Glob patterns to exclude"
    },
    "include_tests": {
      "type": "boolean",
      "default": true,
      "description": "Include test files"
    },
    "git_history": {
      "type": "boolean",
      "default": true,
      "description": "Extract git history for temporal analysis"
    }
  },
  "required": ["path"]
}
```

**Output:**
```json
{
  "graph_id": "g_abc123",
  "acb_path": "/path/to/repo/.acb",
  "stats": {
    "unit_count": 12847,
    "edge_count": 43291,
    "languages": {"python": 8234, "typescript": 4613},
    "compile_time_ms": 2341
  }
}
```

### `impact_analysis`

Determine what would be affected by changing a code unit.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "graph_id": {
      "type": "string",
      "description": "Graph to query"
    },
    "unit_id": {
      "type": "integer",
      "description": "Code unit to analyze"
    },
    "depth": {
      "type": "integer",
      "default": 5,
      "description": "Max traversal depth"
    },
    "include_tests": {
      "type": "boolean",
      "default": true,
      "description": "Include affected tests"
    }
  },
  "required": ["graph_id", "unit_id"]
}
```

**Output:**
```json
{
  "direct_dependents": [
    {"id": 234, "name": "PaymentHandler", "type": "function", "risk": "high"}
  ],
  "transitive_dependents": [
    {"id": 567, "name": "CheckoutFlow", "type": "function", "risk": "medium"}
  ],
  "affected_tests": [
    {"id": 890, "name": "test_payment_success", "coverage": "direct"}
  ],
  "risk_summary": {
    "total_affected": 23,
    "high_risk": 5,
    "medium_risk": 12,
    "low_risk": 6
  },
  "recommendation": "Consider adding tests for PaymentHandler before modifying"
}
```

### `prophecy`

Predict what will break based on historical patterns.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "graph_id": { "type": "string" },
    "scope": {
      "type": "string",
      "enum": ["all", "module", "file"],
      "default": "all"
    },
    "module_path": {
      "type": "string",
      "description": "If scope=module, which module"
    },
    "time_horizon_days": {
      "type": "integer",
      "default": 30
    }
  },
  "required": ["graph_id"]
}
```

**Output:**
```json
{
  "predictions": [
    {
      "unit_id": 123,
      "name": "payments/stripe.py",
      "prediction": "likely_to_break",
      "confidence": 0.78,
      "reasoning": "Changed 12 times in 30 days, 8 resulted in bugfixes",
      "estimated_days_to_incident": 14,
      "recommendation": "Refactor into smaller units"
    }
  ],
  "ecosystem_alerts": [
    {
      "library": "fastapi",
      "current_version": "0.104.1",
      "alert": "Version 0.105.0 broke 23% of similar codebases",
      "recommendation": "Pin version, delay upgrade"
    }
  ]
}
```

### `collective_patterns`

Query collective intelligence about how libraries are used.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "graph_id": { "type": "string" },
    "library": {
      "type": "string",
      "description": "Library name (e.g., 'fastapi', 'react')"
    },
    "symbol": {
      "type": "string",
      "description": "Optional specific symbol within library"
    },
    "pattern_type": {
      "type": "string",
      "enum": ["usage", "mistakes", "performance", "migration"],
      "default": "usage"
    }
  },
  "required": ["graph_id", "library"]
}
```

**Output:**
```json
{
  "library": "fastapi",
  "collective_data": {
    "total_analyses": 847293,
    "last_updated": "2026-02-19T10:00:00Z"
  },
  "patterns": [
    {
      "pattern": "dependency_injection",
      "frequency": 0.89,
      "example": "from fastapi import Depends\n...",
      "recommendation": "Use Depends for shared resources"
    }
  ],
  "common_mistakes": [
    {
      "mistake": "sync_in_async_route",
      "frequency": 0.34,
      "description": "Calling synchronous DB code in async route",
      "fix": "Use async DB driver or run_in_threadpool"
    }
  ],
  "your_usage_score": 0.82,
  "improvement_suggestions": [
    "Consider using Depends for your db_session parameter"
  ]
}
```

---

## Resources (Read-Only Data)

### URI Patterns

| URI Pattern | Returns |
|-------------|---------|
| `acb://graph/{graph_id}/stats` | Graph statistics |
| `acb://graph/{graph_id}/unit/{unit_id}` | Full code unit details |
| `acb://graph/{graph_id}/unit/{unit_id}/source` | Source code snippet |
| `acb://graph/{graph_id}/unit/{unit_id}/edges` | All edges for unit |
| `acb://graph/{graph_id}/modules` | List of all modules |
| `acb://graph/{graph_id}/module/{path}` | Module contents |
| `acb://graph/{graph_id}/languages` | Language breakdown |
| `acb://graph/{graph_id}/stability` | Stability report |
| `acb://collective/{library}` | Collective data for library |
| `acb://collective/{library}/{symbol}` | Collective data for symbol |

### Resource: `acb://graph/{graph_id}/unit/{unit_id}`

Returns complete information about a code unit:

```json
{
  "uri": "acb://graph/g_abc123/unit/4521",
  "mimeType": "application/json",
  "data": {
    "id": 4521,
    "type": "function",
    "language": "python",
    "name": "process_payment",
    "qualified_name": "payments.stripe.process_payment",
    "file": "payments/stripe.py",
    "span": {"start": [45, 0], "end": [89, 0]},
    "signature": "(amount: Decimal, currency: str) -> PaymentResult",
    "doc": "Process a payment through Stripe.",
    "visibility": "public",
    "complexity": 12,
    "is_async": true,
    "stability_score": 0.34,
    "change_count": 47,
    "last_modified": "2026-02-15T14:23:01Z",
    "edges": {
      "calls": [4522, 4523, 4530],
      "imports": [102, 103],
      "tested_by": [8901, 8902]
    }
  }
}
```

---

## Prompts (Pre-Built Workflow Templates)

| Prompt | Purpose | Arguments |
|--------|---------|-----------|
| `understand_codebase` | Get high-level understanding | `path` |
| `investigate_bug` | Trace a bug through code | `symptoms`, `file` |
| `plan_refactor` | Plan a safe refactoring | `target`, `goal` |
| `review_change` | Review impact of a change | `diff` or `files` |
| `learn_library` | Learn how a library is used | `library` |
| `find_similar` | Find similar implementations | `description` |

### Prompt: `understand_codebase`

**Arguments:**
- `path`: Repository path

**Expanded Template:**
```
I'll analyze this codebase and give you a structured understanding.

1. First, let me compile the repository:
   [Call compile tool with path]

2. Now let me gather the high-level structure:
   [Call resource acb://graph/{id}/stats]
   [Call resource acb://graph/{id}/modules]

3. Let me identify the core concepts:
   [Call concept_mapping for common patterns]

4. Here's what I found:

## Codebase Overview
- Languages: {languages}
- Size: {unit_count} symbols across {file_count} files
- Architecture: {detected_patterns}

## Core Modules
{module_summaries}

## Key Concepts
{concept_map}

## Health Indicators
- Stability: {stability_summary}
- Test Coverage: {coverage_summary}
- Potential Issues: {prophecy_summary}

What would you like to explore further?
```

### Prompt: `investigate_bug`

**Arguments:**
- `symptoms`: Description of the bug
- `file`: Optional starting file

**Expanded Template:**
```
I'll help trace this bug through the codebase.

1. Let me search for relevant code:
   [Call semantic_search with symptoms]

2. For each candidate, let me check:
   [Call call_graph to see call paths]
   [Call stability_analysis to check change history]

3. Let me look for hidden couplings:
   [Call coupling_detection on candidates]

4. Analysis:

## Likely Locations
{ranked_candidates_with_reasoning}

## Call Path to Bug
{call_trace_visualization}

## Historical Context
{relevant_changes_and_patterns}

## Recommendation
{suggested_investigation_steps}
```

---

## Claude Desktop Configuration

```json
{
  "mcpServers": {
    "agentic-codebase": {
      "command": "acb-mcp-server",
      "args": [],
      "env": {
        "ACB_COLLECTIVE_ENABLED": "true",
        "ACB_CACHE_DIR": "~/.cache/acb"
      }
    }
  }
}
```

---

## VS Code Configuration

```json
{
  "mcp.servers": {
    "agentic-codebase": {
      "command": "acb-mcp-server",
      "transport": "stdio"
    }
  }
}
```

---

## Error Codes

Standard MCP error codes plus:

| Code | Name | Description |
|------|------|-------------|
| -32900 | `CompilationFailed` | Repository couldn't be compiled |
| -32901 | `GraphNotFound` | Referenced graph_id doesn't exist |
| -32902 | `UnitNotFound` | Referenced unit_id doesn't exist |
| -32903 | `UnsupportedLanguage` | Language not supported |
| -32904 | `CollectiveUnavailable` | Collective service unreachable |
| -32905 | `HistoryUnavailable` | Git history not available |

---

## Implementation Notes

### State Management
- Graphs are loaded into memory and referenced by `graph_id`
- Multiple graphs can be loaded simultaneously
- Graphs are automatically unloaded after inactivity (configurable)

### Caching
- Compiled .acb files are cached
- Incremental recompilation when files change
- Collective data cached locally with TTL

### Concurrency
- Multiple queries can run concurrently on same graph
- Compilation is single-threaded per repository
- Collective sync is async and non-blocking

### Privacy
- Private code never leaves local machine
- Only open-source library patterns sync to collective
- Opt-in collective participation
