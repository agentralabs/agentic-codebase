# SPEC-RESEARCH-PAPER.md (Canonical Template)

> **Status:** Canonical — All sisters use this template
> **Run:** AFTER all build phases complete and all tests pass
> **Output:** Publication-grade research paper with real benchmark data
> **Version:** 2.0

---

## Overview

This template generates a 5-10 page LaTeX research paper presenting the sister as a novel contribution to AI agent infrastructure. The paper must look and feel like a top-tier systems paper from Google Research, DeepSeek, or Meta AI — professional typesetting, real benchmark data, comparison tables, architecture diagrams rendered as TikZ or pgfplots figures, and rigorous technical writing.

This is NOT a README. This is NOT documentation. This is a **research publication** that establishes priority, demonstrates the technical contribution, and provides reproducible results.

**HARD RULE: Every number in the paper must come from an actual measurement. No estimates. No "realistic numbers based on architecture." Run the benchmarks, measure the data, THEN write the paper.**

---

## MANDATORY: Pre-Paper Benchmark Protocol

**The paper CANNOT be written until this protocol is completed. This is non-negotiable.**

### Phase 1: Create Benchmark Suite

Before writing a single line of LaTeX, create a comprehensive benchmark suite using [Criterion](https://bheisler.github.io/criterion.rs/book/) (Rust) or equivalent statistical benchmarking framework.

**Minimum benchmark coverage (adapt per sister):**

| Category | Minimum Benchmarks | What to Measure |
|----------|-------------------|-----------------|
| File I/O | 4+ | create, open at 100/1K/10K entities, save at 100/1K/10K entities |
| Entity Operations | 4+ | add/create each entity type (in-memory insertion latency) |
| Core Computations | 4+ | Primary O(1) operations (decay eval, hash, lookup, etc.) |
| Queries | 5+ | Each query type at representative scale |
| Write Engine | 4+ | End-to-end write operations including fsync |

**Benchmark file location:** `crates/agentic-{sister}/benches/benchmarks.rs` (or equivalent)

**Requirements:**
- Use Criterion 0.5+ with `html_reports` feature
- Minimum 100 samples per measurement (Criterion default)
- Warm-up iterations before measurement
- Build and run with `--release` profile
- Record hardware specs at time of measurement

```toml
# Required in Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmarks"
harness = false
```

### Phase 2: Run Benchmarks and Record Results

```bash
# Run full benchmark suite
cargo bench --release 2>&1 | tee benchmark-output.txt

# Record hardware context
echo "CPU: $(sysctl -n machdep.cpu.brand_string 2>/dev/null || lscpu | grep 'Model name')" > benchmark-context.txt
echo "RAM: $(sysctl -n hw.memsize 2>/dev/null || free -h | grep Mem)" >> benchmark-context.txt
echo "OS: $(uname -srm)" >> benchmark-context.txt
echo "Rust: $(rustc --version)" >> benchmark-context.txt
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> benchmark-context.txt
```

### Phase 3: Measure File Sizes

Create a measurement script or integration test that:
1. Creates files at 10, 100, 1K, 10K entities (minimum 4 scales)
2. Measures actual file size in bytes
3. Calculates bytes-per-entity
4. Tests with single entity types AND mixed types

```bash
# Example: run a file size measurement test
cargo test --release -- filesize_measurement --nocapture
```

### Phase 4: Extract and Tabulate Results

From the Criterion output, extract:
- **Mean time** for each benchmark (the primary number)
- **Standard deviation** (for error bars if needed)
- **Throughput** where applicable (ops/sec)

From the file size measurement:
- **Raw file size** at each scale
- **Bytes per entity** at each scale
- **Header overhead** (measured, not assumed)

**ALL of these numbers go directly into the paper. No rounding to "nice" numbers. Use the measured values.**

### Phase 5: THEN Write the Paper

Only after Phases 1-4 are complete do you open a LaTeX editor. The paper describes what you measured, not what you hope to measure.

---

## MANDATORY: Website Update Protocol

When a new sister is published, the website (agentralabs.tech) must be updated. This is part of the release process, not a separate task.

### Required Website Updates

1. **Navigation Registration**
   - Add sister to `src/config/navigation.ts` (or equivalent)
   - Add sister to docs sidebar with correct grouping
   - Verify sister appears in ecosystem feature reference

2. **Scenario Data**
   - Create `src/data/scenarios-{sister}.ts` with real usage examples
   - Each scenario must demonstrate a primary problem the sister solves
   - Minimum 3 scenarios per sister

3. **Install Route**
   - Register sister in web install flow
   - npm install command must work end-to-end
   - Verify install page renders with correct binary name, commands, MCP config

4. **Documentation Pages**
   - All `docs/public/*.md` files with `status: stable` frontmatter must be wired into web docs
   - `sister.manifest.json` must list all page_ids
   - Each page_id must resolve to a renderable doc page on the website

5. **Visual Assets**
   - `assets/github-hero-pane.svg` and `assets/github-terminal-pane.svg` must render on sister's landing page
   - `assets/benchmark-chart.svg` must render in benchmarks section
   - `assets/architecture-agentra.svg` must render in architecture section

### Website Verification

```bash
# From workspace root
bash scripts/check-canonical-consistency.sh

# Per-sister
cd agentic-{sister} && bash scripts/check-canonical-sister.sh
```

---

## Sister-Specific Variables

Fill these in for each sister:

```
SISTER_NAME:        [e.g., AgenticMemory, AgenticTime, AgenticCodebase]
SISTER_TAGLINE:     [e.g., "Never forget", "When should this happen?"]
FILE_EXTENSION:     [e.g., .amem, .atime, .acb]
BINARY_NAME:        [e.g., amem, atime, acb]
PROBLEM_DOMAIN:     [e.g., memory, temporal reasoning, code understanding]
CORE_ENTITY_COUNT:  [e.g., 6 cognitive events, 5 temporal entities]
EDGE_TYPE_COUNT:    [e.g., 7 edge types]
INVENTION_COUNT:    [e.g., 7 inventions, 16 inventions]
QUERY_TYPE_COUNT:   [e.g., 5 query types]
MCP_TOOL_COUNT:     [e.g., 15 tools, 19 tools]
```

---

## Author

**Author name: [TO BE FILLED BY USER]**

Use this name as the sole author on the paper. Affiliation line should read: "Independent Researcher" or "Agentralabs"

---

## Paper Structure

Follow the standard systems research paper format:

### 1. Title

Format: `{SISTER_NAME}: A Binary {FORMAT_TYPE} for {CORE_VALUE_PROPOSITION}`

Examples:
- `AgenticMemory: A Binary Graph Format for Persistent, Portable, and Navigable AI Agent Memory`
- `AgenticTime: A Binary Temporal Format for Deadline Prophecy, Decay Modeling, and Temporal Reasoning in AI Agents`
- `AgenticCodebase: A Binary Semantic Graph for Code Understanding, Impact Analysis, and Hallucination Prevention in AI Agents`
- `AgenticIdentity: A Binary Receipt Chain for Trust, Competence, and Cryptographic Accountability in AI Agents`

### 2. Abstract (150-250 words)

Must contain these elements in order:

1. **The Problem** — What limitation of current AI agents does this sister solve?
2. **Current State** — Why existing solutions fail (lossy, flat, slow, stateless, etc.)
3. **The Insight** — The key conceptual breakthrough (navigation not search, receipts not claims, etc.)
4. **The Contribution** — What the sister provides (binary format, typed entities, O(1) access, etc.)
5. **Key Results** — Specific numbers from benchmarks (file size, query latency, entity capacity)
6. **Impact Statement** — Why this matters (portable, zero dependencies, sub-millisecond, etc.)

**Every number in the abstract must appear in a table or figure in the Evaluation section.**

Template:
```
AI agents suffer from {CORE_PROBLEM}. Current approaches using {EXISTING_SOLUTIONS}
are {LIMITATIONS}. We present {SISTER_NAME}, a binary {FORMAT_TYPE} format that
treats {DOMAIN} as a {KEY_INSIGHT} problem rather than {NAIVE_APPROACH}.

{SISTER_NAME} provides {CORE_ENTITY_COUNT} typed {ENTITY_NAME}s with
{EDGE_TYPE_COUNT} relationship types, enabling {KEY_CAPABILITY}. The {FILE_EXTENSION}
format achieves {COMPRESSION_RATIO}x compression over JSON, supports {MAX_ENTITIES}+
entities with sub-millisecond queries, and requires zero external dependencies.

We demonstrate {KEY_BENCHMARK_RESULT_1}, {KEY_BENCHMARK_RESULT_2}, and
{KEY_BENCHMARK_RESULT_3}. {SISTER_NAME} enables {IMPACT_1}, {IMPACT_2}, and
{IMPACT_3} — capabilities previously impossible with existing solutions.
```

### 3. Introduction (1-1.5 pages)

Structure (5 paragraphs):

**Paragraph 1: The Problem**
- Open with the fundamental limitation this sister addresses
- Why AI agents need this capability
- What breaks without it

**Paragraph 2: Current Approaches and Limitations**
- Survey existing solutions (3-5 systems)
- Technical specifics of why each fails
- Common pattern of failure

**Paragraph 3: The Key Insight**
- The conceptual breakthrough
- Why this framing changes everything
- The "not X, but Y" statement

**Paragraph 4: The Contribution**
- What {SISTER_NAME} is
- Technical overview (format, entities, queries)
- What makes it different

**Paragraph 5: Results and Organization**
- Summary of benchmark results
- Paper organization ("Section 2 covers..., Section 3 presents...")

**Figure 1 (Required): Motivating Comparison**
Show the same scenario handled by:
- (a) Current approach (flat, disconnected, lossy)
- (b) {SISTER_NAME} approach (structured, connected, navigable)

Use TikZ. Two-panel figure. Clear visual contrast.

### 4. Background and Related Work (1 page)

Cover 5-7 related systems with technical specificity:

For each system:
- What it does (1 sentence)
- How it works technically (1-2 sentences)
- Key strength (1 sentence)
- Key limitation relevant to your contribution (1-2 sentences)

**Categories to cover (adapt per sister):**

| Sister | Related Work Categories |
|--------|------------------------|
| Memory | Vector DBs, RAG systems, Memory frameworks (Mem0, MemGPT, LangMem), LLM native memory |
| Time | Calendar APIs, Temporal databases, Planning systems, Scheduling libraries |
| Codebase | Static analyzers, LSP servers, Code search (Sourcegraph), AI code tools |
| Identity | Auth systems, Capability systems, Audit logs, Blockchain identity |
| Vision | Screenshot tools, Visual diff, OCR systems, Vision APIs |

**Table 1 (Required): Related Work Comparison**

| System | Storage Format | {KEY_DIMENSION_1} | {KEY_DIMENSION_2} | Dependencies | Query Model |
|--------|---------------|-------------------|-------------------|--------------|-------------|
| System A | ... | ... | ... | ... | ... |
| System B | ... | ... | ... | ... | ... |
| {SISTER_NAME} | Binary {FILE_EXTENSION} | Full | Full | None | {QUERY_MODEL} |

### 5. Architecture (2-2.5 pages)

This is the core technical section. Subsections:

#### 5.1 Core Entities (The Atom)

- Define each entity type with precise semantics
- Show the data structure (Rust struct or schema)
- Explain why these types matter (not just storing data — storing meaning)

**Figure 2 (Required): Entity Type Taxonomy**
Clean diagram showing all entity types with one-line example for each.
Use TikZ with consistent styling.

#### 5.2 Relationships / Edges (The Connections)

- Define all edge/relationship types
- Explain semantics of each
- Highlight critical edges (e.g., SUPERSEDES for correction, CAUSED_BY for reasoning)

**Figure 3 (Required): Example Graph/Structure**
Show a realistic subgraph with multiple entity types connected by typed edges.
Use TikZ with colored edges by type. Include legend.

#### 5.3 Binary File Format

- File layout (header, entity table, edge table, content blocks, indexes)
- Why binary over text/JSON (size reduction, parse overhead, mmap support)
- Why single file (portability, atomicity, no external dependencies)
- O(1) access patterns

**Figure 4 (Required): File Format Layout**
Diagram showing sections with byte offsets and sizes.
Use TikZ with labeled blocks.

#### 5.4 Query Model

- Define all query types (count: {QUERY_TYPE_COUNT})
- Explain the core insight (navigation not search, traversal not lookup, etc.)
- Contrast with naive approaches

#### 5.5 Write Pipeline / Formation

- How entities are created from agent interactions
- How relationships are established
- How updates/corrections work
- How data is persisted

### 6. Evaluation (1.5-2 pages)

**HARD RULE: This section uses ONLY real measured data from the Pre-Paper Benchmark Protocol.**

**No estimates. No "realistic numbers based on architecture." No README-sourced approximations. Every single number in every table, figure, and body paragraph must trace back to a Criterion benchmark run or a file size measurement.**

If a benchmark has not been run, the paper is not ready. Go back to Phase 1.

#### 6.1 Benchmark Setup

- Hardware description (CPU, RAM, SSD/HDD, OS) — from benchmark-context.txt
- Dataset description (synthetic graphs/data at multiple scales)
- Compiler version and build flags (--release)
- Measurement methodology: Criterion with 100 samples, warm-up, statistical analysis
- **State the exact command used:** `cargo bench --release`

#### 6.2 Storage Efficiency

**Table 2: File Size Scaling** (measured values from Phase 3)

| {ENTITIES} | {RELATIONSHIPS} | Raw JSON Size | {FILE_EXTENSION} Size | Compression Ratio |
|------------|-----------------|---------------|----------------------|-------------------|
| 100 | ... | ... | ... | ...x |
| 1,000 | ... | ... | ... | ...x |
| 10,000 | ... | ... | ... | ...x |

Include bytes-per-entity calculation. Include header size (measured, not assumed).

**Figure 5 (Required): Storage Scaling Chart**
Log-scale line chart showing file size vs entity count.
Use pgfplots. Show linear scaling property.

#### 6.3 Query Performance

**Table 3: Query Latency (microseconds, from Criterion mean)** (measured values from Phase 2)

| Query Type | 1K {ENTITIES} | 10K {ENTITIES} | 100K {ENTITIES} |
|------------|---------------|----------------|-----------------|
| {QUERY_1} | ... | ... | ... |
| {QUERY_2} | ... | ... | ... |
| {QUERY_3} | ... | ... | ... |
| {QUERY_4} | ... | ... | ... |
| {QUERY_5} | ... | ... | ... |

**Figure 6 (Required): Query Latency Chart**
Grouped bar chart by query type across scales.
Use pgfplots. Log scale if needed.

#### 6.4 Write Performance

**Table 4: Write Operations (microseconds, from Criterion mean)** (measured values)

| Operation | 1K Scale | 10K Scale | 100K Scale |
|-----------|----------|-----------|------------|
| Add {ENTITY} | ... | ... | ... |
| Add {RELATIONSHIP} | ... | ... | ... |
| Batch insert (100) | ... | ... | ... |
| File save | ... | ... | ... |
| File load | ... | ... | ... |

#### 6.5 Comparison with Existing Systems

**Table 5: System Comparison**

| Dimension | {SYSTEM_A} | {SYSTEM_B} | {SYSTEM_C} | {SISTER_NAME} |
|-----------|------------|------------|------------|---------------|
| Storage per 10K entities | ... | ... | ... | ... |
| Query latency (p99) | ... | ... | ... | <1ms |
| {KEY_FEATURE_1} | ... | ... | Partial | Full |
| {KEY_FEATURE_2} | ... | ... | ... | Full |
| External dependencies | Cloud/API | API | Service | None |
| Portability | Vendor-locked | Limited | Limited | Single file |

Note: Comparison numbers for external systems should be sourced from published benchmarks or documented with methodology. If estimated, state clearly that ONLY the external system numbers are estimated.

**Figure 7 (Required): Radar/Spider Chart**
Compare {SISTER_NAME} vs 2-3 existing systems across 5-6 dimensions.
Use pgfplots or TikZ. Dimensions should highlight {SISTER_NAME}'s strengths fairly.

#### 6.6 Capacity Projections

**Table 6: Real-World Capacity** (calculated from measured bytes-per-entity, NOT estimated)

| Use Case | {ACTIVITY}/Day | {ENTITIES}/{ACTIVITY} | Size/Year | Years in 1GB |
|----------|----------------|----------------------|-----------|--------------|
| Light usage | ... | ... | ... | ... |
| Moderate usage | ... | ... | ... | ... |
| Heavy usage | ... | ... | ... | ... |
| Enterprise | ... | ... | ... | ... |

### 7. Discussion (0.5-1 page)

Three parts:

**What This Enables**
- List 3-5 capabilities that weren't possible before
- Be specific and technical

**Limitations**
- Honest assessment of current constraints
- What the system doesn't do (yet)
- Known edge cases or performance cliffs
- **Use measured numbers when discussing performance limits** (e.g., "save latency reaches X ms at 10K entities")

**Future Work**
- Planned improvements (2-3 items)
- Connection to larger ecosystem (mention other sisters if appropriate, but don't reveal full roadmap)
- Research directions opened by this work

### 8. Conclusion (0.5 page)

Structure:
1. Restate the contribution (1-2 sentences)
2. Headline results (3-4 key numbers — **all measured**)
3. Significance statement (why this matters for AI agents)
4. Availability statement (open source, where to find it)

### 9. References

**Minimum 15 references.** Include:

**Foundation papers:**
- Attention Is All You Need (Vaswani et al.) — LLM foundation
- RAG paper (Lewis et al.) — if relevant to domain

**Related systems (pick relevant ones):**
- MemGPT paper (memory)
- Mem0 documentation (memory)
- Temporal logic papers (time)
- Program analysis papers (codebase)
- Capability systems papers (identity)

**Technical foundations:**
- Memory-mapped I/O literature
- Graph database literature (for contrast)
- Binary format papers (Protocol Buffers, FlatBuffers, Cap'n Proto)
- Compression papers (LZ4, etc.)
- Relevant data structure papers

**Format:** Numbered references, standard CS citation style (author-year or numeric).

---

## LaTeX Requirements

### Document Class and Packages

```latex
\documentclass[10pt, twocolumn]{article}
\usepackage[margin=0.75in]{geometry}
\usepackage{graphicx}
\usepackage{tikz}
\usepackage{pgfplots}
\usepackage{booktabs}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{textcomp}
\usepackage{algorithm}
\usepackage{algorithmic}
\usepackage{hyperref}
\usepackage{xcolor}
\usepackage{caption}
\usepackage{subcaption}
\usepackage{fancyhdr}
\usepackage{enumitem}
\usepackage{listings}
\usepackage{microtype}
\pgfplotsset{compat=1.18}

% Color palette (consistent across all sisters)
\definecolor{primary}{RGB}{41, 128, 185}      % Blue
\definecolor{secondary}{RGB}{39, 174, 96}     % Green
\definecolor{accent}{RGB}{230, 126, 34}       % Orange
\definecolor{neutral}{RGB}{127, 140, 141}     % Gray
\definecolor{highlight}{RGB}{155, 89, 182}    % Purple
```

### Visual Quality Standards

- **All figures must be TikZ or pgfplots** — no external image files, no screenshots
- **Color palette:** Use the defined palette above (consistent across all sisters)
- **Tables:** Use booktabs (toprule, midrule, bottomrule — no vertical lines)
- **Code snippets:** Use listings package with syntax highlighting
- **Fonts:** Standard LaTeX Computer Modern or newtxtext/newtxmath
- **Two-column layout** — standard for systems papers
- **Figures span full width when needed** using `figure*` environment
- **Every figure and table must be referenced in the text**
- **Figure captions below, table captions above** — standard convention

### Required Figures (minimum 7)

| Figure | Content | Environment |
|--------|---------|-------------|
| 1 | Motivating comparison (current vs {SISTER_NAME}) | figure* (full width) |
| 2 | Entity type taxonomy | figure |
| 3 | Example graph/structure with typed edges | figure* (full width) |
| 4 | File format layout | figure |
| 5 | Storage scaling chart | figure |
| 6 | Query latency chart | figure |
| 7 | Radar chart system comparison | figure |

### Required Tables (minimum 6)

| Table | Content |
|-------|---------|
| 1 | Related work comparison |
| 2 | File size scaling data |
| 3 | Query latency benchmarks |
| 4 | Write performance benchmarks |
| 5 | System comparison |
| 6 | Capacity projections |

---

## Build Process

### Step 1: Run Benchmarks (BEFORE writing the paper)

```bash
# Create benchmark suite in benches/benchmarks.rs
# Then run:
cargo bench --release 2>&1 | tee benchmark-output.txt
```

### Step 2: Measure File Sizes

```bash
# Create and run file size measurement test
cargo test --release -- filesize_measurement --nocapture
```

### Step 3: Write the Paper (using measured data)

```bash
# Only now do you write the .tex file
# Every number comes from benchmark-output.txt and file size measurements
```

### Step 4: Compile LaTeX

```bash
# Full compilation pipeline:
pdflatex {sister}-paper.tex
bibtex {sister}-paper
pdflatex {sister}-paper.tex
pdflatex {sister}-paper.tex

# Verify output:
# - PDF is 5-10 pages
# - All figures render
# - All tables formatted with booktabs
# - No compilation warnings
```

### Required Output Files

```
{sister}-paper.tex      — LaTeX source
{sister}-paper.pdf      — Compiled PDF
references.bib          — Bibliography (if using bibtex)
```

Naming convention:
- `agenticmemory-paper.pdf`
- `agentictime-paper.pdf`
- `agenticcodebase-paper.pdf`
- `agenticidentity-paper.pdf`
- `agenticvision-paper.pdf`

---

## Tone and Writing Style

### Do:
- **Technical but accessible** — rigorous for researchers, readable for engineers
- **Precise claims** — every number has a table or figure backing it
- **Honest about limitations** — acknowledge what the system doesn't do
- **Active voice** — "We design..." not "A system was designed..."
- **Present tense for system** — "{SISTER_NAME} stores..." not "will store..."
- **Past tense for experiments** — "We measured query latency..."

### Don't:
- **No marketing language** — no "revolutionary," "groundbreaking," "game-changing"
- **No vague claims** — no "significantly faster" without numbers
- **No future promises** — describe what IS, not what WILL BE
- **No competitor bashing** — state limitations factually, not pejoratively
- **No estimated numbers for own system** — every claim about the sister must be measured

---

## Quality Checklist

Before delivering the PDF:

```
BENCHMARKS (must be completed BEFORE writing):
[ ] Criterion benchmark suite exists in benches/benchmarks.rs (or equivalent)
[ ] cargo bench --release runs without errors
[ ] Minimum 20 benchmarks across 5 categories
[ ] File size measurements at 4+ entity scales
[ ] All paper numbers trace to benchmark output

STRUCTURE:
[ ] Paper is 5-10 pages in two-column format
[ ] Abstract is 150-250 words
[ ] All 9 sections present
[ ] Introduction has motivating figure

FIGURES:
[ ] All 7+ figures render correctly
[ ] All figures use TikZ/pgfplots (no external images)
[ ] All figures referenced in text
[ ] Consistent color palette
[ ] Captions are descriptive

TABLES:
[ ] All 6+ tables present
[ ] All tables use booktabs formatting
[ ] All tables referenced in text
[ ] Captions above tables

TECHNICAL:
[ ] ALL benchmark numbers are MEASURED (zero estimates for own system)
[ ] All comparisons are fair and sourced
[ ] Limitations section is honest and uses measured numbers
[ ] References section has 15+ entries
[ ] Abstract numbers match Evaluation section tables

LATEX:
[ ] No compilation warnings
[ ] No orphan figures/tables
[ ] No placeholder text remaining
[ ] PDF renders correctly

WEBSITE:
[ ] Sister navigation registered on agentralabs.tech
[ ] Scenario data file created
[ ] Install route wired
[ ] All stable docs pages wired into web docs
[ ] Visual assets render on sister landing page

META:
[ ] Author name correct
[ ] Affiliation correct
[ ] Sister name consistent throughout
[ ] File extension consistent throughout
```

---

## Guardrail Enforcement Markers

The guardrail scripts (`check-canonical-sister.sh`) enforce the following automatically:

1. **paper/ directory exists** with paper-i-* subfolder
2. **{sister}-paper.tex exists** in the paper-i-* subfolder
3. **references.bib exists** in the paper-i-* subfolder
4. **benches/ directory exists** with at least one benchmark file
5. **Benchmark file references Criterion** (proves real benchmarks, not stubs)

These are minimum structural checks. The full quality checklist above is the responsibility of the paper author.

---

## Sister-Specific Customization Guide

### For AgenticMemory
- Entities: 6 cognitive event types (Fact, Decision, Inference, Correction, Skill, Episode)
- Edges: 7 relationship types
- Key insight: "Memory is navigation, not search"
- Comparison systems: Vector DBs, Mem0, MemGPT, OpenClaw

### For AgenticTime
- Entities: 5 temporal types (Duration, Deadline, Schedule, Sequence, Decay)
- Inventions: 16 temporal inventions
- Key insight: "Time is a landscape to navigate, not a line to follow"
- Comparison systems: Calendar APIs, temporal DBs, scheduling libraries

### For AgenticCodebase
- Entities: Code units (functions, classes, modules) + relationships
- Languages: 8 supported
- Key insight: "Understanding, not generation"
- Comparison systems: LSP, Sourcegraph, static analyzers

### For AgenticIdentity
- Entities: Identity, receipts, grants, competence scores
- Key insight: "Prove, don't claim"
- Comparison systems: Auth systems, audit logs, capability systems

### For AgenticVision
- Entities: Captures, observations, diffs
- Key insight: "Grounded perception, not blind claims"
- Comparison systems: Screenshot tools, visual diff, OCR

---

## Template Maintenance

This template is canonical. Updates require:
1. Approval in the main workspace
2. Propagation to all sister repos
3. Version bump notation

Current version: 2.0
Last updated: 2026-02-26
