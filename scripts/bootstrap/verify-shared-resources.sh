#!/bin/bash
#
# Verify shared AI resources are properly configured.
#
# Checks that all shared resource directories exist and contain valid configurations.
# Validates resource registry and provider configs reference shared paths.
#
# Usage:
#   ./verify-shared-resources.sh
#   ./verify-shared-resources.sh --json

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
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"
SHARED_DIR="$NOA_ROOT/ai/shared"
PROVIDERS_DIR="$NOA_ROOT/ai/providers"

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
WARNINGS=0

if ! $JSON_OUTPUT; then
    echo -e "${CYAN}Verifying Shared AI Resources...${NC}"
    echo -e "${GRAY}NOA Root: $NOA_ROOT${NC}"
    echo -e "${GRAY}Shared Dir: $SHARED_DIR${NC}"
    echo ""
fi

# Check directories
EXPECTED_DIRS=(
    "agents:required"
    "workflows:required"
    "prompts:required"
    "skills:required"
    "tools:required"
    "models:required"
    "commands:required"
    "resources:required"
    "resources/schema:required"
)

if ! $JSON_OUTPUT; then
    echo -e "${YELLOW}Checking directories...${NC}"
fi

for dir_spec in "${EXPECTED_DIRS[@]}"; do
    IFS=':' read -r dir_name required <<< "$dir_spec"
    ((TOTAL++))
    dir_path="$SHARED_DIR/$dir_name"

    if [[ -d "$dir_path" ]]; then
        ((PASSED++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${GREEN}[OK]${NC} $dir_name/"
        fi
    elif [[ "$required" == "required" ]]; then
        ((FAILED++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${RED}[FAIL]${NC} $dir_name/ - MISSING (required)"
        fi
    else
        ((WARNINGS++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${YELLOW}[WARN]${NC} $dir_name/ - missing (optional)"
        fi
    fi
done

# Check files
EXPECTED_FILES=(
    "resources/resource-registry.json:required"
    "resources/resource-aliases.json:optional"
    "resources/execution-memory.db:optional"
    "resources/schema/execution-memory.sql:required"
)

if ! $JSON_OUTPUT; then
    echo ""
    echo -e "${YELLOW}Checking files...${NC}"
fi

for file_spec in "${EXPECTED_FILES[@]}"; do
    IFS=':' read -r file_path required <<< "$file_spec"
    ((TOTAL++))
    full_path="$SHARED_DIR/$file_path"

    if [[ -f "$full_path" ]]; then
        ((PASSED++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${GREEN}[OK]${NC} $file_path"
        fi
    elif [[ "$required" == "required" ]]; then
        ((FAILED++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${RED}[FAIL]${NC} $file_path - MISSING (required)"
        fi
    else
        ((WARNINGS++))
        if ! $JSON_OUTPUT; then
            echo -e "  ${YELLOW}[WARN]${NC} $file_path - missing (optional)"
        fi
    fi
done

# Check provider configs
if ! $JSON_OUTPUT; then
    echo ""
    echo -e "${YELLOW}Checking provider configurations...${NC}"
fi

for provider_type in local cloud hybrid ide; do
    type_dir="$PROVIDERS_DIR/$provider_type"
    if [[ -d "$type_dir" ]]; then
        for config in $(find "$type_dir" -name "config.json" 2>/dev/null); do
            provider_name=$(basename "$(dirname "$config")")
            if grep -q "sharedResource" "$config" 2>/dev/null; then
                if ! $JSON_OUTPUT; then
                    echo -e "  ${GREEN}[OK]${NC} $provider_name ($provider_type) - uses shared resources"
                fi
            else
                ((WARNINGS++))
                if ! $JSON_OUTPUT; then
                    echo -e "  ${YELLOW}[WARN]${NC} $provider_name ($provider_type) - no shared resource reference"
                fi
            fi
        done
    fi
done

# Summary
if ! $JSON_OUTPUT; then
    echo ""
    echo -e "${CYAN}Summary:${NC}"
    echo -e "  ${GRAY}Total checks: $TOTAL${NC}"
    echo -e "  ${GREEN}Passed: $PASSED${NC}"
    if [[ $FAILED -gt 0 ]]; then
        echo -e "  ${RED}Failed: $FAILED${NC}"
    else
        echo -e "  ${GRAY}Failed: $FAILED${NC}"
    fi
    if [[ $WARNINGS -gt 0 ]]; then
        echo -e "  ${YELLOW}Warnings: $WARNINGS${NC}"
    else
        echo -e "  ${GRAY}Warnings: $WARNINGS${NC}"
    fi
    echo ""

    if [[ $FAILED -gt 0 ]]; then
        echo -e "${RED}Some required resources are missing. Run:${NC}"
        echo -e "  ${YELLOW}./scripts/bootstrap/installers/shared-resources/create-directories.sh${NC}"
    else
        echo -e "${GREEN}All required shared resources are configured.${NC}"
    fi
fi

if $JSON_OUTPUT; then
    cat << EOF
{
  "timestamp": "$(date -Iseconds)",
  "noa_root": "$NOA_ROOT",
  "shared_dir": "$SHARED_DIR",
  "summary": {
    "total_checks": $TOTAL,
    "passed": $PASSED,
    "failed": $FAILED,
    "warnings": $WARNINGS
  }
}
EOF
fi

# Exit with error if required resources are missing
if [[ $FAILED -gt 0 ]]; then
    exit 1
fi

