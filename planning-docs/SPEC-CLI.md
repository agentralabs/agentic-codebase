# SPEC-CLI.md

> The `acb` command-line tool. Every command maps to the library API.

---

## Binary Name

`acb` — AgenticCodeBase

---

## Global Flags

```
--format <FORMAT>    Output format: "text" (default) or "json"
--verbose            Enable debug logging
--quiet              Suppress non-essential output
--color <WHEN>       Color output: "auto", "always", "never"
```

---

## Commands

### `acb compile`

Compile a repository into an `.acb` file.

```
acb compile <PATH> [OPTIONS]

Arguments:
  PATH          Path to repository root

Options:
  -o, --output <FILE>      Output file (default: <PATH>/.acb)
  -l, --languages <LANGS>  Languages to include (comma-separated)
  -x, --exclude <PATTERN>  Glob patterns to exclude (can be repeated)
  --no-tests               Exclude test files
  --no-git                 Skip git history analysis
  --incremental            Update existing .acb instead of rebuild
  --dimension <DIM>        Feature vector dimension (default: 256)
```

Output:
```
Compiling ./my-project...
  Scanning files: 1,234 files found
  Parsing: ████████████████████ 100% (2.3s)
  Analyzing: ████████████████████ 100% (1.1s)
  Building graph: 12,847 units, 43,291 edges
  Writing: ./my-project/.acb (45.2 MB)
Done in 4.7s
```

---

### `acb info`

Display information about an `.acb` file.

```
acb info <FILE>
```

Output:
```
File: my-project.acb
Version: 1
Dimension: 256
Compiled: 2026-02-19 14:23:01 UTC

Units: 12,847
  Modules: 234
  Types: 1,892
  Functions: 8,456
  Imports: 2,103
  Tests: 162

Edges: 43,291
  Calls: 28,432
  Imports: 5,234
  Inherits: 892
  Contains: 8,733

Languages:
  Python: 8,234 units (64%)
  TypeScript: 4,613 units (36%)

File size: 45.2 MB
  Units: 1.2 MB
  Edges: 1.7 MB
  Strings: 0.8 MB
  Vectors: 39.2 MB
  Temporal: 2.1 MB
  Indexes: 0.2 MB
```

---

### `acb query`

Run queries against a compiled codebase.

```
acb query <FILE> <QUERY_TYPE> [OPTIONS]
```

#### Symbol Lookup
```
acb query project.acb symbol "process_payment"
acb query project.acb symbol "User" --type class
acb query project.acb symbol "test_" --mode prefix --limit 20
```

#### Dependency Graph
```
acb query project.acb deps 4521 --depth 3
acb query project.acb deps "payments.stripe.process_payment" --depth 5
```

#### Reverse Dependencies
```
acb query project.acb rdeps 4521 --depth 3
```

#### Impact Analysis
```
acb query project.acb impact 4521
acb query project.acb impact "User.email" --include-tests
```

Output:
```
Impact Analysis for: User.email (id: 4521)

Direct Dependents (5):
  ⚠️  HIGH   UserSerializer.to_dict (id: 4522) - No test coverage
  ⚠️  HIGH   NotificationService.send (id: 4530) - No test coverage
  ⚡ MEDIUM OrderConfirmation.render (id: 4567) - Partial coverage
  ✓  LOW    test_user_email (id: 8901) - Test
  ✓  LOW    test_email_validation (id: 8902) - Test

Transitive Dependents: 23 units (use --verbose for details)

Risk Summary:
  High risk: 2
  Medium risk: 3
  Low risk: 18

Recommendation: Add tests for UserSerializer and NotificationService before modifying
```

#### Call Graph
```
acb query project.acb calls 4521 --direction callers
acb query project.acb calls 4521 --direction both --depth 2
```

#### Similarity
```
acb query project.acb similar 4521 --top 10
```

#### Prophecy
```
acb query project.acb prophecy --scope all
acb query project.acb prophecy --scope module "payments"
```

