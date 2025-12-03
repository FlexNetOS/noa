#!/usr/bin/env zsh
# Terminal title management for Claude Code
# Place this file in ~/.config/zsh/claude-wrapper.zsh
# Source it in your ~/.zshrc

# Function to set terminal title
set_terminal_title() {
    echo -ne "\033]0;$1\007"
}

# Wrapper function for Claude Code with terminal title management
cly() {
    local original_title=$(echo -ne "\033]0;\007" && osascript -e 'tell application "Terminal" to get name of front window')
    local project_path=$(pwd)
    local project_name=$(basename "$project_path")
    
    # Set the terminal title with project info
    set_terminal_title "~/${project_path#$HOME/} — Claude"
    
    # Start background process to maintain title
    (
        while kill -0 $$ 2>/dev/null; do
            set_terminal_title "~/${project_path#$HOME/} — Claude"
            sleep 5
        done
    ) &
    local title_pid=$!
    
    # Run Claude Code
    claude "$@"
    
    # Kill the background process and restore title
    kill $title_pid 2>/dev/null
    set_terminal_title "$original_title"
}

# Alternative version for iTerm2 users
if [[ "$TERM_PROGRAM" == "iTerm.app" ]]; then
    cly() {
        local project_path=$(pwd)
        
        # iTerm2 proprietary escape sequences
        echo -ne "\033]1337;SetUserVar=claudeProject=$(echo -n "$project_path" | base64)\007"
        printf "\033]0;%s — Claude\007" "~/${project_path#$HOME/}"
        
        # Run Claude Code
        claude "$@"
        
        # Clear the user var
        echo -ne "\033]1337;SetUserVar=claudeProject=\007"
    }
fi