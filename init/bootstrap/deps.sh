#!/bin/bash
# Bootstrap Dependencies Installation
#
# T089: Create bootstrap deps.sh script
# US1: Initialize NOA Seed Environment

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"

echo "Installing NOA dependencies..."

# Check for required tools
command -v cargo >/dev/null 2>&1 || { echo "Error: cargo not found"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "Error: npm not found"; exit 1; }
command -v pip >/dev/null 2>&1 || { echo "Error: pip not found"; exit 1; }

# Install Rust dependencies
if [ -f "$NOA_ROOT/sys/core/Cargo.toml" ]; then
    echo "Installing Rust dependencies..."
    cd "$NOA_ROOT/sys/core"
    cargo fetch
fi

# Install Node.js dependencies
if [ -f "$NOA_ROOT/sys/ui/package.json" ]; then
    echo "Installing Node.js dependencies..."
    cd "$NOA_ROOT/sys/ui"
    npm install
fi

# Install Python dependencies
if [ -f "$NOA_ROOT/sys/digest/pyproject.toml" ]; then
    echo "Installing Python dependencies..."
    cd "$NOA_ROOT/sys/digest"
    pip install -e .
fi

echo "✓ Dependencies installed"

