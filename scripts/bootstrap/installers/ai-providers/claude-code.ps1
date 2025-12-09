<#
.SYNOPSIS
    Install Claude Code CLI to noa_root/opt/

.DESCRIPTION
    Downloads and installs the Claude Code CLI (@anthropic-ai/claude-code) to the NOA
    portable environment. Supports npm installation and native installer methods.

    Repository: https://github.com/FlexNetOS/claude-code.git
    Provider Config: ai/providers/cloud/claude-code/config.json

.PARAMETER NoaRoot
    Root directory for NOA installation. Defaults to N:\noa

.PARAMETER Method
    Installation method: npm (default), native, or clone
    - npm: Install via npm to opt/node/node_modules/
    - native: Use official claude.ai installer
    - clone: Clone FlexNetOS fork to opt/claude-code/

.EXAMPLE
    .\claude-code.ps1
    Install Claude Code via npm (default)

.EXAMPLE
    .\claude-code.ps1 -Method clone
    Clone FlexNetOS fork for development
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory=$false)]
    [string]$NoaRoot = "N:\noa",

    [Parameter(Mandatory=$false)]
    [ValidateSet("npm", "native", "clone")]
    [string]$Method = "npm"
)

$ErrorActionPreference = "Stop"

# Source logging library if available
$loggingLib = Join-Path $PSScriptRoot "..\..\lib\logging.ps1"
if (Test-Path $loggingLib) {
    . $loggingLib
} else {
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
}

# Configuration
$ClaudeCodeVersion = "latest"
$NpmPackage = "@anthropic-ai/claude-code"
$GitRepo = "https://github.com/FlexNetOS/claude-code.git"
$ProviderConfigPath = Join-Path $NoaRoot "ai/providers/cloud/claude-code"

Write-Log "Installing Claude Code CLI..." -Level Info
Write-Log "  Method: $Method" -Level Info
Write-Log "  NOA Root: $NoaRoot" -Level Info

# Ensure directories exist
$optPath = Join-Path $NoaRoot "opt"
$binPath = Join-Path $NoaRoot "bin"
$nodePath = Join-Path $optPath "node"

foreach ($dir in @($optPath, $binPath, $ProviderConfigPath)) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Log "  Created: $dir" -Level Success
    }
}

switch ($Method) {
    "npm" {
        Write-Log "Installing via npm..." -Level Info

        # Check if portable Node.js exists
        $npmCmd = Join-Path $nodePath "npm.cmd"
        if (-not (Test-Path $npmCmd)) {
            $npmCmd = Get-Command npm -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
            if (-not $npmCmd) {
                Write-Log "npm not found. Please install Node.js first." -Level Error
                exit 1
            }
        }

        # Set npm prefix to install globally within noa_root
        $env:npm_config_prefix = $nodePath

        # Install claude-code
        & $npmCmd install -g $NpmPackage
        if ($LASTEXITCODE -ne 0) {
            Write-Log "npm install failed" -Level Error
            exit 1
        }

        # Create symlink in bin/
        $claudeBin = Join-Path $nodePath "node_modules/.bin/claude.cmd"
        $claudeLink = Join-Path $binPath "claude.cmd"
        if (Test-Path $claudeBin) {
            Copy-Item $claudeBin $claudeLink -Force
            Write-Log "  Created: bin/claude.cmd" -Level Success
        }
    }

    "native" {
        Write-Log "Installing via native installer..." -Level Info

        # Download and run official installer
        $installerUrl = "https://claude.ai/install.ps1"
        $installerPath = Join-Path $env:TEMP "claude-install.ps1"

        try {
            Invoke-WebRequest -Uri $installerUrl -OutFile $installerPath -UseBasicParsing
            & $installerPath
        } catch {
            Write-Log "Failed to download/run native installer: $_" -Level Error
            exit 1
        }
    }

    "clone" {
        Write-Log "Cloning FlexNetOS fork..." -Level Info

        $clonePath = Join-Path $optPath "claude-code"

        if (Test-Path $clonePath) {
            Write-Log "  Directory exists, pulling latest..." -Level Info
            Push-Location $clonePath
            git pull
            Pop-Location
        } else {
            git clone $GitRepo $clonePath
        }

        if ($LASTEXITCODE -ne 0) {
            Write-Log "git clone/pull failed" -Level Error
            exit 1
        }

        # Install dependencies
        Push-Location $clonePath
        npm install
        Pop-Location

        Write-Log "  Cloned to: $clonePath" -Level Success
    }
}

# Verify installation
Write-Log "Verifying installation..." -Level Info
$claudeCmd = Get-Command claude -ErrorAction SilentlyContinue
if ($claudeCmd) {
    $version = & claude --version 2>&1
    Write-Log "  Claude Code installed: $version" -Level Success
} else {
    # Check portable location
    $portableClaude = Join-Path $nodePath "node_modules/.bin/claude.cmd"
    if (Test-Path $portableClaude) {
        Write-Log "  Claude Code installed to portable location" -Level Success
        Write-Log "  Add to PATH: $nodePath\node_modules\.bin" -Level Info
    } else {
        Write-Log "  Claude Code not found in PATH" -Level Warning
    }
}

# Ensure provider config exists
$configFile = Join-Path $ProviderConfigPath "config.json"
if (-not (Test-Path $configFile)) {
    Write-Log "Creating provider config..." -Level Info
    # Config is created by the main setup, but ensure directory exists
}

Write-Log "Claude Code installation complete!" -Level Success
Write-Log "  Provider config: $ProviderConfigPath" -Level Info
Write-Log "  Shared resources: $NoaRoot\ai\shared" -Level Info

exit 0

