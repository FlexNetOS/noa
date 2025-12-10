<#
.SYNOPSIS
    Validate AI provider configuration files against Category 14 quality requirements.

.DESCRIPTION
    Checks all provider config files for compliance with CHK122-CHK130 requirements:
    - CHK122: name, type, priority, enabled, description
    - CHK123: cli (command, package, version, binaryPath)
    - CHK124: modes (cli, cloud, ide where applicable)
    - CHK125: capabilities object
    - CHK126: sharedResources paths
    - CHK127: latency targets and timeout
    - CHK128: priority uniqueness
    - CHK129: binaryPath uses ${NOA_ROOT} syntax
    - CHK130: sharedResources paths consistent

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Json
    Output results in JSON format

.PARAMETER Fix
    Automatically fix common issues (adds missing fields with defaults)

.EXAMPLE
    .\validate-provider-configs.ps1
    .\validate-provider-configs.ps1 -Json
    .\validate-provider-configs.ps1 -Fix
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Json,
    [switch]$Fix
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        $scriptRoot = Split-Path -Parent $PSScriptRoot
        Split-Path -Parent (Split-Path -Parent $scriptRoot)
    }
}

$PROVIDERS_DIR = Join-Path $NoaRoot "ai/providers"

# Standard shared resources paths (using single quotes to prevent variable expansion)
$STANDARD_SHARED_RESOURCES = @{
    "prompts" = '${NOA_ROOT}/ai/shared/prompts'
    "agents" = '${NOA_ROOT}/ai/shared/agents'
    "tools" = '${NOA_ROOT}/ai/shared/tools'
    "commands" = '${NOA_ROOT}/ai/shared/commands'
    "executionMemory" = '${NOA_ROOT}/ai/shared/resources/execution-memory.db'
}

$results = @{
    timestamp = (Get-Date -Format "o")
    noa_root = $NoaRoot
    providers_dir = $PROVIDERS_DIR
    checks = @{}
    summary = @{
        total = 0
        passed = 0
        failed = 0
        fixed = 0
    }
    configErrors = @()
    configWarnings = @()
}

# Find all provider config files
$configFiles = @()
$categories = @("local", "cloud", "hybrid", "ide")

foreach ($category in $categories) {
    $categoryDir = Join-Path $PROVIDERS_DIR $category
    if (Test-Path $categoryDir) {
        $providers = Get-ChildItem -Path $categoryDir -Directory
        foreach ($provider in $providers) {
            $configPath = Join-Path $provider.FullName "config.json"
            if (Test-Path $configPath) {
                $configFiles += @{
                    path = $configPath
                    category = $category
                    name = $provider.Name
                }
            }
        }
        # Also check for direct JSON files in category directory
        $jsonFiles = Get-ChildItem -Path $categoryDir -Filter "*.json" -File
        foreach ($jsonFile in $jsonFiles) {
            $configFiles += @{
                path = $jsonFile.FullName
                category = $category
                name = $jsonFile.BaseName
            }
        }
    }
}

$results.summary.total = $configFiles.Count
$allPriorities = @{}
$allSharedResources = @{}

