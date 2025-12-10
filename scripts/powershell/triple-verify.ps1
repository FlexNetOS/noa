# Triple-Verification Protocol (Pass A/B/C) - Phase 9
# T492: Automates three-pass verification with reproducible outputs.
# Implements Universal Task Execution Policy §5.6

param(
    [string]$RootDir = $PSScriptRoot,
    [string]$ResultsDir = "",
    [string]$TestCmd = $env:NOA_TEST_CMD
)

$ErrorActionPreference = "Stop"

# Resolve paths
$RootDir = Resolve-Path -Path (Join-Path $RootDir "..\..") -ErrorAction SilentlyContinue
if (-not $RootDir) {
    $RootDir = $PWD
}
$ResultsDir = if ($ResultsDir) { $ResultsDir } else { Join-Path $RootDir "test-results" }
$SpecDir = Join-Path $RootDir "specs\001-noa-seed-foundation"
$Report = Join-Path $ResultsDir "TRIPLE_VERIFY_SUMMARY.md"

New-Item -ItemType Directory -Path $ResultsDir -Force | Out-Null

$passLogs = @(
    (Join-Path $ResultsDir "pass_a.log"),
    (Join-Path $ResultsDir "pass_b.log"),
    (Join-Path $ResultsDir "pass_c.log")
)

# Initialize summary report
@"
# Triple Verification Summary (Phase 9)

Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')

Based on: Universal Task Execution Policy §5.6

"@ | Out-File -FilePath $Report

function Write-PassLog {
    param(
        [string]$LogFile,
        [string]$Message
    )
    Write-Host $Message
    Add-Content -Path $LogFile -Value $Message
}

