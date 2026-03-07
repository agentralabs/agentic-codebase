# AgenticCodebase — The Invention

> **Your codebase is a 4-dimensional structure. Everyone else is flatlanding through text search. We navigate all four dimensions: symbols, relationships, time, and patterns.**

---

## The Problem Nobody Is Solving Correctly

Every AI coding assistant reads code like a book — left to right, top to bottom, one file at a time. They use grep to find things, LSP for syntax, and hope the context window is big enough to hold what they need.

This is like exploring a city by reading every street sign in alphabetical order.

**The reality:** A codebase is a living graph. Every symbol connects to other symbols. Every change propagates through those connections. Every commit adds a temporal layer. Every pattern repeats across projects. But nobody navigates this structure — they search through flattened text and pray.

---

## What's Actually Broken

**Problem 1: Syntax Without Semantics**

LSP gives you "this is a function named `process_payment`." It doesn't give you:
- This function is part of the payment authorization flow
- It's called by 3 API endpoints and 2 background jobs
- It has a hidden dependency on `config.stripe_key` being set
- It was refactored twice because of edge cases with international cards
- Changing its signature will break 47 downstream consumers

Current tools see SYNTAX. Agents need SEMANTICS.

**Problem 2: Static Analysis in a Dynamic World**

Every time an agent opens a codebase, it analyzes from scratch. No memory of:
- "I figured out how this authentication system works last week"
- "The last 3 bugs in this file were all race conditions"
- "This module claims to be stateless but secretly caches in globals"

Agents forget everything between sessions. Humans don't. This is why humans still outperform agents on complex codebases.

**Problem 3: Island Analysis**

Your codebase doesn't exist in isolation. It depends on 200 open source packages. Each of those has:
- Known gotchas
- Common misuse patterns
- Performance cliffs
- Upgrade landmines

Every agent discovers these independently. A million agents hit the same `react-router` footgun every day. The collective learns nothing.

**Problem 4: Present-Only Understanding**

Code evolves. History matters:
- Why was this written this way? (Intent)
- What broke the last time someone changed it? (Risk)
- How fast is this area changing? (Stability)
- What changes tend to happen together? (Coupling)

Git has this data. Nobody uses it for intelligent navigation.

---

## The Invention: AgenticCodebase

AgenticCodebase compiles any codebase into a navigable semantic graph where:

1. Every meaningful unit of code is a **typed node**
2. Every relationship between units is a **typed edge**
3. The entire structure lives in a single memory-mappable `.acb` file
4. The graph connects to collective intelligence about shared dependencies
5. Time is a first-class dimension — every change is tracked, patterns emerge

This is NOT a language server. This is NOT a search index. This is a **navigable brain for code**.

---

## The Three Inventions

### Invention 1 — The Semantic Code Compiler

We don't parse code into syntax trees. We **compile** it into a semantic graph where nodes represent CONCEPTS, not syntax.

```python
import agentic_codebase as acb

# Compile a repository
repo = acb.compile("./my-project")

# Navigate to CONCEPTS, not files
repo.User                              # The User concept — wherever it lives
repo.User.authentication_flow          # The auth flow involving User
repo.User.all_mutations                # Every way a User can change
repo.User.test_coverage                # What's tested, what's not
repo.User.stability_score              # How often this breaks

# Cross-language navigation is automatic
repo.PaymentService                    # Python service
repo.PaymentService.rust_core          # Calls Rust via FFI
repo.PaymentService.js_consumer        # Frontend React component calling API
repo.PaymentService.impact("amount")   # What breaks if 'amount' changes

# Traverse relationships
repo.PaymentService.calls()            # Everything it calls
repo.PaymentService.called_by()        # Everything that calls it
repo.PaymentService.tests()            # All tests covering this
repo.PaymentService.breaks_with()      # Hidden couplings discovered from history

# The agent doesn't READ code — it NAVIGATES CONCEPTS
```

**What makes this different from LSP/TreeSitter:**
- **Semantic, not syntactic**: Nodes are concepts (User, PaymentFlow, AuthStrategy), not syntax (FunctionDef, ClassDecl)
- **Cross-language**: Python calling Rust calling C is one continuous graph
- **Cross-repository**: Your microservices are one logical codebase
- **Relationship-rich**: 15+ edge types, not just "references"
- **Memory-integrated**: Connects to AgenticMemory for persistent learning

