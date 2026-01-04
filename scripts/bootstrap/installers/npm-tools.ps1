<#
.SYNOPSIS
    Install npm quality tools (eslint, prettier, typescript)

.DESCRIPTION
    Installs npm-based development tools to the portable Node.js installation.
    Per NOA Constitution 3.1: Self-contained installation.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall even if already installed
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT }
    else { Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))) }
}

$NodeRoot = Join-Path $NoaRoot "opt/node"
$BinDir = Join-Path $NoaRoot "bin"

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        default { "White" }
    }
    $prefix = switch ($Level) {
        "Success" { "[OK]" }
        "Warning" { "[!!]" }
        "Error" { "[XX]" }
        default { "[..]" }
    }
    Write-Host "$prefix $Message" -ForegroundColor $color
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA npm Tools Installer" -ForegroundColor Cyan
Write-Host "Installing: eslint, prettier, typescript" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if npm is available
$npmBin = Join-Path $NodeRoot "npm.cmd"
if (-not (Test-Path $npmBin)) {
    # Try system npm
    $npmBin = Get-Command npm -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
    if (-not $npmBin) {
        Write-Log "npm not found. Please install Node.js first." -Level Error
        Write-Log "Run: scripts/bootstrap/installers/node-portable.ps1" -Level Info
        exit 1
    }
}

# Set npm prefix for portable installation
$env:npm_configs_prefix = $NodeRoot
$env:npm_configs_cache = Join-Path $NoaRoot "opt/npm-cache"

# Tools to install
$tools = @(
    "eslint",
    "prettier",
    "typescript"
)

foreach ($tool in $tools) {
    Write-Log "Installing $tool..." -Level Info
    try {
        & $npmBin install -g $tool 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Log "Installed: $tool" -Level Success
        } else {
            Write-Log "Failed to install $tool" -Level Warning
        }
    } catch {
        Write-Log "Error installing $tool : $_" -Level Warning
    }
}

# Verify installations
Write-Host ""
Write-Log "Verifying installations..." -Level Info

$verifyTools = @(
    @{ Name = "eslint"; Bin = "eslint.cmd" },
    @{ Name = "prettier"; Bin = "prettier.cmd" },
    @{ Name = "tsc"; Bin = "tsc.cmd" }
)

foreach ($t in $verifyTools) {
    $toolBin = Join-Path $NodeRoot $t.Bin
    if (Test-Path $toolBin) {
        $version = & $toolBin --version 2>&1
        Write-Log "$($t.Name): $version" -Level Success
    } else {
        # Check in node_modules/.bin
        $altBin = Join-Path $NodeRoot "node_modules/.bin/$($t.Bin)"
        if (Test-Path $altBin) {
            Write-Log "$($t.Name): installed" -Level Success
        } else {
            Write-Log "$($t.Name) not found" -Level Warning
        }
    }
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "npm tools installation complete!" -ForegroundColor Green
Write-Host "=" * 60 -ForegroundColor Green

exit 0

