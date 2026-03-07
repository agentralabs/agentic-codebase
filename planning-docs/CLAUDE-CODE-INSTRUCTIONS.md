# AgenticCodebase — Claude Code Build Instructions

> **Read this file first. Then read every SPEC file in order. Build everything. Test everything. Do not stop until all tests pass with zero failures.**

---

## What You Are Building

AgenticCodebase is a semantic code compiler that transforms source code into a navigable graph of concepts, relationships, and patterns. It stores the entire semantic structure of a codebase in a single memory-mappable `.acb` file.

This is NOT a language server. This is NOT a grep replacement. This is NOT another AST parser.

This is a **navigable brain for code** — where agents traverse concepts, predict impacts, and learn from collective intelligence.

---

## Reference Standards

**CRITICAL**: AgenticCodebase must match the quality and rigor established by:

1. **AgenticMemory** — 72-byte node records, 16+ query types, sub-millisecond traversal at 100K+ nodes, 96 tests passing, comprehensive benchmarks
2. **AgenticVision** — Binary web cartography, collective graph, temporal intelligence

Before implementing ANY component, mentally verify: "Does this match the standard set by Memory and Vision?"

---

## Language & Toolchain

- **Language**: Rust (latest stable)
- **Build System**: Cargo
- **Parsing**: tree-sitter (multi-language)
- **Minimum Dependencies**: Only what's listed in SPEC-DEPENDENCIES.md
- **Target**: Library crate (`agentic-codebase`) + CLI binary (`acb`) + MCP server
- **Edition**: 2021

---

## Build Order — Follow This Exactly

### Phase 1: Foundation (Do this first)
1. Read `SPEC-PROJECT-STRUCTURE.md` — Create the exact directory structure
2. Read `SPEC-DEPENDENCIES.md` — Set up Cargo.toml
3. Read `SPEC-DATA-STRUCTURES.md` — Implement all types, structs, enums
4. Read `SPEC-FILE-FORMAT.md` — Implement the binary file format (read + write)
5. Run Phase 1 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 2: Parsing Engine
6. Read `SPEC-PARSING-ENGINE.md` — Implement multi-language parsing
7. Implement Python parser
8. Implement Rust parser
9. Implement TypeScript parser
10. Run Phase 2 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 3: Semantic Analysis
11. Read `SPEC-SEMANTIC-ENGINE.md` — Implement semantic extraction
12. Implement cross-file resolution
13. Implement cross-language FFI tracing
14. Run Phase 3 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 4: Query Engine
15. Read `SPEC-QUERY-ENGINE.md` — Implement all 24 query types
16. Core queries first (1-8)
17. Built queries next (9-11, 22-23)
18. Novel queries last (12-21, 24)
19. Run Phase 4 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 5: Indexes & Performance
20. Read `SPEC-INDEXES.md` — Implement all index structures
21. Implement mmap-based file access
22. Run Phase 5 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 6: CLI & Integration
23. Read `SPEC-CLI.md` — Implement the `acb` command-line tool
24. Read `SPEC-FFI.md` — Implement C FFI bindings
25. Run Phase 6 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 7: MCP Server (MCP-First!)
26. Read `SPEC-MCP.md` — Implement the MCP server
27. Implement all Tools
28. Implement all Resources
29. Implement all Prompts
30. Test with Claude Desktop
31. Run Phase 7 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 8: Collective Intelligence
32. Read `SPEC-COLLECTIVE.md` — Implement collective graph sync
33. Implement delta compression
34. Implement pattern aggregation
35. Run Phase 8 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 9: Temporal & Prophecy
36. Read `SPEC-TEMPORAL.md` — Implement temporal analysis
37. Implement stability scoring
38. Implement coupling detection
39. Implement prophecy engine
40. Run Phase 9 tests from `SPEC-TESTS.md` — All must pass before continuing

### Phase 10: Final Validation
41. Run the full test suite — zero failures
42. Run `cargo clippy` — zero warnings
43. Run `cargo fmt --check` — must pass
44. Build release binary: `cargo build --release`
45. Run benchmarks from `SPEC-TESTS.md`