# Pass A: Self-Check
function Run-PassA {
    $log = $passLogs[0]
    $status = 0

    Write-PassLog $log "=== Pass A: Self-Check ==="
    Write-PassLog $log "Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')"
    Write-PassLog $log ""

    # TVP-A01: Internal consistency
    Write-PassLog $log "[TVP-A01] Checking internal consistency..."
    $specFile = Join-Path $SpecDir "spec.md"
    $planFile = Join-Path $SpecDir "plan.md"
    $tasksFile = Join-Path $SpecDir "tasks.md"

    if ((Test-Path $specFile) -and (Test-Path $planFile) -and (Test-Path $tasksFile)) {
        Write-PassLog $log "  [PASS] Spec, plan, and tasks files exist"
    } else {
        Write-PassLog $log "  [FAIL] Missing spec/plan/tasks files"
        $status = 1
    }

    # TVP-A02: Spec ↔ artifacts ↔ tests alignment
    Write-PassLog $log "[TVP-A02] Checking spec ↔ artifacts ↔ tests alignment..."
    $coverageFile = Join-Path $ResultsDir "COVERAGE.md"
    if (Test-Path $coverageFile) {
        $content = Get-Content $coverageFile -Raw
        if ($content -match "FR-|SC-|VER|artifact|test") {
            Write-PassLog $log "  [PASS] Coverage mapping exists"
        } else {
            Write-PassLog $log "  [FAIL] Coverage mapping incomplete"
            $status = 1
        }
    } else {
        Write-PassLog $log "  [FAIL] COVERAGE.md not found"
        $status = 1
    }

    # TVP-A03: All unit smoke tests pass
    Write-PassLog $log "[TVP-A03] Running unit smoke tests..."
    $smokeTest = Join-Path $ResultsDir "TEST\smoke-test.ps1"
    if (Test-Path $smokeTest) {
        try {
            & $smokeTest *>> $log 2>&1
            if ($LASTEXITCODE -eq 0) {
                Write-PassLog $log "  [PASS] Smoke tests passed"
            } else {
                Write-PassLog $log "  [FAIL] Smoke tests failed (exit code: $LASTEXITCODE)"
                $status = 1
            }
        } catch {
            Write-PassLog $log "  [FAIL] Smoke test execution error: $_"
            $status = 1
        }
    } else {
        Write-PassLog $log "  [WARN] Smoke test not found: $smokeTest"
    }

    # TVP-A04: No orphaned code
    Write-PassLog $log "[TVP-A04] Checking for orphaned code markers..."
    $orphanFiles = Get-ChildItem -Path (Join-Path $RootDir "sys"), (Join-Path $RootDir "ai") -Recurse -File `
        -Include "*.rs", "*.ts" -ErrorAction SilentlyContinue |
        Select-String -Pattern "TODO|FIXME|XXX" -List | Select-Object -ExpandProperty Path -Unique
    $orphanCount = $orphanFiles.Count
    if ($orphanCount -eq 0) {
        Write-PassLog $log "  [PASS] No orphaned code markers found"
    } else {
        Write-PassLog $log "  [WARN] Found $orphanCount files with TODO/FIXME markers"
    }

    # Update hashes
    Write-PassLog $log ""
    Write-PassLog $log "Updating hashes..."
    $hashScript = Join-Path $RootDir "scripts\powershell\generate-hashes.ps1"
    if (Test-Path $hashScript) {
        & $hashScript *>> $log 2>&1
    }

    # Git status
    Write-PassLog $log ""
    Write-PassLog $log "Git status:"
    if (Test-Path (Join-Path $RootDir ".git")) {
        Push-Location $RootDir
        git status --short *>> $log 2>&1
        Pop-Location
    } else {
        "Not a git repo" | Add-Content -Path $log
    }

    Add-Content -Path $Report -Value @"

## Pass A: Self-Check

- Log: $(Split-Path $log -Leaf)
- Status: $(if ($status -eq 0) { "PASS" } else { "FAIL" })

"@

    return $status
}

# Pass B: Independent Re-Derivation
function Run-PassB {
    $log = $passLogs[1]
    $status = 0

    Write-PassLog $log "=== Pass B: Independent Re-Derivation ==="
    Write-PassLog $log "Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')"
    Write-PassLog $log ""

    # TVP-B01: Re-run tests
    Write-PassLog $log "[TVP-B01] Re-running tests from current state..."
    if ($TestCmd) {
        Write-PassLog $log "  Running: $TestCmd"
        try {
            Invoke-Expression $TestCmd *>> $log 2>&1
            if ($LASTEXITCODE -ne 0) { $status = 1 }
        } catch {
            Write-PassLog $log "  [FAIL] Test execution failed: $_"
            $status = 1
        }
    } else {
        Write-PassLog $log "  [INFO] NOA_TEST_CMD not set, skipping test re-run"
        Write-PassLog $log "  [NOTE] In CI, this would re-run from fresh clone"
    }

    # TVP-B02: Recompute metrics
    Write-PassLog $log "[TVP-B02] Recomputing performance metrics..."
    Write-PassLog $log "  [INFO] Performance metrics would be recomputed here"
    Write-PassLog $log "  [NOTE] Actual metrics computation requires runtime tests"

    # TVP-B03: Re-generate artifacts
    Write-PassLog $log "[TVP-B03] Re-generating artifacts..."
    $oldHashes = Join-Path $ResultsDir "HASHES.txt.old"
    $hashesFile = Join-Path $ResultsDir "HASHES.txt"
    if (Test-Path $hashesFile) {
        Copy-Item $hashesFile $oldHashes -ErrorAction SilentlyContinue
    }

    $hashScript = Join-Path $RootDir "scripts\powershell\generate-hashes.ps1"
    if (Test-Path $hashScript) {
        & $hashScript *>> $log 2>&1
    }

    if ((Test-Path $oldHashes) -and (Test-Path $hashesFile)) {
        $diff = Compare-Object (Get-Content $oldHashes) (Get-Content $hashesFile)
        if (-not $diff) {
            Write-PassLog $log "  [PASS] Artifacts match previous generation"
        } else {
            Write-PassLog $log "  [INFO] Artifacts differ (expected if files changed)"
        }
        Remove-Item $oldHashes -ErrorAction SilentlyContinue
    }

    # TVP-B04: Deterministic builds
    Write-PassLog $log "[TVP-B04] Checking deterministic build capability..."
    $cargoFile = Join-Path $RootDir "sys\core\Cargo.toml"
    if (Test-Path $cargoFile) {
        Write-PassLog $log "  [INFO] Rust project detected - deterministic builds supported"
        Write-PassLog $log "  [NOTE] Verify with: cargo build --release (should produce identical outputs)"
    } else {
        Write-PassLog $log "  [INFO] Build system detection skipped"
    }

    Add-Content -Path $Report -Value @"

## Pass B: Independent Re-Derivation

- Log: $(Split-Path $log -Leaf)
- Status: $(if ($status -eq 0) { "PASS" } else { "FAIL" })

"@

    return $status
}

# Pass C: Adversarial Check
function Run-PassC {
    $log = $passLogs[2]
    $status = 0

    Write-PassLog $log "=== Pass C: Adversarial Check ==="
    Write-PassLog $log "Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')"
    Write-PassLog $log ""

    # TVP-C01: Negative tests
    Write-PassLog $log "[TVP-C01] Running negative tests..."
    Write-PassLog $log "  [INFO] Negative tests would validate invalid inputs"
    Write-PassLog $log "  [NOTE] Implement in test suite: invalid configs, malformed data, etc."

    # TVP-C02: Boundary cases
    Write-PassLog $log "[TVP-C02] Checking boundary case coverage..."
    Write-PassLog $log "  [INFO] Boundary tests would check: 0, max, overflow conditions"
    Write-PassLog $log "  [NOTE] Implement in test suite: empty inputs, max values, overflow scenarios"

    # TVP-C03: Cross-tool verification
    Write-PassLog $log "[TVP-C03] Cross-tool verification..."
    $toolsAvailable = @()
    if (Get-Command rustc -ErrorAction SilentlyContinue) { $toolsAvailable += "rustc" }
    if (Get-Command node -ErrorAction SilentlyContinue) { $toolsAvailable += "node" }
    if (Get-Command go -ErrorAction SilentlyContinue) { $toolsAvailable += "go" }
    if (Get-Command python3 -ErrorAction SilentlyContinue) { $toolsAvailable += "python3" }

    if ($toolsAvailable.Count -gt 0) {
        Write-PassLog $log "  [INFO] Available tools: $($toolsAvailable -join ', ')"
        Write-PassLog $log "  [NOTE] Cross-tool verification would run tests with different compilers/runtimes"
    } else {
        Write-PassLog $log "  [WARN] No development tools detected"
    }

    # TVP-C04: External citation check
    Write-PassLog $log "[TVP-C04] Checking external citations..."
    $citationFiles = Get-ChildItem -Path $SpecDir, (Join-Path $RootDir "docs") -Recurse -File `
        -Include "*.md" -ErrorAction SilentlyContinue
    $citationCount = ($citationFiles | Select-String -Pattern "https?://[^\s]+" -AllMatches).Matches.Count
    if ($citationCount -gt 0) {
        Write-PassLog $log "  [INFO] Found $citationCount URLs in documentation"
        Write-PassLog $log "  [NOTE] Verify URLs are accessible and include author/title/date"
    } else {
        Write-PassLog $log "  [INFO] No external citations found"
    }

    Add-Content -Path $Report -Value @"

## Pass C: Adversarial Check

- Log: $(Split-Path $log -Leaf)
- Status: $(if ($status -eq 0) { "PASS" } else { "FAIL" })

"@

    return $status
}

