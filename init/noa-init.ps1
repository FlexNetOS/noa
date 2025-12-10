# NOA Initialization Script (Windows)
#
# T091: Create Windows init script noa-init.ps1
# US1: Initialize NOA Seed Environment
# §3.1: Self-Contained & Autonomous

param(
    [string]$NoaRoot = $env:NOA_ROOT,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Determine NOA root
if (-not $NoaRoot) {
    $NoaRoot = $PSScriptRoot
}

Write-Host "Initializing NOA at: $NoaRoot" -ForegroundColor Cyan

# Create directory structure
Write-Host "Creating directory structure..." -ForegroundColor Yellow
$directories = @(
    "sys\core",
    "sys\services",
    "sys\ui",
    "sys\digest",
    "sys\kernel",
    "p2p",
    "opt",
    "init\bootstrap",
    "init\migrations",
    "init\seeds",
    "init\services",
    "containers",
    "config\schemas",
    "config\templates",
    "bin",
    "ai\providers",
    "ai\shared",
    "data\memory",
    "data\knowledge",
    "data\embeddings",
    "data\artifacts",
    "data\modules",
    "data\state",
    "data\cache",
    "data\backups",
    "logs",
    "tmp"
)

foreach ($dir in $directories) {
    $path = Join-Path $NoaRoot $dir
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
        Write-Host "  Created: $dir" -ForegroundColor Green
    }
}

Write-Host "`n✓ Directory structure created" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  1. Run 'noa init' to complete initialization"
Write-Host "  2. Configure providers in config/ai-providers.json"
Write-Host "  3. Run 'noa start' to start services"

