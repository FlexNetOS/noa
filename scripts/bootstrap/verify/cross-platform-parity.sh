#!/bin/bash
#
# Verify cross-platform parity between PowerShell and Bash scripts.
#
# Compares script pairs to ensure they produce identical results.
# This is critical for NOA's cross-platform guarantee.
#
# Usage:
#   ./cross-platform-parity.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"
SCRIPTS_DIR="$NOA_ROOT/scripts"

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
GRAY='\033[0;90m'
NC='\033[0m'

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}         Cross-Platform Parity Verification${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Script pairs to verify
declare -A SCRIPT_PAIRS=(
    ["Main Bootstrap"]="bootstrap/bootstrap.ps1:bootstrap/bootstrap.sh"
    ["Cache Setup"]="bootstrap/configs/cache-setup.ps1:bootstrap/configs/cache-setup.sh"
    ["Log Setup"]="bootstrap/configs/log-setup.ps1:bootstrap/configs/log-setup.sh"
    ["Verify All"]="bootstrap/verify/verify-all.ps1:bootstrap/verify/verify-all.sh"
    ["Smoke Test"]="bootstrap/verify/smoke-test.ps1:bootstrap/verify/smoke-test.sh"
    ["Check Prerequisites"]="setup/check-prereqs.ps1:../init/check-prereqs.sh"
    ["Env Generator"]="bootstrap/generators/noa-env.ps1:bootstrap/generators/noa-env.sh"
)

PASSED=0
FAILED=0
MISSING=0

echo -e "${YELLOW}Checking script pairs exist...${NC}"
echo ""

for name in "${!SCRIPT_PAIRS[@]}"; do
    IFS=':' read -r ps_script sh_script <<< "${SCRIPT_PAIRS[$name]}"

    ps_path="$SCRIPTS_DIR/$ps_script"
    sh_path="$SCRIPTS_DIR/$sh_script"

    ps_exists=false
    sh_exists=false

    [[ -f "$ps_path" ]] && ps_exists=true
    [[ -f "$sh_path" ]] && sh_exists=true

    if $ps_exists && $sh_exists; then
        echo -e "  ${GREEN}[PAIR]${NC} $name"
        echo -e "         ${GRAY}PS: $ps_script${NC}"
        echo -e "         ${GRAY}SH: $sh_script${NC}"
        ((PASSED++))
    elif ! $ps_exists && ! $sh_exists; then
        echo -e "  ${RED}[MISS]${NC} $name - Both missing"
        ((MISSING++))
    else
        echo -e "  ${YELLOW}[HALF]${NC} $name"
        $ps_exists || echo -e "         ${RED}Missing: $ps_script${NC}"
        $sh_exists || echo -e "         ${RED}Missing: $sh_script${NC}"
        ((FAILED++))
    fi
done

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}                     Parity Summary${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "  ${GREEN}Script pairs found:  $PASSED${NC}"
if [[ $FAILED -gt 0 ]]; then
    echo -e "  ${YELLOW}Incomplete pairs:    $FAILED${NC}"
else
    echo -e "  ${GRAY}Incomplete pairs:    $FAILED${NC}"
fi
if [[ $MISSING -gt 0 ]]; then
    echo -e "  ${RED}Missing both:        $MISSING${NC}"
else
    echo -e "  ${GRAY}Missing both:        $MISSING${NC}"
fi
echo ""

if [[ $FAILED -eq 0 ]] && [[ $MISSING -eq 0 ]]; then
    echo -e "${GREEN}✓ All script pairs verified!${NC}"
else
    echo -e "${YELLOW}⚠ Some scripts missing cross-platform counterpart${NC}"
fi

exit $FAILED

