<#
.SYNOPSIS
    Install Trivy to noa_root/bin/

.DESCRIPTION
    Downloads and installs Trivy (vulnerability scanner) from GitHub releases.
    Per NOA Constitution 3.6: Security compliance.

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

$BinDir = Join-Path $NoaRoot "bin"
$TempDir = Join-Path $NoaRoot "tmp"

# Trivy version and URL
$Version = "0.58.0"
$Url = "https://github.com/aquasecurity/trivy/releases/download/v$Version/trivy_${Version}_Windows-64bit.zip"

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
Write-Host "NOA Trivy Installer (Security)" -ForegroundColor Cyan
Write-Host "Constitution 3.6 Compliance" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

$targetBin = Join-Path $BinDir "trivy.exe"

# Check if already installed
if ((Test-Path $targetBin) -and -not $Force) {
    $version = & $targetBin version 2>&1 | Select-Object -First 1
    Write-Log "Trivy already installed: $version" -Level Success
    exit 0
}

# Create directories
foreach ($dir in @($BinDir, $TempDir)) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
}

# Download
$zipPath = Join-Path $TempDir "trivy_${Version}_Windows-64bit.zip"
Write-Log "Downloading Trivy $Version..." -Level Info
try {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $Url -OutFile $zipPath -UseBasicParsing
    Write-Log "Downloaded successfully" -Level Success
} catch {
    Write-Log "Failed to download Trivy: $_" -Level Error
    exit 1
}

# Extract
Write-Log "Extracting..." -Level Info
$extractDir = Join-Path $TempDir "trivy-extract"
Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force

# Move binary
$extractedBin = Join-Path $extractDir "trivy.exe"
if (Test-Path $extractedBin) {
    Copy-Item -Path $extractedBin -Destination $targetBin -Force
    Remove-Item -Path $extractDir -Recurse -Force
    Write-Log "Installed to: $targetBin" -Level Success
} else {
    Write-Log "Binary not found in archive" -Level Error
    exit 1
}

# Verify
$version = & $targetBin version 2>&1 | Select-Object -First 1
Write-Log "Trivy $version installed" -Level Success

exit 0

