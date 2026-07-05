#!/usr/bin/env bash
# Regenerate README.md from lib.rs documentation
# Run this after updating examples in src/lib.rs

set -e

echo "Generating README from src/lib.rs documentation..."
cargo readme --template README_TEMPLATE.md >README.md

echo "✓ README.md updated successfully"
echo ""
echo "The README now matches the crate-level documentation."
echo "Both will render identically on crates.io and GitHub."
