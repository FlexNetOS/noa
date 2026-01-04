#!/usr/bin/env nu

# NOA MCP configs Generator
# Generates provider-specific MCP configss from master agentgateway configs
# Constitution §3.1: All paths resolve under NOA_ROOT

def main [] {
    let noa_root = ($env.NOA_ROOT? | default (pwd))
    let master_configs = $"($noa_root)/gateway/mcp/agentgateway/configs/mcp-servers.json"
    
    if not ($master_configs | path exists) {
        print $"ERROR: Master configs not found at ($master_configs)"
        exit 1
    }
    
    let configs = open $master_configs
    
    # Generate VS Code configs
    generate_vscode_configs $configs $noa_root
    
    # Generate Cursor configs  
    generate_cursor_configs $configs $noa_root
    
    # Generate Claude configs
    generate_claude_configs $configs $noa_root
    
    print "✓ Generated provider-specific MCP configss"
}

def generate_vscode_configs [configs: record, noa_root: string] {
    let output_path = $"($noa_root)/.vscode/mcp.json"
    
    let servers = $configs.servers 
        | transpose key value
        | where { |row| $row.value.enabled == true and ("vscode" in $row.value.providers) }
        | each { |row|
            let server = $row.value
            let cmd = $server.command | str replace "${NOA_ROOT}" $noa_root
            {
                key: $row.key,
                value: {
                    type: $server.type,
                    command: $cmd,
                    args: $server.args,
                    env: ($server.env | default {})
                }
            }
        }
        | transpose -r -d
    
    let vscode_configs = {
        servers: $servers
    }
    
    $vscode_configs | to json -i 2 | save -f $output_path
    print $"  → ($output_path)"
}

def generate_cursor_configs [configs: record, noa_root: string] {
    let output_path = $"($noa_root)/.cursor/mcp.json"
    
    let servers = $configs.servers
        | transpose key value  
        | where { |row| $row.value.enabled == true and ("cursor" in $row.value.providers) }
        | each { |row|
            let server = $row.value
            let cmd = $server.command | str replace "${NOA_ROOT}" $noa_root
            {
                key: $row.key,
                value: {
                    type: $server.type,
                    command: $cmd,
                    args: $server.args
                }
            }
        }
        | transpose -r -d
    
    let cursor_configs = { servers: $servers }
    
    $cursor_configs | to json -i 2 | save -f $output_path
    print $"  → ($output_path)"
}

def generate_claude_configs [configs: record, noa_root: string] {
    let output_dir = $"($noa_root)/etc/claude"
    mkdir $output_dir
    let output_path = $"($output_dir)/mcp.json"
    
    let servers = $configs.servers
        | transpose key value
        | where { |row| $row.value.enabled == true and ("claude" in $row.value.providers) }
        | each { |row|
            let server = $row.value
            let cmd = $server.command | str replace "${NOA_ROOT}" $noa_root
            {
                key: $row.key,
                value: {
                    command: $cmd,
                    args: $server.args,
                    env: ($server.env | default {})
                }
            }
        }
        | transpose -r -d
        
    let claude_configs = {
        mcpServers: $servers
    }
    
    $claude_configs | to json -i 2 | save -f $output_path
    print $"  → ($output_path)"
}

main
