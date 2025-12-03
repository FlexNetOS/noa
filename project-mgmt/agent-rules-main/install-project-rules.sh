#!/bin/bash
# Install Project Rules for Claude Code

echo "🎯 Installing Project Rules for Claude Code"
echo "=========================================="

# Check if running from agent-rules directory
if [ ! -d "project-rules" ]; then
    echo "❌ Error: project-rules directory not found"
    echo "   Please run this from the agent-rules repository root"
    exit 1
fi

# Create Claude directory if it doesn't exist
mkdir -p ~/.claude

# Check if CLAUDE.md exists
if [ -f ~/.claude/CLAUDE.md ]; then
    echo "📝 Found existing ~/.claude/CLAUDE.md"
    echo "   Adding import for project rules..."
    
    # Check if already imported
    if grep -q "@.*project-rules" ~/.claude/CLAUDE.md; then
        echo "✓ Project rules already imported"
    else
        # Add import at the end
        echo "" >> ~/.claude/CLAUDE.md
        echo "# Project Rules" >> ~/.claude/CLAUDE.md
        echo "@$(pwd)/project-rules" >> ~/.claude/CLAUDE.md
        echo "✓ Added project rules import"
    fi
else
    echo "📝 Creating ~/.claude/CLAUDE.md with project rules import..."
    cat > ~/.claude/CLAUDE.md << EOF
# Claude Code User Memory

This file contains personal preferences and custom commands for Claude Code.

# Project Rules
@$(pwd)/project-rules
EOF
    echo "✓ Created CLAUDE.md with project rules"
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "📋 Available Slash Commands in Claude Code:"
echo ""
echo "Git & GitHub:"
echo "  /commit             - Create well-formatted commits with conventional commit messages and emojis"
echo "  /commit-fast        - Generate 3 commit message suggestions and auto-use the first one"
echo "  /bug-fix            - Streamline bug fixing workflow from issue creation to pull request"
echo "  /pr-review          - Comprehensive pull request review from multiple perspectives"
echo "  /analyze-issue      - Fetch GitHub issue details and create implementation specification"
echo ""
echo "Code Quality:"
echo "  /check              - Perform comprehensive code quality and security checks"
echo "  /clean              - Fix all code formatting and quality issues in the entire codebase"
echo "  /code-analysis      - Perform advanced code analysis with multiple inspection options"
echo ""
echo "Documentation:"
echo "  /add-to-changelog   - Update the project's CHANGELOG.md file with a new entry"
echo "  /create-docs        - Create comprehensive documentation for components or features"
echo "  /mermaid            - Generate Mermaid diagrams for visualizing code structure"
echo ""
echo "Development Workflow:"
echo "  /implement-task     - Approach task implementation methodically with planning"
echo "  /context-prime      - Prime Claude with comprehensive project understanding"
echo "  /five               - Use Five Whys root cause analysis to understand problems"
echo ""
echo "Automation & Meta:"
echo "  /create-command     - Guide for creating new custom Claude commands"
echo "  /continuous-improvement - Systematic approach for improving AI assistant rules"
echo "  /safari-automation  - Automating Safari interactions for web UI testing"
echo "  /screenshot-automation - AppleScript patterns for automated screenshots"
echo ""
echo "To use: Type '/' followed by the command name in Claude Code"