#!/bin/bash
#
# Test suite for bootstrap library functions
#
# Tests all library functions for correctness and cross-platform compatibility.
# Per NOA Constitution §3.1: Quality assurance

set -euo pipefail

# Test utilities
TEST_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIB_ROOT="$(cd "$TEST_ROOT/../lib" && pwd)"

# Source library functions
source "$LIB_ROOT/logging.sh"
source "$LIB_ROOT/platform.sh"
source "$LIB_ROOT/directories.sh"
source "$LIB_ROOT/state.sh"
source "$LIB_ROOT/verification.sh"
source "$LIB_ROOT/download.sh"

TESTS_PASSED=0
TESTS_FAILED=0
declare -a TEST_RESULTS

test_result() {
    local name="$1"
    local passed="$2"
    local message="${3:-}"

    if [[ "$passed" == "true" ]]; then
        echo -e "\033[0;32m[PASS]\033[0m $name"
        ((TESTS_PASSED++))
    else
        echo -e "\033[0;31m[FAIL]\033[0m $name"
        if [[ -n "$message" ]]; then
            echo -e "  \033[0;33m$message\033[0m"
        fi
        ((TESTS_FAILED++))
    fi

    TEST_RESULTS+=("$name|$passed|$message")
}

echo -e "\033[0;36m═══════════════════════════════════════════════════════════════\033[0m"
echo -e "\033[0;36m         Bootstrap Library Test Suite\033[0m"
echo -e "\033[0;36m═══════════════════════════════════════════════════════════════\033[0m"
echo ""

# Test 1: Platform Detection
echo -e "\033[0;33mTesting Platform Detection...\033[0m"
if platform_info=$(get_platform_info 2>/dev/null); then
    os=$(echo "$platform_info" | jq -r '.os' 2>/dev/null || echo "unknown")
    test_result "Platform Detection" \
        "$([ "$os" != "unknown" ] && echo "true" || echo "false")" \
        "OS: $os"
else
    test_result "Platform Detection" "false" "Failed to get platform info"
fi

# Test 2: Directory Structure Creation
echo -e "\033[0;33mTesting Directory Structure Creation...\033[0m"
TEST_NOA_ROOT="/tmp/noa-test-$$"
if create_noa_directory_structure "$TEST_NOA_ROOT" --quiet >/dev/null 2>&1; then
    created=$(create_noa_directory_structure "$TEST_NOA_ROOT" --quiet | jq -r '.created' 2>/dev/null || echo "0")
    test_result "Directory Creation" \
        "$([ "$created" -gt 0 ] && echo "true" || echo "false")" \
        "Created directories"
    rm -rf "$TEST_NOA_ROOT"
else
    test_result "Directory Creation" "false" "Failed to create directories"
    rm -rf "$TEST_NOA_ROOT" 2>/dev/null || true
fi

# Test 3: State Management
echo -e "\033[0;33mTesting State Management...\033[0m"
TEST_NOA_ROOT="/tmp/noa-test-state-$$"
mkdir -p "$TEST_NOA_ROOT/config"
if initialize_bootstrap_state "$TEST_NOA_ROOT" >/dev/null 2>&1; then
    set_tool_state "test-tool" "1.0.0" "$TEST_NOA_ROOT/bin/test" "installed" >/dev/null 2>&1
    state=$(get_tool_state "test-tool" 2>/dev/null)
    version=$(echo "$state" | jq -r '.version' 2>/dev/null || echo "")
    test_result "State Management" \
        "$([ "$version" == "1.0.0" ] && echo "true" || echo "false")" \
        "Tool state saved and retrieved"
    rm -rf "$TEST_NOA_ROOT"
else
    test_result "State Management" "false" "Failed to initialize state"
    rm -rf "$TEST_NOA_ROOT" 2>/dev/null || true
fi

# Test 4: Tool Verification
echo -e "\033[0;33mTesting Tool Verification...\033[0m"
if result=$(test_tool_verification "nonexistent-tool" "/tmp/test" "1.0.0" "" 2>/dev/null); then
    action=$(echo "$result" | jq -r '.action' 2>/dev/null || echo "")
    test_result "Tool Verification" \
        "$([ "$action" == "INSTALL" ] && echo "true" || echo "false")" \
        "Correctly identified missing tool"
else
    test_result "Tool Verification" "false" "Verification function failed"
fi

# Test 5: Download Function (checksum support)
echo -e "\033[0;33mTesting Download Function...\033[0m"
if grep -q "checksum" "$LIB_ROOT/download.sh" 2>/dev/null; then
    test_result "Download Checksum Support" "true" "Checksum parameter exists"
else
    test_result "Download Checksum Support" "false" "Checksum parameter not found"
fi

# Summary
echo ""
echo -e "\033[0;36m═══════════════════════════════════════════════════════════════\033[0m"
echo -e "\033[0;36m  Test Summary\033[0m"
echo -e "\033[0;36m═══════════════════════════════════════════════════════════════\033[0m"
echo -e "  \033[0;32mPassed: $TESTS_PASSED\033[0m"
if [[ $TESTS_FAILED -gt 0 ]]; then
    echo -e "  \033[0;31mFailed: $TESTS_FAILED\033[0m"
else
    echo -e "  \033[0;90mFailed: $TESTS_FAILED\033[0m"
fi
echo -e "  \033[0;36mTotal:  $((TESTS_PASSED + TESTS_FAILED))\033[0m"
echo ""

if [[ $TESTS_FAILED -gt 0 ]]; then
    exit 1
else
    exit 0
fi


