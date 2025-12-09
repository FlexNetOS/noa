<#
.SYNOPSIS
    Bundle all libraries for all NOA binaries (Windows)

.DESCRIPTION
    Windows equivalent of scripts/bundle-all-libs (bash)
    Scans all executables in bin/ and bundles their dependencies

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\bundle-all-libs.ps1
#>

param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$NOA_BIN = Join-Path $NoaRoot "bin"
$NOA_LIB = Join-Path $NoaRoot "lib"
$NOA_SCRIPTS = Join-Path $NoaRoot "scripts"
$BundleScript = Join-Path $NOA_SCRIPTS "bundle-libraries.ps1"

Write-Host "Bundling libraries for all NOA binaries..." -ForegroundColor Cyan

if (-not (Test-Path $BundleScript)) {
    Write-Error "Bundle script not found: $BundleScript"
    exit 1
}

# Find all executables in NOA bin directory
$executables = Get-ChildItem -Path $NOA_BIN -Filter "*.exe" -ErrorAction SilentlyContinue

foreach ($exe in $executables) {
    Write-Host "Processing: $($exe.Name)" -ForegroundColor Yellow
    & $BundleScript -TargetBinary $exe.FullName -NoaRoot $NoaRoot
}

# Also bundle DLLs from opt directories
$optDirs = @(
    (Join-Path $NoaRoot "opt/rust/cargo/bin"),
    (Join-Path $NoaRoot "opt/go/bin"),
    (Join-Path $NoaRoot "opt/node"),
    (Join-Path $NoaRoot "opt/python")
)

foreach ($optDir in $optDirs) {
    if (Test-Path $optDir) {
        $optExes = Get-ChildItem -Path $optDir -Filter "*.exe" -ErrorAction SilentlyContinue
        foreach ($exe in $optExes) {
            Write-Host "Processing (opt): $($exe.Name)" -ForegroundColor Yellow
            & $BundleScript -TargetBinary $exe.FullName -NoaRoot $NoaRoot
        }
    }
}

Write-Host ""
Write-Host "Library bundling complete. Libraries in: $NOA_LIB" -ForegroundColor Green
Write-Host "Ensure $NOA_LIB is in PATH or use app.manifest for DLL resolution" -ForegroundColor Gray

