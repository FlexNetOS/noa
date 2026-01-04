#!/bin/bash
#
# Verify entire NOA environment installation.
#
# Runs all verification checks to ensure NOA is properly installed and configsured.
# Checks toolchains, AI providers, shared resources, and environment setup.
#
# Usage:
#   ./verify-all.sh
#   ./verify-all.sh --json

set -euo pipefail

# Parse arguments
JSON_OUTPUT=false
for arg in "$@"; do
    case "$arg" in
        --json) JSON_OUTPUT=true ;;
    esac
done

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"
BOOTSTRAP_DIR="$NOA_ROOT/scripts/bootstrap"

# Colors
if [[ -t 1 ]] && ! $JSON_OUTPUT; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    CYAN='\033[0;36m'
    GRAY='\033[0;90m'
    NC='\033[0m'
else
    RED='' GREEN='' YELLOW='' CYAN='' GRAY='' NC=''
fi

# Counters
TOTAL=0
PASSED=0
FAILED=0

run_check() {
    local name="$1"
    local description="$2"
    local test_cmd="$3"

    ((TOTAL++))

    if eval "$test_cmd" &> /dev/null; then
        ((PASSED++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${GREEN}[PASS]${NC} $description"
        fi
    else
        ((FAILED++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${RED}[FAIL]${NC} $description"
        fi
    fi
}

if ! $JSON_OUTPUT; then
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}         NOA Environment Verification${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GRAY}NOA Root: $NOA_ROOT${NC}"
    echo ""
fi

# 1. Directory Structure
if ! $JSON_OUTPUT; then echo -e "${YELLOW}Checking directory structure...${NC}"; fi

REQUIRED_DIRS=("bin" "configs" "ai" "ai/shared" "ai/providers" "logs" "specs")
for dir in "${REQUIRED_DIRS[@]}"; do
    run_check "dir_$dir" "Directory: $dir" "[[ -d '$NOA_ROOT/$dir' ]]"
done

# 2. Core Tools
if ! $JSON_OUTPUT; then echo ""; echo -e "${YELLOW}Checking core tools...${NC}"; fi

run_check "tool_git" "Tool: git" "command -v git"
run_check "tool_jq" "Tool: jq" "command -v jq || [[ -x '$NOA_ROOT/bin/jq' ]]"
run_check "tool_rg" "Tool: rg" "command -v rg || [[ -x '$NOA_ROOT/bin/rg' ]]"

# 3. AI Provider configss
if ! $JSON_OUTPUT; then echo ""; echo -e "${YELLOW}Checking AI providers...${NC}"; fi

VERIFY_PROVIDERS="$BOOTSTRAP_DIR/verify-ai-providers.sh"
if [[ -x "$VERIFY_PROVIDERS" ]]; then
    run_check "ai_providers" "AI provider verification" "$VERIFY_PROVIDERS --json"
fi

# 4. Shared Resources
if ! $JSON_OUTPUT; then echo ""; echo -e "${YELLOW}Checking shared resources...${NC}"; fi

VERIFY_SHARED="$BOOTSTRAP_DIR/verify-shared-resources.sh"
if [[ -x "$VERIFY_SHARED" ]]; then
    run_check "shared_resources" "Shared resources verification" "$VERIFY_SHARED --json"
fi

# 5. Environment Variables
if ! $JSON_OUTPUT; then echo ""; echo -e "${YELLOW}Checking environment...${NC}"; fi

run_check "env_noa_root" "NOA_ROOT environment variable" "[[ -n \"\$NOA_ROOT\" ]]"

# Summary
if ! $JSON_OUTPUT; then
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}                    Verification Summary${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "  ${GRAY}Total checks: $TOTAL${NC}"
    echo -e "  ${GREEN}Passed:       $PASSED${NC}"
    if [[ $FAILED -gt 0 ]]; then
        echo -e "  ${RED}Failed:       $FAILED${NC}"
    else
        echo -e "  ${GRAY}Failed:       $FAILED${NC}"
    fi
    echo ""

    if [[ $FAILED -eq 0 ]]; then
        echo -e "${GREEN}✓ All checks passed! NOA environment is ready.${NC}"
    else
        echo -e "${RED}✗ Some checks failed. Run bootstrap to fix issues.${NC}"
    fi
fi

if $JSON_OUTPUT; then
    cat << EOF
{
  "timestamp": "$(date -Iseconds)",
  "noa_root": "$NOA_ROOT",
  "summary": {
    "total": $TOTAL,
    "passed": $PASSED,
    "failed": $FAILED
  }
}
EOF
fi

exit $FAILED

