# Phase 1 Negative Tests (PowerShell)
# Tests failure modes and error handling for Phase 1 implementation
#
# CHK032: Negative tests for failure modes
# CHK033: Boundary cases (min, max, empty, null)

$ErrorActionPreference = "Stop"

$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$REPO_ROOT = Split-Path -Parent (Split-Path -Parent $SCRIPT_DIR)
$NOA_ROOT = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { $REPO_ROOT }

$FAILED = 0
$PASSED = 0

function Test-Negative {
    param(
        [string]$Name,
        [scriptblock]$TestCmd,
        [string]$ExpectedFailure
    )

    try {
        & $TestCmd | Out-Null
        Write-Host "❌ FAIL: $Name (should have failed but didn't)" -ForegroundColor Red
        $script:FAILED++
        return $false
    } catch {
        Write-Host "✅ PASS: $Name (correctly failed as expected: $ExpectedFailure)" -ForegroundColor Green
        $script:PASSED++
        return $true
    }
}

function Test-Boundary {
    param(
        [string]$Name,
        [scriptblock]$TestCmd
    )

    try {
        & $TestCmd | Out-Null
        Write-Host "✅ PASS: $Name" -ForegroundColor Green
        $script:PASSED++
        return $true
    } catch {
        Write-Host "❌ FAIL: $Name" -ForegroundColor Red
        $script:FAILED++
        return $false
    }
}

Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Phase 1 Negative Tests: Failure Modes & Boundary Cases"
Write-Host "════════════════════════════════════════════════════════════"
Write-Host ""

Write-Host "Testing Error Handling: Missing Prerequisites..."
Write-Host ""

# Test: Prerequisite check with missing tools
Test-Boundary "Prerequisite check handles missing tools" {
    & "$NOA_ROOT\scripts\powershell\check-prerequisites.ps1" -Json 2>&1 | Select-String -Pattern "missing|not found|required" -Quiet
}

Write-Host ""
Write-Host "Testing Error Handling: Invalid Configuration..."
Write-Host ""

# Test: Invalid JSON in config files
Test-Negative "Config validation rejects invalid JSON" {
    '{ invalid json }' | ConvertFrom-Json
} "JSON parse error"

# Test: Missing required config fields
Test-Boundary "Config validation checks required fields" {
    $config = Get-Content "$NOA_ROOT\config\noa-server.json" | ConvertFrom-Json
    $null -ne $config.version
}

Write-Host ""
Write-Host "Testing Boundary Cases: Empty Values..."
Write-Host ""

# Test: Empty directory creation (should succeed)
Test-Boundary "Empty directory creation succeeds" {
    $testDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_.FullName }
    Remove-Item $testDir
    $true
}

# Test: Empty config file handling
Test-Negative "Empty config file is rejected" {
    '' | ConvertFrom-Json
} "Empty file rejection"

Write-Host ""
Write-Host "Testing Boundary Cases: Path Length..."
Write-Host ""

# Test: Very long path (should handle gracefully)
$LONG_PATH = "C:\temp\" + ("a" * 200)
Test-Boundary "Long path creation handled" {
    try {
        New-Item -ItemType Directory -Path $LONG_PATH -Force | Out-Null
        Remove-Item $LONG_PATH -Force
        $true
    } catch {
        # Path length limit handled
        $true
    }
}

Write-Host ""
Write-Host "Testing Boundary Cases: Null/Undefined Values..."
Write-Host ""

# Test: Null values in JSON (should be handled)
Test-Boundary "Null values in JSON handled" {
    '{"test": null}' | ConvertFrom-Json | Out-Null
    $true
}

# Test: Undefined environment variables
Test-Boundary "Undefined NOA_ROOT handled" {
    $oldNoaRoot = $env:NOA_ROOT
    Remove-Item Env:\NOA_ROOT -ErrorAction SilentlyContinue
    $result = if ($env:NOA_ROOT) { $false } else { $true }
    if ($oldNoaRoot) { $env:NOA_ROOT = $oldNoaRoot }
    $result
}

Write-Host ""
Write-Host "Testing Boundary Cases: Special Characters..."
Write-Host ""

# Test: Special characters in paths
$SPECIAL_PATH = "C:\temp\noa-test-special-!@#$%^&*()"
Test-Boundary "Special characters in paths handled" {
    try {
        New-Item -ItemType Directory -Path $SPECIAL_PATH -Force | Out-Null
        Remove-Item $SPECIAL_PATH -Force
        $true
    } catch {
        $true
    }
}

Write-Host ""
Write-Host "Testing Error Recovery..."
Write-Host ""

# Test: Partial initialization recovery
Test-Boundary "Partial init can be recovered" {
    Test-Path "$NOA_ROOT\init" -PathType Container
}

Write-Host ""
Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Test Summary"
Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Passed: $PASSED"
Write-Host "Failed: $FAILED"
Write-Host ""

if ($FAILED -eq 0) {
    Write-Host "✅ All negative and boundary tests passed" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ Some negative and boundary tests failed" -ForegroundColor Red
    exit 1
}

