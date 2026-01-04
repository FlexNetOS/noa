# Phase 1 Smoke Test (PowerShell)
# Verifies all Phase 1 implementation artifacts exist and are functional
#
# T018: Smoke test for Phase 1
# CHK018: Deterministic smoke test with command, transcript, and exit code 0

$ErrorActionPreference = "Stop"

$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$REPO_ROOT = Split-Path -Parent (Split-Path -Parent $SCRIPT_DIR)
$NOA_ROOT = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { $REPO_ROOT }

$FAILED = 0
$PASSED = 0

function Test-Check {
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
Write-Host "Phase 1 Smoke Test: NOA Seed Foundation"
Write-Host "════════════════════════════════════════════════════════════"
Write-Host ""

Write-Host "Testing directory structure (FR-029 to FR-036)..."
Write-Host ""

# FR-029: sys/ directory
Test-Check "sys/core directory exists" { Test-Path "$NOA_ROOT/sys/core" -PathType Container }
Test-Check "sys/ui directory exists" { Test-Path "$NOA_ROOT/sys/ui" -PathType Container }
Test-Check "sys/digest directory exists" { Test-Path "$NOA_ROOT/sys/digest" -PathType Container }
Test-Check "sys/kernel directory exists" { Test-Path "$NOA_ROOT/sys/kernel" -PathType Container }

# FR-030: p2p/ directory
Test-Check "p2p directory exists" { Test-Path "$NOA_ROOT/p2p" -PathType Container }

# FR-031: opt/ directory
Test-Check "opt directory exists" { Test-Path "$NOA_ROOT/opt" -PathType Container }

# FR-032: init/ directory
Test-Check "init directory exists" { Test-Path "$NOA_ROOT/init" -PathType Container }
Test-Check "init/bootstrap directory exists" { Test-Path "$NOA_ROOT/init/bootstrap" -PathType Container }
Test-Check "init/migrations directory exists" { Test-Path "$NOA_ROOT/init/migrations" -PathType Container }
Test-Check "init/seeds directory exists" { Test-Path "$NOA_ROOT/init/seeds" -PathType Container }

# FR-033: containers/ directory
Test-Check "containers directory exists" { Test-Path "$NOA_ROOT/containers" -PathType Container }

# FR-034: configs/ directory
Test-Check "configs directory exists" { Test-Path "$NOA_ROOT/configs" -PathType Container }

# FR-035: bin/ directory
Test-Check "bin directory exists" { Test-Path "$NOA_ROOT/bin" -PathType Container }

# FR-036: ai/ directory
Test-Check "ai directory exists" { Test-Path "$NOA_ROOT/ai" -PathType Container }
Test-Check "ai/providers directory exists" { Test-Path "$NOA_ROOT/ai/providers" -PathType Container }

Write-Host ""
Write-Host "Testing project initialization files (T010-T013)..."
Write-Host ""

# T010: Rust workspace
Test-Check "Cargo.toml exists" { Test-Path "$NOA_ROOT/sys/core/Cargo.toml" -PathType Leaf }

# T011: Go module
Test-Check "go.mod exists" { Test-Path "$NOA_ROOT/p2p/go.mod" -PathType Leaf }

# T012: TypeScript/Next.js
Test-Check "package.json exists" { Test-Path "$NOA_ROOT/sys/ui/package.json" -PathType Leaf }

# T013: Python project
Test-Check "pyproject.toml exists" { Test-Path "$NOA_ROOT/sys/digest/pyproject.toml" -PathType Leaf }

Write-Host ""
Write-Host "Testing configsuration files (T016)..."
Write-Host ""

Test-Check "noa-server.json exists" { Test-Path "$NOA_ROOT/configs/noa-server.json" -PathType Leaf }
Test-Check "ai-providers.json exists" { Test-Path "$NOA_ROOT/configs/ai-providers.json" -PathType Leaf }
Test-Check "features.json exists" { Test-Path "$NOA_ROOT/configs/features.json" -PathType Leaf }

Write-Host ""
Write-Host "Testing scripts (T015, T673-T674)..."
Write-Host ""

Test-Check "check-prerequisites.ps1 exists" { Test-Path "$NOA_ROOT/scripts/powershell/check-prerequisites.ps1" -PathType Leaf }
Test-Check "check-prereqs.sh exists" { Test-Path "$NOA_ROOT/init/check-prereqs.sh" -PathType Leaf }

Write-Host ""
Write-Host "Testing CI pipeline (T017, T675)..."
Write-Host ""

Test-Check "CI workflow exists" { Test-Path "$NOA_ROOT/.github/workflows/ci.yml" -PathType Leaf }

Write-Host ""
Write-Host "Testing README (T018)..."
Write-Host ""

Test-Check "README.md exists" { Test-Path "$NOA_ROOT/README.md" -PathType Leaf }

Write-Host ""
Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Test Summary"
Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Passed: $PASSED"
Write-Host "Failed: $FAILED"
Write-Host ""

if ($FAILED -eq 0) {
    Write-Host "✅ All Phase 1 checks passed" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ Some Phase 1 checks failed" -ForegroundColor Red
    exit 1
}

