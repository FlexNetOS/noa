#!/bin/bash
# test-cas-phase3.sh - Test CAS implementation (Phase 3)
# Usage: test-cas-phase3.sh [--verbose]
# Respects NOA Constitution §3.1 - uses portable tools from noa_root

set -euo pipefail

# Source portable tool resolver
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -f "$TEST_SCRIPT_DIR/../lib/noa-tools.sh" ]]; then
    source "$TEST_SCRIPT_DIR/../lib/noa-tools.sh"
else
    echo "ERROR: noa-tools.sh not found. Run from NOA repository." >&2
    exit 1
fi

CAS_ROOT="${CAS_ROOT:-${NOA_ROOT}/cas}"
SCRIPT_DIR="${NOA_ROOT}/scripts/cas"
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

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Test function
test_case() {
    local name="$1"
    local command="$2"

    echo -n "Testing: $name... "

    if [[ "$VERBOSE" == "true" ]]; then
        echo ""
        echo "  Command: $command"
    fi

    if eval "$command" > /tmp/test-output.txt 2>&1; then
        echo "PASS"
        ((TESTS_PASSED++)) || true
        if [[ "$VERBOSE" == "true" ]]; then
            cat /tmp/test-output.txt | sed 's/^/  /'
        fi
    else
        echo "FAIL"
        ((TESTS_FAILED++)) || true
        cat /tmp/test-output.txt | sed 's/^/  ERROR: /'
    fi

    rm -f /tmp/test-output.txt
}

# Setup test environment
setup_test_env() {
    echo "=== Setting up test environment ==="

    # Create test directory
    TEST_DIR=$(mktemp -d)
    export TEST_DIR

    # Create test files
    echo "Hello, World!" > "$TEST_DIR/test1.txt"
    echo "Lorem ipsum dolor sit amet" > "$TEST_DIR/test2.txt"
    dd if=/dev/zero of="$TEST_DIR/large.bin" bs=1M count=2 2>/dev/null

    echo "Test directory: $TEST_DIR"
    echo ""
}

# Cleanup test environment
cleanup_test_env() {
    echo ""
    echo "=== Cleaning up test environment ==="
    if [[ -n "${TEST_DIR:-}" ]] && [[ -d "$TEST_DIR" ]]; then
        rm -rf "$TEST_DIR"
    fi
}

trap cleanup_test_env EXIT

# Main test suite
echo "=== NOA CAS Phase 3 Test Suite ==="
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

setup_test_env

# Test 1: Directory structure
echo "=== Test Group: Directory Structure ==="
test_case "CAS root exists" "[[ -d '$CAS_ROOT' ]]"
test_case "Objects directory exists" "[[ -d '$CAS_ROOT/objects' ]]"
test_case "Refs directory exists" "[[ -d '$CAS_ROOT/refs' ]]"
test_case "Tags directory exists" "[[ -d '$CAS_ROOT/tags' ]]"
test_case "Registry directory exists" "[[ -d '$CAS_ROOT/registry' ]]"
test_case "GC directory exists" "[[ -d '$CAS_ROOT/gc' ]]"
test_case "Merkle directory exists" "[[ -d '$CAS_ROOT/merkle' ]]"
echo ""

# Test 2: Configuration files
echo "=== Test Group: Configuration Files ==="
test_case "CAS config exists" "[[ -f '$NOA_ROOT/configs/base/cas/config.json' ]]"
test_case "GC rules exist" "[[ -f '$CAS_ROOT/gc/gc_rules.json' ]]"
test_case "Cache policies exist" "[[ -f '$NOA_ROOT/configs/base/cache/cache-policies.json' ]]"
echo ""

# Test 3: Registry files
echo "=== Test Group: Registry Files ==="
test_case "Models registry exists" "[[ -f '$CAS_ROOT/registry/models.json' ]]"
test_case "Prompts registry exists" "[[ -f '$CAS_ROOT/registry/prompts.json' ]]"
test_case "Snapshots registry exists" "[[ -f '$CAS_ROOT/registry/snapshots.json' ]]"
test_case "Binaries registry exists" "[[ -f '$CAS_ROOT/registry/binaries.json' ]]"
test_case "Packages registry exists" "[[ -f '$CAS_ROOT/registry/packages.json' ]]"
echo ""

