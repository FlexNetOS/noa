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

    # Find all provider configs
    $configs = Get-ChildItem -Path $pTypeDir -Filter "config.json" -Recurse -ErrorAction SilentlyContinue

    foreach ($config in $configs) {
        $providerName = (Split-Path -Parent $config.FullName) | Split-Path -Leaf
        Write-Host "  Updating: $pType/$providerName" -ForegroundColor Gray

        try {
            $configContent = Get-Content $config.FullName -Raw | ConvertFrom-Json

            # Ensure sharedResources section exists and points to correct paths
            $updated = $false

            if (-not $configContent.sharedResourcePath) {
                $configContent | Add-Member -NotePropertyName "sharedResourcePath" -NotePropertyValue "`${NOA_ROOT}/ai/shared" -Force
                $updated = $true
            }

            if (-not $configContent.sharedResources) {
                $sharedResources = @{}
                foreach ($rt in $resourceTypes) {
                    $sharedResources[$rt] = "`${NOA_ROOT}/ai/shared/$rt"
                }
                $sharedResources["executionMemory"] = "`${NOA_ROOT}/ai/shared/resources/execution-memory.db"

                $configContent | Add-Member -NotePropertyName "sharedResources" -NotePropertyValue $sharedResources -Force
                $updated = $true
            }

            if ($updated) {
                $configContent | ConvertTo-Json -Depth 10 | Set-Content -Path $config.FullName -Encoding UTF8
                Write-Host "    [OK] Updated config" -ForegroundColor Green
            } else {
                Write-Host "    [OK] Already configured" -ForegroundColor Gray
            }
        } catch {
            Write-Host "    [!!] Failed to update: $_" -ForegroundColor Yellow
        }
    }
}

# Update main ai-providers.json
$mainConfigPath = Join-Path $NoaRoot "config/ai-providers.json"
if (Test-Path $mainConfigPath) {
    try {
        $mainConfig = Get-Content $mainConfigPath -Raw | ConvertFrom-Json

        if (-not $mainConfig.sharedResources.commands) {
            $mainConfig.sharedResources | Add-Member -NotePropertyName "commands" -NotePropertyValue "`${NOA_ROOT}/ai/shared/commands" -Force
            $mainConfig.sharedResources | Add-Member -NotePropertyName "resources" -NotePropertyValue "`${NOA_ROOT}/ai/shared/resources" -Force

            $mainConfig | ConvertTo-Json -Depth 10 | Set-Content -Path $mainConfigPath -Encoding UTF8
            Write-Host "  [OK] Updated main ai-providers.json" -ForegroundColor Green
        }
    } catch {
        Write-Host "  [!!] Failed to update main config: $_" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Provider sync complete." -ForegroundColor Green
