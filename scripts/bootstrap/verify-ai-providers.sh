#!/bin/bash
#
# Verify all AI provider CLIs are installed and functional.
#
# Checks each configured AI provider CLI for availability and version.
# Reports status of local, cloud, hybrid, and IDE providers.
#
# Usage:
#   ./verify-ai-providers.sh
#   ./verify-ai-providers.sh --json

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
PROVIDERS_DIR="$NOA_ROOT/ai/providers"

# Colors for output
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

# Provider definitions
declare -A PROVIDERS=(
    ["git-cli"]="local:git:--version"
    ["ollama"]="local:ollama:--version"
    ["llama-server"]="local:llama-server:--version"
    ["cursor"]="hybrid:cursor:--version:optional"
    ["claude-code"]="cloud:claude:--version:optional"
    ["codex"]="cloud:codex:--version:optional"
    ["abacus"]="cloud:abacusai:--version:optional"
)

# Counters
TOTAL=0
AVAILABLE=0
MISSING=0
OPTIONAL_MISSING=0

# JSON output storage
declare -A RESULTS

if ! $JSON_OUTPUT; then
    echo -e "${CYAN}Verifying AI Provider CLIs...${NC}"
    echo -e "${GRAY}NOA Root: $NOA_ROOT${NC}"
    echo ""
fi

for provider in "${!PROVIDERS[@]}"; do
    IFS=':' read -r type command version_arg optional <<< "${PROVIDERS[$provider]}"
    ((TOTAL++))

    # Check NOA bin first, then PATH
    noa_bin_path="$NOA_ROOT/bin/$command"

    if [[ -x "$noa_bin_path" ]]; then
        cmd_path="$noa_bin_path"
    elif command -v "$command" &> /dev/null; then
        cmd_path="$(command -v "$command")"
    else
        cmd_path=""
    fi

    if [[ -n "$cmd_path" ]]; then
        # Get version
        version=$("$cmd_path" $version_arg 2>&1 | head -1 || echo "unknown")
        ((AVAILABLE++))

        if ! $JSON_OUTPUT; then
            echo -e "  ${GREEN}[OK]${NC} $provider ($type): $version"
        fi

        RESULTS[$provider]="available:$version:$cmd_path"
    else
        if [[ "$optional" == "optional" ]]; then
            ((OPTIONAL_MISSING++))
            if ! $JSON_OUTPUT; then
                echo -e "  ${YELLOW}[SKIP]${NC} $provider ($type): Not installed (optional)"
            fi
        else
            ((MISSING++))
            if ! $JSON_OUTPUT; then
                echo -e "  ${RED}[MISS]${NC} $provider ($type): Not installed"
            fi
        fi
        RESULTS[$provider]="missing"
    fi
done

# Count configured providers
CONFIGURED=0
for dir in local cloud hybrid ide; do
    if [[ -d "$PROVIDERS_DIR/$dir" ]]; then
        CONFIGURED=$((CONFIGURED + $(find "$PROVIDERS_DIR/$dir" -name "config.json" 2>/dev/null | wc -l)))
    fi
done

if ! $JSON_OUTPUT; then
    echo ""
    echo -e "${CYAN}Summary:${NC}"
    echo -e "  ${GRAY}Total providers checked: $TOTAL${NC}"
    echo -e "  ${GREEN}Available: $AVAILABLE${NC}"
    if [[ $MISSING -gt 0 ]]; then
        echo -e "  ${RED}Missing (required): $MISSING${NC}"
    else
        echo -e "  ${GRAY}Missing (required): $MISSING${NC}"
    fi
    echo -e "  ${YELLOW}Missing (optional): $OPTIONAL_MISSING${NC}"
    echo ""
    echo -e "${GRAY}Configured provider configs found: $CONFIGURED${NC}"
fi

if $JSON_OUTPUT; then
    cat << EOF
{
  "timestamp": "$(date -Iseconds)",
  "noa_root": "$NOA_ROOT",
  "summary": {
    "total": $TOTAL,
    "available": $AVAILABLE,
    "missing": $MISSING,
    "optional_missing": $OPTIONAL_MISSING,
    "configured": $CONFIGURED
  }
}
EOF
fi

# Exit with error if required providers are missing
if [[ $MISSING -gt 0 ]]; then
    exit 1
fi

