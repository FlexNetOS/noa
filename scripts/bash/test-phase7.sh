#!/bin/bash
# Phase 7 Comprehensive Test Suite
# Tests all Phase 7 components and services

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UI_DIR="$SCRIPT_DIR/../../sys/ui"

SMOKE=false
UNIT=false
INTEGRATION=false
E2E=false
COVERAGE=false
ALL=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --smoke) SMOKE=true ;;
        --unit) UNIT=true ;;
        --integration) INTEGRATION=true ;;
        --e2e) E2E=true ;;
        --coverage) COVERAGE=true ;;
        --all) ALL=true ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
    shift
done

echo ""
echo "=== Phase 7 Test Suite ==="
echo "Testing Dynamic Context-Aware UI (US5)"
echo ""

TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

test_command() {
    local name=$1
    local command=$2
    local working_dir=${3:-$UI_DIR}

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -n "Testing: $name... "

    (
        cd "$working_dir"
        if eval "$command" > /dev/null 2>&1; then
            echo "PASS"
            PASSED_TESTS=$((PASSED_TESTS + 1))
            return 0
        else
            echo "FAIL"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            return 1
        fi
    )
}

# Type Checking
echo ""
echo "=== Type Checking ==="
test_command "TypeScript Type Check" "npm run type-check"

# Linting
echo ""
echo "=== Linting ==="
test_command "ESLint" "npm run lint"

# Build
echo ""
echo "=== Build Verification ==="
test_command "Production Build" "npm run build"

# Unit Tests
if [ "$UNIT" = true ] || [ "$ALL" = true ]; then
    echo ""
    echo "=== Unit Tests ==="
    test_command "Jest Unit Tests" "npm test -- --testPathPattern=__tests__/unit"
fi

# Integration Tests
if [ "$INTEGRATION" = true ] || [ "$ALL" = true ]; then
    echo ""
    echo "=== Integration Tests ==="
    test_command "Jest Integration Tests" "npm test -- --testPathPattern=__tests__/integration"
fi

# Smoke Tests
if [ "$SMOKE" = true ] || [ "$ALL" = true ]; then
    echo ""
    echo "=== Smoke Tests ==="
    test_command "Jest Smoke Tests" "npm test -- --testPathPattern=smoke"
fi

# Coverage
if [ "$COVERAGE" = true ] || [ "$ALL" = true ]; then
    echo ""
    echo "=== Coverage Report ==="
    test_command "Coverage Report" "npm run test:coverage"
fi

# E2E Tests
if [ "$E2E" = true ] || [ "$ALL" = true ]; then
    echo ""
    echo "=== E2E Tests ==="
    test_command "E2E Tests" "npm test -- --testPathPattern=__tests__/e2e"
fi

# Summary
echo ""
echo "=== Test Summary ==="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"

if [ $FAILED_TESTS -gt 0 ]; then
    echo ""
    echo "Some tests failed. See output above for details."
    exit 1
else
    echo ""
    echo "All tests passed! ✅"
    exit 0
fi


