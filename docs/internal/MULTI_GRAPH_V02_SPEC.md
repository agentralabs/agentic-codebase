# AgenticCodebase v0.2 — Multi-Graph Migration Support

> **Status**: Planning
> **Origin**: Organic user feedback — GitHub Issue #1 (11philip22)
> **Date**: 2026-02-25

---

## User Pain

> "My agent hallucinates a lot and forgets important parts of the tasks I give it"

When porting large C++ to Rust. The agent loses track of what's been ported, invents functions that don't exist, and can't hold both codebases in context.

## Solution

Multi-graph support with translation mapping, migration tracking, and context anchoring.

---

## The Vision

```
BEFORE (Current):
─────────────────
Agent sees: [partial context window of code]
Agent does: Hallucinates missing parts, forgets task context
User gets: Frustration, manual correction, lost time

AFTER (v0.2):
─────────────────
Agent sees: [complete semantic graph of BOTH codebases]
Agent does: Queries graph before answering, tracks what's ported
User gets: "Port auth module" → agent knows exactly what exists,
           what's been done, what's left, with zero hallucination
```

---

## New Capabilities

### 1. Multi-Graph Workspace

Load multiple codebases simultaneously with explicit roles:

```jsonc
// MCP tool: workspace_create
{
    "name": "cpp-to-rust-port",
    "graphs": [
        { "path": "/projects/legacy-cpp", "role": "source", "language": "cpp" },
        { "path": "/projects/new-rust", "role": "target", "language": "rust" }
    ]
}

// Agent now has both graphs loaded
// All queries can specify which graph or cross-reference
```

**MCP Tools:**
- `workspace_create` — Create multi-graph workspace
- `workspace_list` — List loaded graphs
- `workspace_switch` — Change active graph
- `workspace_query` — Query across all graphs

---

### 2. Translation Mapping

Track semantic equivalence between source and target:

```jsonc
// MCP tool: translation_map
{
    "source_graph": "legacy-cpp",
    "target_graph": "new-rust",
    "mappings": [
        {
            "source": "class AuthManager",
            "target": "struct AuthManager",
            "status": "ported",
            "notes": "Converted to Rust idioms, uses Result<> for errors"
        },
        {
            "source": "void AuthManager::validate()",
            "target": "fn AuthManager::validate() -> Result<(), AuthError>",
            "status": "ported"
        },
        {
            "source": "class SessionCache",
            "target": null,
            "status": "not_started"
        }
    ]
}
```

**Translation Status:**
- `not_started` — Exists in source, not in target
- `in_progress` — Partially ported
- `ported` — Code exists in target
- `verified` — Ported AND tested
- `skipped` — Intentionally not porting (deprecated, not needed)

**MCP Tools:**
- `translation_map` — Record source→target mapping
- `translation_status` — Get status of specific item
- `translation_progress` — Overall migration progress
- `translation_unmapped` — List everything not yet mapped

---

### 3. Migration Queries

Purpose-built queries for migration workflows:

```bash
# "What's left to port in the auth module?"
migration_remaining source=legacy-cpp target=new-rust module=auth

Returns:
├── SessionCache (class) — not_started
├── TokenValidator (class) — not_started
│   ├── validate_token() — not_started
├── refresh_session() — in_progress (70% done)
└── 12 functions remaining, 3 classes, ~2,400 lines

# "Show me the C++ version of what I'm about to port"
migration_source_context target_function="AuthManager::validate"

Returns:
├── C++ source code
├── All callers in C++
├── All dependencies
├── Edge cases handled
├── Test cases that exist
```

**MCP Tools:**
- `migration_remaining` — What's left to port
- `migration_done` — What's been completed
- `migration_source_context` — Full context for what you're porting
- `migration_verify` — Compare semantics between source and target

---

### 4. Context Anchoring (Anti-Hallucination)

The agent MUST ground itself in the graph before responding:

```
// System behavior (automatic, not a tool)

When agent receives: "Port the SessionCache class"

BEFORE responding, agent automatically:
1. Queries source graph: "Does SessionCache exist?" → Yes, found
2. Gets full definition: class, methods, dependencies
3. Queries target graph: "Does SessionCache exist in Rust?" → No
4. Checks translation map: status = not_started
5. Gets all callers/callees in source
6. NOW responds with complete, grounded information

If agent would say something not in graph:
→ BLOCKED: "I don't see X in the codebase graph.
    Did you mean Y? Or should I re-index?"
```

**Hallucination Prevention Rules:**
- Never claim a function exists without graph confirmation
- Never claim a function doesn't exist without graph confirmation
- Always show source when discussing migration
- Always link claims to graph nodes

---

### 5. Task Memory Integration

Link to AgenticMemory for persistent task context:

