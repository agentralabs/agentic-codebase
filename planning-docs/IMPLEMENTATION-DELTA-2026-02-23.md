# Implementation Delta (Codebase) - 2026-02-23

## Purpose

This file records implemented work that is active but not explicitly represented in the current planning-doc baseline.

Baseline planning docs reviewed:
- `planning-docs/CANONICAL_SISTER_KIT.md`
- `planning-docs/SDK_READINESS.md`

## Implemented Post-Plan Items

### 1. MCP launcher/workspace hardening

Implemented:
- Dynamic workspace/repo-root detection in launcher flow.
- Stale-lock protection and lock timeout handling for concurrent graph builds.
- Improved fallback behavior so empty graph state during startup races is avoided.

Evidence:
- `scripts/install.sh:323`
- `scripts/install.sh:408`
- `scripts/install.sh:458`
- `scripts/install.sh:567`

Related release notes:
- `CHANGELOG.md:9-13` (v0.1.4)

### 2. Installer reliability and cross-client MCP setup

Implemented:
- Install profiles for desktop/terminal/server.
- `jq` merge with `python3` fallback.
- Codex MCP registration path and generic MCP guidance.
- Completion output with restart guidance and local `.acb` auto-detect note.

Evidence:
- `scripts/install.sh:11`
- `scripts/install.sh:191`
- `scripts/install.sh:651`
- `scripts/install.sh:771`
- `scripts/install.sh:853`
- `scripts/install.sh:921`

### 3. MCP protocol correctness fixes

Implemented:
- `list_units` filter validation hardening.
- `impact_analysis` dependency coverage improvements.
- Stdio framing compatibility fixes.

Evidence:
- `CHANGELOG.md:18-19`
- `CHANGELOG.md:25`

### 4. Social workflow operational changes

Implemented:
- Secret-check logic fixed to avoid invalid workflow evaluation.
- Manual `workflow_dispatch` support added.
- X bridge integration currently removed.

Evidence:
- `.github/workflows/social-release-broadcast.yml`

## Open Consistency Note

Compared with memory/vision release workflows, codebase publish fallback currently matches only `already uploaded` wording.

Evidence:
- `.github/workflows/release.yml:102`

Cross-sister reference:
- `agentic-memory/planning-docs/CONSISTENCY-VALIDATION-ACROSS-SISTERS-2026-02-23.md`
