<#
.SYNOPSIS
    Add NOA environment to PowerShell profile.

.DESCRIPTION
    Adds a line to source noa-env.ps1 to your PowerShell profile,
    ensuring NOA environment is loaded on every shell startup.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER DryRun
    Show what would be done without making changes

.EXAMPLE
    .\shell-integration.ps1
    .\shell-integration.ps1 -DryRun
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$noaEnvPath = Join-Path $NoaRoot "noa-env.ps1"
$profilePath = $PROFILE.CurrentUserAllHosts

Write-Host "NOA Shell Integration" -ForegroundColor Cyan
Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
Write-Host "Profile:  $profilePath" -ForegroundColor Gray
Write-Host ""

# Check if noa-env.ps1 exists
if (-not (Test-Path $noaEnvPath)) {
    Write-Host "[WARN] noa-env.ps1 not found. Generate it first:" -ForegroundColor Yellow
    Write-Host "  .\scripts\bootstrap\generators\noa-env.ps1" -ForegroundColor Gray
    exit 0
}

# Line to add to profile
$sourceLine = ". `"$noaEnvPath`""

# Check if already integrated
if (Test-Path $profilePath) {
    $profileContent = Get-Content $profilePath -Raw
    if ($profileContent -match [regex]::Escape($noaEnvPath)) {
        Write-Host "[OK] NOA environment already integrated in profile" -ForegroundColor Green
        return
    }
}

if ($DryRun) {
    Write-Host "[DRY RUN] Would add to $profilePath:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  # NOA Environment" -ForegroundColor Gray
    Write-Host "  $sourceLine" -ForegroundColor Gray
    return
}

# Create profile directory if needed
$profileDir = Split-Path -Parent $profilePath
if (-not (Test-Path $profileDir)) {
    New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
}

# Append to profile
$addition = @"

# NOA Environment (added by shell-integration.ps1)
$sourceLine
"@

Add-Content -Path $profilePath -Value $addition -Encoding UTF8

Write-Host "[OK] Added NOA environment to profile" -ForegroundColor Green
Write-Host ""
Write-Host "Reload your shell or run:" -ForegroundColor Yellow
Write-Host "  . `"$noaEnvPath`"" -ForegroundColor Cyan

