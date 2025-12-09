#!/bin/bash
#
# NOA Unified Bootstrap for Linux/WSL
# Single entry point for complete NOA environment setup
#
# Per NOA Constitution §3.1: Self-contained installation to noa_root
#
# Usage:
#   ./scripts/bootstrap/bootstrap.sh [options]
#
# Options:
#   --skip-kernel    Skip kernel module/param setup
#   --skip-services  Skip service setup (docker, ollama, etc.)
#   --force          Force reinstall all tools
#   --help           Show this help
#

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m'

# Auto-detect NOA_ROOT from script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

NOA_BIN="$NOA_ROOT/bin"
NOA_OPT="$NOA_ROOT/opt"
NOA_LIB="$NOA_ROOT/lib"
NOA_LOGS="$NOA_ROOT/logs"
NOA_SCRIPTS="$NOA_ROOT/scripts"

# Parse arguments
SKIP_KERNEL=false
SKIP_SERVICES=false
FORCE=false

for arg in "$@"; do
    case "$arg" in
        --skip-kernel) SKIP_KERNEL=true ;;
        --skip-services) SKIP_SERVICES=true ;;
        --force) FORCE=true ;;
        --help)
            echo "NOA Bootstrap for Linux/WSL"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --skip-kernel    Skip kernel module/param setup"
            echo "  --skip-services  Skip service setup (docker, ollama, etc.)"
            echo "  --force          Force reinstall all tools"
            echo "  --help           Show this help"
            exit 0
            ;;
    esac
done

# Detect platform
PLATFORM="linux"
IS_WSL=false
if [[ -f /proc/version ]] && grep -qi microsoft /proc/version 2>/dev/null; then
    IS_WSL=true
    if [[ -d /run/WSL ]]; then
        PLATFORM="wsl2"
    else
        PLATFORM="wsl1"
    fi
elif [[ "$(uname)" == "Darwin" ]]; then
    PLATFORM="macos"
fi

log() {
    local level="$1" msg="$2"
    local color prefix
    case "$level" in
        success) color="$GREEN"; prefix="[✓]" ;;
        warning) color="$YELLOW"; prefix="[!]" ;;
        error) color="$RED"; prefix="[✗]" ;;
        *) color="$NC"; prefix="[i]" ;;
    esac
    echo -e "${color}${prefix} ${msg}${NC}"
}

# ============================================
# Banner
# ============================================

echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}║           NOA Bootstrap for Linux/WSL                      ║${NC}"
echo -e "${CYAN}║           Constitution §3.1 Compliant                      ║${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "NOA_ROOT: $NOA_ROOT"
echo "Platform: $PLATFORM"
echo ""

# ============================================
# Phase 1: Directory Structure
# ============================================

log info "Phase 1: Creating directory structure..."

directories=(
    "bin" "opt" "lib" "etc" "tmp" "logs"
    "config" "repos" "workspace" "containers"
    "p2p/compute" "p2p/network" "p2p/storage" "p2p/nodes"
    "ai/providers" "ai/devices" "ai/shared/models"
    "sys/kernel/modules" "sys/kernel/params" "sys/namespace"
    "git" "init/run" "init/services"
)

for dir in "${directories[@]}"; do
    dirpath="$NOA_ROOT/$dir"
    if [[ ! -d "$dirpath" ]]; then
        mkdir -p "$dirpath"
        log success "Created: $dir"
    fi
done

# ============================================
# Phase 2: Check Prerequisites
# ============================================

log info "Phase 2: Checking prerequisites..."

PREREQS_SCRIPT="$NOA_ROOT/init/check-prereqs.sh"
if [[ -x "$PREREQS_SCRIPT" ]]; then
    if $SKIP_KERNEL; then
        "$PREREQS_SCRIPT" || true
    else
        "$PREREQS_SCRIPT" --kernel || true
    fi
else
    log warning "Prereqs script not found: $PREREQS_SCRIPT"
fi

# ============================================
# Phase 3: Download Static Binaries
# ============================================

log info "Phase 3: Downloading self-contained utilities..."

STATIC_SCRIPT="$NOA_SCRIPTS/download-static-binaries"
if [[ -x "$STATIC_SCRIPT" ]]; then
    "$STATIC_SCRIPT" || log warning "Some static binaries failed to download"
else
    log warning "Static binaries script not found: $STATIC_SCRIPT"
fi

# ============================================
# Phase 4: Bundle Libraries (for self-contained binaries)
# ============================================

log info "Phase 4: Bundling shared libraries..."

BUNDLE_SCRIPT="$NOA_SCRIPTS/bundle-all-libs"
if [[ -x "$BUNDLE_SCRIPT" ]]; then
    "$BUNDLE_SCRIPT" || log warning "Library bundling had issues"
else
    log warning "Bundle script not found (skipping): $BUNDLE_SCRIPT"
fi

# ============================================
# Phase 5: Kernel Setup (Linux/WSL2 only)
# ============================================

if ! $SKIP_KERNEL && [[ "$PLATFORM" == "linux" || "$PLATFORM" == "wsl2" ]]; then
    log info "Phase 5: Kernel setup..."

    KMOD_SCRIPT="$NOA_SCRIPTS/noa-kmod"
    if [[ -x "$KMOD_SCRIPT" ]]; then
        # Check required modules
        "$KMOD_SCRIPT" check || true

        # Load modules (requires sudo)
        if [[ $EUID -eq 0 ]]; then
            log info "Loading kernel modules (root)..."
            for module in tun ip_tables nf_conntrack bridge; do
                "$KMOD_SCRIPT" load "$module" 2>/dev/null || true
            done
        else
            log warning "Skipping module loading (not root). Run with sudo for full setup."
        fi
    else
        log warning "Kernel module script not found: $KMOD_SCRIPT"
    fi

    KPARAM_SCRIPT="$NOA_SCRIPTS/noa-kernel-params"
    if [[ -x "$KPARAM_SCRIPT" ]] && [[ $EUID -eq 0 ]]; then
        log info "Setting kernel parameters..."
        "$KPARAM_SCRIPT" set net.ipv4.ip_forward 1 2>/dev/null || true
        "$KPARAM_SCRIPT" set net.ipv6.conf.all.forwarding 1 2>/dev/null || true
    fi
