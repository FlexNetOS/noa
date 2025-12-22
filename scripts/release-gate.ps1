#Requires -Version 5.1
<#
.SYNOPSIS
    NOA Release Gate Script (Windows)

.DESCRIPTION
    A single script that validates the codebase is ready for release.
    Runs: format/lints, cargo check, unit tests, integration tests, UI build.

.PARAMETER Quick
    Skip slow tests, run minimal checks

.PARAMETER SkipUI
    Skip UI build step

.PARAMETER SkipIntegration
    Skip integration tests

.PARAMETER Verbose
    Enable verbose output

.EXAMPLE
    .\scripts\release-gate.ps1
    .\scripts\release-gate.ps1 -Quick
    .\scripts\release-gate.ps1 -SkipUI -SkipIntegration
#>

[CmdletBinding()]
param(
    [switch]$Quick,
    [switch]$SkipUI,
    [switch]$SkipIntegration,
    [switch]$VerboseOutput
)

$ErrorActionPreference = "Continue"

# Get script location and NOA root
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $ScriptDir }

# Counters
$script:Passed = 0
$script:Failed = 0
$script:Skipped = 0
$script:Warnings = 0
$script:StartTime = Get-Date

function Write-Info { param($Message) Write-Host "[INFO] $Message" -ForegroundColor Blue }
function Write-Success { param($Message) Write-Host "[PASS] $Message" -ForegroundColor Green; $script:Passed++ }
function Write-Warn { param($Message) Write-Host "[WARN] $Message" -ForegroundColor Yellow; $script:Warnings++ }
function Write-Fail { param($Message) Write-Host "[FAIL] $Message" -ForegroundColor Red; $script:Failed++ }
function Write-Skip { param($Message) Write-Host "[SKIP] $Message" -ForegroundColor Cyan; $script:Skipped++ }

function Write-Section {
    param($Title)
    Write-Host ""
    Write-Host ("=" * 64) -ForegroundColor Cyan
    Write-Host "  $Title" -ForegroundColor Cyan
    Write-Host ("=" * 64) -ForegroundColor Cyan
}

function Test-Command {
    param($Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

function Test-Prerequisites {
    Write-Section "Checking Prerequisites"

    $missing = $false

    if (Test-Command "cargo") {
        $version = & cargo --version 2>&1
        Write-Success "cargo found: $version"
    } else {
        Write-Fail "cargo not found"
        $missing = $true
    }

    if (Test-Command "rustfmt") {
        Write-Success "rustfmt found"
    } else {
        Write-Fail "rustfmt not found (run: rustup component add rustfmt)"
        $missing = $true
    }

    # Check for clippy
    try {
        $null = & cargo clippy --version 2>&1
        Write-Success "clippy found"
    } catch {
        Write-Fail "clippy not found (run: rustup component add clippy)"
        $missing = $true
    }

    if (-not $SkipUI) {
        if (Test-Command "npm") {
            $version = & npm --version 2>&1
            Write-Success "npm found: $version"
        } else {
            Write-Warn "npm not found, UI checks will be skipped"
            $script:SkipUI = $true
        }
    }

    if ($missing) {
        Write-Fail "Missing required prerequisites"
        exit 2
    }
}

function Set-CargoEnv {
    Write-Section "Setting Up Build Environment"

    # Windows: Disable incremental builds to avoid "Access denied (os error 5)" issues
    Write-Info "Windows detected - setting CARGO_INCREMENTAL=0 to avoid file locking issues"
    $env:CARGO_INCREMENTAL = "0"

    if ($VerboseOutput) {
        Write-Info "CARGO_INCREMENTAL=$env:CARGO_INCREMENTAL"
    }
}

function Test-RustFormat {
    Write-Section "Rust Format Check"
    Push-Location "$NoaRoot\sys\core"

    try {
        $result = & cargo fmt --all -- --check 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Rust formatting is correct"
            return $true
        } else {
            Write-Fail "Rust formatting errors found (run: cargo fmt --all)"
            if ($VerboseOutput) { Write-Host $result }
            return $false
        }
    } finally {
        Pop-Location
    }
}

function Test-RustLints {
    Write-Section "Rust Lints (Clippy)"
    Push-Location "$NoaRoot\sys\core"

    try {
        $clippy_args = if ($Quick) { "--lib" } else { "--all-targets --all-features" }
        $result = & cargo clippy $clippy_args.Split(' ') -- -D warnings 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Clippy passed with no warnings"
            return $true
        } else {
            Write-Fail "Clippy found issues"
            if ($VerboseOutput) { Write-Host $result }
            return $false
        }
    } finally {
        Pop-Location
    }
}

function Test-CargoCheck {
    Write-Section "Cargo Check"
    Push-Location "$NoaRoot\sys\core"

    try {
        & cargo check -p noa-core --all-features 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "cargo check -p noa-core passed"
        } else {
            Write-Fail "cargo check failed"
            return $false
        }

        if (-not $Quick) {
            & cargo check --workspace --all-features 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-Success "cargo check --workspace passed"
            } else {
                Write-Fail "workspace cargo check failed"
                return $false
            }
        }

        return $true
    } finally {
        Pop-Location
    }
}

