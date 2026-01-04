<#
.SYNOPSIS
    Verify all paths resolve under noa_root (§3.1 compliance).

.DESCRIPTION
    Checks that all NOA paths are self-contained within noa_root.
    No external dependencies outside the NOA directory.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\verify-paths.ps1
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

# Normalize path
$NoaRoot = (Resolve-Path $NoaRoot).Path

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         Path Self-Containment Verification (§3.1)" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
Write-Host ""

$violations = @()

# Check environment variables point to noa_root
Write-Host "Checking environment variable paths..." -ForegroundColor Yellow

$envVars = @(
    "NOA_ROOT",
    "NOA_BIN",
    "NOA_OPT",
    "NOA_configs",
    "NOA_LOGS",
    "CARGO_HOME",
    "RUSTUP_HOME",
    "GOROOT",
    "GOPATH",
    "GOBIN",
    "GOCACHE",
    "GOMODCACHE",
    "npm_configs_cache",
    "PIP_CACHE_DIR",
    "OLLAMA_MODELS",
    "HF_HOME"
)

foreach ($var in $envVars) {
    $value = [Environment]::GetEnvironmentVariable($var)
    if ($value) {
        if ($value.StartsWith($NoaRoot) -or $value -eq $NoaRoot) {
            Write-Host "  [OK] $var = $value" -ForegroundColor Green
        } else {
            Write-Host "  [!!] $var = $value (OUTSIDE noa_root)" -ForegroundColor Red
            $violations += "ENV: $var points outside noa_root"
        }
    } else {
        Write-Host "  [--] $var (not set)" -ForegroundColor Gray
    }
}

# Check configs files for external paths
Write-Host ""
Write-Host "Checking configsuration files..." -ForegroundColor Yellow

$configsFiles = @(
    "configs/ai-providers.json",
    "configs/shared-resources.json",
    "configs/bootstrap-tools.json"
)

foreach ($configsFile in $configsFiles) {
    $configsPath = Join-Path $NoaRoot $configsFile
    if (Test-Path $configsPath) {
        $content = Get-Content $configsPath -Raw

        # Check for absolute paths not starting with ${NOA_ROOT} or noa_root
        $absolutePaths = [regex]::Matches($content, '(?<!"[^"]*)[A-Z]:\\[^"]+|/(?:usr|opt|home|etc)/[^"]+')

        if ($absolutePaths.Count -eq 0) {
            Write-Host "  [OK] $configsFile - No external paths" -ForegroundColor Green
        } else {
            foreach ($match in $absolutePaths) {
                Write-Host "  [!!] $configsFile contains external path: $($match.Value)" -ForegroundColor Red
                $violations += "configs: $configsFile contains $($match.Value)"
            }
        }
    } else {
        Write-Host "  [--] $configsFile (not found)" -ForegroundColor Gray
    }
}

# Check symlinks don't point outside
Write-Host ""
Write-Host "Checking symlinks..." -ForegroundColor Yellow

$noaDir = Get-Item $NoaRoot
$symlinks = Get-ChildItem -Path $NoaRoot -Recurse -Force -ErrorAction SilentlyContinue |
    Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }

foreach ($link in $symlinks) {
    $target = $link.Target
    if ($target -and -not $target.StartsWith($NoaRoot)) {
        Write-Host "  [!!] $($link.FullName) -> $target (OUTSIDE)" -ForegroundColor Red
        $violations += "SYMLINK: $($link.Name) points to $target"
    }
}

if ($symlinks.Count -eq 0) {
    Write-Host "  [OK] No symlinks found" -ForegroundColor Green
}

# Summary
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "                     Verification Summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

if ($violations.Count -eq 0) {
    Write-Host "✓ All paths resolve under noa_root - §3.1 COMPLIANT" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ Found $($violations.Count) path violations:" -ForegroundColor Red
    foreach ($v in $violations) {
        Write-Host "  - $v" -ForegroundColor Red
    }
    exit 1
}

