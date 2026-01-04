#!/bin/bash
#
# Install Python to noa_root/opt/python/ and venv to opt/venv/
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
PYTHON_VERSION="${PYTHON_VERSION:-3.12.8}"
FORCE="${1:-}"

# Paths
PYTHON_ROOT="$NOA_ROOT/opt/python"
VENV_ROOT="$NOA_ROOT/opt/venv"
PIP_CACHE="$NOA_ROOT/cache/pip"
TEMP_DIR="$NOA_ROOT/tmp"
STATE_FILE="$PYTHON_ROOT/.installed.json"

log_section "NOA Portable Python Installer"
echo "NOA_ROOT:    $NOA_ROOT"
echo "PYTHON_ROOT: $PYTHON_ROOT"
echo "VENV_ROOT:   $VENV_ROOT"
echo "Version:     $PYTHON_VERSION"
echo ""

# Check if already installed
if [[ -f "$STATE_FILE" && -x "$VENV_ROOT/bin/python" && "$FORCE" != "--force" ]]; then
    VERSION=$("$VENV_ROOT/bin/python" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    log_success "Python already installed: v$VERSION"
    log_info "Use --force to reinstall"
    echo ""
    echo "Add to your shell profile:"
    echo "  export VIRTUAL_ENV=\"$VENV_ROOT\""
    echo "  export PATH=\"$VENV_ROOT/bin:\$PATH\""
    exit 0
fi

# Check for system Python first (we'll use it to bootstrap)
SYSTEM_PYTHON=""
for py in python3.12 python3.11 python3.10 python3; do
    if command -v "$py" &>/dev/null; then
        PY_VERSION=$("$py" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+' | head -1)
        PY_MAJOR=$(echo "$PY_VERSION" | cut -d. -f1)
        PY_MINOR=$(echo "$PY_VERSION" | cut -d. -f2)
        if [[ "$PY_MAJOR" -ge 3 && "$PY_MINOR" -ge 10 ]]; then
            SYSTEM_PYTHON="$py"
            break
        fi
    fi
done

if [[ -z "$SYSTEM_PYTHON" ]]; then
    log_error "Python 3.10+ is required but not found"
    echo ""
    echo "Please install Python using your system package manager:"
    os=$(get_os 2>/dev/null || echo "linux")
    case "$os" in
        macos)
            echo "  brew install python@3.12"
            ;;
        linux|wsl1|wsl2)
            echo "  # Debian/Ubuntu:"
            echo "  sudo apt-get update && sudo apt-get install -y python3 python3-venv python3-pip"
            echo ""
            echo "  # Fedora:"
            echo "  sudo dnf install -y python3 python3-pip"
            ;;
    esac
    exit 1
fi

log_info "Using system Python: $SYSTEM_PYTHON"

# Remove existing installation if forcing
if [[ "$FORCE" == "--force" ]]; then
    log_info "Removing existing installation..."
    rm -rf "$VENV_ROOT"
fi

# Create directories
mkdir -p "$PYTHON_ROOT" "$PIP_CACHE" "$TEMP_DIR"

# Create virtual environment
log_info "Creating virtual environment..."
"$SYSTEM_PYTHON" -m venv "$VENV_ROOT"

# Verify venv
if [[ ! -x "$VENV_ROOT/bin/python" ]]; then
    log_error "Failed to create virtual environment"
    exit 1
fi

# Activate and upgrade pip
log_info "Upgrading pip..."
"$VENV_ROOT/bin/python" -m pip install --upgrade pip setuptools wheel --quiet

VERSION=$("$VENV_ROOT/bin/python" --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
PIP_VERSION=$("$VENV_ROOT/bin/pip" --version | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1)

log_success "Installed: Python v$VERSION"
log_success "pip version: $PIP_VERSION"

# configsure pip
"$VENV_ROOT/bin/pip" configs set global.cache-dir "$PIP_CACHE" 2>/dev/null || true

# Save state
cat > "$STATE_FILE" <<EOF
{
    "python_version": "$VERSION",
    "pip_version": "$PIP_VERSION",
    "installed_at": "$(date -Iseconds)",
    "venv_root": "$VENV_ROOT",
    "pip_cache": "$PIP_CACHE",
    "system_python": "$SYSTEM_PYTHON"
}
EOF

log_success "Installation state saved"

echo ""
echo "============================================================"
echo " Python virtual environment created successfully!"
echo "============================================================"
echo ""
echo "Add to your shell profile:"
echo "  export VIRTUAL_ENV=\"$VENV_ROOT\""
echo "  export PIP_CACHE_DIR=\"$PIP_CACHE\""
echo "  export PATH=\"$VENV_ROOT/bin:\$PATH\""

