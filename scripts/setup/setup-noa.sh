#!/usr/bin/env bash
#
# NOA Environment Setup Script for Unix/macOS
#
# Creates the NOA directory structure, generates configsuration files,
# and optionally installs prerequisites and integrates with shell profile.
#
# Usage:
#   ./setup-noa.sh                                    # Default setup to ~/noa
#   ./setup-noa.sh --noa-root /path/to/noa           # Custom location
#   ./setup-noa.sh --install-all-tools               # Install all toolchains
#   ./setup-noa.sh --install-ai-providers            # Install AI provider CLIs
#   ./setup-noa.sh --integrate-profile               # Add to .bashrc/.zshrc
#   ./setup-noa.sh --install-all-tools --install-ai-providers --integrate-profile  # Full setup
#

set -euo pipefail

# Script metadata
SCRIPT_VERSION="2.0.0"
SCRIPT_NAME="NOA Setup"

# Default values
NOA_ROOT="${NOA_ROOT:-$HOME/noa}"
INSTALL_PREREQS=false
INTEGRATE_PROFILE=false
INSTALL_ALL_TOOLS=false
INSTALL_AI_PROVIDERS=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --noa-root)
            NOA_ROOT="$2"
            shift 2
            ;;
        --install-prereqs)
            INSTALL_PREREQS=true
            shift
            ;;
        --integrate-profile)
            INTEGRATE_PROFILE=true
            shift
            ;;
        --install-all-tools)
            INSTALL_ALL_TOOLS=true
            shift
            ;;
        --install-ai-providers)
            INSTALL_AI_PROVIDERS=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --noa-root PATH       Set NOA root directory (default: ~/noa)"
            echo "  --install-prereqs     Install prerequisites"
            echo "  --install-all-tools   Install all toolchains and utilities"
            echo "  --install-ai-providers Install AI provider CLIs (FR-039)"
            echo "  --integrate-profile   Add NOA to shell profile"
            echo "  --help                Show this help"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Resolve absolute path
NOA_ROOT="$(cd "$(dirname "$NOA_ROOT")" 2>/dev/null && pwd)/$(basename "$NOA_ROOT")" || NOA_ROOT="$NOA_ROOT"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Logging functions
log_info()    { echo "[INFO] $*"; }
log_success() { echo "[OK]   $*"; }
log_warning() { echo "[WARN] $*" >&2; }
log_error()   { echo "[ERR]  $*" >&2; }

# Banner
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                                                            ║"
echo "║           $SCRIPT_NAME v$SCRIPT_VERSION                     ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

log_info "NOA Root: $NOA_ROOT"

# Create base directory
if [[ ! -d "$NOA_ROOT" ]]; then
    log_info "Creating root directory..."
    mkdir -p "$NOA_ROOT"
    log_success "Root directory created"
else
    log_info "Root directory exists"
fi

# Initialize log directory and file
LOGS_DIR="$NOA_ROOT/logs"
mkdir -p "$LOGS_DIR"
TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
LOG_FILE="$LOGS_DIR/setup-$TIMESTAMP.log"
log_info "Log file: $LOG_FILE"

# Create directory structure
log_info "Creating NOA directory structure..."

DIRECTORIES=(
    "repos"
    "containers"
    "workspace"
    "configs"
    "scripts"
    "logs"
    "tmp"
    "p2p"
    "ai"
    "git"
    "bin"
    "etc"
    "lib"
    "opt"
    "sys"
    "init"
)

for dir in "${DIRECTORIES[@]}"; do
    dir_path="$NOA_ROOT/$dir"
    if [[ ! -d "$dir_path" ]]; then
        mkdir -p "$dir_path"
        log_success "  Created: $dir"
    else
        log_info "  Exists: $dir"
    fi
done

# Create AI provider directories
AI_PROVIDER_DIRS=(
    "ai/providers/cloud/claude-code"
    "ai/providers/cloud/codex"
    "ai/providers/cloud/abacus"
    "ai/providers/local"
    "ai/providers/hybrid"
    "ai/shared/agents"
    "ai/shared/workflows"
    "ai/shared/prompts"
    "ai/shared/skills"
    "ai/shared/tools"
    "ai/shared/models"
)

for dir in "${AI_PROVIDER_DIRS[@]}"; do
    mkdir -p "$NOA_ROOT/$dir"
done
log_success "  Created AI provider directories"

# Install prerequisites if requested
if [[ "$INSTALL_PREREQS" == "true" ]]; then
    log_info "Checking prerequisites..."

    prereq_checker="$REPO_ROOT/init/check-prereqs.sh"
    if [[ -x "$prereq_checker" ]]; then
        "$prereq_checker" || log_warning "Some prerequisites may be missing"
    else
        log_warning "Prerequisite checker not found: $prereq_checker"
    fi
fi

# Install all tools if requested
if [[ "$INSTALL_ALL_TOOLS" == "true" ]] || [[ "$INSTALL_AI_PROVIDERS" == "true" ]]; then
    log_info "Installing toolchains and utilities..."

    install_all_script="$NOA_ROOT/scripts/setup/install-all-tools.sh"
    if [[ ! -x "$install_all_script" ]]; then
        install_all_script="$REPO_ROOT/scripts/setup/install-all-tools.sh"
    fi

    if [[ -x "$install_all_script" ]]; then
        export NOA_ROOT

        if [[ "$INSTALL_ALL_TOOLS" == "true" ]]; then
            log_info "  Running full tool installation..."
            "$install_all_script"
        elif [[ "$INSTALL_AI_PROVIDERS" == "true" ]]; then
            log_info "  Installing AI Provider CLIs only (FR-039)..."
            "$install_all_script" node ai-providers
        fi

        log_success "  Tool installation complete"
    else
        log_warning "  install-all-tools.sh not found"
    fi
