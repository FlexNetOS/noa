#!/bin/bash

# MCP Server Configuration Synchronizer
# This script finds and compares MCP server configurations across different applications
# It can also switch servers between local and global configurations

# Color codes for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration file paths
CLAUDE_DESKTOP_CONFIG="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
CURSOR_CONFIG="$HOME/.cursor/mcp.json"
CLAUDE_CODE_CONFIG="$HOME/.claude.json"
WINDSURF_CONFIG="$HOME/.codeium/windsurf/mcp_config.json"
VSCODE_USER_CONFIG="$HOME/Library/Application Support/Code/User/settings.json"

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo -e "${RED}Error: jq is not installed. Please install it with: brew install jq${NC}"
    exit 1
fi

# Function to display usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help                Show this help message"
    echo "  -l, --list                List all MCP servers (default)"
    echo "  --to-global <server>      Switch a server from local to global config"
    echo "  --to-local <server>       Switch a server from global to local config"
    echo "  --app <app>               Specify app for switching (claude-desktop|cursor|windsurf|vscode)"
    echo ""
    echo "Examples:"
    echo "  $0                        # List all servers"
    echo "  $0 --to-global terminator --app cursor"
    echo "  $0 --to-local peekaboo --app claude-desktop"
    exit 0
}

# Parse command line arguments
ACTION="list"
SERVER_NAME=""
TARGET_APP=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            ;;
        -l|--list)
            ACTION="list"
            shift
            ;;
        --to-global)
            ACTION="to-global"
            SERVER_NAME="$2"
            shift 2
            ;;
        --to-local)
            ACTION="to-local"
            SERVER_NAME="$2"
            shift 2
            ;;
        --app)
            TARGET_APP="$2"
            shift 2
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            usage
            ;;
    esac
done

# Validate required arguments for switching actions
if [[ "$ACTION" == "to-global" || "$ACTION" == "to-local" ]]; then
    if [[ -z "$SERVER_NAME" ]]; then
        echo -e "${RED}Error: Server name is required for $ACTION action${NC}"
        usage
    fi
    if [[ -z "$TARGET_APP" ]]; then
        echo -e "${RED}Error: App must be specified with --app (claude-desktop or cursor)${NC}"
        usage
    fi
    if [[ "$TARGET_APP" != "claude-desktop" && "$TARGET_APP" != "cursor" && "$TARGET_APP" != "windsurf" && "$TARGET_APP" != "vscode" ]]; then
        echo -e "${RED}Error: App must be one of: 'claude-desktop', 'cursor', 'windsurf', or 'vscode'${NC}"
        usage
    fi
fi

echo "🔍 MCP Server Configuration Synchronizer"
echo "======================================="
echo ""

# Function to extract MCP servers from a config file
extract_servers() {
    local file="$1"
    local format="$2"
    
    if [ ! -f "$file" ]; then
        echo "{}"
        return
    fi
    
    # For large files like Claude Code config, use a more efficient approach
    if [ "$format" = "claude_code" ]; then
        # Extract just the mcpServers section without loading the entire file into memory
        jq -c '.mcpServers // {}' "$file" 2>/dev/null || echo "{}"
    elif [ "$format" = "vscode" ]; then
        # VS Code stores MCP config in user settings under "mcp.servers"
        jq -r '."mcp.servers" // {}' "$file" 2>/dev/null || echo "{}"
    elif [ "$format" = "vscode_legacy" ]; then
        # Legacy format for older VS Code versions
        jq -r '.mcp.servers // {}' "$file" 2>/dev/null || echo "{}"
    else
        jq -r '.mcpServers // {}' "$file" 2>/dev/null || echo "{}"
    fi
}

# Function to get server names from JSON
get_server_names() {
    echo "$1" | jq -r 'keys[]' 2>/dev/null | sort
}