# Validate each config file
foreach ($configFile in $configFiles) {
    $providerName = $configFile.name
    $configPath = $configFile.path
    $providerResults = @{
        file = $configPath
        name = $providerName
        category = $configFile.category
        checks = @{}
        configErrors = @()
        configWarnings = @()
        fixed = @()
    }

    try {
        $config = Get-Content -Path $configPath -Raw | ConvertFrom-Json

        # CHK122: name, type, priority, enabled, description
        $chk122 = @{
            name = $null -ne $config.name
            type = $null -ne $config.type
            priority = $null -ne $config.priority
            enabled = $null -ne $config.enabled
            description = $null -ne $config.description
        }
        $providerResults.checks.CHK122 = $chk122

        if (-not $chk122.description -and $Fix) {
            $config | Add-Member -NotePropertyName "description" -NotePropertyValue "AI provider: $providerName" -Force
            $providerResults.fixed += "Added missing description"
        }

        # CHK123: cli (command, package, version, binaryPath)
        $chk123 = @{
            hasCli = $null -ne $config.cli
        }
        # Check for top-level command or nested command structure (e.g., llama-cpp has llama-server.command)
        if ($chk123.hasCli) {
            $chk123.hasCommand = ($null -ne $config.cli.command) -or
                                ($config.cli.PSObject.Properties | Where-Object { $_.Value.PSObject.Properties.Name -contains 'command' } | Measure-Object).Count -gt 0
            $chk123.hasBinaryPath = ($null -ne $config.cli.binaryPath) -or
                                   ($config.cli.PSObject.Properties | Where-Object { $_.Value.PSObject.Properties.Name -contains 'binaryPath' } | Measure-Object).Count -gt 0
        } else {
            $chk123.hasCommand = $false
            $chk123.hasBinaryPath = $false
        }
        # package and version are optional for some providers
        $chk123.hasPackage = $null -ne $config.cli.package
        $chk123.hasVersion = $null -ne $config.cli.version
        $providerResults.checks.CHK123 = $chk123

        # CHK124: modes (cli, cloud, ide where applicable)
        $chk124 = @{
            hasModes = $null -ne $config.modes
        }
        if ($chk124.hasModes) {
            $chk124.hasCliMode = $null -ne $config.modes.cli
            $chk124.hasCloudMode = $null -ne $config.modes.cloud
            $chk124.hasIdeMode = $null -ne $config.modes.ide
        }
        $providerResults.checks.CHK124 = $chk124

        # CHK125: capabilities object
        $chk125 = @{
            hasCapabilities = $null -ne $config.capabilities
        }
        $providerResults.checks.CHK125 = $chk125

        if (-not $chk125.hasCapabilities -and $Fix) {
            $config | Add-Member -NotePropertyName "capabilities" -NotePropertyValue @{} -Force
            $providerResults.fixed += "Added missing capabilities object"
        }

        # CHK126: sharedResources paths
        $chk126 = @{
            hasSharedResources = $null -ne $config.sharedResources
        }
        if ($chk126.hasSharedResources) {
            $chk126.hasExecutionMemory = $null -ne $config.sharedResources.executionMemory
        }
        $providerResults.checks.CHK126 = $chk126

        if (-not $chk126.hasSharedResources -and $Fix) {
            $config | Add-Member -NotePropertyName "sharedResources" -NotePropertyValue ($STANDARD_SHARED_RESOURCES.Clone()) -Force
            $providerResults.fixed += "Added missing sharedResources"
        }

        # CHK127: latency targets and timeout
        $chk127 = @{
            hasLatency = $null -ne $config.latency
            hasTimeout = $null -ne $config.timeout
        }
        if ($chk127.hasLatency) {
            $chk127.hasTarget = $null -ne $config.latency.target
        }
        $providerResults.checks.CHK127 = $chk127

        if ((-not $chk127.hasLatency -or -not $chk127.hasTimeout) -and $Fix) {
            if (-not $chk127.hasLatency) {
                $config | Add-Member -NotePropertyName "latency" -NotePropertyValue @{ target = "<2s" } -Force
                $providerResults.fixed += "Added missing latency"
            }
            if (-not $chk127.hasTimeout) {
                $config | Add-Member -NotePropertyName "timeout" -NotePropertyValue 30000 -Force
                $providerResults.fixed += "Added missing timeout"
            }
        }

        # CHK128: priority uniqueness (tracked across all providers)
        if ($config.priority) {
            if ($allPriorities.ContainsKey($config.priority)) {
                $providerResults.configErrors += "Priority $($config.priority) is duplicate (also used by $($allPriorities[$config.priority]))"
            } else {
                $allPriorities[$config.priority] = $providerName
            }
        }

        # CHK129: binaryPath uses ${NOA_ROOT} syntax
        $chk129 = @{
            valid = $true
            issues = @()
        }
        if ($config.cli -and $config.cli.binaryPath) {
            $binaryPathStr = if ($config.cli.binaryPath -is [string]) {
                $config.cli.binaryPath
            } elseif ($config.cli.binaryPath -is [PSCustomObject]) {
                ($config.cli.binaryPath.PSObject.Properties | ForEach-Object { $_.Value }) -join " "
            } else {
                ""
            }
            if ($binaryPathStr -and $binaryPathStr -notmatch '\$\{NOA_ROOT\}') {
                $chk129.valid = $false
                $chk129.issues += "binaryPath does not use ${NOA_ROOT} syntax: $binaryPathStr"
            }
        }
        $providerResults.checks.CHK129 = $chk129

        # CHK130: sharedResources paths consistent
        $chk130 = @{
            consistent = $true
            issues = @()
        }
        if ($config.sharedResources) {
            foreach ($key in $STANDARD_SHARED_RESOURCES.Keys) {
                if ($config.sharedResources.$key) {
                    $expected = $STANDARD_SHARED_RESOURCES[$key]
                    $actual = $config.sharedResources.$key
                    if ($actual -ne $expected) {
                        $chk130.consistent = $false
                        $chk130.issues += "${key}: expected '$expected', got '$actual'"
                    }
                    # Track for consistency check across providers
                    if (-not $allSharedResources.ContainsKey($key)) {
                        $allSharedResources[$key] = @()
                    }
                    $allSharedResources[$key] += $actual
                }
            }
        }
        $providerResults.checks.CHK130 = $chk130

        # Save fixed config if Fix mode
        if ($Fix -and $providerResults.fixed.Count -gt 0) {
            $config | ConvertTo-Json -Depth 10 | Set-Content -Path $configPath -Encoding UTF8
            $results.summary.fixed++
        }

        # Determine overall status - check each requirement and report specific failures
        $failedChecks = @()
        if (-not ($chk122.name -and $chk122.type -and $chk122.priority -and $chk122.enabled -and $chk122.description)) {
            $failedChecks += "CHK122: Missing required fields (name, type, priority, enabled, description)"
        }
        if (-not ($chk123.hasCli -and $chk123.hasCommand -and $chk123.hasBinaryPath)) {
            $failedChecks += "CHK123: Missing CLI fields (cli, command, binaryPath)"
        }
        if (-not $chk125.hasCapabilities) {
            $failedChecks += "CHK125: Missing capabilities object"
        }
        if (-not $chk126.hasSharedResources) {
            $failedChecks += "CHK126: Missing sharedResources"
        }
        if (-not ($chk127.hasLatency -and $chk127.hasTimeout)) {
            $failedChecks += "CHK127: Missing latency or timeout"
        }
        if (-not $chk129.valid) {
            $failedChecks += "CHK129: binaryPath does not use ${NOA_ROOT} syntax"
            $failedChecks += $chk129.issues
        }
        if (-not $chk130.consistent) {
            $failedChecks += "CHK130: sharedResources paths inconsistent"
            $failedChecks += $chk130.issues
        }

        $allChecksPass = $failedChecks.Count -eq 0 -and $providerResults.configErrors.Count -eq 0

        if ($allChecksPass) {
            $results.summary.passed++
        } else {
            $results.summary.failed++
            $providerResults.configErrors += $failedChecks
        }

    } catch {
        $results.summary.failed++
        $providerResults.configErrors += "Failed to parse config: $_"
    }

    $results.checks[$providerName] = $providerResults
}

