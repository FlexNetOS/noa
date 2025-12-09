<#
.SYNOPSIS
    Run smoke tests for installed toolchains.

.DESCRIPTION
    Compiles and runs minimal programs to verify toolchain functionality.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\smoke-test.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$TMP_DIR = Join-Path $NoaRoot "tmp/smoke-test"

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         NOA Smoke Tests" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Ensure tmp directory exists
if (-not (Test-Path $TMP_DIR)) {
    New-Item -ItemType Directory -Path $TMP_DIR -Force | Out-Null
}

$passed = 0
$failed = 0

function Test-Toolchain {
    param(
        [string]$Name,
        [string]$TestFile,
        [string]$Content,
        [string]$CompileCmd,
        [string]$RunCmd
    )

    $testPath = Join-Path $TMP_DIR $TestFile

    Write-Host "Testing $Name..." -ForegroundColor Yellow

    try {
        # Write test file
        $Content | Set-Content -Path $testPath -Encoding UTF8

        # Compile if needed
        if ($CompileCmd) {
            Write-Host "  Compiling..." -ForegroundColor Gray
            $compileResult = Invoke-Expression $CompileCmd 2>&1
            if ($LASTEXITCODE -ne 0) {
                throw "Compilation failed: $compileResult"
            }
        }

        # Run
        Write-Host "  Running..." -ForegroundColor Gray
        $output = Invoke-Expression $RunCmd 2>&1

        if ($output -match "Hello from NOA") {
            Write-Host "  [PASS] $Name works correctly" -ForegroundColor Green
            $script:passed++
        } else {
            throw "Unexpected output: $output"
        }
    } catch {
        Write-Host "  [FAIL] $Name - $($_.Exception.Message)" -ForegroundColor Red
        $script:failed++
    } finally {
        # Cleanup test files
        Remove-Item -Path "$TMP_DIR/*" -Force -ErrorAction SilentlyContinue
    }
}

# Test Python
if (Get-Command python -ErrorAction SilentlyContinue) {
    Test-Toolchain -Name "Python" `
        -TestFile "test.py" `
        -Content 'print("Hello from NOA - Python")' `
        -RunCmd "python $TMP_DIR/test.py"
}

# Test Node.js
if (Get-Command node -ErrorAction SilentlyContinue) {
    Test-Toolchain -Name "Node.js" `
        -TestFile "test.js" `
        -Content 'console.log("Hello from NOA - Node.js")' `
        -RunCmd "node $TMP_DIR/test.js"
}

# Test Rust
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    Test-Toolchain -Name "Rust" `
        -TestFile "test.rs" `
        -Content 'fn main() { println!("Hello from NOA - Rust"); }' `
        -CompileCmd "rustc -o $TMP_DIR/test.exe $TMP_DIR/test.rs" `
        -RunCmd "$TMP_DIR/test.exe"
}

# Test Go
if (Get-Command go -ErrorAction SilentlyContinue) {
    Test-Toolchain -Name "Go" `
        -TestFile "test.go" `
        -Content 'package main; import "fmt"; func main() { fmt.Println("Hello from NOA - Go") }' `
        -RunCmd "go run $TMP_DIR/test.go"
}

# Summary
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Smoke Test Summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Passed: $passed" -ForegroundColor Green
Write-Host "  Failed: $failed" -ForegroundColor $(if ($failed -gt 0) { "Red" } else { "Gray" })

# Cleanup
Remove-Item -Path $TMP_DIR -Recurse -Force -ErrorAction SilentlyContinue

exit $failed

