#!/bin/bash
#
# NOA Uninstaller - Remove NOA installation
#
# Removes NOA installation from the system.
# Per NOA Constitution §3.1: Self-contained means clean uninstall.
#
# Usage:
#   ./uninstall.sh                    # Interactive uninstall
#   ./uninstall.sh --dry-run          # Show what would be removed
#   ./uninstall.sh --keep-configs      # Keep configsuration files
#   ./uninstall.sh --keep-logs        # Keep log files
#   ./uninstall.sh --force            # Skip confirmation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$SCRIPT_DIR")}"

# Parse arguments
KEEP_configs=false
KEEP_LOGS=false
DRY_RUN=false
FORCE=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --keep-configs)
            KEEP_configs=true
            shift
            ;;
        --keep-logs)
            KEEP_LOGS=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force|-f)
            FORCE=true
            shift
            ;;
        --noa-root)
            NOA_ROOT="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info()    { echo -e "[..] $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!!]${NC} $1"; }
log_error()   { echo -e "${RED}[XX]${NC} $1"; }
log_dryrun()  { echo -e "${CYAN}[DR]${NC} $1"; }

echo ""
echo "============================================================"
echo " NOA Uninstaller"
echo "============================================================"
echo ""
echo "NOA_ROOT: $NOA_ROOT"
if [[ "$DRY_RUN" == true ]]; then
    echo "MODE: DRY RUN (no changes will be made)"
fi
echo ""

# Verify NOA installation
if [[ ! -d "$NOA_ROOT" ]]; then
    log_error "NOA installation not found at: $NOA_ROOT"
    exit 1
fi

# Directories to remove
DIRS_TO_REMOVE=(
    "opt/rust"
    "opt/go"
    "opt/node"
    "opt/python"
    "opt/venv"
    "opt/protobuf"
    "opt/dev-tools"
    "opt/npm-cache"
    "opt/cache"
    "opt/cursor-cli"
    "opt/claude-code"
    "opt/codex"
    "cache"
    "tmp"
    "lib/shared"
    "init/run"
)

if [[ "$KEEP_configs" != true ]]; then
    DIRS_TO_REMOVE+=("configs/bootstrap-state.json")
fi

if [[ "$KEEP_LOGS" != true ]]; then
    DIRS_TO_REMOVE+=("logs")
fi

# Files to remove
FILES_TO_REMOVE=(
    "noa-env.ps1"
    "noa-env.sh"
    ".env.local"
)

# Confirm
if [[ "$FORCE" != true && "$DRY_RUN" != true ]]; then
    echo "This will remove the following from $NOA_ROOT:"
    for dir in "${DIRS_TO_REMOVE[@]}"; do
        echo "  - $dir"
    done
    for file in "${FILES_TO_REMOVE[@]}"; do
        echo "  - $file"
    done
    echo ""
    read -p "Continue? (y/N) " confirm
    if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
        log_warning "Uninstall cancelled"
        exit 0
    fi
fi

# Remove directories
for dir in "${DIRS_TO_REMOVE[@]}"; do
    full_path="$NOA_ROOT/$dir"
    if [[ -e "$full_path" ]]; then
        if [[ "$DRY_RUN" == true ]]; then
            log_dryrun "Would remove: $full_path"
        else
            rm -rf "$full_path"
            log_success "Removed: $full_path"
        fi
    fi
done

# Remove files
for file in "${FILES_TO_REMOVE[@]}"; do
    full_path="$NOA_ROOT/$file"
    if [[ -e "$full_path" ]]; then
        if [[ "$DRY_RUN" == true ]]; then
            log_dryrun "Would remove: $full_path"
        else
            rm -f "$full_path"
            log_success "Removed: $full_path"
        fi
    fi
done

# Remove symlinks from bin/
BIN_DIR="$NOA_ROOT/bin"
if [[ -d "$BIN_DIR" ]]; then
    while IFS= read -r -d '' link; do
        if [[ "$DRY_RUN" == true ]]; then
            log_dryrun "Would remove symlink: $link"
        else
            rm -f "$link"
            log_success "Removed symlink: $(basename "$link")"
        fi
    done < <(find "$BIN_DIR" -type l -print0 2>/dev/null)
fi

# Remove AI execution memory databases
find "$NOA_ROOT/ai/shared/resources" -name "*.db" -type f 2>/dev/null | while read -r db; do
    if [[ "$DRY_RUN" == true ]]; then
        log_dryrun "Would remove: $db"
    else
        rm -f "$db"
        log_success "Removed: $db"
    fi
done

# Clean environment variables (inform user)
echo ""
log_warning "Manual cleanup required:"
echo "  Remove NOA entries from your shell profile (~/.bashrc or ~/.zshrc):"
echo "    - export NOA_ROOT=..."
echo "    - export RUSTUP_HOME=..., CARGO_HOME=..."
echo "    - export GOROOT=..., GOPATH=..."
echo "    - PATH entries pointing to $NOA_ROOT"

echo ""
if [[ "$DRY_RUN" == true ]]; then
    log_dryrun "DRY RUN complete - no changes were made"
else
    log_success "NOA uninstall complete"
    log_info "Core repository files are preserved"
fi

