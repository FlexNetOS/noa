#!/usr/bin/env bash
#
# NOA Portable LLVM/Clang Installer (Unix/macOS)
#
# Downloads and installs LLVM/Clang as a portable C/C++ compiler.
# Creates symlinks in NOA_ROOT/bin/ for clang, clang++, lld, etc.
#
# Constitutional Compliance: §3.1 Self-Contained & Autonomous
#
# Usage:
#   ./llvm-portable.sh
#   ./llvm-portable.sh --version 19.1.6 --force

set -euo pipefail

# Defaults
VERSION="${VERSION:-19.1.6}"
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
LLVM_DIR="$NOA_OPT/llvm"

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
echo -e "${CYAN}NOA Portable LLVM/Clang Installer${NC}"
echo -e "Version: $VERSION | Target: $LLVM_DIR"
echo -e "${CYAN}============================================================${NC}"
echo ""

# Check if already installed
CLANG_EXE="$LLVM_DIR/bin/clang"
if [[ -f "$CLANG_EXE" ]] && [[ "$FORCE" != "true" ]]; then
    CURRENT_VERSION=$("$CLANG_EXE" --version 2>&1 | head -1)
    log_ok "LLVM already installed: $CURRENT_VERSION"
    log_info "Use --force to reinstall"
    exit 0
fi

# Ensure directories exist
mkdir -p "$NOA_CACHE" "$NOA_OPT" "$NOA_BIN"

# Detect platform and architecture
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$PLATFORM" in
    linux)
        case "$ARCH" in
            x86_64) ARCHIVE_NAME="clang+llvm-$VERSION-x86_64-linux-gnu-ubuntu-22.04.tar.xz" ;;
            aarch64) ARCHIVE_NAME="clang+llvm-$VERSION-aarch64-linux-gnu.tar.xz" ;;
            *) log_error "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    darwin)
        case "$ARCH" in
            x86_64) ARCHIVE_NAME="clang+llvm-$VERSION-x86_64-apple-darwin21.0.tar.xz" ;;
            arm64) ARCHIVE_NAME="clang+llvm-$VERSION-arm64-apple-darwin23.0.tar.xz" ;;
            *) log_error "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        log_error "Unsupported platform: $PLATFORM"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-$VERSION/$ARCHIVE_NAME"
ARCHIVE_FILE="$NOA_CACHE/$ARCHIVE_NAME"

# Download
log_info "Downloading LLVM $VERSION (~500MB)..."
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
    log_info "Using cached download: $(du -h "$ARCHIVE_FILE" | cut -f1)"
fi

# Extract
log_info "Extracting to $NOA_OPT..."
rm -rf "$LLVM_DIR" 2>/dev/null || true

tar -xf "$ARCHIVE_FILE" -C "$NOA_OPT"

# Find and rename extracted directory
EXTRACTED=$(find "$NOA_OPT" -maxdepth 1 -type d -name "clang+llvm-$VERSION*" | head -1)
if [[ -n "$EXTRACTED" ]] && [[ "$EXTRACTED" != "$LLVM_DIR" ]]; then
    mv "$EXTRACTED" "$LLVM_DIR"
fi
log_ok "Extracted to $LLVM_DIR"

# Create symlinks in bin/
log_info "Creating symlinks in $NOA_BIN..."
LLVM_TOOLS=(
    "clang"
    "clang++"
    "lld"
    "ld.lld"
    "llvm-ar"
    "llvm-nm"
    "llvm-objdump"
    "llvm-ranlib"
    "llvm-size"
    "clang-format"
    "clang-tidy"
)

for tool in "${LLVM_TOOLS[@]}"; do
    TARGET="$LLVM_DIR/bin/$tool"
    LINK="$NOA_BIN/$tool"
    if [[ -f "$TARGET" ]]; then
        rm -f "$LINK" 2>/dev/null || true
        ln -sf "$TARGET" "$LINK"
        log_ok "Linked $tool"
    fi
done

# Verify installation
log_info "Verifying installation..."
if [[ -f "$CLANG_EXE" ]]; then
    INSTALLED_VERSION=$("$CLANG_EXE" --version 2>&1 | head -1)
    log_ok "LLVM installed successfully: $INSTALLED_VERSION"
else
    log_error "Installation verification failed - clang not found"
    exit 1
fi

echo ""
echo -e "${GREEN}============================================================${NC}"
echo -e "${GREEN}LLVM $VERSION installed successfully!${NC}"
echo -e "Location: $LLVM_DIR"
echo -e "Symlinks: clang, clang++, lld, clang-format, etc."
echo -e ""
echo -e "${CYAN}Usage:${NC}"
echo -e "  clang --version          # C compiler"
echo -e "  clang++ --version        # C++ compiler"
echo -e "${GREEN}============================================================${NC}"

exit 0

