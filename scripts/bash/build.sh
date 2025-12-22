#!/bin/bash
#
# NOA Build Script (Unix)
#
# Builds all NOA components: Rust, Go, TypeScript, Python
#
# Usage:
#   ./scripts/bash/build.sh [component]
#   Components: all, rust, p2p, ui, digest

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

build_rust() {
    log_info "Building Rust components (sys/core)..."
    cd "$NOA_ROOT/sys/core"
    if command -v cargo &> /dev/null; then
        cargo build --release
        log_success "Rust build complete"
    else
        log_warning "cargo not found, skipping Rust build"
    fi
}

build_p2p() {
    log_info "Building P2P components (rust-libp2p)..."
    cd "$NOA_ROOT/p2p"
    if command -v cargo &> /dev/null; then
        cargo build --release
        log_success "P2P build complete"
    else
        log_warning "cargo not found, skipping P2P build"
    fi
}

build_go() {
    log_warning "Go p2p-node has been replaced by rust-libp2p. Use 'p2p' component to build."
}

build_ui() {
    log_info "Building UI components..."
    cd "$NOA_ROOT/sys/ui"
    if command -v npm &> /dev/null; then
        npm install
        npm run build
        log_success "UI build complete"
    else
        log_warning "npm not found, skipping UI build"
    fi
}

build_digest() {
    log_info "Building Digest pipeline..."
    cd "$NOA_ROOT/sys/digest"
    if command -v pip &> /dev/null; then
        pip install -e .
        log_success "Digest build complete"
    else
        log_warning "pip not found, skipping Digest build"
    fi
}

build_all() {
    log_info "Building all NOA components..."
    build_rust
    build_p2p
    build_ui
    build_digest
    log_success "All builds complete!"
}

# Main
COMPONENT="${1:-all}"

case "$COMPONENT" in
    rust)   build_rust ;;
    p2p)    build_p2p ;;
    go)     build_go ;;
    ui)     build_ui ;;
    digest) build_digest ;;
    all)    build_all ;;
    *)
        log_error "Unknown component: $COMPONENT"
        echo "Usage: $0 [all|rust|p2p|go|ui|digest]"
        exit 1
        ;;
esac