# Final CHK128 check: verify no duplicate priorities
$duplicatePriorities = $allPriorities.GetEnumerator() | Group-Object -Property Value | Where-Object { $_.Count -gt 1 }
if ($duplicatePriorities) {
    $results.errors += "CHK128: Duplicate priorities found: $($duplicatePriorities | ForEach-Object { "$($_.Name): $($_.Group -join ', ')" })"
}

# Final CHK130 check: verify sharedResources consistency across all providers
foreach ($key in $allSharedResources.Keys) {
    $uniqueValues = $allSharedResources[$key] | Select-Object -Unique
    if ($uniqueValues.Count -gt 1) {
        $results.warnings += "CHK130: Inconsistent sharedResources.$key across providers: $($uniqueValues -join ', ')"
    }
}

# Output results
if ($Json) {
    $results | ConvertTo-Json -Depth 10
} else {
    Write-Host "Provider Config Validation Results" -ForegroundColor Cyan
    Write-Host "===================================" -ForegroundColor Cyan
    Write-Host "NOA Root: $NoaRoot"
    Write-Host "Providers Dir: $PROVIDERS_DIR"
    Write-Host ""
    Write-Host "Summary:" -ForegroundColor Yellow
    Write-Host "  Total: $($results.summary.total)"
    Write-Host "  Passed: $($results.summary.passed)" -ForegroundColor Green
    Write-Host "  Failed: $($results.summary.failed)" -ForegroundColor $(if ($results.summary.failed -gt 0) { "Red" } else { "Green" })
    if ($Fix) {
        Write-Host "  Fixed: $($results.summary.fixed)" -ForegroundColor Cyan
    }
    Write-Host ""

    foreach ($providerName in $results.checks.Keys | Sort-Object) {
        $provider = $results.checks[$providerName]
        $status = if ($provider.configErrors.Count -eq 0) { "PASS" } else { "FAIL" }
        $color = if ($status -eq "PASS") { "Green" } else { "Red" }
        Write-Host "$providerName ($status)" -ForegroundColor $color
        if ($provider.configErrors.Count -gt 0) {
            foreach ($err in $provider.configErrors) {
                Write-Host "  ERROR: $err" -ForegroundColor Red
            }
        }
        if ($provider.configWarnings.Count -gt 0) {
            foreach ($warning in $provider.configWarnings) {
                Write-Host "  WARNING: $warning" -ForegroundColor Yellow
            }
        }
        if ($Fix -and $provider.fixed.Count -gt 0) {
            foreach ($fix in $provider.fixed) {
                Write-Host "  FIXED: $fix" -ForegroundColor Cyan
            }
        }
    }

    if ($results.errors.Count -gt 0) {
        Write-Host ""
        Write-Host "Global Errors:" -ForegroundColor Red
        foreach ($err in $results.errors) {
            Write-Host "  $err" -ForegroundColor Red
        }
    }

    if ($results.warnings.Count -gt 0) {
        Write-Host ""
        Write-Host "Global Warnings:" -ForegroundColor Yellow
        foreach ($warning in $results.warnings) {
            Write-Host "  $warning" -ForegroundColor Yellow
        }
    }
}

# Exit with error code if any failures
if ($results.summary.failed -gt 0 -or $results.errors.Count -gt 0) {
    exit 1
}
exit 0

