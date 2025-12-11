#!/usr/bin/env bash
#
# NOA Portable Ninja Installer (Unix/macOS)
#
# Downloads and installs Ninja build tool within the NOA contained environment.
# Creates symlink in NOA_ROOT/bin/ for easy access.
#
# Constitutional Compliance: §3.1 Self-Contained & Autonomous
#
# Usage:
#   ./ninja-portable.sh
#   ./ninja-portable.sh --version 1.12.1 --force

set -euo pipefail

# Defaults
VERSION="${VERSION:-1.12.1}"
FORCE="${FORCE:-false}"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version|-v) VERSION="$2"; shift 2 ;;
        --force|-f) FORCE="true"; shift ;;
        --noa-root) NOA_ROOT="$2"; shift 2 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Auto-detect NOA_ROOT
if [[ -z "${NOA_ROOT:-}" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    NOA_ROOT="$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")"
fi

NOA_BIN="$NOA_ROOT/bin"
NOA_OPT="$NOA_ROOT/opt"
NOA_CACHE="$NOA_ROOT/cache"
NINJA_DIR="$NOA_OPT/ninja"

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
echo -e "${CYAN}NOA Portable Ninja Installer${NC}"
echo -e "Version: $VERSION | Target: $NINJA_DIR"
echo -e "${CYAN}============================================================${NC}"
echo ""

# Check if already installed
NINJA_EXE="$NINJA_DIR/ninja"
if [[ -f "$NINJA_EXE" ]] && [[ "$FORCE" != "true" ]]; then
    CURRENT_VERSION=$("$NINJA_EXE" --version 2>&1)
    log_ok "Ninja already installed: v$CURRENT_VERSION"
    log_info "Use --force to reinstall"
    exit 0
fi

# Ensure directories exist
mkdir -p "$NOA_CACHE" "$NINJA_DIR" "$NOA_BIN"

# Detect platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$PLATFORM" in
    linux) ARCHIVE_NAME="ninja-linux.zip" ;;
    darwin) ARCHIVE_NAME="ninja-mac.zip" ;;
    *)
        log_error "Unsupported platform: $PLATFORM"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/ninja-build/ninja/releases/download/v$VERSION/$ARCHIVE_NAME"
ARCHIVE_FILE="$NOA_CACHE/ninja-$VERSION-$PLATFORM.zip"

# Download
log_info "Downloading Ninja $VERSION..."
if [[ ! -f "$ARCHIVE_FILE" ]]; then
    if command -v curl &> /dev/null; then
        curl -fSL "$DOWNLOAD_URL" -o "$ARCHIVE_FILE"
    elif command -v wget &> /dev/null; then
        wget -q "$DOWNLOAD_URL" -O "$ARCHIVE_FILE"
    else
        log_error "Neither curl nor wget found"
        exit 1
    fi
    log_ok "Downloaded: $(du -h "$ARCHIVE_FILE" | cut -f1)"
else
    log_info "Using cached download"
fi

# Extract
log_info "Extracting to $NINJA_DIR..."
unzip -o -q "$ARCHIVE_FILE" -d "$NINJA_DIR"
chmod +x "$NINJA_DIR/ninja"
log_ok "Extracted to $NINJA_DIR"

# Create symlink in bin/
log_info "Creating symlink in $NOA_BIN..."
rm -f "$NOA_BIN/ninja" 2>/dev/null || true
ln -sf "$NINJA_DIR/ninja" "$NOA_BIN/ninja"
log_ok "Linked ninja"

# Verify installation
log_info "Verifying installation..."
INSTALLED_VERSION=$("$NINJA_EXE" --version 2>&1)
if [[ -n "$INSTALLED_VERSION" ]]; then
    log_ok "Ninja installed successfully: v$INSTALLED_VERSION"
else
    log_error "Installation verification failed"
    exit 1
fi

echo ""
echo -e "${GREEN}============================================================${NC}"
echo -e "${GREEN}Ninja $VERSION installed successfully!${NC}"
echo -e "Location: $NINJA_DIR"
echo -e "Symlink: $NOA_BIN/ninja"
echo -e "${GREEN}============================================================${NC}"

exit 0

