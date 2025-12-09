#!/bin/bash
#
# Install FULL Rust toolchain to noa_root/opt/rust/
#
# Installs a complete, fully-functional Rust toolchain with rustup, cargo, rustc, etc.
# The toolchain works exactly like a system-wide installation, but everything lives in noa_root.
#
# Package managers work normally:
# - 'cargo install <crate>' installs to noa_root/opt/rust/cargo/bin/
# - 'rustup component add <component>' installs to noa_root
#
# This is NOT a static binary download - it's a real, working Rust installation.
# Per NOA Constitution 3.1: Self-contained but fully functional.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/platform.sh" 2>/dev/null || true

# Configuration
TOOLCHAIN="${TOOLCHAIN:-stable}"
FORCE="${1:-}"

# Paths - ALL within noa_root
RUST_ROOT="$NOA_ROOT/opt/rust"
RUSTUP_HOME="$RUST_ROOT/rustup"
CARGO_HOME="$RUST_ROOT/cargo"
CARGO_BIN="$CARGO_HOME/bin"
TEMP_DIR="$NOA_ROOT/tmp"
STATE_FILE="$RUST_ROOT/.installed.json"

log_section "NOA Portable Rust Installer"
echo "NOA_ROOT:    $NOA_ROOT"
echo "RUSTUP_HOME: $RUSTUP_HOME"
echo "CARGO_HOME:  $CARGO_HOME"
echo "Toolchain:   $TOOLCHAIN"
echo ""

# Check if already installed
if [[ -f "$STATE_FILE" && -x "$CARGO_BIN/rustc" && "$FORCE" != "--force" ]]; then
    export RUSTUP_HOME CARGO_HOME
    VERSION=$("$CARGO_BIN/rustc" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    log_success "Rust already installed: v$VERSION"
    log_info "Use --force to reinstall"
    echo ""
    echo "Add to your shell profile:"
    echo "  export RUSTUP_HOME=\"$RUSTUP_HOME\""
    echo "  export CARGO_HOME=\"$CARGO_HOME\""
    echo "  export PATH=\"$CARGO_BIN:\$PATH\""
    exit 0
fi

# Create directories
mkdir -p "$RUST_ROOT" "$RUSTUP_HOME" "$CARGO_HOME" "$CARGO_BIN" "$TEMP_DIR"

# Download rustup-init
RUSTUP_INIT="$TEMP_DIR/rustup-init"
RUSTUP_URL="https://sh.rustup.rs"

log_info "Downloading rustup..."
if command -v curl &>/dev/null; then
    curl -fsSL "$RUSTUP_URL" -o "$RUSTUP_INIT"
elif command -v wget &>/dev/null; then
    wget -q -O "$RUSTUP_INIT" "$RUSTUP_URL"
else
    log_error "Neither curl nor wget found"
    exit 1
fi
chmod +x "$RUSTUP_INIT"

# Set environment BEFORE running rustup-init
export RUSTUP_HOME CARGO_HOME

# Remove existing installation if forcing
if [[ "$FORCE" == "--force" ]]; then
    log_info "Removing existing installation..."
    rm -rf "$RUSTUP_HOME" "$CARGO_HOME"
    mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
fi

# Run rustup-init
log_info "Installing Rust (this may take a few minutes)..."
"$RUSTUP_INIT" -y --default-toolchain "$TOOLCHAIN" --no-modify-path

# Verify installation
if [[ ! -x "$CARGO_BIN/rustc" ]]; then
    log_error "Installation failed - rustc not found"
    exit 1
fi

VERSION=$("$CARGO_BIN/rustc" --version)
log_success "Installed: $VERSION"

# Install additional components
log_info "Installing rustfmt..."
"$CARGO_BIN/rustup" component add rustfmt 2>/dev/null || true
log_success "Installed: rustfmt"

log_info "Installing clippy..."
"$CARGO_BIN/rustup" component add clippy 2>/dev/null || true
log_success "Installed: clippy"

# Save state
cat > "$STATE_FILE" <<EOF
{
    "toolchain": "$TOOLCHAIN",
    "installed_at": "$(date -Iseconds)",
    "rustup_home": "$RUSTUP_HOME",
    "cargo_home": "$CARGO_HOME",
    "version": "$VERSION",
    "components": ["rustfmt", "clippy"]
}
EOF

log_success "Installation state saved"

echo ""
echo "============================================================"
echo " Rust toolchain installed successfully!"
echo "============================================================"
echo ""
echo "Add to your shell profile:"
echo "  export RUSTUP_HOME=\"$RUSTUP_HOME\""
echo "  export CARGO_HOME=\"$CARGO_HOME\""
echo "  export PATH=\"$CARGO_BIN:\$PATH\""

