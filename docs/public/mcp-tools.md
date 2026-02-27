---
status: stable
---

# MCP Tools

AgenticCodebase exposes 60+ tools through the MCP protocol via `agentic-codebase-mcp`.

## Core Tools

### `symbol_lookup`

Look up symbols by name in the code graph.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Symbol name to search for |
| `graph` | string | No | Graph name |
| `mode` | string | No | `exact`, `prefix`, `contains`, `fuzzy` (default: `prefix`) |
| `limit` | number | No | Maximum results (default: 10) |

### `impact_analysis`

Analyse the impact of changing a code unit.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID to analyse |
| `graph` | string | No | Graph name |
| `max_depth` | number | No | Maximum traversal depth (default: 3) |

### `graph_stats`

Get summary statistics about a loaded code graph.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |

### `list_units`

List code units in a graph, optionally filtered by type.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |
| `unit_type` | string | No | Filter: `module`, `symbol`, `type`, `function`, `parameter`, `import`, `test`, `doc`, `config`, `pattern`, `trait`, `impl`, `macro` |
| `limit` | number | No | Maximum results (default: 50) |

### `analysis_log`

Log the intent and context behind a code analysis. Call this to record WHY you are performing a lookup or analysis.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `intent` | string | Yes | Why you are analysing -- the goal or reason for the code query |
| `finding` | string | No | What you found or concluded from the analysis |
| `graph` | string | No | Graph name this analysis relates to |
| `topic` | string | No | Category (e.g., `refactoring`, `bug-hunt`) |

## Grounding Tools

### `codebase_ground`

Verify a claim about code has graph evidence. Use before asserting code exists.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `claim` | string | Yes | The claim to verify (e.g., "function validate_token exists") |
| `graph` | string | No | Graph name |
| `strict` | boolean | No | If true, partial matches return Ungrounded (default: false) |

### `codebase_evidence`

Get graph evidence for a symbol name.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Symbol name to find |
| `graph` | string | No | Graph name |
| `types` | array | No | Filter by type: `function`, `struct`, `enum`, `module`, `trait` |

### `codebase_suggest`

Find symbols similar to a name (for corrections).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Name to find similar matches for |
| `graph` | string | No | Graph name |
| `limit` | number | No | Max suggestions (default: 5) |

## Workspace Tools

### `workspace_create`

Create a workspace to load multiple codebases.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Workspace name (e.g., "cpp-to-rust-migration") |

### `workspace_add`

Add a codebase to an existing workspace.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `graph` | string | Yes | Name of a loaded graph to add |
| `role` | string | Yes | `source`, `target`, `reference`, `comparison` |
| `path` | string | No | Path label for this codebase |
| `language` | string | No | Language hint |

### `workspace_list`

List all contexts in a workspace.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |

### `workspace_query`

Search across all codebases in workspace.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `query` | string | Yes | Search query |
| `roles` | array | No | Filter by role |

### `workspace_compare`

Compare a symbol between source and target.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `symbol` | string | Yes | Symbol to compare |

### `workspace_xref`

Find where symbol exists/doesn't exist across contexts.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `symbol` | string | Yes | Symbol to find |

## Translation Tools

### `translation_record`

Record source-to-target symbol mapping.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `source_symbol` | string | Yes | Symbol in source codebase |
| `status` | string | Yes | `not_started`, `in_progress`, `ported`, `verified`, `skipped` |
| `target_symbol` | string | No | Symbol in target (null if not ported) |
| `notes` | string | No | Optional notes |

### `translation_progress`

Get migration progress statistics.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |

### `translation_remaining`

List symbols not yet ported.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `module` | string | No | Filter by module |

## Enhanced Impact Analysis Tools

### `impact_analyze`

Analyze the full impact of a proposed code change with blast radius and risk assessment.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Target code unit ID |
| `graph` | string | No | Graph name |
| `change_type` | string | No | `signature`, `behavior`, `deletion`, `rename`, `move` (default: `behavior`) |
| `max_depth` | number | No | Maximum depth (default: 5) |

### `impact_path`

