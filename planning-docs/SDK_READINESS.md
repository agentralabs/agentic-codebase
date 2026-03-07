# SDK Readiness - AgenticCodebase

Internal-only tracking board for distribution-channel maturity.

## Scope

This board tracks whether AgenticCodebase package channels are mature enough for long-term public SDK promises.

## Channel State

| Channel | Package(s) | State | Notes |
|---|---|---|---|
| crates.io | `agentic-codebase` | Ready | Single crate publishes both `acb` and `acb-mcp`. |
| GitHub installer | `scripts/install.sh` | Ready | One-line install and MCP config merge active. |
| PyPI SDK | none (canonical) | Not started | No official Python package contract yet. |
| npm SDK | none (canonical) | Not started | No official npm package contract yet. |

## SDK Gate Checklist

| Gate | Requirement | Status | Evidence / Follow-up |
|---|---|---|---|
| G1 | Public SDK API contract documented | Needs Work | Define Python/npm scope if those channels are planned. |
| G2 | SemVer + compatibility policy clear per channel | Needs Work | Rust semver exists; add channel policy for future SDKs. |
| G3 | Release automation present per channel | Partial | Rust release automation exists; add PyPI/npm automation when channels are approved. |
| G4 | Channel docs and examples complete | Partial | Rust/MCP docs are complete; no canonical docs for Python/npm SDKs yet. |
| G5 | Cross-language parity defined | Needs Work | Decide wrapper-only vs full SDK parity targets. |
| G6 | Support boundary stated | Partial | Public docs should explicitly mark non-Rust channels as not yet official. |

## Decision

AgenticCodebase is maintained-SDK ready for Rust+MCP via bundled crate model. PyPI/npm should be launched only after G1-G6 pass.

## Exit Criteria (for official PyPI/npm)

1. Canonical package names finalized.
2. API surface documented and semver-guarded.
3. CI publish pipeline with smoke tests added.
4. README/INSTALL updated with official support statement.

## Review Cadence

- Update on every tagged release.
- Re-check this board before enabling any new public channel.

Last updated: 2026-02-21
