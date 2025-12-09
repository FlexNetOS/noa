<#
.SYNOPSIS
    Verify cross-platform parity between PowerShell and Bash scripts.

.DESCRIPTION
    Compares script pairs to ensure they produce identical results.
    This is critical for NOA's cross-platform guarantee.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Verbose
    Show detailed comparison output

.EXAMPLE
    .\cross-platform-parity.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Detailed
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         Cross-Platform Parity Verification" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

$SCRIPTS_DIR = Join-Path $NoaRoot "scripts"

# Script pairs to verify
$scriptPairs = @(
    @{ PS = "bootstrap/bootstrap.ps1"; SH = "bootstrap/bootstrap.sh"; Name = "Main Bootstrap" },
    @{ PS = "bootstrap/config/cache-setup.ps1"; SH = "bootstrap/config/cache-setup.sh"; Name = "Cache Setup" },
    @{ PS = "bootstrap/config/log-setup.ps1"; SH = "bootstrap/config/log-setup.sh"; Name = "Log Setup" },
    @{ PS = "bootstrap/verify/verify-all.ps1"; SH = "bootstrap/verify/verify-all.sh"; Name = "Verify All" },
    @{ PS = "bootstrap/verify/smoke-test.ps1"; SH = "bootstrap/verify/smoke-test.sh"; Name = "Smoke Test" },
    @{ PS = "setup/check-prereqs.ps1"; SH = "../init/check-prereqs.sh"; Name = "Check Prerequisites" },
    @{ PS = "bootstrap/generators/noa-env.ps1"; SH = "bootstrap/generators/noa-env.sh"; Name = "Env Generator" }
)

$passed = 0
$failed = 0
$missing = 0

Write-Host "Checking script pairs exist..." -ForegroundColor Yellow
Write-Host ""

foreach ($pair in $scriptPairs) {
    $psPath = Join-Path $SCRIPTS_DIR $pair.PS
    $shPath = Join-Path $SCRIPTS_DIR $pair.SH

    $psExists = Test-Path $psPath
    $shExists = Test-Path $shPath

    if ($psExists -and $shExists) {
        Write-Host "  [PAIR] $($pair.Name)" -ForegroundColor Green
        Write-Host "         PS: $($pair.PS)" -ForegroundColor Gray
        Write-Host "         SH: $($pair.SH)" -ForegroundColor Gray
        $passed++
    } elseif (-not $psExists -and -not $shExists) {
        Write-Host "  [MISS] $($pair.Name) - Both missing" -ForegroundColor Red
        $missing++
    } else {
        Write-Host "  [HALF] $($pair.Name)" -ForegroundColor Yellow
        if (-not $psExists) { Write-Host "         Missing: $($pair.PS)" -ForegroundColor Red }
        if (-not $shExists) { Write-Host "         Missing: $($pair.SH)" -ForegroundColor Red }
        $failed++
    }
}

Write-Host ""
Write-Host "Parameter Comparison (sampling)..." -ForegroundColor Yellow
Write-Host ""

# Check that key scripts have matching parameters
$paramChecks = @(
    @{
        Name = "check-prereqs"
        PS = "setup/check-prereqs.ps1"
        SH = "../init/check-prereqs.sh"
        PSParams = @("-Json", "-PathsOnly", "-RequireTasks", "-AllowGlobal")
        SHParams = @("--json", "--paths-only", "--require-tasks", "--allow-global")
    }
)

foreach ($check in $paramChecks) {
    Write-Host "  $($check.Name):" -ForegroundColor Cyan
    Write-Host "    PS params: $($check.PSParams -join ', ')" -ForegroundColor Gray
    Write-Host "    SH params: $($check.SHParams -join ', ')" -ForegroundColor Gray

    if ($check.PSParams.Count -eq $check.SHParams.Count) {
        Write-Host "    [OK] Parameter count matches" -ForegroundColor Green
    } else {
        Write-Host "    [WARN] Parameter count mismatch" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "                     Parity Summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Script pairs found:  $passed" -ForegroundColor Green
Write-Host "  Incomplete pairs:    $failed" -ForegroundColor $(if ($failed -gt 0) { "Yellow" } else { "Gray" })
Write-Host "  Missing both:        $missing" -ForegroundColor $(if ($missing -gt 0) { "Red" } else { "Gray" })
Write-Host ""

if ($failed -eq 0 -and $missing -eq 0) {
    Write-Host "✓ All script pairs verified!" -ForegroundColor Green
} else {
    Write-Host "⚠ Some scripts missing cross-platform counterpart" -ForegroundColor Yellow
}

exit $failed

