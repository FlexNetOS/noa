#!/bin/bash
#
# Install npm tools (eslint, etc.) via npm
#
# Per NOA Constitution 3.1: Self-contained installation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true

# Paths
NODE_ROOT="$NOA_ROOT/opt/node"
NPM_CACHE="$NOA_ROOT/opt/npm-cache"

log_section "NOA npm Tools Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Check if Node.js is installed
if [[ ! -x "$NODE_ROOT/bin/npm" ]]; then
    log_error "Node.js is not installed. Run node-portable.sh first."
    exit 1
fi

# Set environment
export PATH="$NODE_ROOT/bin:$PATH"
export npm_config_cache="$NPM_CACHE"
export npm_config_prefix="$NODE_ROOT"

# Tools to install
NPM_TOOLS=(
    "eslint"
    "prettier"
    "typescript"
    "ts-node"
)

for tool in "${NPM_TOOLS[@]}"; do
    log_info "Installing $tool..."

    if "$NODE_ROOT/bin/npm" install -g "$tool" --silent 2>/dev/null; then
        log_success "Installed: $tool"
    else
        log_warning "Failed to install $tool"
    fi
done

# Verify installations
echo ""
log_info "Installed npm tools:"
for tool in "${NPM_TOOLS[@]}"; do
    tool_path="$NODE_ROOT/bin/$tool"
    if [[ -x "$tool_path" ]]; then
        version=$("$tool_path" --version 2>/dev/null | head -1 || echo "installed")
        echo "  - $tool: $version"
    fi
done

echo ""
log_success "npm tools installation complete!"
echo "Tools installed to: $NODE_ROOT/bin/"

