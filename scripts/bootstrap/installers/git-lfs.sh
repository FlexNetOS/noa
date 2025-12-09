#!/bin/bash
#
# Git LFS installer for NOA bootstrap (Unix)
#
# Installs Git Large File Storage extension.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/platform.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/download.sh" 2>/dev/null || true

NOA_BIN="$NOA_ROOT/bin"
FORCE="${1:-}"

log_section "NOA Git LFS Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Check if already installed
if command -v git-lfs &>/dev/null && [[ "$FORCE" != "--force" ]]; then
    VERSION=$(git-lfs --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    log_success "Git LFS already installed: v$VERSION"
    log_info "Use --force to reinstall"
    exit 0
fi

mkdir -p "$NOA_BIN"

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

# Construct download URL
GIT_LFS_VERSION="3.4.1"
case "$os" in
    macos)
        if [[ "$arch" == "arm64" ]]; then
            ASSET_SUFFIX="darwin-arm64"
        else
            ASSET_SUFFIX="darwin-amd64"
        fi
        ;;
    *)
        ASSET_SUFFIX="linux-amd64"
        ;;
esac

DOWNLOAD_URL="https://github.com/git-lfs/git-lfs/releases/download/v${GIT_LFS_VERSION}/git-lfs-${ASSET_SUFFIX}-v${GIT_LFS_VERSION}.tar.gz"
ARCHIVE_NAME="git-lfs-${ASSET_SUFFIX}-v${GIT_LFS_VERSION}.tar.gz"

log_info "Downloading Git LFS v$GIT_LFS_VERSION..."

# Download and extract
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

if command -v curl &>/dev/null; then
    curl -fsSL -o "$TEMP_DIR/$ARCHIVE_NAME" "$DOWNLOAD_URL"
elif command -v wget &>/dev/null; then
    wget -q -O "$TEMP_DIR/$ARCHIVE_NAME" "$DOWNLOAD_URL"
else
    log_error "Neither curl nor wget found"
    exit 1
fi

log_info "Extracting..."
tar -xzf "$TEMP_DIR/$ARCHIVE_NAME" -C "$TEMP_DIR"

# Find and install binary
GIT_LFS_BIN=$(find "$TEMP_DIR" -name "git-lfs" -type f | head -1)
if [[ -z "$GIT_LFS_BIN" ]]; then
    log_error "Could not find git-lfs binary in archive"
    exit 1
fi

cp "$GIT_LFS_BIN" "$NOA_BIN/git-lfs"
chmod +x "$NOA_BIN/git-lfs"

# Initialize Git LFS
if command -v git &>/dev/null; then
    "$NOA_BIN/git-lfs" install --skip-smudge 2>/dev/null || true
fi

VERSION=$("$NOA_BIN/git-lfs" --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
log_success "Git LFS installed: v$VERSION"
echo "Location: $NOA_BIN/git-lfs"

