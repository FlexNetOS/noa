#!/bin/bash
#
# Git installer for NOA bootstrap (Unix)
#
# Verifies git is available or provides installation instructions.
# Git is typically pre-installed on Unix systems.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/platform.sh" 2>/dev/null || true

# Minimum required version
MIN_GIT_VERSION="2.30.0"

log_section "NOA Git Verification"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Check if git is available
if ! command -v git &>/dev/null; then
    log_error "Git is not installed"
    echo ""
    echo "Please install Git using your system package manager:"
    echo ""

    os=$(get_os 2>/dev/null || echo "linux")
    case "$os" in
        macos)
            echo "  brew install git"
            echo "  # or"
            echo "  xcode-select --install"
            ;;
        linux|wsl1|wsl2)
            echo "  # Debian/Ubuntu:"
            echo "  sudo apt-get update && sudo apt-get install -y git"
            echo ""
            echo "  # Fedora:"
            echo "  sudo dnf install -y git"
            echo ""
            echo "  # Arch:"
            echo "  sudo pacman -S git"
            ;;
        *)
            echo "  Please install Git from: https://git-scm.com/downloads"
            ;;
    esac
    exit 1
fi

# Get installed version
INSTALLED_VERSION=$(git --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
log_success "Git is installed: v$INSTALLED_VERSION"

# Version comparison
version_gte() {
    printf '%s\n%s' "$2" "$1" | sort -V -C
}

if version_gte "$INSTALLED_VERSION" "$MIN_GIT_VERSION"; then
    log_success "Version $INSTALLED_VERSION >= $MIN_GIT_VERSION (required)"
else
    log_warning "Version $INSTALLED_VERSION < $MIN_GIT_VERSION (recommended)"
    echo "Consider updating Git for best compatibility."
fi

# Check git location
GIT_PATH=$(command -v git)
echo ""
echo "Git location: $GIT_PATH"

# Create symlink in NOA bin if needed
NOA_BIN="$NOA_ROOT/bin"
mkdir -p "$NOA_BIN"

if [[ ! -L "$NOA_BIN/git" && ! -f "$NOA_BIN/git" ]]; then
    ln -sf "$GIT_PATH" "$NOA_BIN/git"
    log_success "Created symlink: $NOA_BIN/git -> $GIT_PATH"
fi

echo ""
log_success "Git verification complete!"

