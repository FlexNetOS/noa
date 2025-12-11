# Generate Result Blocks for Phase 11
# Based on Universal Task Execution Policy §8D

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$NoaRoot = Resolve-Path (Join-Path $ScriptDir "../..")

Set-Location $NoaRoot

Write-Host "Generating Result Blocks for Phase 11..." -ForegroundColor Cyan

# Check if Rust is available
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: cargo not found. Please install Rust." -ForegroundColor Red
    exit 1
}

# Build the result block generator
Write-Host "Building result block generator..." -ForegroundColor Yellow
Set-Location sys/core
try {
    cargo build --release --bin result-block-generator 2>$null
    Set-Location $NoaRoot
    & "$NoaRoot/target/release/result-block-generator.exe"
} catch {
    Write-Host "Note: result-block-generator binary not found. Using Python fallback..." -ForegroundColor Yellow
    Set-Location $NoaRoot
    python scripts/python/generate_result_blocks.py
}

Write-Host "Result Blocks generated successfully!" -ForegroundColor Green
Write-Host "Check test-results/result_blocks.json and test-results/PHASE11_RESULT_BLOCKS.md" -ForegroundColor Green

