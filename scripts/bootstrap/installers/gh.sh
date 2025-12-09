#!/bin/bash
#
# GitHub CLI (gh) installer for NOA bootstrap (Unix)
#
# Installs the GitHub CLI to noa_root/bin/

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/platform.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/download.sh" 2>/dev/null || true

NOA_BIN="$NOA_ROOT/bin"
FORCE="${1:-}"

log_section "NOA GitHub CLI Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Check if already installed
if [[ -x "$NOA_BIN/gh" ]] && [[ "$FORCE" != "--force" ]]; then
    VERSION=$("$NOA_BIN/gh" --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    log_success "GitHub CLI already installed: v$VERSION"
    log_info "Use --force to reinstall"
    exit 0
fi

mkdir -p "$NOA_BIN"

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

# Get latest release info
GH_VERSION="2.42.0"
case "$os" in
    macos)
        if [[ "$arch" == "arm64" ]]; then
            ASSET_NAME="gh_${GH_VERSION}_macOS_arm64.zip"
        else
            ASSET_NAME="gh_${GH_VERSION}_macOS_amd64.zip"
        fi
        ;;
    *)
        ASSET_NAME="gh_${GH_VERSION}_linux_amd64.tar.gz"
        ;;
esac

DOWNLOAD_URL="https://github.com/cli/cli/releases/download/v${GH_VERSION}/${ASSET_NAME}"

log_info "Downloading GitHub CLI v$GH_VERSION..."

# Download and extract
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

if command -v curl &>/dev/null; then
    curl -fsSL -o "$TEMP_DIR/$ASSET_NAME" "$DOWNLOAD_URL"
elif command -v wget &>/dev/null; then
    wget -q -O "$TEMP_DIR/$ASSET_NAME" "$DOWNLOAD_URL"
else
    log_error "Neither curl nor wget found"
    exit 1
fi

log_info "Extracting..."
case "$ASSET_NAME" in
    *.tar.gz)
        tar -xzf "$TEMP_DIR/$ASSET_NAME" -C "$TEMP_DIR"
        ;;
    *.zip)
        unzip -q "$TEMP_DIR/$ASSET_NAME" -d "$TEMP_DIR"
        ;;
esac

# Find and install binary
GH_BIN=$(find "$TEMP_DIR" -name "gh" -type f -executable | head -1)
if [[ -z "$GH_BIN" ]]; then
    GH_BIN=$(find "$TEMP_DIR" -path "*/bin/gh" -type f | head -1)
fi

if [[ -z "$GH_BIN" ]]; then
    log_error "Could not find gh binary in archive"
    exit 1
fi

cp "$GH_BIN" "$NOA_BIN/gh"
chmod +x "$NOA_BIN/gh"

VERSION=$("$NOA_BIN/gh" --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
log_success "GitHub CLI installed: v$VERSION"
echo "Location: $NOA_BIN/gh"

echo ""
echo "To authenticate, run:"
echo "  $NOA_BIN/gh auth login"