```jsonc
// When user says "Port the auth module"

// 1. Create memory entry:
{
    "type": "task",
    "content": "Porting auth module from C++ to Rust",
    "context": {
        "source_graph": "legacy-cpp",
        "target_graph": "new-rust",
        "scope": ["AuthManager", "SessionCache", "TokenValidator"],
        "started": "2026-02-25",
        "progress": "15%"
    }
}

// 2. On every subsequent message:
//    - Load task context from memory
//    - Agent knows: what module, what's done, what's next
//    - NO FORGETTING

// 3. On completion:
//    - Update memory: progress = 100%, status = complete
//    - Store lessons learned
```

---

### 6. Semantic Diff (Port Verification)

Verify the port is semantically equivalent:

```
// MCP tool: migration_verify
{
    "source": "cpp::AuthManager::validate",
    "target": "rust::AuthManager::validate"
}

Returns:
├── Signature match: OK (void → Result<()> is valid Rust idiom)
├── All code paths covered: OK
├── Error handling: WARN — C++ throws exception, Rust returns Err
│   └── Recommendation: Ensure all catch blocks map to Err variants
├── Edge cases:
│   ├── null input: C++ crashes, Rust returns Err (improved)
│   ├── expired token: Both handle OK
│   └── network timeout: C++ retries 3x, Rust retries 1x WARN
└── Test coverage: C++ has 12 tests, Rust has 8 (4 missing)
```

---

## MCP Tool Summary

```
WORKSPACE (Multi-Graph)
───────────────────────
workspace_create        Create multi-graph workspace
workspace_list          List loaded graphs
workspace_switch        Change active graph
workspace_close         Close workspace

TRANSLATION (Mapping)
─────────────────────
translation_map         Record source→target mapping
translation_status      Get status of specific item
translation_progress    Overall migration progress
translation_unmapped    List unmapped items
translation_update      Update mapping status

MIGRATION (Queries)
───────────────────
migration_remaining     What's left to port
migration_done          What's been completed
migration_source        Get full source context
migration_verify        Compare semantics
migration_suggest       Suggest next item to port

CONTEXT (Anti-Hallucination)
────────────────────────────
context_anchor          Set current task context
context_ground          Verify claim against graph
context_require         Require graph confirmation before response
```

---

## User Experience

### Before (current):

```
User: Port the SessionCache class to Rust
Agent: [hallucinates structure, forgets methods, loses context mid-task]
User: No, it has 5 methods not 3, and you forgot the dependency on TokenValidator
Agent: [hallucinates again]
User: [gives up, does it manually]
```

### After (v0.2):

```
User: Port the SessionCache class to Rust

Agent: I've loaded both codebases. Here's SessionCache in C++:
        class SessionCache {
           TokenValidator* validator;  // dependency
           void store(Session s);
           Session* get(string id);
           void invalidate(string id);
           void cleanup_expired();
           int active_count();
        }

        Dependencies: TokenValidator (not yet ported)
        Callers: AuthManager::validate(), ApiHandler::check_session()

        Should I port TokenValidator first since it's a dependency,
        or stub it for now?

User: Stub it for now, port SessionCache

Agent: [produces complete, accurate Rust translation]
       [records in translation map: SessionCache = ported]
       [records in memory: task progress = 25%]
       [knows TokenValidator is still pending]

User: [next day] Continue the port

Agent: [loads memory, knows exactly where we left off]
       Last session: Ported SessionCache with TokenValidator stub.
       Remaining in auth module:
       - TokenValidator (class) — not_started
       - validate_token() — not_started

       Ready to port TokenValidator?
```

---

## Implementation Priority

```
PHASE 1 (Immediate — solves 11philip22's problem):
├── Multi-graph loading (workspace_create)
├── Cross-graph queries
├── Basic translation mapping
└── migration_remaining query

PHASE 2 (Full migration support):
├── Semantic diff / verification
├── Context anchoring (anti-hallucination)
├── Memory integration for task persistence
└── Progress tracking

PHASE 3 (Intelligence):
├── Auto-suggest next item to port
├── Detect semantic drift
├── Generate test cases from source
└── Estimate remaining effort
```

---

## Why This Is Huge

```
CURRENT MARKET:
───────────────
- AI coding tools: Work on ONE codebase
- Migration tools: Manual, no AI
- Context window: Limited, causes hallucination

WITH THIS:
──────────
- First tool that understands BOTH codebases semantically
- First tool that tracks migration state
- First tool that prevents hallucination via grounding
- First tool that remembers migration progress across sessions

This is not incremental. This is a new category.
```

---

## Implementation Notes

The thing that makes Phase 1 feasible quickly: the `.acb` graph format already supports cross-language parsing (Rust, C++, Python, Go, etc.) and the MCP server already loads a single graph. The delta is:

- `HashMap<String, LoadedGraph>` instead of a single graph
- Tool parameters gain an optional `graph` field
- A `workspace.json` sidecar to persist the workspace config

The translation mapping is just a new edge type in the existing graph structure — `maps_to` edges between nodes in different graphs. The storage format already supports arbitrary edge types.
