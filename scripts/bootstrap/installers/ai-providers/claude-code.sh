#!/bin/bash
#
# Install Claude Code CLI to noa_root/opt/
#
# Downloads and installs the Claude Code CLI (@anthropic-ai/claude-code) to the NOA
# portable environment. Supports npm installation and native installer methods.
#
# Repository: https://github.com/FlexNetOS/claude-code.git
# Provider configs: ai/providers/cloud/claude-code/configs.json
#
# Usage:
#   ./claude-code.sh                    # Install via npm (default)
#   ./claude-code.sh --method native    # Use official installer
#   ./claude-code.sh --method clone     # Clone FlexNetOS fork
#   ./claude-code.sh --noa-root /path   # Specify NOA root
#

set -euo pipefail

# Default values
NOA_ROOT="${NOA_ROOT:-$HOME/noa}"
METHOD="npm"
CLAUDE_VERSION="latest"
NPM_PACKAGE="@anthropic-ai/claude-code"
GIT_REPO="https://github.com/FlexNetOS/claude-code.git"

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
            echo "Usage: $0 [--noa-root PATH] [--method npm|native|clone]"
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
PROVIDER_configs_PATH="$NOA_ROOT/ai/providers/cloud/claude-code"
OPT_PATH="$NOA_ROOT/opt"
BIN_PATH="$NOA_ROOT/bin"
NODE_PATH="$OPT_PATH/node"

log_info "Installing Claude Code CLI..."
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
        export npm_configs_prefix="$NODE_PATH"

        # Install claude-code
        $NPM_CMD install -g "$NPM_PACKAGE" || {
            log_error "npm install failed"
            exit 1
        }

        # Create symlink in bin/
        CLAUDE_BIN="$NODE_PATH/bin/claude"
        if [[ -f "$CLAUDE_BIN" ]]; then
            ln -sf "$CLAUDE_BIN" "$BIN_PATH/claude"
            log_success "  Created: bin/claude"
        fi
        ;;

    native)
        log_info "Installing via native installer..."

        # Download and run official installer
        curl -fsSL https://claude.ai/install.sh | bash || {
            log_error "Failed to run native installer"
            exit 1
        }
        ;;

    clone)
        log_info "Cloning FlexNetOS fork..."

        CLONE_PATH="$OPT_PATH/claude-code"

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
if command -v claude &> /dev/null; then
    VERSION=$(claude --version 2>&1 || echo "unknown")
    log_success "  Claude Code installed: $VERSION"
else
    # Check portable location
    PORTABLE_CLAUDE="$NODE_PATH/bin/claude"
    if [[ -x "$PORTABLE_CLAUDE" ]]; then
        log_success "  Claude Code installed to portable location"
        log_info "  Add to PATH: $NODE_PATH/bin"
    else
        log_warning "  Claude Code not found in PATH"
    fi
fi

log_success "Claude Code installation complete!"
log_info "  Provider configs: $PROVIDER_configs_PATH"
log_info "  Shared resources: $NOA_ROOT/ai/shared"

exit 0

