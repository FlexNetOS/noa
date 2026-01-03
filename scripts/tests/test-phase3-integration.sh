#!/bin/bash
# test-phase3-integration.sh - Integration tests for Phase 3 (CAS & Data Plane)
# Usage: test-phase3-integration.sh [--verbose]
# Respects NOA Constitution §3.1 - uses portable tools from noa_root

set -euo pipefail

# Source portable tool resolver
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -f "$SCRIPT_DIR/../lib/noa-tools.sh" ]]; then
    source "$SCRIPT_DIR/../lib/noa-tools.sh"
else
    echo "ERROR: noa-tools.sh not found. Run from NOA repository." >&2
    exit 1
fi

CAS_SCRIPTS="${NOA_ROOT}/scripts/cas"
CACHE_SCRIPTS="${NOA_ROOT}/scripts/cache"
VERBOSE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Test helper
run_test() {
    local name="$1"
    local command="$2"

    echo -n "Test: $name ... "

    if [[ "$VERBOSE" == "true" ]]; then
        echo ""
        echo "  Command: $command"
    fi

    # Handle function calls vs eval commands
    local cmd_type
    cmd_type=$(type -t "$command" 2>/dev/null || echo "")
    
    if [[ "$cmd_type" == "function" ]]; then
        # It's a function, call it directly
        if "$command" > /tmp/test-output.txt 2>&1; then
            echo "✓ PASS"
            ((TESTS_PASSED++)) || true
            if [[ "$VERBOSE" == "true" ]]; then
                cat /tmp/test-output.txt | sed 's/^/    /'
            fi
        else
            echo "✗ FAIL"
            ((TESTS_FAILED++)) || true
            echo "  Error output:"
            cat /tmp/test-output.txt | sed 's/^/    /'
        fi
    else
        # Regular command, use eval
        if eval "$command" > /tmp/test-output.txt 2>&1; then
            echo "✓ PASS"
            ((TESTS_PASSED++)) || true
            if [[ "$VERBOSE" == "true" ]]; then
                cat /tmp/test-output.txt | sed 's/^/    /'
            fi
        else
            echo "✗ FAIL"
            ((TESTS_FAILED++)) || true
            echo "  Error output:"
            cat /tmp/test-output.txt | sed 's/^/    /'
        fi
    fi

    rm -f /tmp/test-output.txt
}

# Setup
echo "=== Phase 3 Integration Tests ==="
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

# Create test environment
TEST_DIR=$(mktemp -d)
echo "Test directory: $TEST_DIR"
echo ""

# Test files
echo "Test content" > "$TEST_DIR/test1.txt"
echo "Another test" > "$TEST_DIR/test2.txt"

# Test 1: CAS Operations
echo "=== Test Group: CAS Operations ==="

run_test "Store object in CAS" \
  "bash '$CAS_SCRIPTS/store-object.sh' '$TEST_DIR/test1.txt' generic 2>&1 | grep -q '[a-f0-9]\\{64\\}'"

# Capture hash from previous test
if [[ -f /tmp/test-output.txt ]]; then
    HASH=$(cat /tmp/test-output.txt | grep -oE '[a-f0-9]{64}' | head -1)
else
    # Re-run to get hash
    HASH=$(bash "$CAS_SCRIPTS/store-object.sh" "$TEST_DIR/test1.txt" generic 2>&1 | grep -oE '[a-f0-9]{64}' | tail -1)
fi

if [[ -n "$HASH" ]]; then
    run_test "Retrieve object from CAS" \
      "bash '$CAS_SCRIPTS/retrieve-object.sh' '$HASH' '$TEST_DIR/retrieved.txt' 2>&1"

    run_test "Verify object integrity" \
      "diff '$TEST_DIR/test1.txt' '$TEST_DIR/retrieved.txt'"

    run_test "Create CAS tag" \
      "bash '$CAS_SCRIPTS/create-tag.sh' 'test-tag-001' '$HASH' 'Test tag' 2>&1"

    run_test "Update CAS ref" \
      "bash '$CAS_SCRIPTS/update-ref.sh' 'test/ref' '$HASH' 'Test ref' 2>&1"

    run_test "Verify tag file exists" \
      "[[ -f '$NOA_ROOT/cas/tags/test-tag-001' ]]"

    run_test "Verify ref file exists" \
      "[[ -f '$NOA_ROOT/cas/refs/test/ref' ]]"

    run_test "Verify ref points to correct hash" \
      "grep -q '$HASH' '$NOA_ROOT/cas/refs/test/ref'"
fi

echo ""

# GC dry run test - run directly without eval 
echo -n "Test: GC dry run ... "
# Debug: capture output
GC_OUTPUT=$(bash "$CAS_SCRIPTS/gc-run.sh" --dry-run 2>&1)
if echo "$GC_OUTPUT" | grep -q 'Total objects'; then
    echo "✓ PASS"
    ((TESTS_PASSED++)) || true
else
    echo "✗ FAIL"
    ((TESTS_FAILED++)) || true
    echo "  Error output:"
    echo "$GC_OUTPUT" | sed 's/^/    /'
fi

echo ""

# Test 2: Cache Operations
echo "=== Test Group: Cache Operations ==="

# Cache monitor test - run directly without eval
echo -n "Test: Cache monitor runs ... "
if bash "$CACHE_SCRIPTS/monitor-cache.sh" 2>&1 | grep -q 'TOTAL'; then
    echo "✓ PASS"
    ((TESTS_PASSED++)) || true
else
    echo "✗ FAIL"
    ((TESTS_FAILED++)) || true
    echo "  Error output:"
    bash "$CACHE_SCRIPTS/monitor-cache.sh" 2>&1 | sed 's/^/    /'
fi

run_test "Cache cleanup dry run" \
  "bash '$CACHE_SCRIPTS/cleanup-cache.sh' --dry-run 2>&1"

echo ""

# Test 3: Registry Validation
echo "=== Test Group: Registry Validation ==="

run_test "Models registry is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/cas/registry/models.json' 'true' 2>&1"

run_test "Prompts registry is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/cas/registry/prompts.json' 'true' 2>&1"

echo ""

# Cleanup
rm -rf "$TEST_DIR"
if [[ -n "${HASH:-}" ]]; then
    rm -f "$NOA_ROOT/cas/tags/test-tag-001" 2>/dev/null || true
    rm -f "$NOA_ROOT/cas/refs/test/ref" 2>/dev/null || true
fi

# Summary
echo "=== Test Summary ==="
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo "✓ All Phase 3 integration tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
