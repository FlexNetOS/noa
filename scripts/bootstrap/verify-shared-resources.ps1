<#
.SYNOPSIS
    Verify shared AI resources are properly configsured.

.DESCRIPTION
    Checks that all shared resource directories exist and contain valid configsurations.
    Validates resource registry and provider configss reference shared paths.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Json
    Output results in JSON format

.EXAMPLE
    .\verify-shared-resources.ps1
    .\verify-shared-resources.ps1 -Json
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Json
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    }
}

$SHARED_DIR = Join-Path $NoaRoot "ai/shared"

# Expected directories
$expectedDirs = @(
    @{ Name = "agents"; Required = $true },
    @{ Name = "workflows"; Required = $true },
    @{ Name = "prompts"; Required = $true },
    @{ Name = "skills"; Required = $true },
    @{ Name = "tools"; Required = $true },
    @{ Name = "models"; Required = $true },
    @{ Name = "commands"; Required = $true },
    @{ Name = "resources"; Required = $true },
    @{ Name = "resources/schema"; Required = $true }
)

# Expected files
$expectedFiles = @(
    @{ Path = "resources/resource-registry.json"; Required = $true },
    @{ Path = "resources/resource-aliases.json"; Required = $false },
    @{ Path = "resources/execution-memory.db"; Required = $false },
    @{ Path = "resources/schema/execution-memory.sql"; Required = $true }
)

$results = @{
    timestamp = (Get-Date -Format "o")
    noa_root = $NoaRoot
    shared_dir = $SHARED_DIR
    directories = @{}
    files = @{}
    providers_using_shared = @()
    summary = @{
        total_checks = 0
        passed = 0
        failed = 0
        warnings = 0
    }
}

if (-not $Json) {
    Write-Host "Verifying Shared AI Resources..." -ForegroundColor Cyan
    Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
    Write-Host "Shared Dir: $SHARED_DIR" -ForegroundColor Gray
    Write-Host ""
}

# Check directories
if (-not $Json) {
    Write-Host "Checking directories..." -ForegroundColor Yellow
}

foreach ($dir in $expectedDirs) {
    $results.summary.total_checks++
    $dirPath = Join-Path $SHARED_DIR $dir.Name
    $exists = Test-Path $dirPath -PathType Container

    $results.directories[$dir.Name] = @{
        path = $dirPath
        exists = $exists
        required = $dir.Required
    }

    if ($exists) {
        $results.summary.passed++
        if (-not $Json) {
            Write-Host "  [OK] $($dir.Name)/" -ForegroundColor Green
        }
    } elseif ($dir.Required) {
        $results.summary.failed++
        if (-not $Json) {
            Write-Host "  [FAIL] $($dir.Name)/ - MISSING (required)" -ForegroundColor Red
        }
    } else {
        $results.summary.warnings++
        if (-not $Json) {
            Write-Host "  [WARN] $($dir.Name)/ - missing (optional)" -ForegroundColor Yellow
        }
    }
}

# Check files
if (-not $Json) {
    Write-Host ""
    Write-Host "Checking files..." -ForegroundColor Yellow
}

foreach ($file in $expectedFiles) {
    $results.summary.total_checks++
    $filePath = Join-Path $SHARED_DIR $file.Path
    $exists = Test-Path $filePath -PathType Leaf

    $results.files[$file.Path] = @{
        path = $filePath
        exists = $exists
        required = $file.Required
    }

    if ($exists) {
        $results.summary.passed++
        if (-not $Json) {
            Write-Host "  [OK] $($file.Path)" -ForegroundColor Green
        }
    } elseif ($file.Required) {
        $results.summary.failed++
        if (-not $Json) {
            Write-Host "  [FAIL] $($file.Path) - MISSING (required)" -ForegroundColor Red
        }
    } else {
        $results.summary.warnings++
        if (-not $Json) {
            Write-Host "  [WARN] $($file.Path) - missing (optional)" -ForegroundColor Yellow
        }
    }
}

# Check provider configss reference shared resources
if (-not $Json) {
    Write-Host ""
    Write-Host "Checking provider configsurations..." -ForegroundColor Yellow
}

$providerDirs = @("local", "cloud", "hybrid", "ide")
$PROVIDERS_DIR = Join-Path $NoaRoot "ai/providers"

foreach ($providerType in $providerDirs) {
    $typeDir = Join-Path $PROVIDERS_DIR $providerType
    if (Test-Path $typeDir) {
        $configss = Get-ChildItem -Path $typeDir -Filter "configs.json" -Recurse -ErrorAction SilentlyContinue
        foreach ($configs in $configss) {
            try {
                $content = Get-Content $configs.FullName -Raw | ConvertFrom-Json
                $providerName = Split-Path -Parent $configs.FullName | Split-Path -Leaf

                $usesShared = $false
                if ($content.sharedResourcePath -or $content.sharedResources) {
                    $usesShared = $true
                }

                $results.providers_using_shared += @{
                    name = $providerName
                    type = $providerType
                    uses_shared = $usesShared
                    configs_path = $configs.FullName
                }

                if (-not $Json) {
                    if ($usesShared) {
                        Write-Host "  [OK] $providerName ($providerType) - uses shared resources" -ForegroundColor Green
                    } else {
                        Write-Host "  [WARN] $providerName ($providerType) - no shared resource reference" -ForegroundColor Yellow
                        $results.summary.warnings++
                    }
                }
            } catch {
                if (-not $Json) {
                    Write-Host "  [WARN] Could not parse: $($configs.FullName)" -ForegroundColor Yellow
                }
            }
        }
    }
}

# Summary
if (-not $Json) {
    Write-Host ""
    Write-Host "Summary:" -ForegroundColor Cyan
    Write-Host "  Total checks: $($results.summary.total_checks)" -ForegroundColor Gray
    Write-Host "  Passed: $($results.summary.passed)" -ForegroundColor Green
    Write-Host "  Failed: $($results.summary.failed)" -ForegroundColor $(if ($results.summary.failed -gt 0) { "Red" } else { "Gray" })
    Write-Host "  Warnings: $($results.summary.warnings)" -ForegroundColor $(if ($results.summary.warnings -gt 0) { "Yellow" } else { "Gray" })
    Write-Host ""

    if ($results.summary.failed -gt 0) {
        Write-Host "Some required resources are missing. Run:" -ForegroundColor Red
        Write-Host "  .\scripts\bootstrap\installers\shared-resources\create-directories.ps1" -ForegroundColor Yellow
    } else {
        Write-Host "All required shared resources are configsured." -ForegroundColor Green
    }
}

if ($Json) {
    $results | ConvertTo-Json -Depth 5
}

# Exit with error if required resources are missing
if ($results.summary.failed -gt 0) {
    exit 1
}

