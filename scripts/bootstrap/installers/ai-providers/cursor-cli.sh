#!/bin/bash
#
# Cursor CLI installer for NOA bootstrap (Unix)
#
# Cursor CLI requires manual download from cursor.com
# This script provides instructions and symlink setup.
#
# Per NOA Constitution §3.3: Agentic Orchestration

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")")}"

# Source libraries
[[ -f "$SCRIPT_DIR/../../lib/logging.sh" ]] && source "$SCRIPT_DIR/../../lib/logging.sh"
[[ -f "$SCRIPT_DIR/../../lib/platform.sh" ]] && source "$SCRIPT_DIR/../../lib/platform.sh"

# Paths
NOA_BIN="$NOA_ROOT/bin"
CURSOR_CLI_ROOT="$NOA_ROOT/opt/cursor-cli"
PROVIDER_configs="$NOA_ROOT/ai/providers/hybrid/cursor/configs.json"

log_section "NOA Cursor CLI Setup"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

mkdir -p "$NOA_BIN" "$CURSOR_CLI_ROOT" "$(dirname "$PROVIDER_configs")"

# Check if Cursor is already available
CURSOR_PATH=""

# Check common locations
if command -v cursor &>/dev/null; then
    CURSOR_PATH=$(command -v cursor)
elif [[ -x "/Applications/Cursor.app/Contents/MacOS/Cursor" ]]; then
    CURSOR_PATH="/Applications/Cursor.app/Contents/MacOS/Cursor"
elif [[ -x "$HOME/.local/share/cursor/cursor" ]]; then
    CURSOR_PATH="$HOME/.local/share/cursor/cursor"
fi

if [[ -n "$CURSOR_PATH" ]]; then
    log_success "Cursor found at: $CURSOR_PATH"

    # Create symlink
    if [[ ! -L "$NOA_BIN/cursor" ]]; then
        ln -sf "$CURSOR_PATH" "$NOA_BIN/cursor"
        log_success "Created symlink: $NOA_BIN/cursor"
    fi

    # Try to get version
    VERSION=$("$CURSOR_PATH" --version 2>/dev/null | head -1 || echo "installed")
    log_info "Cursor version: $VERSION"
else
    log_warning "Cursor is not installed"
    echo ""
    echo "Please install Cursor from: https://cursor.com"
    echo ""
    echo "After installation:"
    os=$(get_os 2>/dev/null || echo "linux")
    case "$os" in
        macos)
            echo "  Cursor.app will be available at /Applications/Cursor.app"
            echo "  Re-run this script to create the symlink"
            ;;
        linux|wsl1|wsl2)
            echo "  Download the AppImage from cursor.com"
            echo "  Make it executable: chmod +x cursor.AppImage"
            echo "  Move to: $CURSOR_CLI_ROOT/cursor"
            echo "  Re-run this script to create the symlink"
            ;;
    esac
    echo ""
    echo "For headless CLI usage, see: https://cursor.com/docs/cli/headless"
fi

# Ensure provider configs exists
if [[ ! -f "$PROVIDER_configs" ]]; then
    cat > "$PROVIDER_configs" <<'EOF'
{
  "name": "cursor",
  "type": "hybrid",
  "priority": 2,
  "enabled": true,
  "description": "Cursor IDE CLI - AI-powered code editor with IDE context awareness",
  "cli": {
    "command": "cursor",
    "version": "latest",
    "binaryPath": {
      "unix": "${NOA_ROOT}/bin/cursor"
    }
  },
  "modes": {
    "cli": {
      "description": "Headless CLI mode for CI/CD and automation",
      "command": "cursor",
      "flags": ["--headless", "--print", "--output-format"],
      "documentation": "https://cursor.com/docs/cli/headless"
    }
  },
  "capabilities": {
    "textGeneration": true,
    "codeGeneration": true,
    "codeCompletion": true,
    "fileOperations": true,
    "gitOperations": true,
    "streaming": true,
    "contextAware": true,
    "ideIntegration": true
  },
  "sharedResourcePath": "${NOA_ROOT}/ai/shared"
}
EOF
    log_success "Created provider configs: $PROVIDER_configs"
fi

echo ""
log_success "Cursor CLI setup complete!"