### Invention 2 — The Collective Code Intelligence

When a million agents analyze `express.js` independently, humanity gains nothing. We're burning compute to rediscover the same patterns endlessly.

**The collective graph changes this:**

```
Your agent compiles your code
    ↓
For each dependency (express, react, sqlalchemy, ...)
    ↓
Pull existing semantic map from collective registry
    ↓
Your private code stays private
Open source patterns flow freely
    ↓
Agent starts with EXPERT knowledge of every library

When your agent discovers something new about a library
    ↓
Push compressed delta to collective
    ↓
Every future agent benefits
```

**What the collective knows about each library:**
- **Usage patterns**: "Most projects use X this way"
- **Common mistakes**: "47% of projects using X also hit this bug"
- **Performance cliffs**: "This API is O(n²) despite appearances"
- **Hidden contracts**: "This assumes single-threaded despite no docs saying so"
- **Migration paths**: "Upgrading from v2 to v3? Here's what broke for others"

**The math:**
- 100,000 agents analyzing 50 repositories/day = 5 million repo-analyses daily
- After 30 days: Collective understands patterns from 150 million analyses
- Any new agent starts with distilled expertise from 150 million prior analyses

**Privacy model:**
- Your private code NEVER leaves your machine
- Only patterns from open source dependencies are shared
- The collective learns "how people use React" not "your company's React code"

### Invention 3 — Code Prophecy

The collective graph doesn't just store current state — it stores every delta ever pushed. Stack them temporally and patterns emerge:

**Stability Analysis**
```
repo.module("payments").stability_score     # 0.34 — unstable
repo.module("payments").change_frequency    # 12 changes/month
repo.module("payments").bug_correlation     # 73% of changes followed by bugfix
repo.module("payments").refactor_candidate  # True — recommend extraction
```

**Coupling Detection**
```
repo.coupling_analysis()
# Returns:
# - payments.validate() and audit.log() are secretly coupled
#   (89% of changes to one require change to other within 3 commits)
# - User.email and notifications.send() should be tested together
# - config.json and 7 modules have undeclared dependency
```

**Change Prediction**
```
repo.prophecy("payments/stripe.py")
# Returns:
# - Last 5 changes followed breaking-change pattern
# - Based on current velocity: critical incident in ~6 weeks
# - Similar codebases refactored at this stage
# - Recommended intervention: extract to service

repo.ecosystem_prophecy()
# Returns:
# - fastapi 0.104 → 0.105 broke 23% of similar codebases
# - Your usage pattern matches high-risk group
# - Recommended: pin version, migrate in 2 months with test coverage
```

**Architectural Drift**
```
repo.architecture_health()
# Returns:
# - Original design: clean layers (API → Service → Repository)
# - Current reality: 34 layer violations detected
# - Drift rate: accelerating (was 2/month, now 7/month)
# - Prediction: Architecture will be unmaintainable in 4 months
# - Recommendation: Enforce boundaries at these 5 modules
```

---

## The Semantic Graph

### Node Types (CodeUnits)

Every meaningful unit of code becomes a typed node:

| Type | What It Represents | Example |
|------|-------------------|---------|
| **Module** | A logical grouping (file, package, namespace) | `payments/`, `auth.py` |
| **Symbol** | A named entity (function, class, variable, constant) | `process_payment`, `User`, `MAX_RETRIES` |
| **Type** | A type definition (class, struct, interface, enum) | `PaymentStatus`, `UserRole` |
| **Function** | A callable unit | `validate_card()`, `send_notification()` |
| **Parameter** | A function parameter or field | `amount: Decimal`, `user_id: int` |
| **Import** | A dependency declaration | `from stripe import Charge` |
| **Test** | A test case or test suite | `test_payment_success` |
| **Doc** | Documentation block | Docstring, JSDoc, comment block |
| **Config** | Configuration value | `STRIPE_API_KEY`, `database_url` |
| **Pattern** | An identified design pattern | Singleton, Factory, Repository |

### Edge Types (Relationships)

Relationships are typed and directional:

