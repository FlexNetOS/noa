#!/bin/bash
#
# Install Rust tools (rustfmt, clippy) via rustup
#
# Per NOA Constitution 3.1: Self-contained installation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true

# Paths
RUST_ROOT="$NOA_ROOT/opt/rust"
RUSTUP_HOME="$RUST_ROOT/rustup"
CARGO_HOME="$RUST_ROOT/cargo"
CARGO_BIN="$CARGO_HOME/bin"

log_section "NOA Rust Tools Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Check if Rust is installed
if [[ ! -x "$CARGO_BIN/rustup" ]]; then
    log_error "Rust is not installed. Run rust-portable.sh first."
    exit 1
fi

# Set environment
export RUSTUP_HOME CARGO_HOME

# Install rustfmt
log_info "Installing rustfmt..."
"$CARGO_BIN/rustup" component add rustfmt 2>/dev/null && \
    log_success "Installed: rustfmt" || \
    log_warning "rustfmt may already be installed"

# Install clippy
log_info "Installing clippy..."
"$CARGO_BIN/rustup" component add clippy 2>/dev/null && \
    log_success "Installed: clippy" || \
    log_warning "clippy may already be installed"

# Install rust-analyzer (optional LSP)
log_info "Installing rust-analyzer..."
"$CARGO_BIN/rustup" component add rust-analyzer 2>/dev/null && \
    log_success "Installed: rust-analyzer" || \
    log_warning "rust-analyzer may already be installed or unavailable"

# Verify installations
echo ""
log_info "Installed Rust components:"
"$CARGO_BIN/rustup" component list --installed 2>/dev/null | while read -r component; do
    echo "  - $component"
done

echo ""
log_success "Rust tools installation complete!"

