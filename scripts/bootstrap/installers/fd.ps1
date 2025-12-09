<#
.SYNOPSIS
    Install fd (find alternative) to noa_root/bin/

.DESCRIPTION
    Downloads fd binary from GitHub releases and installs to NOA bin directory.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Version
    fd version to install (default: latest stable)

.EXAMPLE
    .\fd.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "10.2.0"
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

$TOOL_NAME = "fd"
$TARGET_PATH = Join-Path $BIN_DIR "fd.exe"

Write-Host "Installing $TOOL_NAME v$Version..." -ForegroundColor Cyan

# Check if already installed
if (Test-Path $TARGET_PATH) {
    try {
        $currentVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
        Write-Host "  [EXISTS] fd already installed: $currentVersion" -ForegroundColor Gray
        return
    } catch {
        Write-Host "  [WARN] Existing fd binary is invalid, reinstalling..." -ForegroundColor Yellow
    }
}

# Download URL for Windows
$archiveName = "fd-v$Version-x86_64-pc-windows-msvc"
$downloadUrl = "https://github.com/sharkdp/fd/releases/download/v$Version/$archiveName.zip"
$downloadPath = Join-Path $CACHE_DIR "$archiveName.zip"
$extractPath = Join-Path $CACHE_DIR $archiveName

Write-Host "  Downloading from: $downloadUrl" -ForegroundColor Gray

# Download
try {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $downloadUrl -OutFile $downloadPath -UseBasicParsing
    $ProgressPreference = 'Continue'
} catch {
    Write-Error "Failed to download fd: $_"
    exit 1
}

# Extract archive
Write-Host "  Extracting..." -ForegroundColor Gray
if (Test-Path $extractPath) { Remove-Item -Path $extractPath -Recurse -Force }
Expand-Archive -Path $downloadPath -DestinationPath $CACHE_DIR -Force

# Copy binary to bin directory
$sourceBinary = Join-Path $extractPath "fd.exe"
Copy-Item -Path $sourceBinary -Destination $TARGET_PATH -Force

Write-Host "  [OK] Installed fd to $TARGET_PATH" -ForegroundColor Green

# Verify installation
try {
    $installedVersion = & $TARGET_PATH --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] Verified: $installedVersion" -ForegroundColor Green
} catch {
    Write-Warning "Installation may have failed - could not verify fd"
}

