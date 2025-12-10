<#
.SYNOPSIS
    Fix symlinks to point to portable tool installations (§3.1 compliance).

.DESCRIPTION
    Ensures all symlinks in bin/ point to portable installations in opt/,
    not system installations. Installs portable versions if needed.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall portable tools even if system versions exist

.EXAMPLE
    .\fix-symlinks.ps1
    .\fix-symlinks.ps1 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$NOA_BIN = Join-Path $NoaRoot "bin"
$NOA_OPT = Join-Path $NoaRoot "opt"

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         Fix Symlinks - §3.1 Compliance" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
Write-Host ""

$fixed = 0
$skipped = 0
$errors = 0

function Fix-Symlink {
    param(
        [string]$LinkName,
        [string]$PortablePath,
        [string]$InstallerScript
    )

    $linkPath = Join-Path $NOA_BIN $LinkName
    $portableExists = Test-Path $PortablePath

    # Check if symlink exists and where it points
    if (Test-Path $linkPath) {
        $link = Get-Item $linkPath -Force
        if ($link.LinkType -eq "SymbolicLink") {
            $target = $link.Target
            if ($target -and $target.StartsWith($NoaRoot)) {
                Write-Host "  [OK] $LinkName -> $target (already portable)" -ForegroundColor Green
                $script:skipped++
                return
            } elseif ($target -and -not $target.StartsWith($NoaRoot)) {
                Write-Host "  [!!] $LinkName -> $target (system tool)" -ForegroundColor Yellow
            }
        }
    }

    # Install portable version if needed
    if (-not $portableExists) {
        Write-Host "  Installing portable $LinkName..." -ForegroundColor Yellow
        if ($InstallerScript -and (Test-Path $InstallerScript)) {
            try {
                & pwsh -NoLogo -NoProfile -File $InstallerScript -Force:$Force 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "  [OK] Portable $LinkName installed" -ForegroundColor Green
                } else {
                    Write-Host "  [!!] Failed to install portable $LinkName" -ForegroundColor Red
                    $script:errors++
                    return
                }
            } catch {
                Write-Host "  [!!] Error installing $LinkName : $_" -ForegroundColor Red
                $script:errors++
                return
            }
        } else {
            Write-Host "  [!!] Installer script not found: $InstallerScript" -ForegroundColor Red
            $script:errors++
            return
        }
    }

    # Create/update symlink
    if (Test-Path $PortablePath) {
        try {
            if (Test-Path $linkPath) {
                Remove-Item $linkPath -Force -ErrorAction SilentlyContinue
            }
            New-Item -ItemType SymbolicLink -Path $linkPath -Target $PortablePath -Force | Out-Null
            Write-Host "  [OK] $LinkName -> $PortablePath (fixed)" -ForegroundColor Green
            $script:fixed++
        } catch {
            Write-Host "  [!!] Failed to create symlink: $_" -ForegroundColor Red
            $script:errors++
        }
    } else {
        Write-Host "  [!!] Portable path not found: $PortablePath" -ForegroundColor Red
        $script:errors++
    }
}

# Fix Git
$gitPortable = Join-Path $NOA_OPT "git\cmd\git.exe"
$gitInstaller = Join-Path $NoaRoot "scripts\bootstrap\installers\git-portable.ps1"
Fix-Symlink -LinkName "git.exe" -PortablePath $gitPortable -InstallerScript $gitInstaller

# Fix GitHub CLI
$ghPortable = Join-Path $NOA_OPT "gh\bin\gh.exe"
$ghInstaller = Join-Path $NoaRoot "scripts\bootstrap\installers\gh-portable.ps1"
if (-not (Test-Path $ghInstaller)) {
    $ghInstaller = Join-Path $NoaRoot "scripts\bootstrap\installers\gh.ps1"
}
Fix-Symlink -LinkName "gh.exe" -PortablePath $ghPortable -InstallerScript $ghInstaller

# Fix Git LFS
$gitLfsPortable = Join-Path $NOA_OPT "git-lfs\git-lfs.exe"
$gitLfsInstaller = Join-Path $NoaRoot "scripts\bootstrap\installers\git-lfs.ps1"
Fix-Symlink -LinkName "git-lfs.exe" -PortablePath $gitLfsPortable -InstallerScript $gitLfsInstaller

# Fix Rust tools (if rustup is system-installed)
$rustTools = @("rustc.exe", "cargo.exe", "rustfmt.exe", "rustup.exe")
foreach ($tool in $rustTools) {
    $rustPortable = Join-Path $NOA_OPT "rust\cargo\bin\$tool"
    if (Test-Path $rustPortable) {
        $linkPath = Join-Path $NOA_BIN $tool
        if (Test-Path $linkPath) {
            $link = Get-Item $linkPath -Force -ErrorAction SilentlyContinue
            if ($link -and $link.LinkType -eq "SymbolicLink") {
                $target = $link.Target
                if ($target -and -not $target.StartsWith($NoaRoot)) {
                    Write-Host "  Fixing $tool..." -ForegroundColor Yellow
                    try {
                        Remove-Item $linkPath -Force -ErrorAction SilentlyContinue
                        New-Item -ItemType SymbolicLink -Path $linkPath -Target $rustPortable -Force | Out-Null
                        Write-Host "  [OK] $tool -> $rustPortable (fixed)" -ForegroundColor Green
                        $fixed++
                    } catch {
                        Write-Host "  [!!] Failed to fix $tool : $_" -ForegroundColor Red
                        $errors++
                    }
                }
            }
        }
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
    Write-Host "✓ All symlinks fixed - §3.1 COMPLIANT" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ Some symlinks could not be fixed" -ForegroundColor Red
    exit 1
}

