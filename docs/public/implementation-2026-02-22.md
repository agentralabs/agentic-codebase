# Implementation Report (2026-02-22)

This page records the codebase-system upgrades implemented in this cycle.

## What was added

1. Ingestion coverage accounting:
   - Detailed parser coverage counters for seen/candidate/skipped/error files.
   - Skip-reason breakdown (unknown language, filters, excludes, file size, tests).
2. Compile report output:
   - `acb compile ... --coverage-report <path>` writes machine-readable coverage JSON.
3. New operational commands:
   - `acb health <graph.acb>` for graph risk/test-gap/hotspot/dead-code summary.
   - `acb gate <graph.acb> --unit-id <id>` for CI-style change-risk gating.
4. Query surface expansion:
   - Added CLI query types:
     - `test-gap`
     - `hotspots`
     - `dead-code`

## Why this matters

- Makes code ingestion transparent and auditable.
- Adds CI-grade quality gates for automated agent workflows.
- Extends actionable query coverage for planning and maintenance.

## Verified commands

```bash
acb compile ./repo -o repo.acb --coverage-report coverage.json
acb query repo.acb test-gap
acb query repo.acb hotspots
acb query repo.acb dead-code
acb health repo.acb
acb gate repo.acb --unit-id 42 --max-risk 0.60 --require-tests
```

## Files changed

- `src/parse/parser.rs`
- `src/cli/commands.rs`
- `docs/public/command-surface.md`
- `docs/public/quickstart.md`
- `README.md`

