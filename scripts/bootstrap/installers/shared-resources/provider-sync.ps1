<#
.SYNOPSIS
    Sync shared resources across AI providers (B058t).

.DESCRIPTION
    Updates resource mappings and ensures all providers have access to shared resources.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\provider-sync.ps1
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

Write-Host "Syncing provider shared resources..." -ForegroundColor Cyan

$providersDir = Join-Path $NoaRoot "ai/providers"
$sharedDir = Join-Path $NoaRoot "ai/shared"

# Provider categories
$providerTypes = @("local", "cloud", "hybrid", "ide")

# Shared resource types
$resourceTypes = @("prompts", "agents", "tools", "commands", "workflows")

foreach ($pType in $providerTypes) {
    $pTypeDir = Join-Path $providersDir $pType
    if (-not (Test-Path $pTypeDir)) {
        continue
    }

    # Find all provider configss
    $configss = Get-ChildItem -Path $pTypeDir -Filter "configs.json" -Recurse -ErrorAction SilentlyContinue

    foreach ($configs in $configss) {
        $providerName = (Split-Path -Parent $configs.FullName) | Split-Path -Leaf
        Write-Host "  Updating: $pType/$providerName" -ForegroundColor Gray

        try {
            $configsContent = Get-Content $configs.FullName -Raw | ConvertFrom-Json

            # Ensure sharedResources section exists and points to correct paths
            $updated = $false

            if (-not $configsContent.sharedResourcePath) {
                $configsContent | Add-Member -NotePropertyName "sharedResourcePath" -NotePropertyValue "`${NOA_ROOT}/ai/shared" -Force
                $updated = $true
            }

            if (-not $configsContent.sharedResources) {
                $sharedResources = @{}
                foreach ($rt in $resourceTypes) {
                    $sharedResources[$rt] = "`${NOA_ROOT}/ai/shared/$rt"
                }
                $sharedResources["executionMemory"] = "`${NOA_ROOT}/ai/shared/resources/execution-memory.db"

                $configsContent | Add-Member -NotePropertyName "sharedResources" -NotePropertyValue $sharedResources -Force
                $updated = $true
            }

            if ($updated) {
                $configsContent | ConvertTo-Json -Depth 10 | Set-Content -Path $configs.FullName -Encoding UTF8
                Write-Host "    [OK] Updated configs" -ForegroundColor Green
            } else {
                Write-Host "    [OK] Already configsured" -ForegroundColor Gray
            }
        } catch {
            Write-Host "    [!!] Failed to update: $_" -ForegroundColor Yellow
        }
    }
}

# Update main ai-providers.json
$mainconfigsPath = Join-Path $NoaRoot "configs/ai-providers.json"
if (Test-Path $mainconfigsPath) {
    try {
        $mainconfigs = Get-Content $mainconfigsPath -Raw | ConvertFrom-Json

        if (-not $mainconfigs.sharedResources.commands) {
            $mainconfigs.sharedResources | Add-Member -NotePropertyName "commands" -NotePropertyValue "`${NOA_ROOT}/ai/shared/commands" -Force
            $mainconfigs.sharedResources | Add-Member -NotePropertyName "resources" -NotePropertyValue "`${NOA_ROOT}/ai/shared/resources" -Force

            $mainconfigs | ConvertTo-Json -Depth 10 | Set-Content -Path $mainconfigsPath -Encoding UTF8
            Write-Host "  [OK] Updated main ai-providers.json" -ForegroundColor Green
        }
    } catch {
        Write-Host "  [!!] Failed to update main configs: $_" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Provider sync complete." -ForegroundColor Green