| Edge Type | Meaning | Example |
|-----------|---------|---------|
| **CALLS** | Runtime invocation | `handler()` → CALLS → `process()` |
| **IMPORTS** | Static dependency | `auth.py` → IMPORTS → `jwt` |
| **INHERITS** | Type hierarchy | `AdminUser` → INHERITS → `User` |
| **IMPLEMENTS** | Interface conformance | `StripeGateway` → IMPLEMENTS → `PaymentGateway` |
| **OVERRIDES** | Method override | `AdminUser.save()` → OVERRIDES → `User.save()` |
| **CONTAINS** | Structural containment | `UserModule` → CONTAINS → `User` |
| **REFERENCES** | Non-call reference | Code references symbol without calling |
| **TESTS** | Test coverage | `test_user_create` → TESTS → `User.create()` |
| **DOCUMENTS** | Documentation target | `docstring` → DOCUMENTS → `User` |
| **CONFIGURES** | Configuration relationship | `config.json` → CONFIGURES → `Database` |
| **COUPLES_WITH** | Hidden coupling (from history) | Changes together >70% of time |
| **BREAKS_WITH** | Breaking relationship | Changing A historically breaks B |
| **PATTERN_OF** | Pattern instance | `UserFactory` → PATTERN_OF → `Factory` |
| **VERSION_OF** | Temporal relationship | `User@v2` → VERSION_OF → `User@v1` |
| **FFI_BINDS** | Cross-language binding | `py_func` → FFI_BINDS → `rust_func` |

### The CodeUnit Node Structure

```rust
struct CodeUnit {
    id:              u64,           // Unique identifier
    unit_type:       CodeUnitType,  // Module, Symbol, Function, etc.
    language:        Language,      // Python, Rust, TypeScript, etc.
    name:            String,        // Symbol name
    qualified_name:  String,        // Full path: "payments.stripe.process_payment"
    file_path:       PathBuf,       // Source file location
    span:            Span,          // Line/column range in source
    signature:       Option<String>,// Type signature if applicable
    doc_summary:     Option<String>,// First line of documentation
    
    // Semantic metadata
    visibility:      Visibility,    // Public, Private, Internal
    purity:          Purity,        // Pure, Impure, Unknown
    complexity:      u32,           // Cyclomatic complexity
    
    // Temporal metadata
    created_at:      u64,           // First seen (commit timestamp)
    last_modified:   u64,           // Last change timestamp
    change_count:    u32,           // Total changes in history
    stability_score: f32,           // 0.0 = chaotic, 1.0 = stable
    
    // Collective metadata
    usage_count:     u64,           // How often referenced (across collective)
    pattern_id:      Option<u64>,   // Identified pattern, if any
    
    // Vector for semantic search
    feature_vec:     Vec<f32>,      // Embedding for similarity
    
    // Graph position
    edge_offset:     u64,           // Pointer to edge list
    edge_count:      u32,           // Number of outgoing edges
}
```

---

## The Query Engine

### Complete Query Types

| # | Query Type | What It Answers | Category |
|---|------------|-----------------|----------|
| 1 | **Symbol Lookup** | "What is this symbol?" | ✅ Core |
| 2 | **Dependency Graph** | "What does this depend on?" | ✅ Core |
| 3 | **Reverse Dependency** | "What depends on this?" | ✅ Core |
| 4 | **Call Graph** | "What calls this? What does it call?" | ✅ Core |
| 5 | **Type Hierarchy** | "What inherits from this?" | ✅ Core |
| 6 | **Containment** | "What's inside this module?" | ✅ Core |
| 7 | **Pattern Match** | "Find all code matching this structure" | ✅ Core |
| 8 | **Semantic Search** | "Find code that does X" (vector similarity) | ✅ Core |
| 9 | **Impact Analysis** | "What breaks if I change this?" | ✅ Built |
| 10 | **Test Coverage** | "What tests cover this? What's untested?" | ✅ Built |
| 11 | **Cross-Language Trace** | "Trace this call across FFI boundary" | ✅ Built |
| 12 | **Collective Patterns** | "How do experts use this library?" | 🆕 Novel |
| 13 | **Temporal Evolution** | "How has this changed over time?" | 🆕 Novel |
| 14 | **Stability Analysis** | "How risky is changing this?" | 🆕 Novel |
| 15 | **Coupling Detection** | "What's secretly coupled?" | 🆕 Novel |
| 16 | **Dead Code** | "What's never executed?" | 🆕 Novel |
| 17 | **Prophecy** | "What will break next?" | 🆕 Novel |
| 18 | **Concept Mapping** | "Where is X implemented across codebase?" | 🆕 Novel |
| 19 | **Migration Path** | "How do I safely change X to Y?" | 🆕 Novel |
| 20 | **Test Gap** | "What recent changes lack test coverage?" | 🆕 Novel |
| 21 | **Architectural Drift** | "Is codebase diverging from design?" | 🆕 Novel |
| 22 | **Similarity** | "What code is similar to this?" | ✅ Built |
| 23 | **Shortest Path** | "How are these two symbols connected?" | ✅ Built |
| 24 | **Hotspot Detection** | "Where do bugs cluster?" | 🆕 Novel |

