#!/usr/bin/env bash
#
# NOA Portable MinGW-w64 Installer (Unix/macOS)
#
# Note: MinGW-w64 is Windows-specific. On Unix systems, use native GCC.
# This script provides a wrapper that checks for system GCC or suggests alternatives.
#
# Constitutional Compliance: §3.1 Self-Contained & Autonomous
#
# Usage:
#   ./mingw-portable.sh

set -euo pipefail

# Auto-detect NOA_ROOT
if [[ -z "${NOA_ROOT:-}" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    NOA_ROOT="$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")"
fi

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() { echo -e "${NC}[..] $1${NC}"; }
log_ok() { echo -e "${GREEN}[OK] $1${NC}"; }
log_warn() { echo -e "${YELLOW}[!!] $1${NC}"; }
log_error() { echo -e "${RED}[XX] $1${NC}"; }

echo ""
echo -e "${CYAN}============================================================${NC}"
echo -e "${CYAN}NOA GCC Compiler Setup (Unix/macOS)${NC}"
echo -e "${CYAN}============================================================${NC}"
echo ""

# MinGW-w64 is Windows-specific
log_info "MinGW-w64 is Windows-specific. On Unix systems, use native GCC."

# Check for system GCC
if command -v gcc &> /dev/null; then
    GCC_VERSION=$(gcc --version 2>&1 | head -1)
    log_ok "System GCC found: $GCC_VERSION"
    log_info "GCC is already available in PATH"

    # Create symlinks in NOA bin/ for consistency
    NOA_BIN="$NOA_ROOT/bin"
    mkdir -p "$NOA_BIN"

    for tool in gcc g++ gfortran make; do
        if command -v $tool &> /dev/null; then
            TOOL_PATH=$(command -v $tool)
            rm -f "$NOA_BIN/$tool" 2>/dev/null || true
            ln -sf "$TOOL_PATH" "$NOA_BIN/$tool"
            log_ok "Linked $tool -> $TOOL_PATH"
        fi
    done
else
    log_warn "GCC not found in PATH"
    log_info "Install GCC using your system package manager:"
    echo ""
    echo -e "${CYAN}  Ubuntu/Debian:${NC} sudo apt-get install build-essential"
    echo -e "${CYAN}  Fedora/RHEL:${NC}   sudo dnf install gcc gcc-c++ make"
    echo -e "${CYAN}  macOS:${NC}         xcode-select --install"
    echo -e "${CYAN}  Arch:${NC}          sudo pacman -S base-devel"
    echo ""
fi

exit 0

