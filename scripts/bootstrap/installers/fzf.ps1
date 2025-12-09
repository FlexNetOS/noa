<#
.SYNOPSIS
    Install fzf (fuzzy finder) to noa_root/bin/

.DESCRIPTION
    Downloads fzf binary from GitHub releases and installs to NOA bin directory.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Version
    fzf version to install (default: latest stable)

.EXAMPLE
    .\fzf.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "0.56.3"
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

$TOOL_NAME = "fzf"
$TARGET_PATH = Join-Path $BIN_DIR "fzf.exe"

Write-Host "Installing $TOOL_NAME v$Version..." -ForegroundColor Cyan

# Check if already installed
if (Test-Path $TARGET_PATH) {
    try {
        $currentVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
        Write-Host "  [EXISTS] fzf already installed: $currentVersion" -ForegroundColor Gray
        return
    } catch {
        Write-Host "  [WARN] Existing fzf binary is invalid, reinstalling..." -ForegroundColor Yellow
    }
}

# Download URL for Windows
$archiveName = "fzf-$Version-windows_amd64"
$downloadUrl = "https://github.com/junegunn/fzf/releases/download/v$Version/$archiveName.zip"
$downloadPath = Join-Path $CACHE_DIR "$archiveName.zip"
$extractPath = Join-Path $CACHE_DIR "fzf-$Version"

Write-Host "  Downloading from: $downloadUrl" -ForegroundColor Gray

# Download
try {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $downloadUrl -OutFile $downloadPath -UseBasicParsing
    $ProgressPreference = 'Continue'
} catch {
    Write-Error "Failed to download fzf: $_"
    exit 1
}

# Extract archive
Write-Host "  Extracting..." -ForegroundColor Gray
if (Test-Path $extractPath) { Remove-Item -Path $extractPath -Recurse -Force }
New-Item -ItemType Directory -Path $extractPath -Force | Out-Null
Expand-Archive -Path $downloadPath -DestinationPath $extractPath -Force

# Copy binary to bin directory
$sourceBinary = Join-Path $extractPath "fzf.exe"
Copy-Item -Path $sourceBinary -Destination $TARGET_PATH -Force

Write-Host "  [OK] Installed fzf to $TARGET_PATH" -ForegroundColor Green

# Verify installation
try {
    $installedVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] Verified: $installedVersion" -ForegroundColor Green
} catch {
    Write-Warning "Installation may have failed - could not verify fzf"
}

