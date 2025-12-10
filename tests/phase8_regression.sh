#!/bin/bash
# Phase 8: Regression Test Suite
# REG001-REG017: Critical path, provider integration, and data integrity tests

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"
CORE_DIR="$NOA_ROOT/sys/core"

echo "=========================================="
echo "Phase 8: Regression Test Suite"
echo "=========================================="
echo ""

cd "$CORE_DIR"

echo "Running regression tests..."
echo ""

# Run all regression tests
cargo test --lib regression::tests::regression_tests -- --nocapture

echo ""
echo "=========================================="
echo "Regression tests completed"
echo "=========================================="

