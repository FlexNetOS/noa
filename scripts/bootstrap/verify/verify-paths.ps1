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
    "NOA_CONFIG",
    "NOA_LOGS",
    "CARGO_HOME",
    "RUSTUP_HOME",
    "GOROOT",
    "GOPATH",
    "GOBIN",
    "GOCACHE",
    "GOMODCACHE",
    "npm_config_cache",
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

# Check config files for external paths
Write-Host ""
Write-Host "Checking configuration files..." -ForegroundColor Yellow

$configFiles = @(
    "config/ai-providers.json",
    "config/shared-resources.json",
    "config/bootstrap-tools.json"
)

foreach ($configFile in $configFiles) {
    $configPath = Join-Path $NoaRoot $configFile
    if (Test-Path $configPath) {
        $content = Get-Content $configPath -Raw

        # Check for absolute paths not starting with ${NOA_ROOT} or noa_root
        # Allow /opt/ paths in config files (template paths for cross-platform compatibility)
        $absolutePaths = [regex]::Matches($content, '(?<!"[^"]*)[A-Z]:\\[^"]+|/(?:usr|home|etc)/[^"]+')
        # Also check for Windows absolute paths outside noa_root
        $windowsPaths = [regex]::Matches($content, '(?<!"[^"]*)([A-Z]:\\[^"]+)')
        foreach ($match in $windowsPaths) {
            $path = $match.Groups[1].Value
            # Extract just the path part (before any command or version flag)
            $cleanPath = $path -replace '\s+--version.*$', '' -replace '\s+.*$', ''
            if ($cleanPath -and -not $cleanPath.StartsWith($NoaRoot) -and -not $cleanPath -match '^[A-Z]:\\Program Files') {
                # Only flag if it's not a known system path and not in noa_root
                $absolutePaths = $absolutePaths + $match
            }
        }

        if ($absolutePaths.Count -eq 0) {
            Write-Host "  [OK] $configFile - No external paths" -ForegroundColor Green
        } else {
            foreach ($match in $absolutePaths) {
                Write-Host "  [!!] $configFile contains external path: $($match.Value)" -ForegroundColor Red
                $violations += "CONFIG: $configFile contains $($match.Value)"
            }
        }
    } else {
        Write-Host "  [--] $configFile (not found)" -ForegroundColor Gray
    }
}

# Check symlinks don't point outside
Write-Host ""
Write-Host "Checking symlinks..." -ForegroundColor Yellow

$noaDir = Get-Item $NoaRoot
$symlinks = Get-ChildItem -Path $NoaRoot -Recurse -Force -ErrorAction SilentlyContinue |
    Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }

foreach ($link in $symlinks) {
    # Get the actual target - use ResolvedTarget if available, otherwise Target
    $target = if ($link.PSObject.Properties['ResolvedTarget']) {
        $link.ResolvedTarget
    } else {
        try {
            # Try to get the actual target path
            $linkInfo = Get-Item $link.FullName -Force
            if ($linkInfo.LinkType -eq "SymbolicLink") {
                # Use Target property but resolve it
                $rawTarget = $linkInfo.Target
                if ($rawTarget -and -not [System.IO.Path]::IsPathRooted($rawTarget)) {
                    # Relative path, resolve it
                    $resolved = Join-Path $link.DirectoryName $rawTarget
                    $resolved = [System.IO.Path]::GetFullPath($resolved)
                    $resolved
                } else {
                    $rawTarget
                }
            } else {
                $linkInfo.Target
            }
        } catch {
            $link.Target
        }
    }

    if ($target -and -not $target.StartsWith($NoaRoot)) {
        # Check if portable version exists (graceful degradation)
        $linkName = $link.Name
        $portablePaths = @{
            "git.exe" = Join-Path $NoaRoot "opt/git/cmd/git.exe"
            "gh.exe" = Join-Path $NoaRoot "opt/gh/bin/gh.exe"
            "git-lfs.exe" = Join-Path $NoaRoot "opt/git-lfs/git-lfs.exe"
            "rustc.exe" = Join-Path $NoaRoot "opt/rust/cargo/bin/rustc.exe"
            "cargo.exe" = Join-Path $NoaRoot "opt/rust/cargo/bin/cargo.exe"
            "rustfmt.exe" = Join-Path $NoaRoot "opt/rust/cargo/bin/rustfmt.exe"
        }

        if ($portablePaths.ContainsKey($linkName) -and (Test-Path $portablePaths[$linkName])) {
            Write-Host "  [!!] $($link.FullName) -> $target (OUTSIDE, but portable version available)" -ForegroundColor Yellow
            Write-Host "       Run: .\scripts\bootstrap\verify\fix-symlinks.ps1" -ForegroundColor Gray
            $violations += "SYMLINK: $($link.Name) points to $target (portable available at $($portablePaths[$linkName]))"
        } else {
            Write-Host "  [!!] $($link.FullName) -> $target (OUTSIDE)" -ForegroundColor Red
            $violations += "SYMLINK: $($link.Name) points to $target"
        }
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

