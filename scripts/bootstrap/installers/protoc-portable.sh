#!/bin/bash
#
# Install Protocol Buffers (protoc) to noa_root/bin/
#
# Per NOA Constitution 3.1: Self-contained installation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/platform.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/download.sh" 2>/dev/null || true

# configsuration
PROTOC_VERSION="${PROTOC_VERSION:-28.3}"
FORCE="${1:-}"

# Paths
NOA_BIN="$NOA_ROOT/bin"
PROTOBUF_ROOT="$NOA_ROOT/opt/protobuf"
TEMP_DIR="$NOA_ROOT/tmp"

log_section "NOA Portable Protoc Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo "Version:  $PROTOC_VERSION"
echo ""

# Check if already installed
if [[ -x "$NOA_BIN/protoc" && "$FORCE" != "--force" ]]; then
    VERSION=$("$NOA_BIN/protoc" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1)
    log_success "protoc already installed: v$VERSION"
    log_info "Use --force to reinstall"
    exit 0
fi

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

# Construct download URL
case "$os" in
    macos)
        if [[ "$arch" == "arm64" ]]; then
            ASSET_SUFFIX="osx-aarch_64"
        else
            ASSET_SUFFIX="osx-x86_64"
        fi
        ;;
    *)
        ASSET_SUFFIX="linux-x86_64"
        ;;
esac

ARCHIVE_NAME="protoc-${PROTOC_VERSION}-${ASSET_SUFFIX}.zip"
DOWNLOAD_URL="https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/${ARCHIVE_NAME}"

# Create directories
mkdir -p "$NOA_BIN" "$PROTOBUF_ROOT" "$TEMP_DIR"

# Download
log_info "Downloading protoc v$PROTOC_VERSION..."
ARCHIVE_PATH="$TEMP_DIR/$ARCHIVE_NAME"

if command -v curl &>/dev/null; then
    curl -fsSL -o "$ARCHIVE_PATH" "$DOWNLOAD_URL"
elif command -v wget &>/dev/null; then
    wget -q -O "$ARCHIVE_PATH" "$DOWNLOAD_URL"
else
    log_error "Neither curl nor wget found"
    exit 1
fi

# Extract
log_info "Extracting..."
EXTRACT_DIR=$(mktemp -d)
unzip -q "$ARCHIVE_PATH" -d "$EXTRACT_DIR"

# Install binary
cp "$EXTRACT_DIR/bin/protoc" "$NOA_BIN/protoc"
chmod +x "$NOA_BIN/protoc"

# Install includes
cp -r "$EXTRACT_DIR/include" "$PROTOBUF_ROOT/"

# Cleanup
rm -rf "$EXTRACT_DIR"

VERSION=$("$NOA_BIN/protoc" --version | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1)
log_success "Installed: protoc v$VERSION"
echo "Binary: $NOA_BIN/protoc"
echo "Includes: $PROTOBUF_ROOT/include/"

