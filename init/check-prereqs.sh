#!/bin/bash
#
# NOA Comprehensive Prerequisites Check (T673 - Bash version)
#
# Per NOA Constitution §3.1: The system MUST operate entirely inside noa_root,
# EXCEPT for build toolchains which require system-wide installation.
#
# Usage: ./init/check-prereqs.sh [--json]
#

set -euo pipefail

# Auto-detect NOA_ROOT from script location
NOA_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
NOA_BIN="$NOA_ROOT/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m'

JSON_OUTPUT=false
[[ "${1:-}" == "--json" ]] && JSON_OUTPUT=true

# Results
declare -a INSTALLED=()
declare -a MISSING_CRITICAL=()
declare -a MISSING_HIGH=()

version_gte() {
    printf '%s\n%s' "$2" "$1" | sort -V -C
}

check_tool() {
    local name="$1" min_version="$2" severity="$3" install_cmd="$4" version_cmd="$5" category="$6"

    if command -v "$(echo "$version_cmd" | awk '{print $1}')" &>/dev/null; then
        local current_version
        current_version=$(eval "$version_cmd" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1 || echo "unknown")

        if [[ "$current_version" != "unknown" ]] && version_gte "$current_version" "$min_version"; then
            INSTALLED+=("$name:$current_version:$category")
            $JSON_OUTPUT || echo -e "  ${GREEN}[OK]${NC} $name $current_version"
        else
            $JSON_OUTPUT || echo -e "  ${YELLOW}[!!]${NC} $name $current_version (need >= $min_version)"
        fi
    else
        if [[ "$severity" == "CRITICAL" ]]; then
            MISSING_CRITICAL+=("$name:$install_cmd")
            $JSON_OUTPUT || { echo -e "  ${RED}[X]${NC} $name NOT FOUND (CRITICAL)"; echo -e "      ${GRAY}Install: $install_cmd${NC}"; }
        else
            MISSING_HIGH+=("$name:$install_cmd")
            $JSON_OUTPUT || { echo -e "  ${RED}[X]${NC} $name NOT FOUND (HIGH)"; echo -e "      ${GRAY}Install: $install_cmd${NC}"; }
        fi
    fi
}

check_self_contained() {
    local name="$1" exe_name="$2"
    local tool_path="$NOA_BIN/$exe_name"

    if [[ -f "$tool_path" ]]; then
        INSTALLED+=("$name:self-contained:Self-Contained")
        $JSON_OUTPUT || echo -e "  ${GREEN}[OK]${NC} $name (self-contained: $tool_path)"
    else
        MISSING_HIGH+=("$name:./scripts/download-static-binaries")
        $JSON_OUTPUT || echo -e "  ${YELLOW}[--]${NC} $name not in bin/ (optional)"
    fi
}

# ========================================
# Main Checks
# ========================================

if ! $JSON_OUTPUT; then
    echo ""
    echo -e "${CYAN}============================================================${NC}"
    echo -e "${CYAN}NOA Prerequisites Check${NC}"
    echo -e "${GRAY}Constitution: §3.1 (Self-Contained), FR-015 (Security)${NC}"
    echo -e "${CYAN}============================================================${NC}"
    echo ""
    echo "NOA_ROOT: $NOA_ROOT"
    echo "NOA_BIN:  $NOA_BIN"
    echo ""

    echo -e "${YELLOW}1. SYSTEM-WIDE Build Toolchains (CRITICAL)${NC}"
    echo -e "${GRAY}   Note: These CANNOT be self-contained per language requirements${NC}"
    echo "------------------------------------------------------------"
fi

# CRITICAL - Build Toolchains
check_tool "Rust (rustc)" "1.83.0" "CRITICAL" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" "rustc --version" "Build-SystemWide"
check_tool "Cargo" "1.83.0" "CRITICAL" "(installed with Rust)" "cargo --version" "Build-SystemWide"
check_tool "Go" "1.23.0" "CRITICAL" "brew install go || sudo apt install golang-go" "go version" "Build-SystemWide"
check_tool "Node.js" "20.0.0" "CRITICAL" "brew install node || sudo apt install nodejs" "node --version" "Build-SystemWide"
check_tool "Python" "3.12.0" "CRITICAL" "brew install python@3.12 || sudo apt install python3.12" "python3 --version" "Build-SystemWide"
check_tool "protoc" "28.0.0" "CRITICAL" "brew install protobuf || sudo apt install protobuf-compiler" "protoc --version" "Build-SystemWide"

