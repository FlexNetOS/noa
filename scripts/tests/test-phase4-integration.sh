#!/bin/bash
# test-phase4-integration.sh - Integration tests for Phase 4 (System Core)
# Usage: test-phase4-integration.sh [--verbose]
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

    rm -f /tmp/test-output.txt
}

echo "=== Phase 4 Integration Tests ==="
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

# Test 1: Identity System
echo "=== Test Group: Identity System ==="

run_test "Identity configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/identity/identity.json' 'true' 2>&1"

run_test "System principal exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/identity/identity.json' '\"system\" in data.principals' 2>&1"

run_test "Agent principal exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/identity/identity.json' '\"agent\" in data.principals' 2>&1"

run_test "Agent has 7 capabilities" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/identity/identity.json' 'data.principals.agent.capabilities.length === 7' 2>&1"

run_test "All 7 capabilities defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/identity/identity.json' 'Object.keys(data.capabilities).length >= 7' 2>&1"

echo ""

# Test 2: Policy Enforcement
echo "=== Test Group: Policy Enforcement ==="

run_test "Policy configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/enforcement/policy.json' 'true' 2>&1"

run_test "6 policy categories exist" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/enforcement/policy.json' 'Object.keys(data.policies).length === 6' 2>&1"

run_test "Capability enforcement enabled" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/enforcement/policy.json' 'data.policies.capability_enforcement.enabled === true' 2>&1"

run_test "Budget enforcement enabled" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/enforcement/policy.json' 'data.policies.budget_enforcement.enabled === true' 2>&1"

echo ""

# Test 3: Audit System
echo "=== Test Group: Audit System ==="

run_test "Audit configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/audit/audit-configs.json' 'true' 2>&1"

run_test "Audit enabled" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/audit/audit-configs.json' 'data.audit_enabled === true' 2>&1"

run_test "8 event categories defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/audit/audit-configs.json' 'Object.keys(data.audit_events).length === 8' 2>&1"

echo ""

# Test 4: Registry System
echo "=== Test Group: Registry System ==="

run_test "Registry configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/registry/registry.json' 'true' 2>&1"

run_test "5 services registered" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/registry/registry.json' 'Object.keys(data.services).length === 5' 2>&1"

run_test "Identity service registered" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/registry/registry.json' '\"identity\" in data.services' 2>&1"

echo ""

# Test 5: Scheduler
echo "=== Test Group: Scheduler ==="

run_test "Scheduler configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/scheduler/configs.json' 'true' 2>&1"

run_test "5 scheduled tasks defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/scheduler/configs.json' 'Object.keys(data.scheduled_tasks).length === 5' 2>&1"

run_test "CAS GC task enabled" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/sys/core/scheduler/configs.json' 'data.scheduled_tasks.cas_gc.enabled === true' 2>&1"

echo ""

# Summary
echo "=== Test Summary ==="
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo "✓ All Phase 4 integration tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
