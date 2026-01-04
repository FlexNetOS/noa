#!/bin/bash
#
# VS Code with Copilot installer for NOA bootstrap (Unix)
#
# Installs VS Code portable and GitHub Copilot extensions.
# Per NOA Constitution §3.3: Agentic Orchestration

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")")}"

# Source libraries
[[ -f "$SCRIPT_DIR/../../lib/logging.sh" ]] && source "$SCRIPT_DIR/../../lib/logging.sh"
[[ -f "$SCRIPT_DIR/../../lib/platform.sh" ]] && source "$SCRIPT_DIR/../../lib/platform.sh"

# Paths
NOA_BIN="$NOA_ROOT/bin"
VSCODE_ROOT="$NOA_ROOT/opt/dev-tools/vscode"
PROVIDER_configs="$NOA_ROOT/ai/providers/ide/vscode-copilot/configs.json"
FORCE="${1:-}"

log_section "NOA VS Code with Copilot Setup"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

mkdir -p "$NOA_BIN" "$VSCODE_ROOT" "$(dirname "$PROVIDER_configs")"

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

# Check if VS Code is already available
CODE_PATH=""

if [[ -x "$VSCODE_ROOT/bin/code" ]]; then
    CODE_PATH="$VSCODE_ROOT/bin/code"
elif command -v code &>/dev/null; then
    CODE_PATH=$(command -v code)
elif [[ -x "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code" ]]; then
    CODE_PATH="/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"
fi

if [[ -z "$CODE_PATH" || "$FORCE" == "--force" ]]; then
    log_info "Installing VS Code..."

    # Download URL based on platform
    case "$os" in
        macos)
            if [[ "$arch" == "arm64" ]]; then
                DOWNLOAD_URL="https://code.visualstudio.com/sha/download?build=stable&os=darwin-arm64"
            else
                DOWNLOAD_URL="https://code.visualstudio.com/sha/download?build=stable&os=darwin"
            fi
            ;;
        *)
            DOWNLOAD_URL="https://code.visualstudio.com/sha/download?build=stable&os=linux-x64"
            ;;
    esac

    TEMP_DIR="$NOA_ROOT/tmp"
    mkdir -p "$TEMP_DIR"

    case "$os" in
        macos)
            ARCHIVE_PATH="$TEMP_DIR/vscode.zip"
            curl -fsSL -o "$ARCHIVE_PATH" "$DOWNLOAD_URL"
            unzip -q "$ARCHIVE_PATH" -d "$TEMP_DIR"
            mv "$TEMP_DIR/Visual Studio Code.app" "$VSCODE_ROOT/VSCode.app"
            CODE_PATH="$VSCODE_ROOT/VSCode.app/Contents/Resources/app/bin/code"
            ;;
        *)
            ARCHIVE_PATH="$TEMP_DIR/vscode.tar.gz"
            curl -fsSL -o "$ARCHIVE_PATH" "$DOWNLOAD_URL"
            tar -xzf "$ARCHIVE_PATH" -C "$VSCODE_ROOT" --strip-components=1
            CODE_PATH="$VSCODE_ROOT/bin/code"
            ;;
    esac

    log_success "VS Code installed to: $VSCODE_ROOT"
fi

if [[ -n "$CODE_PATH" && -x "$CODE_PATH" ]]; then
    # Create portable mode marker
    mkdir -p "$VSCODE_ROOT/data"

    # Create symlink in NOA bin
    if [[ ! -L "$NOA_BIN/code" ]]; then
        ln -sf "$CODE_PATH" "$NOA_BIN/code"
        log_success "Created symlink: $NOA_BIN/code"
    fi

    # Install Copilot extensions
    log_info "Installing GitHub Copilot extensions..."

    "$CODE_PATH" --install-extension GitHub.copilot --force 2>/dev/null && \
        log_success "Installed: GitHub.copilot" || \
        log_warning "Failed to install GitHub.copilot (may require authentication)"

    "$CODE_PATH" --install-extension GitHub.copilot-chat --force 2>/dev/null && \
        log_success "Installed: GitHub.copilot-chat" || \
        log_warning "Failed to install GitHub.copilot-chat"

    # Verify
    VERSION=$("$CODE_PATH" --version 2>/dev/null | head -1 || echo "installed")
    log_info "VS Code version: $VERSION"

    EXTENSIONS=$("$CODE_PATH" --list-extensions 2>/dev/null | grep -i copilot || echo "none")
    log_info "Copilot extensions: $EXTENSIONS"
else
    log_warning "VS Code installation failed or not found"
    echo ""
    echo "Manual installation:"
    echo "  1. Download VS Code from: https://code.visualstudio.com"
    echo "  2. Install GitHub Copilot extension"
    echo "  3. Re-run this script"
fi

# Ensure provider configs exists
if [[ ! -f "$PROVIDER_configs" ]]; then
    cat > "$PROVIDER_configs" <<'EOF'
{
  "name": "vscode-copilot",
  "type": "ide",
  "priority": 5,
  "enabled": true,
  "description": "VS Code with GitHub Copilot - IDE-based AI assistance",
  "cli": {
    "command": "code",
    "version": "latest",
    "binaryPath": {
      "unix": "${NOA_ROOT}/bin/code"
    }
  },
  "extensions": [
    "GitHub.copilot",
    "GitHub.copilot-chat"
  ],
  "capabilities": {
    "codeCompletion": true,
    "inlineCompletion": true,
    "chat": true,
    "contextAware": true
  },
  "sharedResourcePath": "${NOA_ROOT}/ai/shared"
}
EOF
    log_success "Created provider configs: $PROVIDER_configs"
fi

echo ""
log_success "VS Code with Copilot setup complete!"

