<#
.SYNOPSIS
    Install ripgrep (rg) to noa_root/bin/

.DESCRIPTION
    Downloads ripgrep binary from GitHub releases and installs to NOA bin directory.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Version
    ripgrep version to install (default: latest stable)

.EXAMPLE
    .\ripgrep.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "14.1.1"
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

$TOOL_NAME = "ripgrep"
$TARGET_PATH = Join-Path $BIN_DIR "rg.exe"

Write-Host "Installing $TOOL_NAME v$Version..." -ForegroundColor Cyan

# Check if already installed
if (Test-Path $TARGET_PATH) {
    try {
        $currentVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
        Write-Host "  [EXISTS] ripgrep already installed: $currentVersion" -ForegroundColor Gray
        return
    } catch {
        Write-Host "  [WARN] Existing rg binary is invalid, reinstalling..." -ForegroundColor Yellow
    }
}

# Download URL for Windows
$archiveName = "ripgrep-$Version-x86_64-pc-windows-msvc"
$downloadUrl = "https://github.com/BurntSushi/ripgrep/releases/download/$Version/$archiveName.zip"
$downloadPath = Join-Path $CACHE_DIR "$archiveName.zip"
$extractPath = Join-Path $CACHE_DIR $archiveName

Write-Host "  Downloading from: $downloadUrl" -ForegroundColor Gray

# Download with progress
try {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $downloadUrl -OutFile $downloadPath -UseBasicParsing
    $ProgressPreference = 'Continue'
} catch {
    Write-Error "Failed to download ripgrep: $_"
    exit 1
}

# Extract archive
Write-Host "  Extracting..." -ForegroundColor Gray
if (Test-Path $extractPath) { Remove-Item -Path $extractPath -Recurse -Force }
Expand-Archive -Path $downloadPath -DestinationPath $CACHE_DIR -Force

# Copy binary to bin directory
$sourceBinary = Join-Path $extractPath "rg.exe"
Copy-Item -Path $sourceBinary -Destination $TARGET_PATH -Force

Write-Host "  [OK] Installed ripgrep to $TARGET_PATH" -ForegroundColor Green

# Verify installation
try {
    $installedVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] Verified: $installedVersion" -ForegroundColor Green
} catch {
    Write-Warning "Installation may have failed - could not verify ripgrep"
}

