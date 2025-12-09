#!/bin/bash
#
# Install Go tools (golangci-lint, etc.) via go install
#
# Per NOA Constitution 3.1: Self-contained installation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true

# Paths
GO_ROOT="$NOA_ROOT/opt/go"
GOPATH="$GO_ROOT/workspace"
GOBIN="$GOPATH/bin"
GOCACHE="$GO_ROOT/cache"
GOMODCACHE="$GO_ROOT/pkg/mod"

log_section "NOA Go Tools Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Check if Go is installed
if [[ ! -x "$GO_ROOT/bin/go" ]]; then
    log_error "Go is not installed. Run go-portable.sh first."
    exit 1
fi

# Set environment
export GOROOT="$GO_ROOT"
export GOPATH
export GOBIN
export GOCACHE
export GOMODCACHE
export PATH="$GO_ROOT/bin:$GOBIN:$PATH"

# Tools to install
declare -A GO_TOOLS=(
    ["golangci-lint"]="github.com/golangci/golangci-lint/cmd/golangci-lint@latest"
    ["gofumpt"]="mvdan.cc/gofumpt@latest"
    ["goimports"]="golang.org/x/tools/cmd/goimports@latest"
    ["staticcheck"]="honnef.co/go/tools/cmd/staticcheck@latest"
)

mkdir -p "$GOBIN"

for tool in "${!GO_TOOLS[@]}"; do
    package="${GO_TOOLS[$tool]}"
    log_info "Installing $tool..."

    if "$GO_ROOT/bin/go" install "$package" 2>/dev/null; then
        log_success "Installed: $tool"
    else
        log_warning "Failed to install $tool"
    fi
done

# Verify installations
echo ""
log_info "Installed Go tools:"
for tool in "${!GO_TOOLS[@]}"; do
    if [[ -x "$GOBIN/$tool" ]]; then
        version=$("$GOBIN/$tool" --version 2>/dev/null | head -1 || echo "installed")
        echo "  - $tool: $version"
    fi
done

echo ""
log_success "Go tools installation complete!"
echo "Tools installed to: $GOBIN"

