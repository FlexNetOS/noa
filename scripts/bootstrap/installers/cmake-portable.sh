#!/usr/bin/env bash
#
# NOA Portable CMake Installer (Unix/macOS)
#
# Downloads and installs CMake as a portable tool within the NOA contained environment.
# Creates symlinks in NOA_ROOT/bin/ for easy access.
#
# Constitutional Compliance: §3.1 Self-Contained & Autonomous
#
# Usage:
#   ./cmake-portable.sh
#   ./cmake-portable.sh --version 3.31.3 --force

set -euo pipefail

# Defaults
VERSION="${VERSION:-3.31.3}"
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
CMAKE_DIR="$NOA_OPT/cmake"

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
echo -e "${CYAN}NOA Portable CMake Installer${NC}"
echo -e "Version: $VERSION | Target: $CMAKE_DIR"
echo -e "${CYAN}============================================================${NC}"
echo ""

# Check if already installed
CMAKE_EXE="$CMAKE_DIR/bin/cmake"
if [[ -f "$CMAKE_EXE" ]] && [[ "$FORCE" != "true" ]]; then
    CURRENT_VERSION=$("$CMAKE_EXE" --version 2>&1 | head -1)
    log_ok "CMake already installed: $CURRENT_VERSION"
    log_info "Use --force to reinstall"
    exit 0
fi

# Ensure directories exist
mkdir -p "$NOA_CACHE" "$NOA_OPT" "$NOA_BIN"

# Detect platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$PLATFORM" in
    linux)
        if [[ "$ARCH" == "x86_64" ]]; then
            ARCHIVE_NAME="cmake-$VERSION-linux-x86_64.tar.gz"
        elif [[ "$ARCH" == "aarch64" ]]; then
            ARCHIVE_NAME="cmake-$VERSION-linux-aarch64.tar.gz"
        else
            log_error "Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    darwin)
        ARCHIVE_NAME="cmake-$VERSION-macos-universal.tar.gz"
        ;;
    *)
        log_error "Unsupported platform: $PLATFORM"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/Kitware/CMake/releases/download/v$VERSION/$ARCHIVE_NAME"
ARCHIVE_FILE="$NOA_CACHE/$ARCHIVE_NAME"
EXTRACT_DIR="$NOA_OPT/cmake-$VERSION-${PLATFORM}-${ARCH}"

# Download
log_info "Downloading CMake $VERSION..."
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
log_info "Extracting to $NOA_OPT..."
rm -rf "$CMAKE_DIR" "$EXTRACT_DIR" 2>/dev/null || true

tar -xzf "$ARCHIVE_FILE" -C "$NOA_OPT"

# Find and rename extracted directory
EXTRACTED=$(find "$NOA_OPT" -maxdepth 1 -type d -name "cmake-$VERSION*" | head -1)
if [[ -n "$EXTRACTED" ]] && [[ "$EXTRACTED" != "$CMAKE_DIR" ]]; then
    mv "$EXTRACTED" "$CMAKE_DIR"
fi

# Handle macOS .app bundle
if [[ "$PLATFORM" == "darwin" ]] && [[ -d "$CMAKE_DIR/CMake.app" ]]; then
    # Symlink the actual binaries from the .app bundle
    CMAKE_APP_BIN="$CMAKE_DIR/CMake.app/Contents/bin"
    if [[ -d "$CMAKE_APP_BIN" ]]; then
        mkdir -p "$CMAKE_DIR/bin"
        ln -sf "$CMAKE_APP_BIN/cmake" "$CMAKE_DIR/bin/cmake"
        ln -sf "$CMAKE_APP_BIN/ctest" "$CMAKE_DIR/bin/ctest"
        ln -sf "$CMAKE_APP_BIN/cpack" "$CMAKE_DIR/bin/cpack"
    fi
fi

log_ok "Extracted to $CMAKE_DIR"

# Create symlinks in bin/
log_info "Creating symlinks in $NOA_BIN..."
for tool in cmake ctest cpack; do
    TARGET="$CMAKE_DIR/bin/$tool"
    LINK="$NOA_BIN/$tool"
    if [[ -f "$TARGET" ]]; then
        rm -f "$LINK" 2>/dev/null || true
        ln -sf "$TARGET" "$LINK"
        log_ok "Linked $tool"
    fi
done

# Verify installation
log_info "Verifying installation..."
INSTALLED_VERSION=$("$CMAKE_DIR/bin/cmake" --version 2>&1 | head -1)
if [[ "$INSTALLED_VERSION" == *"cmake version"* ]]; then
    log_ok "CMake installed successfully: $INSTALLED_VERSION"
else
    log_error "Installation verification failed"
    exit 1
fi

echo ""
echo -e "${GREEN}============================================================${NC}"
echo -e "${GREEN}CMake $VERSION installed successfully!${NC}"
echo -e "Location: $CMAKE_DIR"
echo -e "Symlinks: $NOA_BIN/cmake, ctest, cpack"
echo -e "${GREEN}============================================================${NC}"

exit 0

