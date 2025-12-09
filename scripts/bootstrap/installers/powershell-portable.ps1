<#
.SYNOPSIS
    Install portable PowerShell to noa_root/opt/powershell

.DESCRIPTION
    Downloads and extracts PowerShell portable to maintain NOA Constitution §3.1 containment.
    All NOA operations should use this portable PowerShell instead of system PowerShell.

.NOTES
    Per NOA Constitution §3.1: Self-Contained Installation
    "All dependencies, caches, and configurations reside within noa_root."
#>

[CmdletBinding()]
param(
    [string]$NoaRoot = (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))),
    [string]$Version = "7.5.1"
)

$ErrorActionPreference = "Stop"

# Paths
$OptDir = Join-Path $NoaRoot "opt"
$PwshDir = Join-Path $OptDir "powershell"
$TempDir = Join-Path $NoaRoot "tmp"
$CacheDir = Join-Path $NoaRoot "cache"

Write-Host "=== NOA Portable PowerShell Installer ===" -ForegroundColor Cyan
Write-Host "NOA Root: $NoaRoot"
Write-Host "Target: $PwshDir"
Write-Host "Version: $Version"
Write-Host ""

# Create directories
New-Item -ItemType Directory -Force -Path $OptDir, $TempDir, $CacheDir | Out-Null

# Check if already installed
$PwshExe = Join-Path $PwshDir "pwsh.exe"
if (Test-Path $PwshExe) {
    $installedVersion = & $PwshExe -NoProfile -Command '$PSVersionTable.PSVersion.ToString()'
    Write-Host "[EXISTS] PowerShell $installedVersion already installed" -ForegroundColor Green
    exit 0
}

# Download URL for Windows x64
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "x86" }
$ZipName = "PowerShell-$Version-win-$Arch.zip"
$DownloadUrl = "https://github.com/PowerShell/PowerShell/releases/download/v$Version/$ZipName"
$ZipPath = Join-Path $CacheDir $ZipName

Write-Host "[DOWNLOAD] $DownloadUrl" -ForegroundColor Yellow

# Download if not cached
if (-not (Test-Path $ZipPath)) {
    try {
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing
        Write-Host "[OK] Downloaded to cache" -ForegroundColor Green
    } catch {
        Write-Host "[ERROR] Failed to download: $_" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "[CACHED] Using cached download" -ForegroundColor Green
}

# Extract
Write-Host "[EXTRACT] Extracting to $PwshDir..." -ForegroundColor Yellow
if (Test-Path $PwshDir) {
    Remove-Item -Recurse -Force $PwshDir
}
New-Item -ItemType Directory -Force -Path $PwshDir | Out-Null
Expand-Archive -Path $ZipPath -DestinationPath $PwshDir -Force

# Verify
if (Test-Path $PwshExe) {
    $installedVersion = & $PwshExe -NoProfile -Command '$PSVersionTable.PSVersion.ToString()'
    Write-Host "[OK] PowerShell $installedVersion installed successfully" -ForegroundColor Green

    # Create wrapper in bin
    $BinDir = Join-Path $NoaRoot "bin"
    New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

    $WrapperPath = Join-Path $BinDir "pwsh.cmd"
    @"
@echo off
REM NOA Portable PowerShell Wrapper
REM Per NOA Constitution §3.1: Self-Contained Installation
"$PwshDir\pwsh.exe" %*
"@ | Set-Content -Path $WrapperPath -Encoding ASCII

    Write-Host "[OK] Created wrapper at $WrapperPath" -ForegroundColor Green
} else {
    Write-Host "[ERROR] Installation failed - pwsh.exe not found" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "=== Installation Complete ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "To use portable PowerShell:" -ForegroundColor Yellow
Write-Host "  N:\noa\opt\powershell\pwsh.exe" -ForegroundColor Gray
Write-Host "  or" -ForegroundColor Gray
Write-Host "  N:\noa\bin\pwsh.cmd" -ForegroundColor Gray

