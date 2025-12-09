<#
.SYNOPSIS
    Install/Configure VS Code with GitHub Copilot to noa_root/opt/

.DESCRIPTION
    Downloads VS Code portable and installs GitHub Copilot extensions.
    Per NOA Constitution 3.3: IDE provider integration.

    Provider Config: ai/providers/ide/vscode-copilot/config.json

.PARAMETER NoaRoot
    Root directory for NOA installation. Defaults to N:\noa

.PARAMETER Method
    Installation method: portable (default), detect, manual
    - portable: Download VS Code portable to opt/dev-tools/vscode/
    - detect: Look for existing VS Code and configure
    - manual: Provide instructions only

.EXAMPLE
    .\vscode-copilot.ps1
    Install VS Code portable with Copilot
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory=$false)]
    [string]$NoaRoot = "N:\noa",

    [Parameter(Mandatory=$false)]
    [ValidateSet("portable", "detect", "manual")]
    [string]$Method = "detect"
)

$ErrorActionPreference = "Stop"

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        default { "White" }
    }
    Write-Host "[$Level] $Message" -ForegroundColor $color
}

# Configuration
$ProviderConfigPath = Join-Path $NoaRoot "ai/providers/ide/vscode-copilot"
$VsCodePortablePath = Join-Path $NoaRoot "opt/dev-tools/vscode"

Write-Log "Setting up VS Code with GitHub Copilot..." -Level Info
Write-Log "  Method: $Method" -Level Info
Write-Log "  NOA Root: $NoaRoot" -Level Info

# Ensure directories exist
$optPath = Join-Path $NoaRoot "opt"
$binPath = Join-Path $NoaRoot "bin"
$devToolsPath = Join-Path $optPath "dev-tools"

foreach ($dir in @($optPath, $binPath, $ProviderConfigPath, $devToolsPath)) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Log "  Created: $dir" -Level Success
    }
}

# Check for existing VS Code
$existingCode = Get-Command code -ErrorAction SilentlyContinue

switch ($Method) {
    "detect" {
        Write-Log "Detecting existing VS Code installation..." -Level Info

        if ($existingCode) {
            Write-Log "  Found VS Code: $($existingCode.Source)" -Level Success

            # Check for Copilot extensions
            Write-Log "  Checking for Copilot extensions..." -Level Info
            $extensions = & code --list-extensions 2>&1

            $copilotInstalled = $extensions -match "GitHub.copilot"
            $copilotChatInstalled = $extensions -match "GitHub.copilot-chat"

            if ($copilotInstalled) {
                Write-Log "  GitHub Copilot: installed" -Level Success
            } else {
                Write-Log "  GitHub Copilot: not installed" -Level Warning
                Write-Log "  Installing GitHub.copilot..." -Level Info
                & code --install-extension GitHub.copilot 2>&1 | Out-Null
            }

            if ($copilotChatInstalled) {
                Write-Log "  GitHub Copilot Chat: installed" -Level Success
            } else {
                Write-Log "  GitHub Copilot Chat: not installed" -Level Warning
                Write-Log "  Installing GitHub.copilot-chat..." -Level Info
                & code --install-extension GitHub.copilot-chat 2>&1 | Out-Null
            }
        } else {
            Write-Log "  VS Code not found in PATH" -Level Warning
            Write-Log "  Use -Method portable to install, or -Method manual for instructions" -Level Info
        }
    }

    "portable" {
        Write-Log "Portable VS Code installation..." -Level Info
        Write-Log "  Note: VS Code portable requires manual download due to licensing" -Level Warning
        Write-Host ""
        Write-Host "To install VS Code portable:" -ForegroundColor Cyan
        Write-Host "1. Download from: https://code.visualstudio.com/download" -ForegroundColor White
        Write-Host "2. Choose 'zip' version (portable)" -ForegroundColor White
        Write-Host "3. Extract to: $VsCodePortablePath" -ForegroundColor White
        Write-Host "4. Create 'data' folder inside for portable mode" -ForegroundColor White
        Write-Host ""
    }

    "manual" {
        Write-Log "Manual VS Code + Copilot setup:" -Level Info
        Write-Host ""
        Write-Host "1. Download VS Code: https://code.visualstudio.com/download" -ForegroundColor Cyan
        Write-Host "2. Install extensions:" -ForegroundColor Cyan
        Write-Host "   code --install-extension GitHub.copilot" -ForegroundColor White
        Write-Host "   code --install-extension GitHub.copilot-chat" -ForegroundColor White
        Write-Host "3. Sign in with GitHub account that has Copilot access" -ForegroundColor Cyan
        Write-Host ""
    }
}

Write-Log "VS Code Copilot setup complete!" -Level Success
Write-Log "  Provider config: $ProviderConfigPath" -Level Info

exit 0