Output:
```
Code Prophecy Report

🔮 Predictions:

  1. payments/stripe.py — LIKELY TO BREAK
     Confidence: 78%
     Reasoning: Changed 12 times in 30 days, 8 resulted in bugfixes
     Estimated days to incident: 14
     Recommendation: Refactor into smaller units

  2. auth/session.py — NEEDS REFACTORING
     Confidence: 65%
     Reasoning: Complexity increasing, test coverage declining
     Recommendation: Extract SessionManager class

⚠️ Ecosystem Alerts:

  fastapi 0.104.1 → 0.105.0
  Alert: Broke 23% of similar codebases
  Your risk: HIGH (matches affected patterns)
  Recommendation: Pin version, delay upgrade
```

#### Stability
```
acb query project.acb stability 4521
acb query project.acb stability --all --min-risk medium
```

#### Coupling
```
acb query project.acb coupling --all
acb query project.acb coupling 4521
```

Output:
```
Hidden Couplings Detected:

  payments.validate() ↔ audit.log()
    Type: HIDDEN (no explicit dependency)
    Strength: 0.89 (change together 89% of time)
    Evidence: 47 co-changes in last 6 months

  User.email ↔ notifications.send()
    Type: TEMPORAL
    Strength: 0.72
    Evidence: 31 co-changes
```

---

### `acb get`

Get detailed information about a specific unit.

```
acb get <FILE> <UNIT_ID>
```

Output:
```
Code Unit 4521

Type: function
Language: Python
Name: process_payment
Qualified: payments.stripe.process_payment
File: payments/stripe.py
Lines: 45-89

Signature: (amount: Decimal, currency: str) -> PaymentResult
Doc: "Process a payment through Stripe."

Visibility: public
Complexity: 12
Async: true

Stability: 0.34 (VOLATILE)
Changes: 47
Last modified: 2026-02-15

Edges Out (15):
  CALLS → stripe.Charge.create (4522)
  CALLS → validate_amount (4523)
  IMPORTS → decimal.Decimal (102)
  ...

Edges In (8):
  CALLED_BY ← PaymentHandler.process (4600)
  TESTED_BY ← test_payment_success (8901)
  ...
```

---

### `acb traverse`

Interactive graph traversal.

```
acb traverse <FILE> <START_ID>
```

Output:
```
Starting at: process_payment (4521)

Edges:
  [1] CALLS → stripe.Charge.create (4522)
  [2] CALLS → validate_amount (4523)
  [3] CALLS → log_payment (4530)
  [4] IMPORTS → Decimal (102)
  [b] ← Back
  [q] Quit

Select: 1

Now at: stripe.Charge.create (4522)
Edges:
  [1] CALLS → requests.post (external)
  [2] IMPORTS → stripe.api_key (103)
  [b] ← Back
  [q] Quit
```

---

### `acb export`

Export graph data to other formats.

```
acb export <FILE> --format <FORMAT> --output <PATH>

Formats:
  json      Full graph as JSON
  dot       Graphviz DOT format
  csv       Units and edges as CSV files
  cypher    Neo4j Cypher import statements
```

---

### `acb serve`

Start the MCP server (for integration testing).

```
acb serve [OPTIONS]

Options:
  --stdio             Use stdio transport (default)
  --port <PORT>       Use HTTP transport on port
  --preload <FILE>    Preload an .acb file
```

---

### `acb collective`

Manage collective intelligence.

```
acb collective status           Show collective sync status
acb collective sync             Sync library patterns
acb collective enable           Enable collective (opt-in)
acb collective disable          Disable collective
acb collective query <library>  Query collective for library patterns
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | File not found |
| 4 | Parse error |
| 5 | Query error |

---

## Configuration File

`~/.config/acb/config.toml`:

```toml
[compile]
default_dimension = 256
default_exclude = ["node_modules", "target", ".git", "__pycache__"]

[collective]
enabled = true
endpoint = "https://collective.agenticcodebase.dev"

[display]
color = "auto"
format = "text"
```

---

## Shell Completion

```bash
# Bash
acb completions bash > /etc/bash_completion.d/acb

# Zsh
acb completions zsh > ~/.zfunc/_acb

# Fish
acb completions fish > ~/.config/fish/completions/acb.fish
```
