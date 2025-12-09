<#
.SYNOPSIS
    Configure Docker for NOA integration.

.DESCRIPTION
    Verifies Docker installation and creates configuration for NOA.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\docker.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$BIN_DIR = Join-Path $NoaRoot "bin"
$CONFIG_DIR = Join-Path $NoaRoot "config"

Write-Host "Configuring Docker integration..." -ForegroundColor Cyan

# Check if docker is available
$dockerCmd = Get-Command docker -ErrorAction SilentlyContinue
if (-not $dockerCmd) {
    Write-Host "  [SKIP] Docker not found" -ForegroundColor Yellow
    Write-Host "  Install Docker Desktop from: https://www.docker.com/products/docker-desktop" -ForegroundColor Gray
    exit 0
}

Write-Host "  Found: $($dockerCmd.Source)" -ForegroundColor Green

# Verify Docker daemon is running
try {
    $dockerInfo = & docker info 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  [WARN] Docker found but daemon is not running" -ForegroundColor Yellow
        Write-Host "  Start Docker Desktop to enable container features" -ForegroundColor Gray
    } else {
        Write-Host "  [OK] Docker daemon is running" -ForegroundColor Green
    }
} catch {
    Write-Host "  [WARN] Could not verify Docker daemon status" -ForegroundColor Yellow
}

# Get Docker version
try {
    $version = & docker --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] Version: $version" -ForegroundColor Green
} catch {
    Write-Host "  [WARN] Could not get Docker version" -ForegroundColor Yellow
}

# Check docker-compose
$composeCmd = Get-Command docker-compose -ErrorAction SilentlyContinue
if ($composeCmd) {
    Write-Host "  [OK] docker-compose available" -ForegroundColor Green
} else {
    # Try docker compose (v2 plugin)
    try {
        $composeV2 = & docker compose version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [OK] docker compose (v2) available" -ForegroundColor Green
        }
    } catch {
        Write-Host "  [INFO] docker-compose not found (optional)" -ForegroundColor Gray
    }
}

# Create NOA Docker config directory
$noaDockerConfig = Join-Path $NoaRoot "etc/docker"
if (-not (Test-Path $noaDockerConfig)) {
    New-Item -ItemType Directory -Path $noaDockerConfig -Force | Out-Null
    Write-Host "  [OK] Created Docker config dir: $noaDockerConfig" -ForegroundColor Green
}

Write-Host ""
Write-Host "Docker integration configured." -ForegroundColor Green

