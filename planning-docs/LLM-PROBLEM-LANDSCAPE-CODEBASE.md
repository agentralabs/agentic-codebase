# LLM Problem Landscape and Distribution (Codebase Lens)

Status: Planning-only (gitignored local working document)
Scope: Global LLM coding/system problems, with AgenticCodebase ownership focus
Updated: 2026-02-24

## 1. Exhaustive working catalog (plain language)

| ID | Problem all LLMs face today | Plain-language explanation | Primary sister |
|---|---|---|---|
| P01 | Context-window limits | The model cannot reliably hold the full project/problem state at once. | Memory |
| P02 | Retrieval noise | Search often returns text that looks related but is not decision-useful. | Memory |
| P03 | Provenance gaps | Answers may miss exact source/trace, reducing trust and auditability. | Codebase + Memory |
| P04 | Temporal staleness | The model may use old context after code/config changed. | Codebase + Memory |
| P05 | Cross-session amnesia | Important details are forgotten between sessions/restarts. | Memory |
| P06 | Contradiction persistence | Old wrong beliefs remain active and conflict with new facts. | Memory |
| P07 | Weak uncertainty calibration | Model sounds certain when confidence should be low. | Memory |
| P08 | Intent ambiguity | User requirement is under-specified; model fills gaps incorrectly. | External + Memory |
| P09 | Whole-repo topology blindness | Model understands files locally but misses global structure. | Codebase |
| P10 | Change-impact blindness | Hard to know what breaks when one unit changes. | Codebase |
| P11 | Hidden coupling | Fragile dependencies remain invisible until failure. | Codebase |
| P12 | Test-gap blindness | Missing or weak test coverage is not consistently surfaced. | Codebase |
| P13 | Refactor-safety uncertainty | Large edits are risky without structure-aware guardrails. | Codebase |
| P14 | Multi-language boundary gaps | Cross-language edges (Rust/Python/TS/etc.) are hard to reason about. | Codebase |
| P15 | Dependency/version drift | Upstream library changes silently invalidate assumptions. | Codebase |
| P16 | Build-system variance | Different build tools/profiles create inconsistent behavior. | Codebase |
| P17 | Config/env mismatch | Local/dev/prod differences cause hidden bugs. | Codebase + Memory |
| P18 | Migration risk | Schema/data migrations can cause irreversible damage if unsafe. | Codebase + External |
| P19 | Performance-regression risk | Code changes may degrade latency/cost without obvious signs. | Codebase + Vision |
| P20 | Security coverage gaps | Subtle auth/input/permission bugs remain underdetected. | Codebase + External |
| P21 | Reproducibility failure | Same prompt/code path yields non-repeatable outcomes. | Memory + Codebase |
| P22 | Spec-to-code drift | Implementation no longer matches design/intended contract. | Codebase |
| P23 | UI-state blindness | Text-only reasoning misses what users actually saw on screen. | Vision |
| P24 | Non-text signal blindness | Layout, color, interaction state are not captured by logs alone. | Vision |
| P25 | Observability gaps | Missing logs/metrics/traces block root-cause analysis. | Vision + Memory |
| P26 | Incident timeline reconstruction | Hard to rebuild exact sequence of events after failure. | Memory + Vision |
| P27 | Artifact portability friction | Knowledge/state is tied to one runtime/client machine. | All sisters |
| P28 | Cloud-local divide | Cloud agents cannot directly read local artifacts without sync/auth. | All sisters |
| P29 | Auth integration friction | Secure remote execution is inconsistent across clients. | All sisters |
| P30 | Latency/cost optimization uncertainty | Hard to pick best runtime policy for quality vs speed vs cost. | Memory + Codebase |
| P31 | Long-session reliability decay | Performance and quality degrade over long autonomous runs. | Memory + Vision |
| P32 | Long-horizon storage governance | "Capture everything" can explode storage/cost if unmanaged. | Memory |
| P33 | Privacy/redaction control | Sensitive user/org data needs policy-aware capture. | Memory + Vision |
| P34 | Feedback incorporation lag | User corrections are not consistently merged into future behavior. | Memory |
| P35 | Multi-agent coordination drift | Multiple agents diverge on facts/tasks/contracts. | Memory + Codebase |
| P36 | Evaluation drift | Benchmarks stop reflecting real production workloads. | All sisters |
| P37 | Requirement ambiguity | Stakeholders ask for outcomes without testable acceptance criteria. | External |
| P38 | Tacit business-rule knowledge | Critical rules live in people, not docs/code. | External + Memory |
| P39 | Priority conflict | Teams disagree on quality/speed/cost tradeoffs. | External |
| P40 | Compliance interpretation uncertainty | Legal/policy language is hard to map safely to code behavior. | External + Codebase |
| P41 | Third-party API volatility | Vendor behavior changes without warning. | External + Memory |
| P42 | Handoff quality gaps | Context loss across teams/time zones causes repeated work. | Memory |
| P43 | Incentive misalignment | Metrics reward speed while quality risk accumulates. | External |
| P44 | Explainability for non-technical stakeholders | Teams cannot explain risk/decision rationale clearly. | Codebase + Memory |

## 2. AgenticCodebase: primary ownership

### 2.1 Problems Codebase should solve directly
- P09, P10, P11, P12, P13, P14, P15, P16, P22
- Contributing: P03, P04, P17, P18, P19, P20, P21, P30, P35, P40, P44

### 2.2 Why Codebase is the right owner
- These are structure/risk/impact problems over source code and dependency graphs.
- They require deterministic graph compilation + query semantics, not just long-form text memory.

### 2.3 Planned capability tracks (Codebase)
- Track C1: richer impact/gate models (risk by module boundaries, test ownership, blast radius score)
- Track C2: multi-language cross-boundary edge fidelity
- Track C3: drift detectors (spec vs code + config overlays)
- Track C4: security-pattern graph checks (permission flow, taint-like path hints)
- Track C5: CI fitness gates as first-class policy objects

## 3. What Codebase cannot solve alone

- P01/P05/P06/P34/P42: requires longitudinal cognitive state (Memory).
- P23/P24/P25/P26: requires visual/runtime perception and evidence (Vision).
- P37/P39/P43: organization/leadership process problems outside model runtime.

## 4. Integration contracts needed from other sisters

- From Memory:
  - stable session/decision lineage for mapping code changes to historical rationale
  - feedback/correction stream to tune gate thresholds over time
- From Vision:
  - runtime UI incident evidence linked to code units for postmortem accuracy
  - visual regression events as input to code risk prioritization

## 5. Acceptance signal for this planning document

This document is complete when every Codebase roadmap item is traceable to one or more catalog IDs above.