# Function to switch server from local to global npm package
switch_to_global() {
    local server_name="$1"
    local app="$2"
    local config_file=""
    
    # Determine config file based on app
    if [[ "$app" == "claude-desktop" ]]; then
        config_file="$CLAUDE_DESKTOP_CONFIG"
    elif [[ "$app" == "cursor" ]]; then
        config_file="$CURSOR_CONFIG"
    elif [[ "$app" == "windsurf" ]]; then
        config_file="$WINDSURF_CONFIG"
    else
        config_file="$VSCODE_USER_CONFIG"
    fi
    
    echo -e "${BLUE}Switching '$server_name' to global configuration in $app...${NC}"
    
    # Check if server exists
    local server_exists=$(jq --arg name "$server_name" '.mcpServers | has($name)' "$config_file")
    if [[ "$server_exists" != "true" ]]; then
        echo -e "${RED}Error: Server '$server_name' not found in $app${NC}"
        return 1
    fi
    
    # Get current configuration
    local current_config=$(jq --arg name "$server_name" '.mcpServers[$name]' "$config_file")
    echo "Current configuration:"
    echo "$current_config" | jq .
    
    # Common npm package mappings
    local npm_package=""
    case "$server_name" in
        "terminator")
            npm_package="@steipete/terminator-mcp@beta"
            ;;
        "agent")
            npm_package="@steipete/claude-code-mcp@latest"
            ;;
        "automator")
            npm_package="@steipete/macos-automator-mcp@latest"
            ;;
        "conduit")
            npm_package="@steipete/conduit-mcp@beta"
            ;;
        "peekaboo")
            npm_package="@steipete/peekaboo-mcp@beta"
            ;;
        *)
            echo -e "${YELLOW}Warning: Unknown server '$server_name'. Please specify the npm package name:${NC}"
            read -p "NPM package name: " npm_package
            if [[ -z "$npm_package" ]]; then
                echo -e "${RED}Error: NPM package name is required${NC}"
                return 1
            fi
            ;;
    esac
    
    # Create new global configuration
    local new_config=$(echo "$current_config" | jq --arg pkg "$npm_package" '.command = "npx" | .args = ["-y", $pkg]')
    
    echo ""
    echo "New global configuration:"
    echo "$new_config" | jq .
    
    # Confirm before applying
    read -p "Apply this change? (y/N): " confirm
    if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
        echo "Change cancelled."
        return 0
    fi
    
    # Apply the change
    local temp_file=$(mktemp)
    jq --arg name "$server_name" --argjson config "$new_config" '.mcpServers[$name] = $config' "$config_file" > "$temp_file"
    
    if [[ $? -eq 0 ]]; then
        mv "$temp_file" "$config_file"
        echo -e "${GREEN}✓ Successfully switched '$server_name' to global configuration${NC}"
    else
        rm "$temp_file"
        echo -e "${RED}Error: Failed to update configuration${NC}"
        return 1
    fi
}

# Function to switch server from global to local path
switch_to_local() {
    local server_name="$1"
    local app="$2"
    local config_file=""
    
    # Determine config file based on app
    if [[ "$app" == "claude-desktop" ]]; then
        config_file="$CLAUDE_DESKTOP_CONFIG"
    elif [[ "$app" == "cursor" ]]; then
        config_file="$CURSOR_CONFIG"
    elif [[ "$app" == "windsurf" ]]; then
        config_file="$WINDSURF_CONFIG"
    else
        config_file="$VSCODE_USER_CONFIG"
    fi
    
    echo -e "${BLUE}Switching '$server_name' to local configuration in $app...${NC}"
    
    # Check if server exists
    local server_exists=$(jq --arg name "$server_name" '.mcpServers | has($name)' "$config_file")
    if [[ "$server_exists" != "true" ]]; then
        echo -e "${RED}Error: Server '$server_name' not found in $app${NC}"
        return 1
    fi
    
    # Get current configuration
    local current_config=$(jq --arg name "$server_name" '.mcpServers[$name]' "$config_file")
    echo "Current configuration:"
    echo "$current_config" | jq .
    
    # Common local paths
    local local_path=""
    case "$server_name" in
        "terminator")
            local_path="$HOME/Projects/Terminator/dist/index.js"
            ;;
        *)
            echo -e "${YELLOW}Please specify the local path for '$server_name':${NC}"
            read -p "Local path: " local_path
            if [[ -z "$local_path" ]]; then
                echo -e "${RED}Error: Local path is required${NC}"
                return 1
            fi
            # Expand tilde if present
            local_path="${local_path/#\~/$HOME}"
            ;;
    esac
    
    # Check if local path exists
    if [[ ! -f "$local_path" ]]; then
        echo -e "${YELLOW}Warning: Local path does not exist: $local_path${NC}"
        read -p "Continue anyway? (y/N): " confirm
        if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
            return 0
        fi
    fi
    
    # Determine command based on file extension
    local command="node"
    if [[ "$local_path" == *.py ]]; then
        command="python"
    elif [[ "$local_path" == *.sh ]]; then
        command="bash"
    fi
    
    # Create new local configuration
    local new_config=$(echo "$current_config" | jq --arg cmd "$command" --arg path "$local_path" '.command = $cmd | .args = [$path]')
    
    echo ""
    echo "New local configuration:"
    echo "$new_config" | jq .
    
    # Confirm before applying
    read -p "Apply this change? (y/N): " confirm
    if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
        echo "Change cancelled."
        return 0
    fi
    
    # Apply the change
    local temp_file=$(mktemp)
    jq --arg name "$server_name" --argjson config "$new_config" '.mcpServers[$name] = $config' "$config_file" > "$temp_file"
    
    if [[ $? -eq 0 ]]; then
        mv "$temp_file" "$config_file"
        echo -e "${GREEN}✓ Successfully switched '$server_name' to local configuration${NC}"
    else
        rm "$temp_file"
        echo -e "${RED}Error: Failed to update configuration${NC}"
        return 1
    fi
}