# Test 4: Script utilities
echo "=== Test Group: Script Utilities ==="
test_case "store-object.sh exists" "[[ -f '$SCRIPT_DIR/store-object.sh' ]]"
test_case "retrieve-object.sh exists" "[[ -f '$SCRIPT_DIR/retrieve-object.sh' ]]"
test_case "update-ref.sh exists" "[[ -f '$SCRIPT_DIR/update-ref.sh' ]]"
test_case "create-tag.sh exists" "[[ -f '$SCRIPT_DIR/create-tag.sh' ]]"
test_case "gc-run.sh exists" "[[ -f '$SCRIPT_DIR/gc-run.sh' ]]"
test_case "registry-add.sh exists" "[[ -f '$SCRIPT_DIR/registry-add.sh' ]]"
echo ""

# Test 5: CAS operations (if scripts are executable)
if command -v bash >/dev/null 2>&1; then
    echo "=== Test Group: CAS Operations ==="

    # Test store operation (using sha256 fallback since blake3 may not be installed)
    test_case "Store object" "bash '$SCRIPT_DIR/store-object.sh' '$TEST_DIR/test1.txt' generic > /tmp/hash1.txt 2>&1"

    if [[ -f /tmp/hash1.txt ]]; then
        HASH1=$(cat /tmp/hash1.txt | tail -n1)
        echo "  Stored hash: $HASH1"

        # Test retrieve operation
        test_case "Retrieve object" "bash '$SCRIPT_DIR/retrieve-object.sh' '$HASH1' '$TEST_DIR/retrieved.txt' 2>&1"

        # Test content integrity
        test_case "Content integrity" "diff '$TEST_DIR/test1.txt' '$TEST_DIR/retrieved.txt' 2>&1"

        # Test ref operations
        test_case "Create ref" "bash '$SCRIPT_DIR/update-ref.sh' test/myref '$HASH1' 'Test ref' 2>&1"
        test_case "Ref file exists" "[[ -f '$CAS_ROOT/refs/test/myref' ]]"
        test_case "Ref points to hash" "grep -q '$HASH1' '$CAS_ROOT/refs/test/myref'"

        # Test tag operations
        test_case "Create tag" "bash '$SCRIPT_DIR/create-tag.sh' test-v1 '$HASH1' 'Test tag' 2>&1"
        test_case "Tag file exists" "[[ -f '$CAS_ROOT/tags/test-v1' ]]"
        test_case "Tag points to hash" "grep -q '$HASH1' '$CAS_ROOT/tags/test-v1'"

        # Test GC (dry run)
        test_case "GC dry run" "bash '$SCRIPT_DIR/gc-run.sh' --dry-run 2>&1"
    fi

    rm -f /tmp/hash1.txt
    echo ""
fi

# Test 6: Cache management
echo "=== Test Group: Cache Management ==="
test_case "cleanup-cache.sh exists" "[[ -f '$NOA_ROOT/scripts/cache/cleanup-cache.sh' ]]"
test_case "monitor-cache.sh exists" "[[ -f '$NOA_ROOT/scripts/cache/monitor-cache.sh' ]]"

if command -v bash >/dev/null 2>&1; then
    # Test cache monitoring
    test_case "Cache monitor runs" "bash '$NOA_ROOT/scripts/cache/monitor-cache.sh' 2>&1"

    # Test cache cleanup dry run
    test_case "Cache cleanup dry run" "bash '$NOA_ROOT/scripts/cache/cleanup-cache.sh' --dry-run 2>&1"
fi
echo ""

# Summary
echo "=== Test Summary ==="
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo "All tests PASSED!"
    exit 0
else
    echo "Some tests FAILED!"
    exit 1
fi
