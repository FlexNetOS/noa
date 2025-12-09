<#
.SYNOPSIS
    Install Codex CLI to noa_root/opt/

.DESCRIPTION
    Downloads and installs the Codex CLI (@openai/codex) to the NOA
    portable environment. Supports npm installation and clone methods.

    Repository: https://github.com/FlexNetOS/codex.git
    Provider Config: ai/providers/cloud/codex/config.json

.PARAMETER NoaRoot
    Root directory for NOA installation. Defaults to N:\noa

.PARAMETER Method
    Installation method: npm (default) or clone
    - npm: Install via npm to opt/node/node_modules/
    - clone: Clone FlexNetOS fork to opt/codex/

.EXAMPLE
    .\codex-cli.ps1
    Install Codex via npm (default)

.EXAMPLE
    .\codex-cli.ps1 -Method clone
    Clone FlexNetOS fork for development
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory=$false)]
    [string]$NoaRoot = "N:\noa",

    [Parameter(Mandatory=$false)]
    [ValidateSet("npm", "clone")]
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
$CodexVersion = "latest"
$NpmPackage = "@openai/codex"
$NpmPackageFallback = "codex-cli"
$GitRepo = "https://github.com/FlexNetOS/codex.git"
$ProviderConfigPath = Join-Path $NoaRoot "ai/providers/cloud/codex"

Write-Log "Installing Codex CLI..." -Level Info
Write-Log "  Method: $Method" -Level Info
Write-Log "  NOA Root: $NoaRoot" -Level Info

# Ensure directories exist
$optPath = Join-Path $NoaRoot "opt"
$binPath = Join-Path $NoaRoot "bin"
$nodePath = Join-Path $optPath "node"
$devToolsDir = Join-Path $optPath "dev-tools"

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
        $npmPrefix = Join-Path $devToolsDir "npm-global"
        $npmCache = Join-Path $optPath "npm-cache"
        New-Item -ItemType Directory -Force -Path $npmPrefix, $npmCache | Out-Null
        $env:npm_config_prefix = $npmPrefix
        $env:npm_config_cache = $npmCache

        # Try @openai/codex first, fallback to codex-cli
        try {
            & $npmCmd install -g $NpmPackage 2>$null
        } catch {
            Write-Log "  $NpmPackage not found, trying $NpmPackageFallback..." -Level Info
            & $npmCmd install -g $NpmPackageFallback
        }

        if ($LASTEXITCODE -ne 0) {
            Write-Log "npm install failed" -Level Error
            exit 1
        }

        # Create symlink in bin/
        $codexBin = Join-Path $npmPrefix "node_modules\.bin\codex.cmd"
        $codexLink = Join-Path $binPath "codex.cmd"
        if (Test-Path $codexBin) {
            Copy-Item $codexBin $codexLink -Force
            Write-Log "  Created: bin/codex.cmd" -Level Success
        }
    }

    "clone" {
        Write-Log "Cloning FlexNetOS fork..." -Level Info

        $clonePath = Join-Path $optPath "codex"

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
$codexCmd = Get-Command codex -ErrorAction SilentlyContinue
if ($codexCmd) {
    $version = & codex --version 2>&1
    Write-Log "  Codex CLI installed: $version" -Level Success
} else {
    # Check portable location
    $portableCodex = Join-Path $devToolsDir "npm-global\node_modules\.bin\codex.cmd"
    if (Test-Path $portableCodex) {
        Write-Log "  Codex CLI installed to portable location" -Level Success
        Write-Log "  Add to PATH: $(Split-Path $portableCodex -Parent)" -Level Info
    } else {
        Write-Log "  Codex CLI not found in PATH" -Level Warning
    }
}

# Ensure provider config exists
$configFile = Join-Path $ProviderConfigPath "config.json"
if (-not (Test-Path $configFile)) {
    Write-Log "Creating provider config..." -Level Info
    # Config is created by the main setup, but ensure directory exists
}

Write-Log "Codex CLI installation complete!" -Level Success
Write-Log "  Provider config: $ProviderConfigPath" -Level Info
Write-Log "  Shared resources: $NoaRoot\ai\shared" -Level Info

exit 0