# Only show listing output if action is "list"
if [[ "$ACTION" == "list" ]]; then

# Check which config files exist
echo "📁 Checking configuration files:"
echo ""

if [ -f "$CLAUDE_DESKTOP_CONFIG" ]; then
    echo -e "${GREEN}✓${NC} Claude Desktop: $CLAUDE_DESKTOP_CONFIG"
    CLAUDE_DESKTOP_SERVERS=$(extract_servers "$CLAUDE_DESKTOP_CONFIG" "standard")
else
    echo -e "${RED}✗${NC} Claude Desktop: Not found"
    CLAUDE_DESKTOP_SERVERS="{}"
fi

if [ -f "$CURSOR_CONFIG" ]; then
    echo -e "${GREEN}✓${NC} Cursor: $CURSOR_CONFIG"
    CURSOR_SERVERS=$(extract_servers "$CURSOR_CONFIG" "standard")
else
    echo -e "${RED}✗${NC} Cursor: Not found"
    CURSOR_SERVERS="{}"
fi

if [ -f "$CLAUDE_CODE_CONFIG" ]; then
    echo -e "${GREEN}✓${NC} Claude Code: $CLAUDE_CODE_CONFIG"
    # Claude Code doesn't store MCP servers in its config file
    echo -e "  ${YELLOW}Note:${NC} Claude Code doesn't use MCP servers configuration"
    CLAUDE_CODE_SERVERS="{}"
else
    echo -e "${RED}✗${NC} Claude Code: Not found"
    CLAUDE_CODE_SERVERS="{}"
fi

if [ -f "$WINDSURF_CONFIG" ]; then
    echo -e "${GREEN}✓${NC} Windsurf: $WINDSURF_CONFIG"
    WINDSURF_SERVERS=$(extract_servers "$WINDSURF_CONFIG" "standard")
else
    echo -e "${RED}✗${NC} Windsurf: Not found"
    WINDSURF_SERVERS="{}"
fi

if [ -f "$VSCODE_USER_CONFIG" ]; then
    echo -e "${GREEN}✓${NC} VS Code (User): $VSCODE_USER_CONFIG"
    VSCODE_SERVERS=$(extract_servers "$VSCODE_USER_CONFIG" "vscode")
else
    echo -e "${RED}✗${NC} VS Code (User): Not found"
    VSCODE_SERVERS="{}"
fi

echo ""
echo "📊 Server Analysis:"
echo "=================="

# Get all unique server names
ALL_SERVERS=$(echo "$CLAUDE_DESKTOP_SERVERS $CURSOR_SERVERS $CLAUDE_CODE_SERVERS $WINDSURF_SERVERS $VSCODE_SERVERS" | \
    jq -s 'add | keys' | jq -r '.[]' | sort -u)

# Create arrays for tracking
declare -a COMMON_SERVERS=()
declare -a CLAUDE_DESKTOP_ONLY=()
declare -a CURSOR_ONLY=()
declare -a CLAUDE_CODE_ONLY=()
declare -a WINDSURF_ONLY=()
declare -a VSCODE_ONLY=()