---

## The File Format: `.acb`

Single binary file, memory-mappable, same philosophy as `.amem`:

```
┌─────────────────────────────────┐
│  FILE HEADER (128 bytes)        │
│  Magic: "ACDB", version, counts │
├─────────────────────────────────┤
│  CODE UNIT TABLE                │
│  (unit_count × 96 bytes)        │
├─────────────────────────────────┤
│  EDGE TABLE                     │
│  (edge_count × 40 bytes)        │
├─────────────────────────────────┤
│  STRING POOL                    │
│  (all names, paths, signatures) │
├─────────────────────────────────┤
│  FEATURE VECTOR BLOCK           │
│  (unit_count × dim × 4 bytes)   │
├─────────────────────────────────┤
│  TEMPORAL BLOCK                 │
│  (change history, compressed)   │
├─────────────────────────────────┤
│  INDEX BLOCK                    │
│  (type, name, path indexes)     │
└─────────────────────────────────┘
```

---

## Integration with the Ecosystem

### AgenticCodebase + AgenticMemory

The agent remembers what it learned about codebases:

```python
# Agent analyzes codebase, discovers a pattern
brain.add_fact(
    "payments.stripe module has race condition under concurrent refunds",
    session=session_id,
    links_to=[codebase.symbol("payments.stripe.refund").id]
)

# Next session, agent remembers
brain.query("What do I know about payments.stripe?")
# Returns: "race condition under concurrent refunds" + link to exact code
```

### AgenticCodebase + AgenticVision

The agent understands how code connects to the web:

```python
# Code calls an API
codebase.symbol("stripe_client.charge")
    .external_api  # → Vision's map of api.stripe.com
    .rate_limits   # 100/sec
    .known_errors  # From collective Vision data
    
# The agent knows: this code calls this API which has these characteristics
```

### AgenticCodebase + AgenticContract (Future)

Code behavior becomes contractually verifiable:

```python
contract = Contract.between(frontend, backend)
contract.requires("User.create response time < 200ms")
contract.requires("User.delete is idempotent")

# AgenticCodebase verifies the implementation matches the contract
codebase.verify(contract)
# Returns: violations, near-violations, recommendations
```

---

## Technical Requirements

- **Languages**: Rust core engine, Python SDK, TypeScript SDK
- **Parsing**: tree-sitter for syntax, custom semantic analysis layer
- **Supported Languages (V1)**: Python, Rust, TypeScript/JavaScript, Go
- **File I/O**: Memory-mapped via `mmap()`
- **Collective**: gRPC for delta sync, content-addressed storage
- **Embedding**: Pluggable (local or API-based code embeddings)
- **Minimum Dependencies**: tree-sitter, compression, standard library
- **Performance Target**: Compile 100K LOC repo in <30 seconds, queries <10ms

---

## What This Enables

1. **Zero-context coding agents**: Agent loads `.acb` file, instantly understands entire codebase
2. **Impact-aware changes**: Every modification shows downstream effects before execution
3. **Collective expertise**: New developers get expert-level library knowledge immediately
4. **Predictive maintenance**: Know what will break before it breaks
5. **Cross-language reasoning**: Polyglot codebases become single navigable graphs
6. **Persistent learning**: Agent remembers codebase insights across sessions
7. **Architectural enforcement**: Drift detection prevents technical debt accumulation

---

## Build Priority

AgenticCodebase is **Product #3** in the ecosystem:

```
AgenticMemory   ← SHIPPED: The brain
AgenticVision   ← SHIPPED: The eyes (web perception)
AgenticCodebase ← NOW: The hands (code manipulation)
AgenticIdentity ← NEXT: Trust layer
AgenticContract ← NEXT: Transaction layer
AgenticOS       ← FINAL: The ambient layer
```

**Ship AgenticCodebase. Give agents hands that understand what they're touching.**

---

*The codebase isn't files. It's a living graph of concepts, relationships, history, and predictions. Navigate it.*
