#!/bin/bash
# MCP and Project Rules Tweaker

echo "🔧 MCP & Project Rules Customization"
echo "===================================="
echo ""
echo "What would you like to do?"
echo "1. Add/Remove MCP servers"
echo "2. Add/Remove project rules" 
echo "3. Update API keys"
echo "4. Show current configuration"
echo "5. Exit"
echo ""
read -p "Choose an option (1-5): " choice

case $choice in
    1)
        echo ""
        echo "MCP Server Management:"
        echo "a. Add Peekaboo (screenshot tool)"
        echo "b. Add GitHub (API access)"
        echo "c. Add Firecrawl (web scraping)"
        echo "d. Remove a server"
        echo "e. List all servers"
        read -p "Choose: " mcp_choice
        
        case $mcp_choice in
            a) 
                # Extract OpenAI key
                OPENAI_KEY=$(grep "export OPENAI_API_KEY=" ~/.zshrc | sed 's/export OPENAI_API_KEY="//' | sed 's/"$//')
                if [ -n "$OPENAI_KEY" ]; then
                    claude mcp add-json -s user peekaboo "{\"command\": \"npx\", \"args\": [\"-y\", \"@steipete/peekaboo-mcp@beta\"], \"env\": {\"PEEKABOO_AI_PROVIDERS\": \"openai/gpt-4o,ollama/llava:latest\", \"OPENAI_API_KEY\": \"$OPENAI_KEY\"}}"
                else
                    claude mcp add-json -s user peekaboo '{"command": "npx", "args": ["-y", "@steipete/peekaboo-mcp@beta"], "env": {"PEEKABOO_AI_PROVIDERS": "ollama/llava:latest"}}'
                    echo "Note: No OpenAI key found, using Ollama only"
                fi
                ;;
            b) 
                GITHUB_TOKEN=$(grep "export GITHUB_PERSONAL_ACCESS_TOKEN=" ~/.zshrc | sed 's/export GITHUB_PERSONAL_ACCESS_TOKEN="//' | sed 's/"$//')
                if [ -n "$GITHUB_TOKEN" ]; then
                    claude mcp add-json -s user github "{\"command\": \"npx\", \"args\": [\"-y\", \"@modelcontextprotocol/server-github\"], \"env\": {\"GITHUB_PERSONAL_ACCESS_TOKEN\": \"$GITHUB_TOKEN\"}}"
                else
                    echo "Error: Add GITHUB_PERSONAL_ACCESS_TOKEN to ~/.zshrc first!"
                    echo "Get token at: https://github.com/settings/tokens"
                fi
                ;;
            c) 
                FIRECRAWL_KEY=$(grep "export FIRECRAWL_API_KEY=" ~/.zshrc | sed 's/export FIRECRAWL_API_KEY="//' | sed 's/"$//')
                if [ -n "$FIRECRAWL_KEY" ]; then
                    claude mcp add-json -s user firecrawl-mcp "{\"command\": \"npx\", \"args\": [\"-y\", \"firecrawl-mcp\"], \"env\": {\"FIRECRAWL_API_KEY\": \"$FIRECRAWL_KEY\"}}"
                else
                    echo "Error: Add FIRECRAWL_API_KEY to ~/.zshrc first!"
                    echo "Get key at: https://www.firecrawl.dev"
                fi
                ;;
            d) 
                read -p "Server name to remove: " name 
                claude mcp remove -s user "$name" 
                ;;
            e) 
                claude mcp list 
                ;;
        esac
        ;;
    2)
        echo ""
        echo "Project Rules Management:"
        echo "a. Install all project rules"
        echo "b. Remove all project rules"
        echo "c. Show installed rules"
        read -p "Choose: " rules_choice
        
        case $rules_choice in
            a) 
                if ! grep -q "@.*project-rules" ~/.claude/CLAUDE.md 2>/dev/null; then
                    mkdir -p ~/.claude
                    echo "" >> ~/.claude/CLAUDE.md
                    echo "# Project Rules" >> ~/.claude/CLAUDE.md
                    echo "@/Users/steipete/Projects/agent-rules/project-rules" >> ~/.claude/CLAUDE.md
                    echo "✓ Added all project rules"
                    echo ""
                    echo "Available commands:"
                    echo "  Git: /commit, /commit-fast, /bug-fix, /pr-review"
                    echo "  Code: /check, /clean, /code-analysis"
                    echo "  Docs: /create-docs, /mermaid, /add-to-changelog"
                    echo "  ...and more!"
                else
                    echo "Project rules already installed"
                fi
                ;;
            b)
                if [ -f ~/.claude/CLAUDE.md ]; then
                    sed -i '' '/@.*project-rules/d' ~/.claude/CLAUDE.md
                    sed -i '' '/# Project Rules/d' ~/.claude/CLAUDE.md
                    echo "✓ Removed project rules"
                else
                    echo "No CLAUDE.md file found"
                fi
                ;;
            c)
                if grep -q "@.*project-rules" ~/.claude/CLAUDE.md 2>/dev/null; then
                    echo "✓ Project rules are installed:"
                    grep "@.*project-rules" ~/.claude/CLAUDE.md
                    echo ""
                    echo "Location: /Users/steipete/Projects/agent-rules/project-rules/"
                    echo "Files: $(ls /Users/steipete/Projects/agent-rules/project-rules/*.mdc 2>/dev/null | wc -l) rules"
                else
                    echo "✗ Project rules not installed"
                fi
                ;;
        esac
        ;;
    3)
        echo ""
        echo "Update API Keys in ~/.zshrc:"
        echo ""
        echo "1. OPENAI_API_KEY (for Peekaboo AI vision)"
        echo "   Get from: https://platform.openai.com/api-keys"
        echo ""
        echo "2. GITHUB_PERSONAL_ACCESS_TOKEN (for GitHub MCP)"
        echo "   Get from: https://github.com/settings/tokens"
        echo "   Required scope: repo"
        echo ""
        echo "3. FIRECRAWL_API_KEY (for web scraping)"
        echo "   Get from: https://www.firecrawl.dev"
        echo ""
        echo "Add to ~/.zshrc:"
        echo '  export OPENAI_API_KEY="sk-..."'
        echo '  export GITHUB_PERSONAL_ACCESS_TOKEN="ghp_..."'
        echo '  export FIRECRAWL_API_KEY="fc-..."'
        echo ""
        echo "Then run: source ~/.zshrc"
        ;;
    4)
        echo ""
        echo "Current Configuration:"
        echo "====================="
        echo ""
        echo "MCP Servers:"
        claude mcp list
        echo ""
        echo "Project Rules:"
        if grep -q "@.*project-rules" ~/.claude/CLAUDE.md 2>/dev/null; then
            echo "✓ Project rules installed"
            grep "@.*project-rules" ~/.claude/CLAUDE.md
        else
            echo "✗ Project rules not installed"
        fi
        echo ""
        echo "Claude Settings:"
        if [ -f ~/.claude/settings.json ]; then
            echo "✓ Settings configured"
            jq '{model, includeCoAuthoredBy, DISABLE_COST_WARNINGS, DISABLE_AUTOUPDATER}' ~/.claude/settings.json 2>/dev/null || cat ~/.claude/settings.json
        else
            echo "✗ No settings file"
        fi
        ;;
    5)
        exit 0
        ;;
esac