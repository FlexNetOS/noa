#!/bin/bash
#
# Run smoke tests for installed toolchains.
#
# Compiles and runs minimal programs to verify toolchain functionality.
#
# Usage:
#   ./smoke-test.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"
TMP_DIR="$NOA_ROOT/tmp/smoke-test"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m'

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}         NOA Smoke Tests${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Ensure tmp directory exists
mkdir -p "$TMP_DIR"

PASSED=0
FAILED=0

test_toolchain() {
    local name="$1"
    local test_file="$2"
    local content="$3"
    local compile_cmd="${4:-}"
    local run_cmd="$5"

    local test_path="$TMP_DIR/$test_file"

    echo -e "${YELLOW}Testing $name...${NC}"

    # Write test file
    echo "$content" > "$test_path"

    # Compile if needed
    if [[ -n "$compile_cmd" ]]; then
        echo -e "  ${GRAY}Compiling...${NC}"
        if ! eval "$compile_cmd" 2>&1; then
            echo -e "  ${RED}[FAIL]${NC} $name - Compilation failed"
            ((FAILED++))
            rm -f "$test_path"
            return
        fi
    fi

    # Run
    echo -e "  ${GRAY}Running...${NC}"
    local output
    output=$(eval "$run_cmd" 2>&1) || true

    if [[ "$output" == *"Hello from NOA"* ]]; then
        echo -e "  ${GREEN}[PASS]${NC} $name works correctly"
        ((PASSED++))
    else
        echo -e "  ${RED}[FAIL]${NC} $name - Unexpected output: $output"
        ((FAILED++))
    fi

    # Cleanup
    rm -rf "$TMP_DIR"/*
}

# Test Python
if command -v python3 &> /dev/null || command -v python &> /dev/null; then
    PYTHON_CMD=$(command -v python3 || command -v python)
    test_toolchain "Python" \
        "test.py" \
        'print("Hello from NOA - Python")' \
        "" \
        "$PYTHON_CMD $TMP_DIR/test.py"
fi

# Test Node.js
if command -v node &> /dev/null; then
    test_toolchain "Node.js" \
        "test.js" \
        'console.log("Hello from NOA - Node.js")' \
        "" \
        "node $TMP_DIR/test.js"
fi

# Test Rust
if command -v rustc &> /dev/null; then
    test_toolchain "Rust" \
        "test.rs" \
        'fn main() { println!("Hello from NOA - Rust"); }' \
        "rustc -o $TMP_DIR/test $TMP_DIR/test.rs" \
        "$TMP_DIR/test"
fi

# Test Go
if command -v go &> /dev/null; then
    test_toolchain "Go" \
        "test.go" \
        'package main; import "fmt"; func main() { fmt.Println("Hello from NOA - Go") }' \
        "" \
        "go run $TMP_DIR/test.go"
fi

# Summary
echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Smoke Test Summary${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "  ${GREEN}Passed: $PASSED${NC}"
if [[ $FAILED -gt 0 ]]; then
    echo -e "  ${RED}Failed: $FAILED${NC}"
else
    echo -e "  ${GRAY}Failed: $FAILED${NC}"
fi

# Cleanup
rm -rf "$TMP_DIR"

exit $FAILED

