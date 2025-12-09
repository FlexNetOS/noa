<#
.SYNOPSIS
    Install Go quality tools (golangci-lint) via go install

.DESCRIPTION
    Installs Go tools to the portable Go installation.
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

$GoRoot = Join-Path $NoaRoot "opt/go"
$GoBin = Join-Path $GoRoot "bin"
$GoPath = Join-Path $GoRoot "workspace"
$GoPathBin = Join-Path $GoPath "bin"

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
Write-Host "NOA Go Tools Installer" -ForegroundColor Cyan
Write-Host "Installing: golangci-lint" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if go is available
$goBinary = Join-Path $GoBin "go.exe"
if (-not (Test-Path $goBinary)) {
    # Try system go
    $goBinary = Get-Command go -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
    if (-not $goBinary) {
        Write-Log "go not found. Please install Go first." -Level Error
        Write-Log "Run: scripts/bootstrap/installers/go-portable.ps1" -Level Info
        exit 1
    }
}

# Set environment for portable Go
$env:GOROOT = $GoRoot
$env:GOPATH = $GoPath
$env:GOBIN = $GoPathBin
$env:GOCACHE = Join-Path $GoRoot "cache"
$env:GOMODCACHE = Join-Path $GoRoot "pkg/mod"

# Create GOBIN if not exists
if (-not (Test-Path $GoPathBin)) {
    New-Item -ItemType Directory -Path $GoPathBin -Force | Out-Null
}

# Install golangci-lint
Write-Log "Installing golangci-lint..." -Level Info
try {
    & $goBinary install github.com/golangci/golangci-lint/cmd/golangci-lint@latest 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Log "Installed: golangci-lint" -Level Success
    } else {
        Write-Log "Failed to install golangci-lint" -Level Warning
    }
} catch {
    Write-Log "Error installing golangci-lint: $_" -Level Warning
}

# Verify installation
Write-Host ""
Write-Log "Verifying installations..." -Level Info

$golangciLintBin = Join-Path $GoPathBin "golangci-lint.exe"
if (Test-Path $golangciLintBin) {
    $version = & $golangciLintBin --version 2>&1 | Select-Object -First 1
    Write-Log "golangci-lint: $version" -Level Success

    # Copy to bin/
    $binDir = Join-Path $NoaRoot "bin"
    Copy-Item -Path $golangciLintBin -Destination (Join-Path $binDir "golangci-lint.exe") -Force
    Write-Log "Copied to bin/golangci-lint.exe" -Level Success
} else {
    Write-Log "golangci-lint not found" -Level Warning
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "Go tools installation complete!" -ForegroundColor Green
Write-Host "=" * 60 -ForegroundColor Green

exit 0