elif [[ "$PLATFORM" == "wsl1" ]]; then
    log warning "Phase 5: Kernel setup SKIPPED (WSL1 has limited kernel access)"
elif [[ "$PLATFORM" == "macos" ]]; then
    log info "Phase 5: Kernel setup SKIPPED (macOS uses different kernel management)"
else
    log info "Phase 5: Kernel setup SKIPPED (--skip-kernel specified)"
fi

# ============================================
# Phase 6: Generate Environment File
# ============================================

log info "Phase 6: Generating environment configuration..."

ENV_FILE="$NOA_ROOT/noa-env.sh"

cat > "$ENV_FILE" << EOF
#!/bin/bash
# NOA Environment Configuration
# Auto-generated by bootstrap.sh
# Last Updated: $(date -Iseconds)

# NOA Root Directory
export NOA_ROOT="$NOA_ROOT"
export NOA_BIN="$NOA_BIN"
export NOA_OPT="$NOA_OPT"
export NOA_LIB="$NOA_LIB"
export NOA_CONFIG="$NOA_ROOT/config"
export NOA_LOGS="$NOA_LOGS"
export NOA_TMP="$NOA_ROOT/tmp"

# Portable Toolchains (if installed)
EOF

# Add portable toolchain paths if they exist
if [[ -d "$NOA_OPT/rust/cargo/bin" ]]; then
    cat >> "$ENV_FILE" << EOF

# Rust (portable)
export RUSTUP_HOME="$NOA_OPT/rust/rustup"
export CARGO_HOME="$NOA_OPT/rust/cargo"
export PATH="$NOA_OPT/rust/cargo/bin:\$PATH"
EOF
fi

if [[ -d "$NOA_OPT/go/bin" ]]; then
    cat >> "$ENV_FILE" << EOF

# Go (portable)
export GOROOT="$NOA_OPT/go"
export GOPATH="$NOA_OPT/go/workspace"
export GOBIN="$NOA_OPT/go/workspace/bin"
export GOCACHE="$NOA_OPT/go/cache"
export GOMODCACHE="$NOA_OPT/go/pkg/mod"
export PATH="$NOA_OPT/go/bin:$NOA_OPT/go/workspace/bin:\$PATH"
EOF
fi

if [[ -d "$NOA_OPT/node" ]]; then
    cat >> "$ENV_FILE" << EOF

# Node.js (portable)
export npm_config_prefix="$NOA_OPT/node"
export npm_config_cache="$NOA_OPT/npm-cache"
export PATH="$NOA_OPT/node:\$PATH"
EOF
fi

if [[ -d "$NOA_OPT/python" ]]; then
    cat >> "$ENV_FILE" << EOF

# Python (portable)
export PATH="$NOA_OPT/python:$NOA_OPT/python/bin:\$PATH"
# Activate venv: source "$NOA_OPT/venv/bin/activate"
EOF
fi

if [[ -d "$NOA_OPT/protobuf/bin" ]]; then
    cat >> "$ENV_FILE" << EOF

# protoc (portable)
export PATH="$NOA_OPT/protobuf/bin:\$PATH"
EOF
fi

# Always add NOA bin and lib
cat >> "$ENV_FILE" << EOF

# NOA bin and lib
export PATH="$NOA_BIN:\$PATH"
export LD_LIBRARY_PATH="$NOA_LIB:\${LD_LIBRARY_PATH:-}"

# Helper functions
cda() { cd "\$NOA_ROOT"; }
cdopt() { cd "\$NOA_OPT"; }
cdbin() { cd "\$NOA_BIN"; }

echo "NOA environment loaded: \$NOA_ROOT"
EOF

chmod +x "$ENV_FILE"
log success "Created: $ENV_FILE"

# ============================================
# Phase 7: Create .noa marker
# ============================================

log info "Phase 7: Creating marker file..."

MARKER_FILE="$NOA_ROOT/.noa"
cat > "$MARKER_FILE" << EOF
# NOA Root Directory Marker
# Created: $(date -Iseconds)
version=2.0.0
platform=$PLATFORM
root=$NOA_ROOT
EOF
log success "Created: .noa"

# ============================================
# Summary
# ============================================

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}║              Bootstrap Completed Successfully!             ║${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

log info "Summary:"
echo "  NOA_ROOT:    $NOA_ROOT"
echo "  Platform:    $PLATFORM"
echo "  Env file:    $ENV_FILE"
echo ""

log info "Next steps:"
echo -e "  1. Load environment: ${CYAN}source $ENV_FILE${NC}"
echo -e "  2. Verify prereqs:   ${CYAN}$NOA_ROOT/init/check-prereqs.sh${NC}"
if [[ "$PLATFORM" == "linux" || "$PLATFORM" == "wsl2" ]] && ! $SKIP_KERNEL; then
    echo -e "  3. Kernel setup:     ${CYAN}sudo $NOA_SCRIPTS/noa-kmod check${NC}"
fi
echo ""

# Optional: Add to shell profile
log info "To auto-load NOA on login, add to ~/.bashrc or ~/.zshrc:"
echo -e "  ${CYAN}[ -f \"$ENV_FILE\" ] && source \"$ENV_FILE\"${NC}"
echo ""

exit 0