### Phase 11: Research Paper
46. Read `SPEC-RESEARCH-PAPER.md` — Generate the full LaTeX research paper
47. Use REAL benchmark data from Phase 10 benchmarks
48. Compile to PDF with pdflatex
49. Verify 8-12 pages, all figures render, all tables formatted
50. Output: `agenticcodebase-paper.tex` + `agenticcodebase-paper.pdf`

---

## Rules

1. **Do not add dependencies not listed in SPEC-DEPENDENCIES.md.** If you think you need something, implement it yourself or find a way without it.
2. **Do not skip tests.** Every phase has tests. Run them. If they fail, fix the code, don't fix the test.
3. **Do not use unsafe Rust unless explicitly required** (mmap implementation, FFI boundaries).
4. **Every public function must have a doc comment.**
5. **Every module must have a module-level doc comment.**
6. **No unwrap() in library code.** All errors must be properly typed and propagated.
7. **No println!() in library code.** Use the log crate for diagnostics.
8. **Binary format must be little-endian on all platforms.**
9. **All timestamps are Unix epoch microseconds (u64).**
10. **Feature vectors use f32, not f64.** Memory efficiency matters.
11. **MCP interface is PRIMARY.** The library API exists to serve the MCP server.
12. **Match AgenticMemory's rigor.** If Memory has it, Codebase needs equivalent or better.

---

## Success Criteria

The build is complete when:

- [ ] `cargo test` — all tests pass, zero failures (300+ tests minimum)
- [ ] `cargo clippy` — zero warnings
- [ ] `cargo fmt --check` — passes
- [ ] `cargo build --release` — compiles successfully
- [ ] The `acb` CLI can compile, query, and inspect `.acb` files
- [ ] A 100,000 LOC codebase compiles in under 30 seconds
- [ ] All 24 query types execute in under 10ms on 100K symbol graphs
- [ ] The `.acb` file for 100K symbols is under 100MB
- [ ] FFI bindings compile and basic C tests pass
- [ ] MCP server passes all protocol compliance tests
- [ ] MCP server works with Claude Desktop (manual verification)
- [ ] All benchmarks run and produce output
- [ ] Research paper PDF generated — 8-12 pages, all figures and tables render
- [ ] Research paper uses real benchmark data from the build

---

## File Reading Order

```
1.  CLAUDE-CODE-INSTRUCTIONS.md     (this file)
2.  SPEC-PROJECT-STRUCTURE.md
3.  SPEC-DEPENDENCIES.md
4.  SPEC-DATA-STRUCTURES.md
5.  SPEC-FILE-FORMAT.md
6.  SPEC-PARSING-ENGINE.md
7.  SPEC-SEMANTIC-ENGINE.md
8.  SPEC-QUERY-ENGINE.md
9.  SPEC-INDEXES.md
10. SPEC-CLI.md
11. SPEC-FFI.md
12. SPEC-MCP.md
13. SPEC-COLLECTIVE.md
14. SPEC-TEMPORAL.md
15. SPEC-TESTS.md
16. SPEC-RESEARCH-PAPER.md          (Phase 11 — after all tests pass)
```

---

## Quality Benchmarks (from AgenticMemory)

AgenticCodebase must meet or exceed these metrics established by AgenticMemory:

| Metric | AgenticMemory | AgenticCodebase Target |
|--------|---------------|----------------------|
| Node record size | 72 bytes | 96 bytes |
| Edge record size | 32 bytes | 40 bytes |
| Query latency (100K nodes) | <1ms | <10ms |
| File size per 100K records | <50MB | <100MB |
| Test count | 96+ | 300+ |
| Query types | 16 | 24 |
| Build phases | 6 | 11 |

---

## The Ecosystem Integration Requirement

AgenticCodebase MUST integrate with:

1. **AgenticMemory** — Agents remember what they learned about codebases
   - CodeUnit IDs can be linked in Memory facts
   - Sessions track which codebase was analyzed

2. **AgenticVision** (future) — Code ↔ Web API mapping
   - External API calls traced to Vision's web maps
   - Rate limits, known errors visible in code context

3. **MCP** — Primary interface for LLM consumption
   - Every query type exposed as MCP tool
   - Every resource URI pattern defined
   - Every prompt template provided

---

## Start Now. Phase 1 First.

---

*Build the hands that understand what they touch.*
