#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

find_fixed() {
  local pattern="$1"
  shift
  if command -v rg >/dev/null 2>&1; then
    rg -nF "$pattern" "$@"
  else
    grep -R -n -F -- "$pattern" "$@"
  fi
}

assert_contains() {
  local pattern="$1"
  shift
  if ! find_fixed "$pattern" "$@" >/dev/null; then
    fail "Missing required install command: ${pattern}"
  fi
}

http_ok() {
  local url="$1"
  curl -fsSL --retry 3 --retry-delay 1 --retry-connrefused \
    -A "agentra-install-guardrails/1.0 (+https://agentralabs.tech)" \
    "$url" >/dev/null
}

# Front-facing command requirements
assert_contains "curl -fsSL https://agentralabs.tech/install/codebase | bash" README.md docs/quickstart.md
assert_contains "cargo install agentic-codebase" README.md docs/quickstart.md

# Installer health
bash -n scripts/install.sh
bash scripts/install.sh --dry-run >/dev/null

# Public package/repo health (stable URLs for CI)
http_ok https://raw.githubusercontent.com/agentralabs/codebase/main/scripts/install.sh
http_ok https://crates.io/api/v1/crates/agentic-codebase

echo "Install command guardrails passed (codebase)."
