<#
.SYNOPSIS
    Install GitHub CLI to noa_root/opt/ (Windows portable)

.DESCRIPTION
    Downloads and installs GitHub CLI (gh) to the NOA environment.
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

$GhRoot = Join-Path $NoaRoot "opt/gh"
$BinDir = Join-Path $NoaRoot "bin"
$TempDir = Join-Path $NoaRoot "tmp"
$StateFile = Join-Path $GhRoot ".installed.json"

# GitHub CLI URL
$GhVersion = "2.62.0"
$GhUrl = "https://github.com/cli/cli/releases/download/v$GhVersion/gh_${GhVersion}_windows_amd64.zip"

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

function Test-GhInstalled {
    if (-not (Test-Path $StateFile)) { return $false }
    $ghBin = Join-Path $GhRoot "bin/gh.exe"
    return Test-Path $ghBin
}

# Check if portable gh is already installed
if ((Test-GhInstalled) -and -not $Force) {
    Write-Log "Portable GitHub CLI already installed in noa_root" -Level Success
    exit 0
}

# Check if system gh is available (informational only)
$systemGh = Get-Command gh -ErrorAction SilentlyContinue
if ($systemGh -and -not $Force) {
    $version = & gh --version 2>&1 | Select-Object -First 1
    Write-Log "System GitHub CLI available: $version" -Level Info
    Write-Log "Installing portable version to noa_root for §3.1 compliance..." -Level Info
    # Continue with installation instead of exiting
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable GitHub CLI Installer" -ForegroundColor Cyan
Write-Host "Constitution 3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host "Target:   $GhRoot" -ForegroundColor White
Write-Host "Version:  $GhVersion" -ForegroundColor White
Write-Host ""

# Check if already installed
if ((Test-GhInstalled) -and -not $Force) {
    Write-Log "GitHub CLI is already installed in noa_root" -Level Success
    exit 0
}

# Create directories
foreach ($dir in @($GhRoot, $BinDir, $TempDir)) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Log "Created directory: $dir" -Level Success
    }
}

# Download
$zipPath = Join-Path $TempDir "gh_${GhVersion}_windows_amd64.zip"
if (-not (Test-Path $zipPath)) {
    Write-Log "Downloading GitHub CLI $GhVersion..." -Level Info
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $GhUrl -OutFile $zipPath -UseBasicParsing
        Write-Log "Downloaded: gh_${GhVersion}_windows_amd64.zip" -Level Success
    } catch {
        Write-Log "Failed to download GitHub CLI: $_" -Level Error
        exit 1
    }
}

# Extract
Write-Log "Extracting GitHub CLI to $GhRoot..." -Level Info
if ($Force -and (Test-Path $GhRoot)) {
    Remove-Item -Path $GhRoot -Recurse -Force -ErrorAction SilentlyContinue
}
$extractDir = Join-Path $TempDir "gh-extract"
Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force

# Move from nested folder
$nestedDir = Get-ChildItem -Path $extractDir -Directory | Select-Object -First 1
if ($nestedDir) {
    if (-not (Test-Path $GhRoot)) {
        New-Item -ItemType Directory -Path $GhRoot -Force | Out-Null
    }
    # Copy all contents from nested directory
    Get-ChildItem -Path $nestedDir.FullName | ForEach-Object {
        Copy-Item -Path $_.FullName -Destination $GhRoot -Recurse -Force
    }
    Remove-Item -Path $extractDir -Recurse -Force
} else {
    # No nested directory, files are at root of extract
    if (-not (Test-Path $GhRoot)) {
        New-Item -ItemType Directory -Path $GhRoot -Force | Out-Null
    }
    Get-ChildItem -Path $extractDir | ForEach-Object {
        Copy-Item -Path $_.FullName -Destination $GhRoot -Recurse -Force
    }
    Remove-Item -Path $extractDir -Recurse -Force
}
Write-Log "Extracted GitHub CLI successfully" -Level Success

# Verify - check both bin/gh.exe and gh.exe at root
$ghBin = Join-Path $GhRoot "bin/gh.exe"
if (-not (Test-Path $ghBin)) {
    $ghBin = Join-Path $GhRoot "gh.exe"
}
if (Test-Path $ghBin) {
    $version = & $ghBin --version 2>&1 | Select-Object -First 1
    Write-Log "Installed: $version" -Level Success

    # Create symlink in bin/ (preferred) or copy if symlink fails
    $binGh = Join-Path $BinDir "gh.exe"
    $absGhBin = (Resolve-Path $ghBin).Path
    try {
        if (Test-Path $binGh) {
            Remove-Item $binGh -Force -ErrorAction SilentlyContinue
        }
        New-Item -ItemType SymbolicLink -Path $binGh -Target $absGhBin -Force | Out-Null
        Write-Log "Created symlink: bin/gh.exe -> $absGhBin" -Level Success
    } catch {
        # Fallback to copy if symlink creation fails (permissions, etc.)
        Copy-Item -Path $ghBin -Destination $binGh -Force
        Write-Log "Copied to bin/gh.exe (symlink failed)" -Level Warning
    }
} else {
    Write-Log "gh binary not found after extraction" -Level Error
    exit 1
}

# Save state
$state = @{
    version = $GhVersion
    installed_at = (Get-Date -Format "o")
    path = $GhRoot
}
$state | ConvertTo-Json | Set-Content -Path $StateFile -Encoding UTF8
Write-Log "Installation state saved" -Level Success

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "GitHub CLI $GhVersion installed successfully!" -ForegroundColor Green
Write-Host "=" * 60 -ForegroundColor Green
Write-Host ""
Write-Host '# Add to PATH:' -ForegroundColor Cyan
Write-Host "`$env:PATH = `"$GhRoot\bin;`$env:PATH`"" -ForegroundColor White

exit 0

