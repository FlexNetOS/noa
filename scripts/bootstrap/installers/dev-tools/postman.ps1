<#
.SYNOPSIS
    Detect Postman API client for NOA integration.

.DESCRIPTION
    Detects Postman installation and creates workspace configsuration.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\postman.ps1
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

Write-Host "Checking Postman installation..." -ForegroundColor Cyan

# Common Postman installation paths
$postmanPaths = @(
    "$env:LOCALAPPDATA\Postman\Postman.exe"
    "$env:LOCALAPPDATA\Programs\Postman\Postman.exe"
    "C:\Program Files\Postman\Postman.exe"
    "$env:USERPROFILE\AppData\Local\Postman\app-*\Postman.exe"
)

$postmanExe = $null
foreach ($path in $postmanPaths) {
    # Handle wildcard paths
    $resolved = Resolve-Path -Path $path -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($resolved -and (Test-Path $resolved)) {
        $postmanExe = $resolved.Path
        break
    }
}

if ($postmanExe) {
    Write-Host "  [OK] Postman found: $postmanExe" -ForegroundColor Green

    # Create NOA workspace template
    $workspaceDir = Join-Path $NoaRoot "workspace/tools/postman"
    if (-not (Test-Path $workspaceDir)) {
        New-Item -ItemType Directory -Path $workspaceDir -Force | Out-Null
        Write-Host "  [OK] Created Postman workspace directory: $workspaceDir" -ForegroundColor Green
    }

    # Create collection template
    $collectionPath = Join-Path $workspaceDir "noa-api-collection.json"
    if (-not (Test-Path $collectionPath)) {
        $collection = @{
            info = @{
                name = "NOA API Collection"
                description = "API endpoints for NOA services"
                schema = "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
            }
            item = @(
                @{
                    name = "Health Check"
                    request = @{
                        method = "GET"
                        url = "{{base_url}}/health"
                    }
                }
            )
            variable = @(
                @{
                    key = "base_url"
                    value = "http://localhost:8080"
                }
            )
        }

        $collection | ConvertTo-Json -Depth 10 | Set-Content -Path $collectionPath -Encoding UTF8
        Write-Host "  [OK] Created API collection template: $collectionPath" -ForegroundColor Green
    }
} else {
    Write-Host "  [SKIP] Postman not found" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  To install Postman:" -ForegroundColor Gray
    Write-Host "    1. Visit https://www.postman.com/downloads/" -ForegroundColor Gray
    Write-Host "    2. Download the Windows installer" -ForegroundColor Gray
    Write-Host "    3. Run the installer" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Postman check complete." -ForegroundColor Green

