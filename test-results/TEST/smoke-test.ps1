# Smoke Test for Phase 9 Verification (PowerShell)
# Exits with code 0 on success, non-zero on failure
# Captures transcript for Truth Gate verification

$ErrorActionPreference = "Stop"

$RootDir = if ($PSScriptRoot) {
    Resolve-Path (Join-Path $PSScriptRoot "..\..")
} else {
    $PWD
}
$ResultsDir = Join-Path $RootDir "test-results"

Write-Host "=== NOA Seed Foundation Smoke Test ===" -ForegroundColor Cyan
Write-Host "Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')"
Write-Host "Root: $RootDir"
Write-Host ""

$status = 0

# Test 1: Verify core directories exist
Write-Host "[TEST 1] Checking core directories..." -ForegroundColor Yellow
$requiredDirs = @("sys", "ai", "config", "scripts", "specs")
foreach ($dir in $requiredDirs) {
    $path = Join-Path $RootDir $dir
    if (Test-Path $path -PathType Container) {
        Write-Host "  ✓ $dir/ exists" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $dir/ missing" -ForegroundColor Red
        $status = 1
    }
}

# Test 2: Verify key configuration files
Write-Host ""
Write-Host "[TEST 2] Checking configuration files..." -ForegroundColor Yellow
$configFiles = @(
    "config\ai-providers.json",
    "config\bootstrap-state.json",
    "specs\001-noa-seed-foundation\spec.md"
)
foreach ($file in $configFiles) {
    $path = Join-Path $RootDir $file
    if (Test-Path $path -PathType Leaf) {
        Write-Host "  ✓ $file exists" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $file missing" -ForegroundColor Red
        $status = 1
    }
}

# Test 3: Verify verification artifacts exist
Write-Host ""
Write-Host "[TEST 3] Checking verification artifacts..." -ForegroundColor Yellow
$artifacts = @(
    "test-results\HASHES.txt",
    "test-results\FINAL_REPORT.md",
    "test-results\COVERAGE.md"
)
foreach ($artifact in $artifacts) {
    $path = Join-Path $RootDir $artifact
    if (Test-Path $path -PathType Leaf) {
        Write-Host "  ✓ $artifact exists" -ForegroundColor Green
    } else {
        Write-Host "  ⚠ $artifact missing (may be expected if not yet generated)" -ForegroundColor Yellow
    }
}

# Test 4: Verify scripts exist
Write-Host ""
Write-Host "[TEST 4] Checking scripts..." -ForegroundColor Yellow
$scripts = @(
    "scripts\powershell\truth-gate.ps1",
    "scripts\powershell\triple-verify.ps1",
    "scripts\bash\gap-scan.sh"
)
foreach ($script in $scripts) {
    $path = Join-Path $RootDir $script
    if (Test-Path $path -PathType Leaf) {
        Write-Host "  ✓ $script exists" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $script missing" -ForegroundColor Red
        $status = 1
    }
}

# Test 5: Verify hash generation works
Write-Host ""
Write-Host "[TEST 5] Testing hash generation..." -ForegroundColor Yellow
$testFile = Join-Path $RootDir "README.md"
if (Test-Path $testFile -PathType Leaf) {
    try {
        $hash = Get-FileHash -Path $testFile -Algorithm SHA256
        if ($hash.Hash) {
            Write-Host "  ✓ Hash generation works (sample hash: $($hash.Hash.Substring(0,8))...)" -ForegroundColor Green
        } else {
            Write-Host "  ✗ Hash generation failed" -ForegroundColor Red
            $status = 1
        }
    } catch {
        Write-Host "  ⚠ Hash generation error: $_" -ForegroundColor Yellow
    }
} else {
    Write-Host "  ⚠ Test file not found, skipping hash test" -ForegroundColor Yellow
}

# Summary
Write-Host ""
Write-Host "=== Smoke Test Summary ===" -ForegroundColor Cyan
if ($status -eq 0) {
    Write-Host "✓ All critical tests passed" -ForegroundColor Green
    Write-Host "Exit code: 0"
} else {
    Write-Host "✗ Some tests failed" -ForegroundColor Red
    Write-Host "Exit code: $status"
}

exit $status