Find the impact path between two code units.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `from` | number | Yes | Source unit ID |
| `to` | number | Yes | Target unit ID |
| `graph` | string | No | Graph name |

## Code Prophecy Tools

### `prophecy`

Predict the future of a code unit based on history, complexity, and dependencies.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID to predict |
| `graph` | string | No | Graph name |

### `prophecy_if`

What-if scenario: predict impact of a hypothetical change.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |
| `change_type` | string | No | `signature`, `behavior`, `deletion`, `rename`, `move` (default: `behavior`) |

## Regression Oracle Tools

### `regression_predict`

Predict which tests are most likely affected by a change.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Changed code unit ID |
| `graph` | string | No | Graph name |
| `max_depth` | number | No | Maximum depth (default: 5) |

### `regression_minimal`

Get the minimal test set needed for a change.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Changed code unit ID |
| `graph` | string | No | Graph name |
| `threshold` | number | No | Minimum probability threshold, 0.0-1.0 (default: 0.5) |

## Citation Engine Tools

### `codebase_ground_claim`

Ground a claim with full citations including file locations and code snippets.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `claim` | string | Yes | The claim to verify and cite |
| `graph` | string | No | Graph name |

### `codebase_cite`

Get a citation for a specific code unit.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID to cite |
| `graph` | string | No | Graph name |

## Hallucination Detection Tools

### `hallucination_check`

Check AI-generated output for hallucinations about code.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `output` | string | Yes | AI-generated text to check |
| `graph` | string | No | Graph name |

## Truth Maintenance Tools

### `truth_register`

Register a truth claim for ongoing maintenance.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `claim` | string | Yes | The truth claim to maintain |
| `graph` | string | No | Graph name |

### `truth_check`

Check if a registered truth is still valid.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `claim` | string | Yes | The truth claim to check |
| `graph` | string | No | Graph name |

## Concept Navigation Tools

### `concept_find`

Find code implementing a concept (e.g., authentication, payment).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `concept` | string | Yes | Concept to find (e.g., "authentication", "payment") |
| `graph` | string | No | Graph name |

### `concept_map`

Map all detected concepts in the codebase.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |

### `concept_explain`

Explain how a concept is implemented with details.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `concept` | string | Yes | Concept to explain |
| `graph` | string | No | Graph name |

## Architecture Inference Tools

### `architecture_infer`

Infer the architecture pattern of the codebase.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |

### `architecture_validate`

Validate the codebase against its inferred architecture.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |

## Semantic Search Tools

### `search_semantic`

Natural-language semantic search across the codebase.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | string | Yes | Natural-language search query |
| `graph` | string | No | Graph name |
| `top_k` | number | No | Maximum results (default: 10) |

### `search_similar`

Find code units similar to a given unit.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Unit ID to find similar units for |
| `graph` | string | No | Graph name |
| `top_k` | number | No | Maximum results (default: 10) |

### `search_explain`

Explain why a unit matched a search query.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Unit ID |
| `query` | string | Yes | The search query |
| `graph` | string | No | Graph name |

## Multi-Codebase Compare Tools

### `compare_codebases`

Full structural, conceptual, and pattern comparison between two codebases in a workspace.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |

### `compare_concept`

Compare how a concept is implemented across two codebases.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `concept` | string | Yes | Concept to compare (e.g., "authentication") |

### `compare_migrate`

Generate a migration plan from source to target codebase.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |

## Version Archaeology Tools

### `archaeology_node`

Investigate the full history and evolution of a code unit.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID to investigate |
| `graph` | string | No | Graph name |

### `archaeology_why`

Explain why code looks the way it does based on its history.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |

### `archaeology_when`

Get the timeline of changes for a code unit.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |

## Pattern Extraction Tools

### `pattern_extract`

Extract all detected patterns from the codebase.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |

### `pattern_check`

Check a code unit against detected patterns for violations.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID to check |
| `graph` | string | No | Graph name |

### `pattern_suggest`

