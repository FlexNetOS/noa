<#
.SYNOPSIS
    Install jq JSON processor to noa_root/bin/

.DESCRIPTION
    Downloads jq binary from GitHub releases and installs to NOA bin directory.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Version
    jq version to install (default: latest)

.EXAMPLE
    .\jq.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "1.7.1"
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))
    }
}

$BIN_DIR = Join-Path $NoaRoot "bin"
$CACHE_DIR = Join-Path $NoaRoot "cache/downloads"

# Ensure directories exist
@($BIN_DIR, $CACHE_DIR) | ForEach-Object {
    if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ -Force | Out-Null }
}

$TOOL_NAME = "jq"
$TARGET_PATH = Join-Path $BIN_DIR "jq.exe"

Write-Host "Installing $TOOL_NAME v$Version..." -ForegroundColor Cyan

# Check if already installed
if (Test-Path $TARGET_PATH) {
    try {
        $currentVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
        Write-Host "  [EXISTS] jq already installed: $currentVersion" -ForegroundColor Gray
        return
    } catch {
        Write-Host "  [WARN] Existing jq binary is invalid, reinstalling..." -ForegroundColor Yellow
    }
}

# Download URL for Windows
$downloadUrl = "https://github.com/jqlang/jq/releases/download/jq-$Version/jq-windows-amd64.exe"
$downloadPath = Join-Path $CACHE_DIR "jq-$Version.exe"

Write-Host "  Downloading from: $downloadUrl" -ForegroundColor Gray

# Download with progress
try {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $downloadUrl -OutFile $downloadPath -UseBasicParsing
    $ProgressPreference = 'Continue'
} catch {
    Write-Error "Failed to download jq: $_"
    exit 1
}

# Copy to bin directory
Copy-Item -Path $downloadPath -Destination $TARGET_PATH -Force

Write-Host "  [OK] Installed jq to $TARGET_PATH" -ForegroundColor Green

# Verify installation
try {
    $installedVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] Verified: $installedVersion" -ForegroundColor Green
} catch {
    Write-Warning "Installation may have failed - could not verify jq"
}

