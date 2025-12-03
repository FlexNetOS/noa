<#
  Simplified NOA Setup Script for Windows 11
  This version uses straightforward PowerShell without complex string building
#>

param(
    [string]$NoaRoot = "N:\noa"
)

$ErrorActionPreference = "Stop"

Write-Host "=== NOA Setup Starting ===" -ForegroundColor Cyan
Write-Host "Root: $NoaRoot" -ForegroundColor White

# Create base directories
$dirs = @(
    "repos", "containers", "workspace", "config", "scripts",
    "logs", "tmp", "p2p", "ai", "git", "bin"
)

Write-Host "`nCreating directories..." -ForegroundColor Cyan
foreach ($dir in $dirs) {
    $path = Join-Path $NoaRoot $dir
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
        Write-Host "  Created: $dir" -ForegroundColor Green
    } else {
        Write-Host "  Exists: $dir" -ForegroundColor Gray
    }
}

# Create profile file using simple array join
Write-Host "`nCreating profile..." -ForegroundColor Cyan
$profilePath = Join-Path $NoaRoot "noa-profile.ps1"

# Build profile content as array of lines, then join them
$profileLines = @(
    '# NOA Environment Profile'
    ''
    "`$env:NOA_ROOT = `"$NoaRoot`""
    '$env:NOA_REPOS = "$env:NOA_ROOT\repos"'
    '$env:NOA_CONTAINERS = "$env:NOA_ROOT\containers"'
    '$env:NOA_WORKSPACE = "$env:NOA_ROOT\workspace"'
    '$env:NOA_CONFIG = "$env:NOA_ROOT\config"'
    '$env:NOA_SCRIPTS = "$env:NOA_ROOT\scripts"'
    ''
    'function cda { Set-Location $env:NOA_ROOT }'
    'function cdr { Set-Location $env:NOA_REPOS }'
    'function cdw { Set-Location $env:NOA_WORKSPACE }'
    ''
    'Write-Host "NOA environment loaded" -ForegroundColor Green'
)

# Write to file
$profileLines -join "`r`n" | Set-Content -Path $profilePath -Encoding UTF8
Write-Host "  Created: noa-profile.ps1" -ForegroundColor Green

# Create marker file
Write-Host "`nCreating marker..." -ForegroundColor Cyan
$markerPath = Join-Path $NoaRoot ".noa"
"# NOA Root Directory" | Set-Content -Path $markerPath -Encoding UTF8
Write-Host "  Created: .noa" -ForegroundColor Green

# Create basic config file
Write-Host "`nCreating config..." -ForegroundColor Cyan
$configPath = Join-Path $NoaRoot "config\noa.json"
@'
{
  "version": "1.0.0",
  "name": "NOA"
}
'@ | Set-Content -Path $configPath -Encoding UTF8
Write-Host "  Created: config\noa.json" -ForegroundColor Green

Write-Host "`n=== Setup Complete ===" -ForegroundColor Green
Write-Host "`nTo load the environment:" -ForegroundColor Yellow
Write-Host "  . .\noa-profile.ps1" -ForegroundColor White
Write-Host ""
