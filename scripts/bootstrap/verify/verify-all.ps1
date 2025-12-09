<#
.SYNOPSIS
    Verify entire NOA environment installation.

.DESCRIPTION
    Runs all verification checks to ensure NOA is properly installed and configured.
    Checks toolchains, AI providers, shared resources, and environment setup.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Json
    Output results in JSON format

.EXAMPLE
    .\verify-all.ps1
    .\verify-all.ps1 -Json
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
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))
    }
}

$SCRIPTS_DIR = Join-Path $NoaRoot "scripts"
$BOOTSTRAP_DIR = Join-Path $SCRIPTS_DIR "bootstrap"

$results = @{
    timestamp = (Get-Date -Format "o")
    noa_root = $NoaRoot
    checks = @{}
    summary = @{
        total = 0
        passed = 0
        failed = 0
        warnings = 0
    }
}

if (-not $Json) {
    Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "         NOA Environment Verification" -ForegroundColor Cyan
    Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
    Write-Host ""
}

# Verification functions
function Run-Check {
    param(
        [string]$Name,
        [string]$Description,
        [scriptblock]$Test
    )

    $results.summary.total++
    $check = @{
        name = $Name
        description = $Description
        status = "pending"
        details = $null
        error = $null
    }

    try {
        $result = & $Test
        if ($result.success) {
            $check.status = "passed"
            $check.details = $result.details
            $results.summary.passed++
            if (-not $Json) {
                Write-Host "  [PASS] $Description" -ForegroundColor Green
            }
        } else {
            $check.status = "failed"
            $check.error = $result.error
            $results.summary.failed++
            if (-not $Json) {
                Write-Host "  [FAIL] $Description - $($result.error)" -ForegroundColor Red
            }
        }
    } catch {
        $check.status = "error"
        $check.error = $_.Exception.Message
        $results.summary.failed++
        if (-not $Json) {
            Write-Host "  [ERR]  $Description - $($_.Exception.Message)" -ForegroundColor Red
        }
    }

    $results.checks[$Name] = $check
}

# 1. Directory Structure
if (-not $Json) { Write-Host "Checking directory structure..." -ForegroundColor Yellow }

$requiredDirs = @("bin", "config", "ai", "ai/shared", "ai/providers", "logs", "specs")
foreach ($dir in $requiredDirs) {
    Run-Check -Name "dir_$($dir -replace '/','-')" -Description "Directory: $dir" -Test {
        $path = Join-Path $NoaRoot $dir
        if (Test-Path $path -PathType Container) {
            return @{ success = $true; details = $path }
        } else {
            return @{ success = $false; error = "Not found" }
        }
    }
}

# 2. Core Tools
if (-not $Json) { Write-Host "" ; Write-Host "Checking core tools..." -ForegroundColor Yellow }

$coreTools = @(
    @{ Name = "git"; Cmd = "git --version" },
    @{ Name = "jq"; Cmd = "jq --version" },
    @{ Name = "rg"; Cmd = "rg --version" }
)

foreach ($tool in $coreTools) {
    Run-Check -Name "tool_$($tool.Name)" -Description "Tool: $($tool.Name)" -Test {
        $binPath = Join-Path $NoaRoot "bin/$($tool.Name).exe"
        if (Test-Path $binPath) {
            return @{ success = $true; details = "Found in NOA bin" }
        }
        $cmd = Get-Command $tool.Name -ErrorAction SilentlyContinue
        if ($cmd) {
            return @{ success = $true; details = $cmd.Source }
        }
        return @{ success = $false; error = "Not installed" }
    }
}

# 3. AI Provider Configs
if (-not $Json) { Write-Host "" ; Write-Host "Checking AI providers..." -ForegroundColor Yellow }

$verifyProviders = Join-Path $BOOTSTRAP_DIR "verify-ai-providers.ps1"
if (Test-Path $verifyProviders) {
    Run-Check -Name "ai_providers" -Description "AI provider verification" -Test {
        $null = & pwsh -NoLogo -NoProfile -File $verifyProviders -Json 2>&1
        $exitCode = $LASTEXITCODE
        if ($exitCode -eq 0) {
            return @{ success = $true; details = "All required providers configured" }
        } else {
            return @{ success = $false; error = "Some providers missing" }
        }
    }
}

# 4. Shared Resources
if (-not $Json) { Write-Host "" ; Write-Host "Checking shared resources..." -ForegroundColor Yellow }

$verifyShared = Join-Path $BOOTSTRAP_DIR "verify-shared-resources.ps1"
if (Test-Path $verifyShared) {
    Run-Check -Name "shared_resources" -Description "Shared resources verification" -Test {
        $null = & pwsh -NoLogo -NoProfile -File $verifyShared -Json 2>&1
        $exitCode = $LASTEXITCODE
        if ($exitCode -eq 0) {
            return @{ success = $true; details = "All shared resources configured" }
        } else {
            return @{ success = $false; error = "Some resources missing" }
        }
    }
}

# 5. Environment Variables
if (-not $Json) { Write-Host "" ; Write-Host "Checking environment..." -ForegroundColor Yellow }

Run-Check -Name "env_noa_root" -Description "NOA_ROOT environment variable" -Test {
    if ($env:NOA_ROOT) {
        return @{ success = $true; details = $env:NOA_ROOT }
    } else {
        return @{ success = $false; error = "Not set" }
    }
}

# Summary
if (-not $Json) {
    Write-Host ""
    Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "                    Verification Summary" -ForegroundColor Cyan
    Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "  Total checks: $($results.summary.total)" -ForegroundColor Gray
    Write-Host "  Passed:       $($results.summary.passed)" -ForegroundColor Green
    Write-Host "  Failed:       $($results.summary.failed)" -ForegroundColor $(if ($results.summary.failed -gt 0) { "Red" } else { "Gray" })
    Write-Host ""

    if ($results.summary.failed -eq 0) {
        Write-Host "✓ All checks passed! NOA environment is ready." -ForegroundColor Green
    } else {
        Write-Host "✗ Some checks failed. Run bootstrap to fix issues." -ForegroundColor Red
    }
}

if ($Json) {
    $results | ConvertTo-Json -Depth 5
}

exit $results.summary.failed

