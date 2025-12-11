#!/usr/bin/env bash
# Generate Result Blocks for Phase 11
# Based on Universal Task Execution Policy §8D

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${NOA_ROOT}"

echo "Generating Result Blocks for Phase 11..."

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust."
    exit 1
fi

# Build the result block generator
echo "Building result block generator..."
cd sys/core
cargo build --release --bin result-block-generator 2>/dev/null || {
    echo "Note: result-block-generator binary not found. Using Python fallback..."
    cd "${NOA_ROOT}"
    python3 scripts/python/generate_result_blocks.py
    exit 0
}

# Run the generator
cd "${NOA_ROOT}"
./target/release/result-block-generator

echo "Result Blocks generated successfully!"
echo "Check test-results/result_blocks.json and test-results/PHASE11_RESULT_BLOCKS.md"

