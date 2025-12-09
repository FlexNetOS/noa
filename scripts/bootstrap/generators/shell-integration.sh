#!/bin/bash
#
# Add NOA environment to shell profile (.bashrc/.zshrc).
#
# Adds a line to source .noa-env to your shell profile,
# ensuring NOA environment is loaded on every shell startup.
#
# Usage:
#   ./shell-integration.sh
#   ./shell-integration.sh --dry-run

set -euo pipefail

# Parse arguments
DRY_RUN=false
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
    esac
done

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"

NOA_ENV_PATH="$NOA_ROOT/.noa-env"

# Detect shell and profile
SHELL_NAME=$(basename "$SHELL")
case "$SHELL_NAME" in
    zsh)  PROFILE_PATH="$HOME/.zshrc" ;;
    bash) PROFILE_PATH="$HOME/.bashrc" ;;
    *)    PROFILE_PATH="$HOME/.profile" ;;
esac

echo -e "\033[36mNOA Shell Integration\033[0m"
echo -e "\033[90mNOA Root: $NOA_ROOT\033[0m"
echo -e "\033[90mShell:    $SHELL_NAME\033[0m"
echo -e "\033[90mProfile:  $PROFILE_PATH\033[0m"
echo ""

# Check if .noa-env exists
if [[ ! -f "$NOA_ENV_PATH" ]]; then
    echo -e "\033[33m[WARN]\033[0m .noa-env not found. Generate it first:"
    echo -e "  \033[90m./scripts/bootstrap/generators/noa-env.sh\033[0m"
    exit 0
fi

# Line to add to profile
SOURCE_LINE="source \"$NOA_ENV_PATH\""

# Check if already integrated
if [[ -f "$PROFILE_PATH" ]]; then
    if grep -q "$NOA_ENV_PATH" "$PROFILE_PATH"; then
        echo -e "\033[32m[OK]\033[0m NOA environment already integrated in profile"
        exit 0
    fi
fi

if $DRY_RUN; then
    echo -e "\033[33m[DRY RUN]\033[0m Would add to $PROFILE_PATH:"
    echo ""
    echo -e "  \033[90m# NOA Environment\033[0m"
    echo -e "  \033[90m$SOURCE_LINE\033[0m"
    exit 0
fi

# Append to profile
cat >> "$PROFILE_PATH" << EOF

# NOA Environment (added by shell-integration.sh)
$SOURCE_LINE
EOF

echo -e "\033[32m[OK]\033[0m Added NOA environment to profile"
echo ""
echo -e "\033[33mReload your shell or run:\033[0m"
echo -e "  \033[36msource \"$NOA_ENV_PATH\"\033[0m"

