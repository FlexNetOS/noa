#!/bin/bash
# run-all-tests.sh - Master test runner for all NOA validation and integration tests
# Usage: run-all-tests.sh [--verbose] [--phase N] [--stop-on-fail]
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

TEST_SCRIPTS="$NOA_ROOT/scripts/tests"
VERBOSE=false
STOP_ON_FAIL=false
RUN_PHASE=""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --stop-on-fail)
            STOP_ON_FAIL=true
            shift
            ;;
        --phase)
            RUN_PHASE="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

# Test suite counters
SUITES_PASSED=0
SUITES_FAILED=0
TOTAL_TESTS_PASSED=0
TOTAL_TESTS_FAILED=0

# Function to run a test suite
run_suite() {
    local name="$1"
    local script="$2"
    local phase="$3"

    # Skip if specific phase requested and this isn't it
    if [[ -n "$RUN_PHASE" ]] && [[ "$phase" != "$RUN_PHASE" ]]; then
        return 0
    fi

    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Running: $name${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    local args=""
    if [[ "$VERBOSE" == "true" ]]; then
        args="--verbose"
    fi

    if bash "$script" $args; then
        echo ""
        echo -e "${GREEN}✓ $name PASSED${NC}"
        ((SUITES_PASSED++)) || true

        # Extract test counts from output if available
        if [[ -f /tmp/test-suite-output.txt ]]; then
            local passed=$(grep "Tests passed:" /tmp/test-suite-output.txt | grep -oE '[0-9]+' || echo "0")
            local failed=$(grep "Tests failed:" /tmp/test-suite-output.txt | grep -oE '[0-9]+' || echo "0")
            TOTAL_TESTS_PASSED=$((TOTAL_TESTS_PASSED + passed))
            TOTAL_TESTS_FAILED=$((TOTAL_TESTS_FAILED + failed))
            rm -f /tmp/test-suite-output.txt
        fi
    else
        echo ""
        echo -e "${RED}✗ $name FAILED${NC}"
        ((SUITES_FAILED++)) || true

        if [[ "$STOP_ON_FAIL" == "true" ]]; then
            echo ""
            echo -e "${RED}Stopping on first failure (--stop-on-fail)${NC}"
            exit 1
        fi
    fi

    echo ""
}

# Print header
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                   NOA Test Suite Runner                        ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo "NOA Root: $NOA_ROOT"
echo "Verbose: $VERBOSE"
echo "Stop on fail: $STOP_ON_FAIL"
if [[ -n "$RUN_PHASE" ]]; then
    echo "Running phase: $RUN_PHASE"
fi
echo ""

# Phase 0: configsuration Validation
run_suite "configsuration Validation" \
  "$TEST_SCRIPTS/validate-configss.sh" \
  "0"

# Phase 3: CAS & Data Plane
run_suite "Phase 3 Integration Tests (CAS & Data Plane)" \
  "$TEST_SCRIPTS/test-phase3-integration.sh" \
  "3"

# Phase 4: System Core
run_suite "Phase 4 Integration Tests (System Core)" \
  "$TEST_SCRIPTS/test-phase4-integration.sh" \
  "4"

# Phase 5: Resource Registry
run_suite "Phase 5 Integration Tests (Resource Registry)" \
  "$TEST_SCRIPTS/test-phase5-integration.sh" \
  "5"

# Phase 6: Third-Party Integrations
run_suite "Phase 6 Integration Tests (Third-Party Integrations)" \
  "$TEST_SCRIPTS/test-phase6-integration.sh" \
  "6"

# Print summary
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                      Test Suite Summary                        ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "Test Suites:"
echo "  Passed: $SUITES_PASSED"
echo "  Failed: $SUITES_FAILED"
echo "  Total:  $((SUITES_PASSED + SUITES_FAILED))"
echo ""

if [[ $TOTAL_TESTS_PASSED -gt 0 ]] || [[ $TOTAL_TESTS_FAILED -gt 0 ]]; then
    echo "Individual Tests:"
    echo "  Passed: $TOTAL_TESTS_PASSED"
    echo "  Failed: $TOTAL_TESTS_FAILED"
    echo "  Total:  $((TOTAL_TESTS_PASSED + TOTAL_TESTS_FAILED))"
    echo ""
fi

# Final result
if [[ $SUITES_FAILED -eq 0 ]]; then
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                    ✓ ALL TESTS PASSED                         ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${RED}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║                    ✗ SOME TESTS FAILED                        ║${NC}"
    echo -e "${RED}╚════════════════════════════════════════════════════════════════╝${NC}"
    exit 1
fi
