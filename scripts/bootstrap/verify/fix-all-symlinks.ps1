<#
.SYNOPSIS
    Fix all symlinks to point to portable installations (§3.1 compliance).

.DESCRIPTION
    Comprehensive script to fix all symlink violations found by verify-paths.ps1.
    Installs portable versions if needed and updates all symlinks.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall portable tools

.EXAMPLE
    .\fix-all-symlinks.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force
)

$ErrorActionPreference = "Continue"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$NOA_BIN = Join-Path $NoaRoot "bin"
$NOA_OPT = Join-Path $NoaRoot "opt"

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         Fix All Symlinks - §3.1 Compliance" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
Write-Host ""

$fixed = 0
$skipped = 0
$errors = 0

# Fix gh.exe - install portable if needed
Write-Host "Fixing gh.exe..." -ForegroundColor Yellow
$ghPortable = Join-Path $NOA_OPT "gh\bin\gh.exe"
$ghLink = Join-Path $NOA_BIN "gh.exe"

if (-not (Test-Path $ghPortable)) {
    Write-Host "  Installing portable GitHub CLI..." -ForegroundColor Gray
    $ghInstaller = Join-Path $NoaRoot "scripts\bootstrap\installers\gh.ps1"
    if (Test-Path $ghInstaller) {
        try {
            & pwsh -NoLogo -NoProfile -File $ghInstaller -Force:$Force 2>&1 | Out-Null
            Start-Sleep -Seconds 2
        } catch {
            Write-Host "  [!!] Failed to install portable gh: $_" -ForegroundColor Red
            $errors++
        }
    }
}

if (Test-Path $ghPortable) {
    if (Test-Path $ghLink) {
        $link = Get-Item $ghLink -Force -ErrorAction SilentlyContinue
        if ($link.LinkType -eq "SymbolicLink") {
            $target = $link.Target
            if ($target -and -not $target.StartsWith($NoaRoot)) {
                Remove-Item $ghLink -Force -ErrorAction SilentlyContinue
                $absPath = (Resolve-Path $ghPortable).Path
                New-Item -ItemType SymbolicLink -Path $ghLink -Target $absPath -Force | Out-Null
                Write-Host "  [OK] gh.exe -> $absPath (fixed)" -ForegroundColor Green
                $fixed++
            } else {
                Write-Host "  [OK] gh.exe already points to portable" -ForegroundColor Green
                $skipped++
            }
        } else {
            # It's a file, replace with symlink
            Remove-Item $ghLink -Force -ErrorAction SilentlyContinue
            $absPath = (Resolve-Path $ghPortable).Path
            New-Item -ItemType SymbolicLink -Path $ghLink -Target $absPath -Force | Out-Null
            Write-Host "  [OK] gh.exe -> $absPath (replaced file with symlink)" -ForegroundColor Green
            $fixed++
        }
    } else {
        $absPath = (Resolve-Path $ghPortable).Path
        New-Item -ItemType SymbolicLink -Path $ghLink -Target $absPath -Force | Out-Null
        Write-Host "  [OK] gh.exe -> $absPath (created)" -ForegroundColor Green
        $fixed++
    }
} else {
    Write-Host "  [!!] Portable gh.exe not found after installation attempt" -ForegroundColor Yellow
    $skipped++
}

# Fix Rust tools that point to rustup.exe
Write-Host ""
Write-Host "Fixing Rust tool symlinks..." -ForegroundColor Yellow
$rustTools = @(
    "cargo.exe", "rustc.exe", "rustfmt.exe", "cargo-clippy.exe",
    "cargo-fmt.exe", "cargo-miri.exe", "clippy-driver.exe",
    "rls.exe", "rust-analyzer.exe", "rust-gdb.exe", "rust-gdbgui.exe",
    "rust-lldb.exe", "rustdoc.exe"
)

foreach ($tool in $rustTools) {
    $rustPortable = Join-Path $NOA_OPT "rust\cargo\bin\$tool"
    $linkPath = Join-Path $NOA_BIN $tool
    
    if (-not (Test-Path $rustPortable)) {
        continue  # Skip if portable version doesn't exist
    }
    
    if (Test-Path $linkPath) {
        $link = Get-Item $linkPath -Force -ErrorAction SilentlyContinue
        if ($link.LinkType -eq "SymbolicLink") {
            $target = $link.Target
            if ($target -and ($target -like "*rustup.exe*" -or -not $target.StartsWith($NoaRoot))) {
                Remove-Item $linkPath -Force -ErrorAction SilentlyContinue
                $absPath = (Resolve-Path $rustPortable).Path
                New-Item -ItemType SymbolicLink -Path $linkPath -Target $absPath -Force | Out-Null
                Write-Host "  [OK] $tool -> $absPath (fixed)" -ForegroundColor Green
                $fixed++
            } else {
                Write-Host "  [OK] $tool already points to portable" -ForegroundColor Gray
                $skipped++
            }
        }
    } else {
        # Create symlink if it doesn't exist
        $absPath = (Resolve-Path $rustPortable).Path
        New-Item -ItemType SymbolicLink -Path $linkPath -Target $absPath -Force | Out-Null
        Write-Host "  [OK] $tool -> $absPath (created)" -ForegroundColor Green
        $fixed++
    }
}

# Summary
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "                     Fix Summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Fixed:   $fixed" -ForegroundColor Green
Write-Host "  Skipped: $skipped" -ForegroundColor Gray
Write-Host "  Errors:  $errors" -ForegroundColor $(if ($errors -gt 0) { "Red" } else { "Gray" })
Write-Host ""

if ($errors -eq 0) {
    Write-Host "✓ Symlink fixes completed" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run verify-paths.ps1 to confirm all paths are compliant" -ForegroundColor Cyan
    exit 0
} else {
    Write-Host "✗ Some symlinks could not be fixed" -ForegroundColor Red
    exit 1
}

