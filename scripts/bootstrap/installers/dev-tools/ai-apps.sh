#!/bin/bash
#
# Detect and configsure AI desktop applications (ChatGPT, Claude) for NOA.
#
# These apps must be manually installed; this script just detects and configsures.
#
# Usage:
#   ./ai-apps.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"

echo -e "\033[36mChecking AI Desktop Applications...\033[0m"
echo ""

# Detect OS
OS="$(uname -s)"

# ===== ChatGPT Desktop =====
echo -e "\033[33mChatGPT Desktop:\033[0m"

CHATGPT_FOUND=false
case "$OS" in
    Darwin)
        if [[ -d "/Applications/ChatGPT.app" ]]; then
            echo -e "  \033[32m[OK]\033[0m Found: /Applications/ChatGPT.app"
            CHATGPT_FOUND=true
        fi
        ;;
    Linux)
        # ChatGPT Desktop doesn't have official Linux support
        echo -e "  \033[90m[INFO]\033[0m ChatGPT Desktop not available for Linux"
        ;;
esac

if ! $CHATGPT_FOUND && [[ "$OS" == "Darwin" ]]; then
    echo -e "  \033[33m[SKIP]\033[0m ChatGPT Desktop not found"
    echo -e "  \033[90mInstall from: https://openai.com/chatgpt/desktop/\033[0m"
fi

echo ""

# ===== Claude Desktop =====
echo -e "\033[33mClaude Desktop:\033[0m"

CLAUDE_FOUND=false
CLAUDE_configs=""

case "$OS" in
    Darwin)
        if [[ -d "/Applications/Claude.app" ]]; then
            echo -e "  \033[32m[OK]\033[0m Found: /Applications/Claude.app"
            CLAUDE_FOUND=true
            CLAUDE_configs="$HOME/Library/Application Support/Claude/claude_desktop_configs.json"
        fi
        ;;
    Linux)
        # Check common Linux paths
        CLAUDE_PATHS=(
            "/usr/bin/claude"
            "/opt/claude/claude"
            "$HOME/.local/bin/claude"
        )
        for path in "${CLAUDE_PATHS[@]}"; do
            if [[ -x "$path" ]]; then
                echo -e "  \033[32m[OK]\033[0m Found: $path"
                CLAUDE_FOUND=true
                CLAUDE_configs="$HOME/.configs/claude/claude_desktop_configs.json"
                break
            fi
        done
        ;;
esac

if $CLAUDE_FOUND && [[ -n "$CLAUDE_configs" ]]; then
    if [[ -f "$CLAUDE_configs" ]]; then
        echo -e "  \033[32m[OK]\033[0m MCP configs found: $CLAUDE_configs"
    else
        echo -e "  \033[33m[INFO]\033[0m Creating MCP configs template..."
        mkdir -p "$(dirname "$CLAUDE_configs")"
        cat > "$CLAUDE_configs" << EOF
{
  "mcpServers": {
    "noa-tools": {
      "command": "node",
      "args": ["$NOA_ROOT/ai/mcp/server.js"],
      "env": {
        "NOA_ROOT": "$NOA_ROOT"
      }
    }
  }
}
EOF
        echo -e "  \033[32m[OK]\033[0m Created: $CLAUDE_configs"
    fi
elif ! $CLAUDE_FOUND; then
    echo -e "  \033[33m[SKIP]\033[0m Claude Desktop not found"
    echo -e "  \033[90mInstall from: https://claude.ai/download\033[0m"
fi

echo ""
echo -e "\033[32mAI Desktop Applications check complete.\033[0m"

