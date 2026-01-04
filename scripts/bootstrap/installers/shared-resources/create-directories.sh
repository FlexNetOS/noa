#!/bin/bash
#
# Create shared AI resource directories for NOA.
#
# Creates the ai/shared/ directory structure for agents, prompts, tools,
# workflows, skills, models, commands, and resources.
#
# Usage:
#   ./create-directories.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"
SHARED_DIR="$NOA_ROOT/ai/shared"

echo -e "\033[36mCreating shared AI resource directories...\033[0m"
echo -e "\033[90mNOA Root: $NOA_ROOT\033[0m"
echo -e "\033[90mShared Dir: $SHARED_DIR\033[0m"
echo ""

# Define shared resource directories
declare -A SHARED_DIRS=(
    ["agents"]="AI agent definitions"
    ["prompts"]="Prompt templates"
    ["tools"]="MCP tool definitions"
    ["workflows"]="Orchestration workflows"
    ["skills"]="Agent skill modules"
    ["models"]="Model adapters and configss"
    ["commands"]="Shared commands"
    ["resources"]="configsuration and data"
    ["resources/schema"]="Database schemas"
)

for dir_name in "${!SHARED_DIRS[@]}"; do
    dir_path="$SHARED_DIR/$dir_name"
    if [[ ! -d "$dir_path" ]]; then
        mkdir -p "$dir_path"
        echo -e "  \033[32m[CREATE]\033[0m $dir_path (${SHARED_DIRS[$dir_name]})"
    else
        echo -e "  \033[90m[EXISTS]\033[0m $dir_path"
    fi
done

echo ""
echo -e "\033[32mShared resource directories created.\033[0m"

