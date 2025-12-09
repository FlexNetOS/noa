<#
.SYNOPSIS
    Install Abacus AI CLI to noa_root/opt/

.DESCRIPTION
    Installs the Abacus AI CLI (@abacus-ai/cli) via npm.
    Requires Abacus Desktop app for initial authentication.

    Provider Config: ai/providers/cloud/abacus/config.json

.PARAMETER NoaRoot
    Root directory for NOA installation. Defaults to N:\noa

.PARAMETER Method
    Installation method: npm (default) or manual
    - npm: Install via npm to opt/node/node_modules/
    - manual: Provide installation instructions

.EXAMPLE
    .\abacus-cli.ps1
    Install Abacus CLI via npm
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory=$false)]
    [string]$NoaRoot = "N:\noa",

    [Parameter(Mandatory=$false)]
    [ValidateSet("npm", "manual")]
    [string]$Method = "npm"
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
$NpmPackage = "@abacus-ai/cli"
$ProviderConfigPath = Join-Path $NoaRoot "ai/providers/cloud/abacus"

Write-Log "Installing Abacus AI CLI..." -Level Info
Write-Log "  Method: $Method" -Level Info
Write-Log "  NOA Root: $NoaRoot" -Level Info

# Ensure directories exist
$optPath = Join-Path $NoaRoot "opt"
$binPath = Join-Path $NoaRoot "bin"
$nodePath = Join-Path $optPath "node"
$devToolsPath = Join-Path $optPath "dev-tools"
$npmGlobalPath = Join-Path $devToolsPath "npm-global"

foreach ($dir in @($optPath, $binPath, $ProviderConfigPath, $npmGlobalPath)) {
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

        # Set npm prefix
        $npmCache = Join-Path $optPath "npm-cache"
        New-Item -ItemType Directory -Force -Path $npmGlobalPath, $npmCache | Out-Null
        $env:npm_config_prefix = $npmGlobalPath
        $env:npm_config_cache = $npmCache

        # Install abacus-ai
        try {
            & $npmCmd install -g $NpmPackage 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-Log "Installed: $NpmPackage" -Level Success
            } else {
                Write-Log "npm install returned exit code $LASTEXITCODE" -Level Warning
            }
        } catch {
            Write-Log "Failed to install $NpmPackage : $_" -Level Warning
        }

        # Create symlink in bin/
        $abacusBin = Join-Path $npmGlobalPath "node_modules\.bin\abacusai.cmd"
        $abacusLink = Join-Path $binPath "abacusai.cmd"
        if (Test-Path $abacusBin) {
            Copy-Item $abacusBin $abacusLink -Force
            Write-Log "  Created: bin/abacusai.cmd" -Level Success
        }
    }

    "manual" {
        Write-Log "Manual installation instructions:" -Level Info
        Write-Host ""
        Write-Host "Option 1 - npm:" -ForegroundColor Cyan
        Write-Host "  npm install -g @abacus-ai/cli" -ForegroundColor White
        Write-Host ""
        Write-Host "Option 2 - Desktop App:" -ForegroundColor Cyan
        Write-Host "  Download from: https://desktop.abacus.ai/" -ForegroundColor White
        Write-Host ""
        Write-Host "Note: CLI requires Abacus Desktop for authentication" -ForegroundColor Yellow
    }
}

# Verify installation
Write-Log "Verifying installation..." -Level Info
$abacusCmd = Get-Command abacusai -ErrorAction SilentlyContinue
if ($abacusCmd) {
    Write-Log "  Abacus AI CLI installed" -Level Success
} else {
    $portableAbacus = Join-Path $npmGlobalPath "node_modules\.bin\abacusai.cmd"
    if (Test-Path $portableAbacus) {
        Write-Log "  Abacus AI CLI installed to portable location" -Level Success
        Write-Log "  Add to PATH: $(Split-Path $portableAbacus -Parent)" -Level Info
    } else {
        Write-Log "  Abacus AI CLI not found in PATH" -Level Warning
    }
}

Write-Log "Abacus AI CLI installation complete!" -Level Success
Write-Log "  Provider config: $ProviderConfigPath" -Level Info
Write-Log "  Shared resources: $NoaRoot\ai\shared" -Level Info
Write-Log ""
Write-Log "Important: Download Abacus Desktop from https://desktop.abacus.ai/ for authentication" -Level Warning

exit 0