# Analyze each server
for server in $ALL_SERVERS; do
    in_claude_desktop=$(echo "$CLAUDE_DESKTOP_SERVERS" | jq --arg s "$server" 'has($s)')
    in_cursor=$(echo "$CURSOR_SERVERS" | jq --arg s "$server" 'has($s)')
    in_claude_code=$(echo "$CLAUDE_CODE_SERVERS" | jq --arg s "$server" 'has($s)')
    in_windsurf=$(echo "$WINDSURF_SERVERS" | jq --arg s "$server" 'has($s)')
    in_vscode=$(echo "$VSCODE_SERVERS" | jq --arg s "$server" 'has($s)')
    
    count=0
    [ "$in_claude_desktop" = "true" ] && ((count++))
    [ "$in_cursor" = "true" ] && ((count++))
    [ "$in_claude_code" = "true" ] && ((count++))
    [ "$in_windsurf" = "true" ] && ((count++))
    [ "$in_vscode" = "true" ] && ((count++))
    
    if [ $count -ge 2 ]; then
        COMMON_SERVERS+=("$server")
    elif [ "$in_claude_desktop" = "true" ] && [ $count -eq 1 ]; then
        CLAUDE_DESKTOP_ONLY+=("$server")
    elif [ "$in_cursor" = "true" ] && [ $count -eq 1 ]; then
        CURSOR_ONLY+=("$server")
    elif [ "$in_claude_code" = "true" ] && [ $count -eq 1 ]; then
        CLAUDE_CODE_ONLY+=("$server")
    elif [ "$in_windsurf" = "true" ] && [ $count -eq 1 ]; then
        WINDSURF_ONLY+=("$server")
    elif [ "$in_vscode" = "true" ] && [ $count -eq 1 ]; then
        VSCODE_ONLY+=("$server")
    fi
done

