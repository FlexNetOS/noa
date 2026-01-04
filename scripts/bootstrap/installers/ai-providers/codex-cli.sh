#!/bin/bash
#
# Install Codex CLI to noa_root/opt/
#
# Downloads and installs the Codex CLI (@openai/codex) to the NOA
# portable environment. Supports npm installation and clone methods.
#
# Repository: https://github.com/FlexNetOS/codex.git
# Provider configs: ai/providers/cloud/codex/configs.json
#
# Usage:
#   ./codex-cli.sh                    # Install via npm (default)
#   ./codex-cli.sh --method clone     # Clone FlexNetOS fork
#   ./codex-cli.sh --noa-root /path   # Specify NOA root
#

set -euo pipefail

# Default values
NOA_ROOT="${NOA_ROOT:-$HOME/noa}"
METHOD="npm"
CODEX_VERSION="latest"
NPM_PACKAGE="@openai/codex"
NPM_PACKAGE_FALLBACK="codex-cli"
GIT_REPO="https://github.com/FlexNetOS/codex.git"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --noa-root)
            NOA_ROOT="$2"
            shift 2
            ;;
        --method)
            METHOD="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [--noa-root PATH] [--method npm|clone]"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Source logging library if available
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOGGING_LIB="$SCRIPT_DIR/../../lib/logging.sh"
if [[ -f "$LOGGING_LIB" ]]; then
    source "$LOGGING_LIB"
else
    log_info() { echo "[Info] $1"; }
    log_success() { echo "[Success] $1"; }
    log_warning() { echo "[Warning] $1"; }
    log_error() { echo "[Error] $1" >&2; }
fi

# configsuration
PROVIDER_configs_PATH="$NOA_ROOT/ai/providers/cloud/codex"
OPT_PATH="$NOA_ROOT/opt"
BIN_PATH="$NOA_ROOT/bin"
NODE_PATH="$OPT_PATH/node"
DEV_TOOLS_DIR="$OPT_PATH/dev-tools"

log_info "Installing Codex CLI..."
log_info "  Method: $METHOD"
log_info "  NOA Root: $NOA_ROOT"

# Ensure directories exist
for dir in "$OPT_PATH" "$BIN_PATH" "$PROVIDER_configs_PATH"; do
    if [[ ! -d "$dir" ]]; then
        mkdir -p "$dir"
        log_success "  Created: $dir"
    fi
done

case "$METHOD" in
    npm)
        log_info "Installing via npm..."

        # Check if portable Node.js exists
        if [[ -x "$NODE_PATH/bin/npm" ]]; then
            NPM_CMD="$NODE_PATH/bin/npm"
        elif command -v npm &> /dev/null; then
            NPM_CMD="npm"
        else
            log_error "npm not found. Please install Node.js first."
            exit 1
        fi

        # Set npm prefix to install globally within noa_root
        NPM_PREFIX="$DEV_TOOLS_DIR/npm-global"
        NPM_CACHE="$OPT_PATH/npm-cache"
        mkdir -p "$NPM_PREFIX" "$NPM_CACHE"
        export npm_configs_prefix="$NPM_PREFIX"
        export npm_configs_cache="$NPM_CACHE"

        # Try @openai/codex first, fallback to codex-cli
        $NPM_CMD install -g "$NPM_PACKAGE" 2>/dev/null || $NPM_CMD install -g "$NPM_PACKAGE_FALLBACK" || {
            log_error "npm install failed"
            exit 1
        }

        # Create symlink in bin/
        CODEX_BIN="$NPM_PREFIX/bin/codex"
        if [[ -f "$CODEX_BIN" ]]; then
            ln -sf "$CODEX_BIN" "$BIN_PATH/codex"
            log_success "  Created: bin/codex"
        fi
        ;;

    clone)
        log_info "Cloning FlexNetOS fork..."

        CLONE_PATH="$OPT_PATH/codex"

        if [[ -d "$CLONE_PATH" ]]; then
            log_info "  Directory exists, pulling latest..."
            cd "$CLONE_PATH"
            git pull
        else
            git clone "$GIT_REPO" "$CLONE_PATH" || {
                log_error "git clone failed"
                exit 1
            }
        fi

        # Install dependencies
        cd "$CLONE_PATH"
        npm install

        log_success "  Cloned to: $CLONE_PATH"
        ;;

    *)
        log_error "Unknown method: $METHOD"
        exit 1
        ;;
esac

# Verify installation
log_info "Verifying installation..."
if command -v codex &> /dev/null; then
    VERSION=$(codex --version 2>&1 || echo "unknown")
    log_success "  Codex CLI installed: $VERSION"
else
    # Check portable location
    PORTABLE_CODEX="$DEV_TOOLS_DIR/npm-global/bin/codex"
    if [[ -x "$PORTABLE_CODEX" ]]; then
        log_success "  Codex CLI installed to portable location"
        log_info "  Add to PATH: $(dirname "$PORTABLE_CODEX")"
    else
        log_warning "  Codex CLI not found in PATH"
    fi
fi

log_success "Codex CLI installation complete!"
log_info "  Provider configs: $PROVIDER_configs_PATH"
log_info "  Shared resources: $NOA_ROOT/ai/shared"

exit 0

