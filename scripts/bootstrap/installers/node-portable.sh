#!/bin/bash
#
# Install Node.js to noa_root/opt/node/
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
NODE_VERSION="${NODE_VERSION:-22.11.0}"
FORCE="${1:-}"

# Paths
NODE_ROOT="$NOA_ROOT/opt/node"
NPM_CACHE="$NOA_ROOT/opt/npm-cache"
TEMP_DIR="$NOA_ROOT/tmp"
STATE_FILE="$NODE_ROOT/.installed.json"

log_section "NOA Portable Node.js Installer"
echo "NOA_ROOT:  $NOA_ROOT"
echo "NODE_ROOT: $NODE_ROOT"
echo "Version:   $NODE_VERSION"
echo ""

# Check if already installed
if [[ -f "$STATE_FILE" && -x "$NODE_ROOT/bin/node" && "$FORCE" != "--force" ]]; then
    VERSION=$("$NODE_ROOT/bin/node" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    log_success "Node.js already installed: v$VERSION"
    log_info "Use --force to reinstall"
    echo ""
    echo "Add to your shell profile:"
    echo "  export PATH=\"$NODE_ROOT/bin:\$PATH\""
    echo "  export npm_configs_cache=\"$NPM_CACHE\""
    exit 0
fi

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

# Map architecture
case "$arch" in
    amd64) NODE_ARCH="x64" ;;
    arm64) NODE_ARCH="arm64" ;;
    *) NODE_ARCH="x64" ;;
esac

# Construct download URL
case "$os" in
    macos)
        ARCHIVE_NAME="node-v${NODE_VERSION}-darwin-${NODE_ARCH}.tar.gz"
        ;;
    *)
        ARCHIVE_NAME="node-v${NODE_VERSION}-linux-${NODE_ARCH}.tar.xz"
        ;;
esac

DOWNLOAD_URL="https://nodejs.org/dist/v${NODE_VERSION}/${ARCHIVE_NAME}"

# Remove existing installation if forcing
if [[ "$FORCE" == "--force" && -d "$NODE_ROOT" ]]; then
    log_info "Removing existing installation..."
    rm -rf "$NODE_ROOT"
fi

# Create directories
mkdir -p "$NODE_ROOT" "$NPM_CACHE" "$TEMP_DIR"

# Download
log_info "Downloading Node.js v$NODE_VERSION..."
ARCHIVE_PATH="$TEMP_DIR/$ARCHIVE_NAME"

if command -v curl &>/dev/null; then
    curl -fsSL -o "$ARCHIVE_PATH" "$DOWNLOAD_URL"
elif command -v wget &>/dev/null; then
    wget -q -O "$ARCHIVE_PATH" "$DOWNLOAD_URL"
else
    log_error "Neither curl nor wget found"
    exit 1
fi

# Extract (strip leading directory)
log_info "Extracting..."
case "$ARCHIVE_NAME" in
    *.tar.xz)
        tar -xJf "$ARCHIVE_PATH" -C "$NODE_ROOT" --strip-components=1
        ;;
    *.tar.gz)
        tar -xzf "$ARCHIVE_PATH" -C "$NODE_ROOT" --strip-components=1
        ;;
esac

# Verify installation
if [[ ! -x "$NODE_ROOT/bin/node" ]]; then
    log_error "Installation failed - node binary not found"
    exit 1
fi

VERSION=$("$NODE_ROOT/bin/node" --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
log_success "Installed: Node.js v$VERSION"

NPM_VERSION=$("$NODE_ROOT/bin/npm" --version 2>/dev/null || echo "unknown")
log_success "npm version: $NPM_VERSION"

# configsure npm to use NOA directories
"$NODE_ROOT/bin/npm" configs set cache "$NPM_CACHE" --global 2>/dev/null || true
"$NODE_ROOT/bin/npm" configs set prefix "$NODE_ROOT" --global 2>/dev/null || true

# Save state
cat > "$STATE_FILE" <<EOF
{
    "node_version": "$VERSION",
    "npm_version": "$NPM_VERSION",
    "installed_at": "$(date -Iseconds)",
    "node_root": "$NODE_ROOT",
    "npm_cache": "$NPM_CACHE"
}
EOF

log_success "Installation state saved"

echo ""
echo "============================================================"
echo " Node.js installed successfully!"
echo "============================================================"
echo ""
echo "Add to your shell profile:"
echo "  export PATH=\"$NODE_ROOT/bin:\$PATH\""
echo "  export npm_configs_cache=\"$NPM_CACHE\""
echo "  export npm_configs_prefix=\"$NODE_ROOT\""