Suggest patterns for new code based on file location.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file_path` | string | Yes | File path for pattern suggestions |
| `graph` | string | No | Graph name |

## Code Resurrection Tools

### `resurrect_search`

Search for traces of deleted code.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | string | Yes | Search query for deleted code traces |
| `graph` | string | No | Graph name |
| `max_results` | number | No | Maximum results (default: 10) |

### `resurrect_attempt`

Attempt to reconstruct deleted code from traces.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | string | Yes | Description of the code to resurrect |
| `graph` | string | No | Graph name |

### `resurrect_verify`

Verify a resurrection attempt is accurate.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `original_name` | string | Yes | Original name of the deleted code |
| `reconstructed` | string | Yes | Reconstructed code to verify |
| `graph` | string | No | Graph name |

### `resurrect_history`

Get resurrection history for the codebase.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |

## Code Genetics Tools

### `genetics_dna`

Extract the DNA (core patterns) of a code unit.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |

### `genetics_lineage`

Trace the lineage of a code unit through evolution.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |
| `max_depth` | number | No | Maximum depth (default: 10) |

### `genetics_mutations`

Detect mutations (unexpected changes) in code patterns.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |

### `genetics_diseases`

Diagnose inherited code diseases (anti-patterns passed through lineage).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |

## Code Telepathy Tools

### `telepathy_connect`

Establish telepathic connection between codebases.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `source_graph` | string | No | Source graph name |
| `target_graph` | string | No | Target graph name |

### `telepathy_broadcast`

Broadcast a code insight to connected codebases.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `insight` | string | Yes | The code insight to broadcast |
| `source_graph` | string | No | Source graph name |

### `telepathy_listen`

Listen for insights from connected codebases.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `target_graph` | string | No | Target graph name to listen from |

### `telepathy_consensus`

Find consensus patterns across connected codebases.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `workspace` | string | Yes | Workspace name or id |
| `concept` | string | Yes | Concept to find consensus on |

## Code Soul Tools

### `soul_extract`

Extract the soul (essential purpose and values) of code.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |

### `soul_compare`

Compare souls across code reincarnations.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id_a` | number | Yes | First code unit ID |
| `unit_id_b` | number | Yes | Second code unit ID |
| `graph` | string | No | Graph name |

### `soul_preserve`

Preserve a code soul during rewrite.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |
| `new_language` | string | No | Target language for rewrite |

### `soul_reincarnate`

Guide a soul to a new code manifestation.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `soul_id` | string | Yes | Soul identifier |
| `target_context` | string | Yes | Target context for reincarnation |
| `graph` | string | No | Graph name |

### `soul_karma`

Analyze the karma (positive/negative impact history) of code.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID |
| `graph` | string | No | Graph name |

## Code Omniscience Tools

### `omniscience_search`

Search across global code knowledge.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | string | Yes | Search query |
| `languages` | array | No | Filter by languages |
| `max_results` | number | No | Maximum results (default: 10) |

### `omniscience_best`

Find the best implementation of a concept globally.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `capability` | string | Yes | Capability to find best implementation for |
| `criteria` | array | No | Evaluation criteria |

### `omniscience_census`

Global code census for a concept.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `concept` | string | Yes | Concept to census |
| `languages` | array | No | Filter by languages |

### `omniscience_vuln`

Scan for known vulnerability patterns.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `graph` | string | No | Graph name |
| `pattern` | string | No | Vulnerability pattern to scan for |
| `cve` | string | No | CVE identifier to check |

### `omniscience_trend`

Find emerging or declining code patterns.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `domain` | string | Yes | Domain to analyze trends in |
| `threshold` | number | No | Threshold (default: 0.5) |

### `omniscience_compare`

Compare your code to global best practices.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `unit_id` | number | Yes | Code unit ID to compare |
| `graph` | string | No | Graph name |

### `omniscience_api_usage`

Find all usages of an API globally.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `api` | string | Yes | API name to search for |
| `method` | string | No | Specific method within the API |

### `omniscience_solve`

Find code that solves a specific problem.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `problem` | string | Yes | Problem description to solve |
| `languages` | array | No | Preferred languages |
| `max_results` | number | No | Maximum results (default: 5) |
