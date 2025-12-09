<#
.SYNOPSIS
    Detect and configure Claude Desktop for NOA integration.

.DESCRIPTION
    Detects Claude Desktop installation and configures MCP integration.
    Claude Desktop must be manually installed from Anthropic.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\claude-desktop.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

Write-Host "Checking Claude Desktop installation..." -ForegroundColor Cyan

# Common Claude Desktop installation paths
$claudePaths = @(
    "$env:LOCALAPPDATA\Programs\claude-desktop\Claude.exe"
    "$env:LOCALAPPDATA\Claude\Claude.exe"
    "C:\Program Files\Claude\Claude.exe"
    "$env:USERPROFILE\AppData\Local\AnthropicClaude\Claude.exe"
)

$claudeExe = $null
foreach ($path in $claudePaths) {
    if (Test-Path $path) {
        $claudeExe = $path
        break
    }
}

if ($claudeExe) {
    Write-Host "  [OK] Claude Desktop found: $claudeExe" -ForegroundColor Green

    # Check for MCP config
    $mcpConfigPath = "$env:APPDATA\Claude\claude_desktop_config.json"
    if (Test-Path $mcpConfigPath) {
        Write-Host "  [OK] MCP config found: $mcpConfigPath" -ForegroundColor Green
    } else {
        Write-Host "  [INFO] MCP config not found - creating template..." -ForegroundColor Yellow

        # Create MCP config directory
        $mcpConfigDir = Split-Path -Parent $mcpConfigPath
        if (-not (Test-Path $mcpConfigDir)) {
            New-Item -ItemType Directory -Path $mcpConfigDir -Force | Out-Null
        }

        # Create template MCP config
        $mcpConfig = @{
            mcpServers = @{
                "noa-tools" = @{
                    command = "node"
                    args = @("$NoaRoot/ai/mcp/server.js")
                    env = @{
                        NOA_ROOT = $NoaRoot
                    }
                }
            }
        }

        $mcpConfig | ConvertTo-Json -Depth 4 | Set-Content -Path $mcpConfigPath -Encoding UTF8
        Write-Host "  [OK] Created MCP config template: $mcpConfigPath" -ForegroundColor Green
        Write-Host "  [INFO] Edit the config to customize MCP server settings" -ForegroundColor Gray
    }
} else {
    Write-Host "  [SKIP] Claude Desktop not found" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  To install Claude Desktop:" -ForegroundColor Gray
    Write-Host "    1. Visit https://claude.ai/download" -ForegroundColor Gray
    Write-Host "    2. Download the Windows installer" -ForegroundColor Gray
    Write-Host "    3. Run the installer" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  Claude Desktop supports MCP (Model Context Protocol) for tool integration" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Claude Desktop check complete." -ForegroundColor Green

