# Phase 8: Regression Test Suite
# REG001-REG017: Critical path, provider integration, and data integrity tests

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $ScriptDir }
$CoreDir = Join-Path $NoaRoot "sys\core"

Write-Host "=========================================="
Write-Host "Phase 8: Regression Test Suite"
Write-Host "=========================================="
Write-Host ""

Set-Location $CoreDir

Write-Host "Running regression tests..."
Write-Host ""

# Run all regression tests
cargo test --lib regression::tests::regression_tests -- --nocapture

Write-Host ""
Write-Host "=========================================="
Write-Host "Regression tests completed"
Write-Host "=========================================="