fi

# Generate noa-profile.sh
log_info "Generating noa-profile.sh..."

PROFILE_PATH="$NOA_ROOT/noa-profile.sh"

cat > "$PROFILE_PATH" << 'EOF'
# NOA Environment Profile
# Auto-generated by setup-noa.sh
# Source this file in your .bashrc or .zshrc

EOF

cat >> "$PROFILE_PATH" << EOF
# Generated: $(date '+%Y-%m-%d %H:%M:%S')

# Environment Variables
export NOA_ROOT="$NOA_ROOT"
export NOA_REPOS="\$NOA_ROOT/repos"
export NOA_CONTAINERS="\$NOA_ROOT/containers"
export NOA_WORKSPACE="\$NOA_ROOT/workspace"
export NOA_configs="\$NOA_ROOT/configs"
export NOA_SCRIPTS="\$NOA_ROOT/scripts"
export NOA_LOGS="\$NOA_ROOT/logs"
export NOA_TMP="\$NOA_ROOT/tmp"
export NOA_P2P="\$NOA_ROOT/p2p"
export NOA_AI="\$NOA_ROOT/ai"
export NOA_GIT="\$NOA_ROOT/git"
export NOA_BIN="\$NOA_ROOT/bin"

# Add NOA bin to PATH
export PATH="\$NOA_BIN:\$PATH"

# Navigation Helper Functions
cda()   { cd "\$NOA_ROOT"; }
cdr()   { cd "\$NOA_REPOS"; }
cdc()   { cd "\$NOA_CONTAINERS"; }
cdw()   { cd "\$NOA_WORKSPACE"; }
cds()   { cd "\$NOA_SCRIPTS"; }
cdl()   { cd "\$NOA_LOGS"; }
cdp()   { cd "\$NOA_P2P"; }
cdai()  { cd "\$NOA_AI"; }
cdgit() { cd "\$NOA_GIT"; }

# Status indicator
echo "NOA environment loaded from: \$NOA_ROOT"
EOF

chmod +x "$PROFILE_PATH"
log_success "  Created: noa-profile.sh"

# Create .noa marker file
log_info "Creating .noa marker file..."
MARKER_PATH="$NOA_ROOT/.noa"
cat > "$MARKER_PATH" << EOF
# NOA Root Directory Marker
# Created: $(date '+%Y-%m-%d %H:%M:%S')
version=$SCRIPT_VERSION
root=$NOA_ROOT
EOF
log_success "  Created: .noa"

# Create configs/noa.json
log_info "Creating configs/noa.json..."
configs_PATH="$NOA_ROOT/configs/noa.json"
cat > "$configs_PATH" << EOF
{
  "version": "2.0.0",
  "name": "NOA",
  "description": "Network Orchestration and Automation",
  "created": "$(date '+%Y-%m-%d %H:%M:%S')",
  "root": "$NOA_ROOT",
  "directories": {
    "repos": "Git repositories",
    "containers": "Container images and configss",
    "workspace": "Active project workspace",
    "configs": "configsuration files",
    "scripts": "Automation scripts",
    "logs": "Log files",
    "tmp": "Temporary files",
    "p2p": "Peer-to-peer networking",
    "ai": "AI models and configss",
    "git": "Git workflows and hooks",
    "bin": "Executables",
    "etc": "Additional configsuration",
    "lib": "Libraries",
    "opt": "Optional packages",
    "sys": "System files",
    "init": "Initialization scripts"
  }
}
EOF
log_success "  Created: configs/noa.json"

# Integrate with shell profile if requested
if [[ "$INTEGRATE_PROFILE" == "true" ]]; then
    log_info "Integrating with shell profile..."

    SOURCE_LINE="source \"$PROFILE_PATH\""

    # Determine which profile to use
    if [[ -n "${ZSH_VERSION:-}" ]] || [[ "$SHELL" == */zsh ]]; then
        SHELL_PROFILE="$HOME/.zshrc"
    else
        SHELL_PROFILE="$HOME/.bashrc"
    fi

    # Check if already integrated
    if grep -qF "$PROFILE_PATH" "$SHELL_PROFILE" 2>/dev/null; then
        log_info "  Profile already integrated"
    else
        echo "" >> "$SHELL_PROFILE"
        echo "# NOA Environment" >> "$SHELL_PROFILE"
        echo "$SOURCE_LINE" >> "$SHELL_PROFILE"
        log_success "  Added to: $SHELL_PROFILE"
    fi
fi

# Summary
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                                                            ║"
echo "║              Setup Completed Successfully!                 ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

log_info "=== Setup Summary ==="
log_info "Root Directory: $NOA_ROOT"
log_info "Profile Script: $PROFILE_PATH"
log_info "configs File: $configs_PATH"
log_info "Log File: $LOG_FILE"

echo ""
echo "Next Steps:"
echo "  1. Load the environment:"
echo "     source \"$PROFILE_PATH\""
echo ""

if [[ "$INTEGRATE_PROFILE" != "true" ]]; then
    echo "  2. (Optional) To auto-load NOA in all shell sessions:"
    echo "     Run setup again with --integrate-profile"
    echo ""
fi

echo "  Navigation commands available after loading profile:"
echo "     cda   - Navigate to NOA root"
echo "     cdr   - Navigate to repos"
echo "     cdw   - Navigate to workspace"
echo "     cdai  - Navigate to AI providers"
echo ""

log_success "=== NOA Setup Completed Successfully ==="

exit 0

