#!/usr/bin/env nu

# NOA MCP Config Generator
# Generates provider-specific MCP configs from master agentgateway config
# Constitution §3.1: All paths resolve under NOA_ROOT

def main [] {
    let noa_root = ($env.NOA_ROOT? | default (pwd))
    let master_config = $"($noa_root)/gateway/mcp/agentgateway/config/mcp-servers.json"
    
    if not ($master_config | path exists) {
        print $"ERROR: Master config not found at ($master_config)"
        exit 1
    }
    
    let config = open $master_config
    
    # Generate VS Code config
    generate_vscode_config $config $noa_root
    
    # Generate Cursor config  
    generate_cursor_config $config $noa_root
    
    # Generate Claude config
    generate_claude_config $config $noa_root
    
    print "✓ Generated provider-specific MCP configs"
}

def generate_vscode_config [config: record, noa_root: string] {
    let output_path = $"($noa_root)/.vscode/mcp.json"
    
    let servers = $config.servers 
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
    
    let vscode_config = {
        servers: $servers
    }
    
    $vscode_config | to json -i 2 | save -f $output_path
    print $"  → ($output_path)"
}

def generate_cursor_config [config: record, noa_root: string] {
    let output_path = $"($noa_root)/.cursor/mcp.json"
    
    let servers = $config.servers
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
    
    let cursor_config = { servers: $servers }
    
    $cursor_config | to json -i 2 | save -f $output_path
    print $"  → ($output_path)"
}

def generate_claude_config [config: record, noa_root: string] {
    let output_dir = $"($noa_root)/etc/claude"
    mkdir $output_dir
    let output_path = $"($output_dir)/mcp.json"
    
    let servers = $config.servers
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
        
    let claude_config = {
        mcpServers: $servers
    }
    
    $claude_config | to json -i 2 | save -f $output_path
    print $"  → ($output_path)"
}

main
