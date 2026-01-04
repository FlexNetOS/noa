#!/bin/bash
#
# Abacus AI CLI installer for NOA bootstrap (Unix)
#
# Installs @abacus-ai/cli via npm.
# Per NOA Constitution §3.3: Agentic Orchestration

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")")}"

# Source libraries
[[ -f "$SCRIPT_DIR/../../lib/logging.sh" ]] && source "$SCRIPT_DIR/../../lib/logging.sh"

# Paths
NODE_ROOT="$NOA_ROOT/opt/node"
NOA_BIN="$NOA_ROOT/bin"
PROVIDER_configs="$NOA_ROOT/ai/providers/cloud/abacus/configs.json"
FORCE="${1:-}"

log_section "NOA Abacus AI CLI Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

mkdir -p "$NOA_BIN" "$(dirname "$PROVIDER_configs")"

# Check if Node.js is available
NPM_PATH=""
if [[ -x "$NODE_ROOT/bin/npm" ]]; then
    NPM_PATH="$NODE_ROOT/bin/npm"
elif command -v npm &>/dev/null; then
    NPM_PATH=$(command -v npm)
fi

if [[ -z "$NPM_PATH" ]]; then
    log_error "npm is not installed"
    echo "Please install Node.js first: ./node-portable.sh"
    exit 1
fi

# Set environment
export PATH="$NODE_ROOT/bin:$PATH"
export npm_configs_cache="$NOA_ROOT/opt/npm-cache"
export npm_configs_prefix="$NODE_ROOT"

# Check if already installed
ABACUS_PATH="$NODE_ROOT/bin/abacusai"
if [[ -x "$ABACUS_PATH" && "$FORCE" != "--force" ]]; then
    VERSION=$("$ABACUS_PATH" --version 2>/dev/null || echo "installed")
    log_success "Abacus AI CLI already installed: $VERSION"
    log_info "Use --force to reinstall"
    exit 0
fi

# Install via npm
log_info "Installing @abacus-ai/cli..."
"$NPM_PATH" install -g @abacus-ai/cli --silent 2>/dev/null && \
    log_success "Installed: @abacus-ai/cli" || \
    log_error "Failed to install @abacus-ai/cli"

# Verify installation
if [[ -x "$ABACUS_PATH" ]]; then
    VERSION=$("$ABACUS_PATH" --version 2>/dev/null || echo "installed")
    log_success "Abacus AI CLI version: $VERSION"

    # Create symlink
    if [[ ! -L "$NOA_BIN/abacusai" ]]; then
        ln -sf "$ABACUS_PATH" "$NOA_BIN/abacusai"
        log_success "Created symlink: $NOA_BIN/abacusai"
    fi
else
    log_warning "Abacus AI CLI not found after installation"
    echo ""
    echo "You may need to install it manually:"
    echo "  npm install -g @abacus-ai/cli"
    echo ""
    echo "Note: Abacus CLI requires Abacus Desktop app for authentication"
    echo "Download from: https://desktop.abacus.ai/"
fi

# Update provider configs
if [[ -f "$PROVIDER_configs" ]]; then
    log_info "Provider configs already exists: $PROVIDER_configs"
else
    log_info "Provider configs should be at: $PROVIDER_configs"
fi

echo ""
log_success "Abacus AI CLI setup complete!"
echo ""
echo "Note: Authentication requires Abacus Desktop app"
echo "Download from: https://desktop.abacus.ai/"