$JSON_OUTPUT || { echo ""; echo -e "${YELLOW}2. Code Quality Tools (HIGH)${NC}"; echo "------------------------------------------------------------"; }

check_tool "rustfmt" "1.0.0" "HIGH" "rustup component add rustfmt" "rustfmt --version" "Quality"
check_tool "clippy" "0.1.0" "HIGH" "rustup component add clippy" "cargo clippy --version" "Quality"
check_tool "golangci-lint" "1.62.0" "HIGH" "go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest" "golangci-lint --version" "Quality"
check_tool "eslint" "9.0.0" "HIGH" "npm install -g eslint" "eslint --version" "Quality"
check_tool "ruff" "0.8.0" "HIGH" "pip install ruff" "ruff --version" "Quality"

$JSON_OUTPUT || { echo ""; echo -e "${YELLOW}3. Security Tools - FR-015 (HIGH)${NC}"; echo "------------------------------------------------------------"; }

check_tool "Gitleaks" "8.21.0" "HIGH" "brew install gitleaks" "gitleaks version" "Security"
check_tool "Trivy" "0.57.0" "HIGH" "brew install trivy" "trivy --version" "Security"
check_tool "Grype" "0.84.0" "HIGH" "brew install grype" "grype version" "Security"
check_tool "Semgrep" "1.97.0" "HIGH" "pip install semgrep" "semgrep --version" "Security"

$JSON_OUTPUT || { echo ""; echo -e "${YELLOW}4. Self-Contained Utilities (noa_root/bin/)${NC}"; echo "------------------------------------------------------------"; }

# Check self-contained (Unix binaries, no .exe)
check_self_contained "jq" "jq"
check_self_contained "ripgrep" "rg"
check_self_contained "fd" "fd"
check_self_contained "bat" "bat"

$JSON_OUTPUT || { echo ""; echo -e "${YELLOW}5. Basic Prerequisites${NC}"; echo "------------------------------------------------------------"; }

check_tool "Git" "2.40.0" "CRITICAL" "brew install git || sudo apt install git" "git --version" "Basic"
check_tool "GitHub CLI" "2.40.0" "HIGH" "brew install gh || sudo apt install gh" "gh --version" "Basic"

# ========================================
# Output Results
# ========================================

if $JSON_OUTPUT; then
    echo "{"
    echo "  \"noa_root\": \"$NOA_ROOT\","
    echo "  \"installed\": ${#INSTALLED[@]},"
    echo "  \"missing_critical\": ${#MISSING_CRITICAL[@]},"
    echo "  \"missing_high\": ${#MISSING_HIGH[@]}"
    echo "}"
else
    echo ""
    echo -e "${CYAN}============================================================${NC}"
    echo -e "${CYAN}Summary${NC}"
    echo -e "${CYAN}============================================================${NC}"
    echo -e "Installed:        ${GREEN}${#INSTALLED[@]}${NC}"
    [[ ${#MISSING_CRITICAL[@]} -gt 0 ]] && echo -e "Missing CRITICAL: ${RED}${#MISSING_CRITICAL[@]}${NC}" || echo -e "Missing CRITICAL: ${GREEN}0${NC}"
    [[ ${#MISSING_HIGH[@]} -gt 0 ]] && echo -e "Missing HIGH:     ${YELLOW}${#MISSING_HIGH[@]}${NC}" || echo -e "Missing HIGH:     ${GREEN}0${NC}"
fi

# Exit code
if [[ ${#MISSING_CRITICAL[@]} -gt 0 ]]; then
    $JSON_OUTPUT || { echo ""; echo -e "${RED}ERROR: Critical prerequisites missing. Install before building.${NC}"; }
    exit 1
elif [[ ${#MISSING_HIGH[@]} -gt 0 ]]; then
    $JSON_OUTPUT || { echo ""; echo -e "${YELLOW}WARNING: High-priority tools missing. Quality gates may fail.${NC}"; }
    exit 2
else
    $JSON_OUTPUT || { echo ""; echo -e "${GREEN}All prerequisites met! Ready for implementation.${NC}"; }
    exit 0
fi
