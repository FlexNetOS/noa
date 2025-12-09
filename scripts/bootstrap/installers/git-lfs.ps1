<#
.SYNOPSIS
    Git LFS installer for NOA bootstrap (Windows)

.DESCRIPTION
    Installs Git Large File Storage extension.
    Per NOA Constitution §3.1: Self-contained installation.

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
    $NoaRoot = if ($env:NOA_ROOT) {
        $env:NOA_ROOT
    } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$NOA_BIN = Join-Path $NoaRoot "bin"
$TempDir = Join-Path $NoaRoot "tmp"
$GIT_LFS_VERSION = "3.4.1"

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
Write-Host "NOA Git LFS Installer" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host ""

# Check if already installed
$GitLfsPath = Join-Path $NOA_BIN "git-lfs.exe"
if ((Test-Path $GitLfsPath) -and -not $Force) {
    $version = & $GitLfsPath --version 2>&1
    Write-Log "Git LFS already installed: $version" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info
    exit 0
}

# Create directories
foreach ($dir in @($NOA_BIN, $TempDir)) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
}

# Download URL
$DownloadUrl = "https://github.com/git-lfs/git-lfs/releases/download/v${GIT_LFS_VERSION}/git-lfs-windows-amd64-v${GIT_LFS_VERSION}.zip"
$ArchiveName = "git-lfs-windows-amd64-v${GIT_LFS_VERSION}.zip"
$ArchivePath = Join-Path $TempDir $ArchiveName

Write-Log "Downloading Git LFS v$GIT_LFS_VERSION..." -Level Info

try {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath -UseBasicParsing
    Write-Log "Downloaded: $ArchiveName" -Level Success
} catch {
    Write-Log "Failed to download Git LFS: $_" -Level Error
    exit 1
}

# Extract
Write-Log "Extracting..." -Level Info
$ExtractDir = Join-Path $TempDir "git-lfs-extract"
if (Test-Path $ExtractDir) {
    Remove-Item $ExtractDir -Recurse -Force
}

Expand-Archive -Path $ArchivePath -DestinationPath $ExtractDir -Force

# Find and copy binary
$GitLfsBin = Get-ChildItem -Path $ExtractDir -Filter "git-lfs.exe" -Recurse | Select-Object -First 1
if (-not $GitLfsBin) {
    Write-Log "Could not find git-lfs.exe in archive" -Level Error
    exit 1
}

Copy-Item -Path $GitLfsBin.FullName -Destination $GitLfsPath -Force

# Initialize Git LFS
$gitCmd = Get-Command git -ErrorAction SilentlyContinue
if ($gitCmd) {
    & $GitLfsPath install --skip-smudge 2>&1 | Out-Null
}

# Verify
$version = & $GitLfsPath --version 2>&1
Write-Log "Git LFS installed: $version" -Level Success
Write-Log "Location: $GitLfsPath" -Level Info

# Cleanup
Remove-Item $ExtractDir -Recurse -Force -ErrorAction SilentlyContinue