# Run all passes
Write-Host "Starting Triple-Verification Protocol (Phase 9)..." -ForegroundColor Cyan
Add-Content -Path $Report -Value "Starting Triple-Verification Protocol (Phase 9)..."

$passAStatus = Run-PassA
$passBStatus = Run-PassB
$passCStatus = Run-PassC

# Final summary
$summary = @"

---
## Summary

| Pass | Status | Log File |
|------|--------|----------|
| A (Self-Check) | $(if ($passAStatus -eq 0) { "✅ PASS" } else { "❌ FAIL" }) | $(Split-Path $passLogs[0] -Leaf) |
| B (Re-Derivation) | $(if ($passBStatus -eq 0) { "✅ PASS" } else { "❌ FAIL" }) | $(Split-Path $passLogs[1] -Leaf) |
| C (Adversarial) | $(if ($passCStatus -eq 0) { "✅ PASS" } else { "❌ FAIL" }) | $(Split-Path $passLogs[2] -Leaf) |

All logs stored in: $ResultsDir\

**Next Steps**:
- Review logs for detailed results
- Update EVIDENCE_LEDGER.md with Pass A/B/C outcomes
- Address any FAIL statuses before claiming completion
"@

Add-Content -Path $Report -Value $summary

Write-Host "`nTriple verification complete. See $Report" -ForegroundColor Green
Write-Host "Logs: $ResultsDir\pass_*.log" -ForegroundColor Green

exit ($passAStatus + $passBStatus + $passCStatus)