# Display results
echo ""
echo "🤝 Common Servers (in 2+ apps):"
if [ ${#COMMON_SERVERS[@]} -eq 0 ]; then
    echo "  None"
else
    for server in "${COMMON_SERVERS[@]}"; do
        # Get package/URL info from first available config
        server_info=""
        for servers in "$CLAUDE_DESKTOP_SERVERS" "$CURSOR_SERVERS" "$WINDSURF_SERVERS" "$VSCODE_SERVERS"; do
            config=$(echo "$servers" | jq --arg s "$server" '.[$s]' 2>/dev/null)
            if [ "$config" != "null" ] && [ -n "$config" ]; then
                # Extract package name or URL
                if echo "$config" | jq -e '.url' >/dev/null 2>&1; then
                    server_info=$(echo "$config" | jq -r '.url')
                elif echo "$config" | jq -e '.args' >/dev/null 2>&1; then
                    # Look for package name in args, skip -y flag
                    pkg=$(echo "$config" | jq -r '.args[]' | grep -v '^-y$' | grep -E '^@|^[a-z-]+$' | grep -v '^-' | head -1)
                    [ -n "$pkg" ] && server_info="$pkg"
                else
                    server_info=$(echo "$config" | jq -r '.command' | head -1)
                fi
                [ -n "$server_info" ] && break
            fi
        done
        
        echo -e "  ${GREEN}•${NC} $server ($server_info)"
        
        # Show which apps have it
        apps=""
        [ "$(echo "$CLAUDE_DESKTOP_SERVERS" | jq --arg s "$server" 'has($s)')" = "true" ] && apps+="Claude Desktop, "
        [ "$(echo "$CURSOR_SERVERS" | jq --arg s "$server" 'has($s)')" = "true" ] && apps+="Cursor, "
        [ "$(echo "$CLAUDE_CODE_SERVERS" | jq --arg s "$server" 'has($s)')" = "true" ] && apps+="Claude Code, "
        [ "$(echo "$WINDSURF_SERVERS" | jq --arg s "$server" 'has($s)')" = "true" ] && apps+="Windsurf, "
        [ "$(echo "$VSCODE_SERVERS" | jq --arg s "$server" 'has($s)')" = "true" ] && apps+="VS Code, "
        apps=${apps%, }
        echo "    In: $apps"
    done
fi

echo ""
echo "📱 App-Specific Servers:"

if [ ${#CLAUDE_DESKTOP_ONLY[@]} -gt 0 ]; then
    echo -e "\n  ${YELLOW}Claude Desktop only:${NC}"
    for server in "${CLAUDE_DESKTOP_ONLY[@]}"; do
        config=$(echo "$CLAUDE_DESKTOP_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
        server_info=""
        if echo "$config" | jq -e '.url' >/dev/null 2>&1; then
            server_info=$(echo "$config" | jq -r '.url')
        elif echo "$config" | jq -e '.args' >/dev/null 2>&1; then
            pkg=$(echo "$config" | jq -r '.args[]' | grep -v '^-y$' | grep -E '^@|^[a-z-]+$' | grep -v '^-' | head -1)
            [ -n "$pkg" ] && server_info="$pkg"
        else
            server_info=$(echo "$config" | jq -r '.command' | head -1)
        fi
        echo "  • $server ($server_info)"
    done
fi

if [ ${#CURSOR_ONLY[@]} -gt 0 ]; then
    echo -e "\n  ${YELLOW}Cursor only:${NC}"
    for server in "${CURSOR_ONLY[@]}"; do
        config=$(echo "$CURSOR_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
        server_info=""
        if echo "$config" | jq -e '.url' >/dev/null 2>&1; then
            server_info=$(echo "$config" | jq -r '.url')
        elif echo "$config" | jq -e '.args' >/dev/null 2>&1; then
            pkg=$(echo "$config" | jq -r '.args[]' | grep -v '^-y$' | grep -E '^@|^[a-z-]+$' | grep -v '^-' | head -1)
            [ -n "$pkg" ] && server_info="$pkg"
        else
            server_info=$(echo "$config" | jq -r '.command' | head -1)
        fi
        echo "  • $server ($server_info)"
    done
fi

if [ ${#CLAUDE_CODE_ONLY[@]} -gt 0 ]; then
    echo -e "\n  ${YELLOW}Claude Code only:${NC}"
    for server in "${CLAUDE_CODE_ONLY[@]}"; do
        echo "  • $server"
    done
fi

if [ ${#WINDSURF_ONLY[@]} -gt 0 ]; then
    echo -e "\n  ${YELLOW}Windsurf only:${NC}"
    for server in "${WINDSURF_ONLY[@]}"; do
        config=$(echo "$WINDSURF_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
        server_info=""
        if echo "$config" | jq -e '.url' >/dev/null 2>&1; then
            server_info=$(echo "$config" | jq -r '.url')
        elif echo "$config" | jq -e '.args' >/dev/null 2>&1; then
            pkg=$(echo "$config" | jq -r '.args[]' | grep -v '^-y$' | grep -E '^@|^[a-z-]+$' | grep -v '^-' | head -1)
            [ -n "$pkg" ] && server_info="$pkg"
        else
            server_info=$(echo "$config" | jq -r '.command' | head -1)
        fi
        echo "  • $server ($server_info)"
    done
fi

if [ ${#VSCODE_ONLY[@]} -gt 0 ]; then
    echo -e "\n  ${YELLOW}VS Code only:${NC}"
    for server in "${VSCODE_ONLY[@]}"; do
        config=$(echo "$VSCODE_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
        server_info=""
        if echo "$config" | jq -e '.url' >/dev/null 2>&1; then
            server_info=$(echo "$config" | jq -r '.url')
        elif echo "$config" | jq -e '.args' >/dev/null 2>&1; then
            pkg=$(echo "$config" | jq -r '.args[]' | grep -v '^-y$' | grep -E '^@|^[a-z-]+$' | grep -v '^-' | head -1)
            [ -n "$pkg" ] && server_info="$pkg"
        else
            server_info=$(echo "$config" | jq -r '.command' | head -1)
        fi
        echo "  • $server ($server_info)"
    done
fi

# Configuration differences for common servers
echo ""
echo "🔧 Configuration Status:"
echo "======================="

# First, show servers with matching configs
echo ""
echo "✅ Shared across all apps (matching configs):"
matching_found=false
for server in "${COMMON_SERVERS[@]}"; do
    # Get configurations from each app
    claude_desktop_config=$(echo "$CLAUDE_DESKTOP_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    cursor_config=$(echo "$CURSOR_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    windsurf_config=$(echo "$WINDSURF_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    vscode_config=$(echo "$VSCODE_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    
    # Get the command/package info for display
    config_cmd=""
    if [ "$claude_desktop_config" != "null" ]; then
        config_cmd=$(echo "$claude_desktop_config" | jq -r 'if .args then (.args | map(select(. != "-y")) | join(" ")) else .command end' 2>/dev/null)
    elif [ "$cursor_config" != "null" ]; then
        config_cmd=$(echo "$cursor_config" | jq -r 'if .args then (.args | map(select(. != "-y")) | join(" ")) else .command end' 2>/dev/null)
    fi
    
    # Check if all non-null configs match
    configs_match=true
    first_config=""
    for config in "$claude_desktop_config" "$cursor_config" "$windsurf_config" "$vscode_config"; do
        if [ "$config" != "null" ]; then
            if [ -z "$first_config" ]; then
                first_config="$config"
            elif [ "$config" != "$first_config" ]; then
                configs_match=false
                break
            fi
        fi
    done
    
    if [ "$configs_match" = true ]; then
        matching_found=true
        echo -e "  ${GREEN}✓${NC} $server: $config_cmd"
    fi
done

if [ "$matching_found" = false ]; then
    echo "  None"
fi

# Then show servers with differing configs
echo ""
echo "⚠️  Configuration differences:"
diff_found=false
for server in "${COMMON_SERVERS[@]}"; do
    # Get configurations from each app
    claude_desktop_config=$(echo "$CLAUDE_DESKTOP_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    cursor_config=$(echo "$CURSOR_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    windsurf_config=$(echo "$WINDSURF_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    vscode_config=$(echo "$VSCODE_SERVERS" | jq --arg s "$server" '.[$s]' 2>/dev/null)
    
    # Check if all non-null configs match
    configs_match=true
    first_config=""
    for config in "$claude_desktop_config" "$cursor_config" "$windsurf_config" "$vscode_config"; do
        if [ "$config" != "null" ]; then
            if [ -z "$first_config" ]; then
                first_config="$config"
            elif [ "$config" != "$first_config" ]; then
                configs_match=false
                break
            fi
        fi
    done
    
    if [ "$configs_match" = false ]; then
        diff_found=true
        echo ""
        echo -e "  ${RED}✗${NC} $server:"
        
        if [ "$claude_desktop_config" != "null" ]; then
            cmd=$(echo "$claude_desktop_config" | jq -r 'if .args then (.args | map(select(. != "-y")) | join(" ")) else .command end' 2>/dev/null)
            echo "    • Claude Desktop: $cmd"
        fi
        
        if [ "$cursor_config" != "null" ] && [ "$cursor_config" != "$claude_desktop_config" ]; then
            cmd=$(echo "$cursor_config" | jq -r 'if .args then (.args | map(select(. != "-y")) | join(" ")) else .command end' 2>/dev/null)
            echo "    • Cursor: $cmd"
        fi
        
        if [ "$windsurf_config" != "null" ] && [ "$windsurf_config" != "$claude_desktop_config" ] && [ "$windsurf_config" != "$cursor_config" ]; then
            cmd=$(echo "$windsurf_config" | jq -r 'if .args then (.args | map(select(. != "-y")) | join(" ")) else .command end' 2>/dev/null)
            echo "    • Windsurf: $cmd"
        fi
        
        if [ "$vscode_config" != "null" ] && [ "$vscode_config" != "$claude_desktop_config" ] && [ "$vscode_config" != "$cursor_config" ] && [ "$vscode_config" != "$windsurf_config" ]; then
            cmd=$(echo "$vscode_config" | jq -r 'if .args then (.args | map(select(. != "-y")) | join(" ")) else .command end' 2>/dev/null)
            echo "    • VS Code: $cmd"
        fi
    fi
done

if [ "$diff_found" = false ]; then
    echo "  None"
fi

# Summary
echo ""
echo "📈 Summary:"
echo "=========="
total_servers=$(echo "$ALL_SERVERS" | wc -w | tr -d ' ')
echo "Total unique servers: $total_servers"
echo "Common servers: ${#COMMON_SERVERS[@]}"
echo "App-specific servers: $((total_servers - ${#COMMON_SERVERS[@]}))"

# Sync recommendation
echo ""
echo "💡 Sync Recommendations:"
echo "======================="

if [ ${#COMMON_SERVERS[@]} -gt 0 ]; then
    echo "• Review configuration differences for common servers"
fi

total_unique=$((${#CLAUDE_DESKTOP_ONLY[@]} + ${#CURSOR_ONLY[@]} + ${#CLAUDE_CODE_ONLY[@]} + ${#WINDSURF_ONLY[@]} + ${#VSCODE_ONLY[@]}))
if [ $total_unique -gt 0 ]; then
    echo "• Consider adding app-specific servers to other apps for consistency"
fi

echo ""
echo "To sync configurations manually, copy the mcpServers section between config files."
echo "Remember to adjust the format for VS Code (wrap in 'mcp.servers' instead of 'mcpServers')."
fi

# Handle switching actions
if [[ "$ACTION" == "to-global" ]]; then
    switch_to_global "$SERVER_NAME" "$TARGET_APP"
elif [[ "$ACTION" == "to-local" ]]; then
    switch_to_local "$SERVER_NAME" "$TARGET_APP"
fi