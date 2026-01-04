#!/bin/bash
#
# Install Go toolchain to noa_root/opt/go/
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
GO_VERSION="${GO_VERSION:-1.23.4}"
FORCE="${1:-}"

# Paths
GO_ROOT="$NOA_ROOT/opt/go"
GOPATH="$GO_ROOT/workspace"
GOBIN="$GOPATH/bin"
GOCACHE="$GO_ROOT/cache"
GOMODCACHE="$GO_ROOT/pkg/mod"
TEMP_DIR="$NOA_ROOT/tmp"
STATE_FILE="$GO_ROOT/.installed.json"

log_section "NOA Portable Go Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo "GOROOT:   $GO_ROOT"
echo "GOPATH:   $GOPATH"
echo "Version:  $GO_VERSION"
echo ""

# Check if already installed
if [[ -f "$STATE_FILE" && -x "$GO_ROOT/bin/go" && "$FORCE" != "--force" ]]; then
    export GOROOT="$GO_ROOT"
    VERSION=$("$GO_ROOT/bin/go" version 2>/dev/null | grep -oE 'go[0-9]+\.[0-9]+(\.[0-9]+)?' | sed 's/go//' | head -1)
    log_success "Go already installed: v$VERSION"
    log_info "Use --force to reinstall"
    echo ""
    echo "Add to your shell profile:"
    echo "  export GOROOT=\"$GO_ROOT\""
    echo "  export GOPATH=\"$GOPATH\""
    echo "  export PATH=\"$GO_ROOT/bin:$GOBIN:\$PATH\""
    exit 0
fi

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

# Construct download URL
case "$os" in
    macos)
        if [[ "$arch" == "arm64" ]]; then
            ARCHIVE_NAME="go${GO_VERSION}.darwin-arm64.tar.gz"
        else
            ARCHIVE_NAME="go${GO_VERSION}.darwin-amd64.tar.gz"
        fi
        ;;
    *)
        ARCHIVE_NAME="go${GO_VERSION}.linux-amd64.tar.gz"
        ;;
esac

DOWNLOAD_URL="https://go.dev/dl/${ARCHIVE_NAME}"

# Remove existing installation if forcing
if [[ "$FORCE" == "--force" && -d "$GO_ROOT" ]]; then
    log_info "Removing existing installation..."
    rm -rf "$GO_ROOT"
fi

# Create directories
mkdir -p "$GO_ROOT" "$GOPATH" "$GOBIN" "$GOCACHE" "$GOMODCACHE" "$TEMP_DIR"

# Download
log_info "Downloading Go $GO_VERSION..."
ARCHIVE_PATH="$TEMP_DIR/$ARCHIVE_NAME"

if command -v curl &>/dev/null; then
    curl -fsSL -o "$ARCHIVE_PATH" "$DOWNLOAD_URL"
elif command -v wget &>/dev/null; then
    wget -q -O "$ARCHIVE_PATH" "$DOWNLOAD_URL"
else
    log_error "Neither curl nor wget found"
    exit 1
fi

# Extract (strip leading 'go/' directory)
log_info "Extracting..."
tar -xzf "$ARCHIVE_PATH" -C "$NOA_ROOT/opt"
# The archive extracts to 'go/' so it should be in the right place

# Verify installation
if [[ ! -x "$GO_ROOT/bin/go" ]]; then
    log_error "Installation failed - go binary not found"
    exit 1
fi

export GOROOT="$GO_ROOT"
export GOPATH
export GOBIN
export GOCACHE
export GOMODCACHE

VERSION=$("$GO_ROOT/bin/go" version | grep -oE 'go[0-9]+\.[0-9]+(\.[0-9]+)?' | sed 's/go//' | head -1)
log_success "Installed: Go v$VERSION"

# Save state
cat > "$STATE_FILE" <<EOF
{
    "version": "$VERSION",
    "installed_at": "$(date -Iseconds)",
    "goroot": "$GO_ROOT",
    "gopath": "$GOPATH",
    "gobin": "$GOBIN"
}
EOF

log_success "Installation state saved"

echo ""
echo "============================================================"
echo " Go toolchain installed successfully!"
echo "============================================================"
echo ""
echo "Add to your shell profile:"
echo "  export GOROOT=\"$GO_ROOT\""
echo "  export GOPATH=\"$GOPATH\""
echo "  export GOBIN=\"$GOBIN\""
echo "  export GOCACHE=\"$GOCACHE\""
echo "  export GOMODCACHE=\"$GOMODCACHE\""
echo "  export PATH=\"$GO_ROOT/bin:$GOBIN:\$PATH\""

