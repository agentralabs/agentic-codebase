#!/bin/bash
# Pre-publish checks and dry-run for crates.io
set -euo pipefail

echo "Running pre-publish checks..."
echo ""

echo "1. Running tests..."
cargo test --workspace
echo ""

echo "2. Checking formatting..."
cargo fmt --all -- --check
echo ""

echo "3. Running clippy..."
cargo clippy --workspace -- -D warnings
echo ""

echo "4. Dry-run publish (single crate ships both acb and acb-mcp binaries)..."
cargo publish --dry-run
echo ""

echo "All checks passed!"
echo ""
echo "To publish:"
echo "  cargo publish"
echo ""
echo "Note: agentic-codebase publishes both binaries:"
echo "  - acb"
echo "  - acb-mcp"
