<#
.SYNOPSIS
    Installs portable Ninja build tool to NOA_ROOT/opt/ninja/

.DESCRIPTION
    Downloads and installs Ninja as a portable build tool within the NOA contained environment.
    Creates symlink in NOA_ROOT/bin/ for easy access.

    Constitutional Compliance: §3.1 Self-Contained & Autonomous

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from NOA_ROOT env or script location)

.PARAMETER Version
    Ninja version to install (default: 1.12.1)

.PARAMETER Force
    Force reinstallation even if already installed

.EXAMPLE
    .\ninja-portable.ps1
    .\ninja-portable.ps1 -Version 1.12.1 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "1.12.1",
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) {
        $env:NOA_ROOT
    } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$NOA_BIN = Join-Path $NoaRoot "bin"
$NOA_OPT = Join-Path $NoaRoot "opt"
$NOA_CACHE = Join-Path $NoaRoot "cache"
$NINJA_DIR = Join-Path $NOA_OPT "ninja"

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
Write-Host "NOA Portable Ninja Installer" -ForegroundColor Cyan
Write-Host "Version: $Version | Target: $NINJA_DIR" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$ninjaExe = Join-Path $NINJA_DIR "ninja.exe"
if ((Test-Path $ninjaExe) -and -not $Force) {
    $currentVersion = & $ninjaExe --version 2>&1
    Write-Log "Ninja already installed: v$currentVersion" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info
    exit 0
}

# Ensure directories exist
New-Item -ItemType Directory -Path $NOA_CACHE -Force | Out-Null
New-Item -ItemType Directory -Path $NINJA_DIR -Force | Out-Null
New-Item -ItemType Directory -Path $NOA_BIN -Force | Out-Null

# Download Ninja
$downloadUrl = "https://github.com/ninja-build/ninja/releases/download/v$Version/ninja-win.zip"
$zipFile = Join-Path $NOA_CACHE "ninja-$Version-win.zip"

Write-Log "Downloading Ninja $Version..." -Level Info
try {
    if (-not (Test-Path $zipFile)) {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipFile -UseBasicParsing
        Write-Log "Downloaded: $([math]::Round((Get-Item $zipFile).Length / 1KB, 1)) KB" -Level Success
    } else {
        Write-Log "Using cached download" -Level Info
    }
} catch {
    Write-Log "Download failed: $_" -Level Error
    exit 1
}

# Extract
Write-Log "Extracting to $NINJA_DIR..." -Level Info
try {
    Expand-Archive -Path $zipFile -DestinationPath $NINJA_DIR -Force
    Write-Log "Extracted to $NINJA_DIR" -Level Success
} catch {
    Write-Log "Extraction failed: $_" -Level Error
    exit 1
}

# Create symlink in bin/
Write-Log "Creating symlink in $NOA_BIN..." -Level Info
$target = Join-Path $NINJA_DIR "ninja.exe"
$link = Join-Path $NOA_BIN "ninja.exe"
if (Test-Path $target) {
    Remove-Item $link -Force -ErrorAction SilentlyContinue
    New-Item -ItemType SymbolicLink -Path $link -Target $target -Force | Out-Null
    Write-Log "Linked ninja.exe" -Level Success
}

# Verify installation
Write-Log "Verifying installation..." -Level Info
$installedVersion = & $ninjaExe --version 2>&1
if ($installedVersion) {
    Write-Log "Ninja installed successfully: v$installedVersion" -Level Success
} else {
    Write-Log "Installation verification failed" -Level Error
    exit 1
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "Ninja $Version installed successfully!" -ForegroundColor Green
Write-Host "Location: $NINJA_DIR" -ForegroundColor Gray
Write-Host "Symlink: $NOA_BIN\ninja.exe" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Green

exit 0
