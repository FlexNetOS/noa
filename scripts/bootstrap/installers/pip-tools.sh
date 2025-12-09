#!/bin/bash
#
# Install pip tools (ruff, semgrep, etc.) in venv
#
# Per NOA Constitution 3.1: Self-contained installation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true

# Paths
VENV_ROOT="$NOA_ROOT/opt/venv"
PIP_CACHE="$NOA_ROOT/cache/pip"

log_section "NOA pip Tools Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Check if Python venv exists
if [[ ! -x "$VENV_ROOT/bin/pip" ]]; then
    log_error "Python venv not found. Run python-portable.sh first."
    exit 1
fi

# Set environment
export VIRTUAL_ENV="$VENV_ROOT"
export PIP_CACHE_DIR="$PIP_CACHE"
export PATH="$VENV_ROOT/bin:$PATH"

# Tools to install
PIP_TOOLS=(
    "ruff"
    "black"
    "mypy"
    "pytest"
    "semgrep"
)

mkdir -p "$PIP_CACHE"

for tool in "${PIP_TOOLS[@]}"; do
    log_info "Installing $tool..."

    if "$VENV_ROOT/bin/pip" install "$tool" --quiet 2>/dev/null; then
        log_success "Installed: $tool"
    else
        log_warning "Failed to install $tool"
    fi
done

# Verify installations
echo ""
log_info "Installed pip tools:"
for tool in "${PIP_TOOLS[@]}"; do
    tool_path="$VENV_ROOT/bin/$tool"
    if [[ -x "$tool_path" ]]; then
        version=$("$tool_path" --version 2>/dev/null | head -1 || echo "installed")
        echo "  - $tool: $version"
    fi
done

echo ""
log_success "pip tools installation complete!"
echo "Tools installed to: $VENV_ROOT/bin/"