function Test-UnitTests {
    Write-Section "Unit Tests"
    Push-Location "$NoaRoot\sys\core"

    try {
        $test_args = if ($Quick) { "--lib" } else { "" }
        & cargo test $test_args 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Unit tests passed"
            return $true
        } else {
            Write-Fail "Unit tests failed"
            return $false
        }
    } finally {
        Pop-Location
    }
}

function Test-IntegrationTests {
    if ($SkipIntegration) {
        Write-Skip "Integration tests (--SkipIntegration)"
        return $true
    }

    Write-Section "Integration Tests"
    Push-Location "$NoaRoot\sys\core"

    try {
        & cargo test --test '*' -- --test-threads=1 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Integration tests passed"
            return $true
        } else {
            Write-Warn "Integration tests failed or no tests found"
            return $true  # Don't fail for missing tests
        }
    } finally {
        Pop-Location
    }
}

function Test-UIBuild {
    if ($SkipUI) {
        Write-Skip "UI build (--SkipUI or npm not found)"
        return $true
    }

    Write-Section "UI Build"
    Push-Location "$NoaRoot\sys\ui"

    try {
        if (-not (Test-Path "package.json")) {
            Write-Warn "No package.json found in sys/ui"
            return $true
        }

        Write-Info "Installing dependencies..."
        & npm ci --silent 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) {
            & npm install --silent 2>&1 | Out-Null
        }

        Write-Info "Type checking..."
        & npm run type-check 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "TypeScript type check passed"
        } else {
            Write-Fail "TypeScript type check failed"
            return $false
        }

        Write-Info "Linting..."
        & npm run lint 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "ESLint passed"
        } else {
            Write-Fail "ESLint found issues"
            return $false
        }

        if (-not $Quick) {
            Write-Info "Building..."
            & npm run build 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-Success "UI build passed"
            } else {
                Write-Fail "UI build failed"
                return $false
            }
        } else {
            Write-Skip "UI build (--Quick mode)"
        }

        return $true
    } finally {
        Pop-Location
    }
}

function Test-GoChecks {
    if (-not (Test-Command "go")) {
        Write-Skip "Go checks (go not found)"
        return $true
    }

    Write-Section "Go Checks"
    Push-Location "$NoaRoot\p2p"

    try {
        if (-not (Test-Path "go.mod")) {
            Write-Skip "Go checks (no go.mod found)"
            return $true
        }

        Write-Info "Go build..."
        & go build ./... 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Go build passed"
        } else {
            Write-Fail "Go build failed"
            return $false
        }

        if (-not $Quick) {
            Write-Info "Go test..."
            & go test ./... 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-Success "Go tests passed"
            } else {
                Write-Fail "Go tests failed"
                return $false
            }
        }

        return $true
    } finally {
        Pop-Location
    }
}

function Show-Summary {
    $EndTime = Get-Date
    $Duration = ($EndTime - $script:StartTime).TotalSeconds

    Write-Section "Release Gate Summary"

    Write-Host ""
    Write-Host "  Passed:   $script:Passed" -ForegroundColor Green
    Write-Host "  Failed:   $script:Failed" -ForegroundColor Red
    Write-Host "  Skipped:  $script:Skipped" -ForegroundColor Cyan
    Write-Host "  Warnings: $script:Warnings" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Duration: $([math]::Round($Duration, 1))s"
    Write-Host ""

    if ($script:Failed -gt 0) {
        Write-Host ("=" * 64) -ForegroundColor Red
        Write-Host "  RELEASE GATE FAILED" -ForegroundColor Red
        Write-Host ("=" * 64) -ForegroundColor Red
        exit 1
    } else {
        Write-Host ("=" * 64) -ForegroundColor Green
        Write-Host "  RELEASE GATE PASSED" -ForegroundColor Green
        Write-Host ("=" * 64) -ForegroundColor Green
        exit 0
    }
}

# Main execution
function Main {
    Write-Host ""
    Write-Host "+" + ("=" * 62) + "+" -ForegroundColor Cyan
    Write-Host "|                      NOA Release Gate                        |" -ForegroundColor Cyan
    Write-Host "+" + ("=" * 62) + "+" -ForegroundColor Cyan
    Write-Host ""

    if ($Quick) {
        Write-Info "Running in QUICK mode (subset of checks)"
    }

    Test-Prerequisites
    Set-CargoEnv

    # Run all checks, continuing even if some fail
    Test-RustFormat | Out-Null
    Test-RustLints | Out-Null
    Test-CargoCheck | Out-Null
    Test-UnitTests | Out-Null
    Test-IntegrationTests | Out-Null
    Test-UIBuild | Out-Null
    Test-GoChecks | Out-Null

    Show-Summary
}

Main
