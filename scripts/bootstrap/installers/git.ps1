<#
.SYNOPSIS
    Install Git to noa_root/opt/ (Windows portable)

.DESCRIPTION
    Downloads and installs Git portable for Windows to the NOA environment.
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

$GitRoot = Join-Path $NoaRoot "opt/git"
$TempDir = Join-Path $NoaRoot "tmp"
$StateFile = Join-Path $GitRoot ".installed.json"

# Git portable URL (MinGit)
$GitVersion = "2.47.1"
$GitUrl = "https://github.com/git-for-windows/git/releases/download/v$GitVersion.windows.1/MinGit-$GitVersion-64-bit.zip"

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

function Test-GitInstalled {
    if (-not (Test-Path $StateFile)) { return $false }
    $gitBin = Join-Path $GitRoot "cmd/git.exe"
    return Test-Path $gitBin
}

# Check if system git is available
$systemGit = Get-Command git -ErrorAction SilentlyContinue
if ($systemGit -and -not $Force) {
    $version = & git --version 2>&1
    Write-Log "Git already available (system): $version" -Level Success
    Write-Log "Use -Force to install portable version to noa_root" -Level Info
    exit 0
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable Git Installer" -ForegroundColor Cyan
Write-Host "Constitution 3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host "Target:   $GitRoot" -ForegroundColor White
Write-Host "Version:  $GitVersion" -ForegroundColor White
Write-Host ""

# Check if already installed
if ((Test-GitInstalled) -and -not $Force) {
    Write-Log "Git is already installed in noa_root" -Level Success
    exit 0
}

# Create directories
foreach ($dir in @($GitRoot, $TempDir)) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Log "Created directory: $dir" -Level Success
    }
}

# Download
$zipPath = Join-Path $TempDir "mingit-$GitVersion.zip"
if (-not (Test-Path $zipPath)) {
    Write-Log "Downloading Git $GitVersion..." -Level Info
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $GitUrl -OutFile $zipPath -UseBasicParsing
        Write-Log "Downloaded: mingit-$GitVersion.zip" -Level Success
    } catch {
        Write-Log "Failed to download Git: $_" -Level Error
        exit 1
    }
}

# Extract
Write-Log "Extracting Git to $GitRoot..." -Level Info
if ($Force -and (Test-Path $GitRoot)) {
    Remove-Item -Path $GitRoot -Recurse -Force -ErrorAction SilentlyContinue
    New-Item -ItemType Directory -Path $GitRoot -Force | Out-Null
}
Expand-Archive -Path $zipPath -DestinationPath $GitRoot -Force
Write-Log "Extracted Git successfully" -Level Success

# Verify
$gitBin = Join-Path $GitRoot "cmd/git.exe"
if (Test-Path $gitBin) {
    $version = & $gitBin --version 2>&1
    Write-Log "Installed: $version" -Level Success
} else {
    Write-Log "Git binary not found after extraction" -Level Error
    exit 1
}

# Save state
$state = @{
    version = $GitVersion
    installed_at = (Get-Date -Format "o")
    path = $GitRoot
}
$state | ConvertTo-Json | Set-Content -Path $StateFile -Encoding UTF8
Write-Log "Installation state saved" -Level Success

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "Git $GitVersion installed successfully!" -ForegroundColor Green
Write-Host "=" * 60 -ForegroundColor Green
Write-Host ""
Write-Host '# Add to PATH:' -ForegroundColor Cyan
Write-Host "`$env:PATH = `"$GitRoot\cmd;`$env:PATH`"" -ForegroundColor White

exit 0

