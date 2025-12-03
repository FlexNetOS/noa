#!/bin/bash
# MCP Setup Helper Script
# This script downloads all necessary files and prepares the MCP installation

set -e

echo "🚀 MCP Setup Helper for Claude Code"
echo "===================================="
echo "This script will download all necessary files for MCP server setup in Claude Code."
echo "Note: This is for Claude Code (the CLI tool), not Claude Desktop."
echo ""

# Create directory structure
echo "Creating directory structure..."
mkdir -p ~/Projects/agent-rules/global-rules
cd ~/Projects/agent-rules/global-rules

# Download files
echo "Downloading MCP documentation and scripts..."

# Main documentation
echo "  • Downloading steipete-mcps.md..."
curl -sO https://raw.githubusercontent.com/steipete/agent-rules/refs/heads/main/global-rules/steipete-mcps.md

# Sync script
echo "  • Downloading mcp-sync.sh..."
curl -sO https://raw.githubusercontent.com/steipete/agent-rules/refs/heads/main/global-rules/mcp-sync.sh
chmod +x mcp-sync.sh

# Sync rule documentation
echo "  • Downloading mcp-sync-rule.md..."
curl -sO https://raw.githubusercontent.com/steipete/agent-rules/refs/heads/main/global-rules/mcp-sync-rule.md

echo ""
echo "✅ All files downloaded successfully!"
echo ""

# Extract installation scripts from the documentation
echo "Extracting installation scripts..."

# Extract the pre-installation check script
awk '/^```bash$/{p=1; next} /^```$/{if(p && script ~ /Checking for required API keys/) print script > "check-api-keys.sh"; p=0; script=""} p{script = script $0 "\n"}' steipete-mcps.md
chmod +x check-api-keys.sh 2>/dev/null || true

# Extract the quick installation script
awk '/^```bash$/{p=1; next} /^```$/{if(p && script ~ /Quick MCP Installation/) print script > "quick-install-mcps.sh"; p=0; script=""} p{script = script $0 "\n"}' steipete-mcps.md
chmod +x quick-install-mcps.sh 2>/dev/null || true

echo ""
echo "📁 Files ready in: ~/Projects/agent-rules/global-rules/"
echo ""
echo "Next steps:"
echo "1. Check API keys:     ./check-api-keys.sh"
echo "2. Quick install:      ./quick-install-mcps.sh"
echo "3. Full install:       Follow instructions in steipete-mcps.md"
echo "4. Verify setup:       ./mcp-sync.sh"
echo ""

# Offer to install project rules
echo "📋 Project Rules (Slash Commands)"
echo "================================="
echo "The repository includes 20 project rules that add slash commands to Claude Code:"
echo "  • /commit - Create well-formatted commits"
echo "  • /bug-fix - Complete bug fix workflow"
echo "  • /pr-review - Comprehensive PR reviews"
echo "  • ...and 17 more!"
echo ""
read -p "Would you like to install project rules? (y/N): " install_rules
if [[ "$install_rules" == "y" || "$install_rules" == "Y" ]]; then
    # Check if we have the full repository
    if [ -d "$HOME/Projects/agent-rules/project-rules" ]; then
        cd "$HOME/Projects/agent-rules"
        ./install-project-rules.sh
    else
        echo "Cloning full repository to get project rules..."
        git clone https://github.com/steipete/agent-rules.git ~/Projects/agent-rules-full
        cd ~/Projects/agent-rules-full
        ./install-project-rules.sh
    fi
    cd ~/Projects/agent-rules/global-rules
fi

echo ""
echo "To view the documentation:"
echo "  cat steipete-mcps.md"
echo ""
echo "To add missing API keys:"
echo "  • OPENAI_API_KEY: https://platform.openai.com/api-keys"
echo "  • GITHUB_PERSONAL_ACCESS_TOKEN: https://github.com/settings/tokens"
echo "  • FIRECRAWL_API_KEY: https://www.firecrawl.dev"