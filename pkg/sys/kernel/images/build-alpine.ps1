<#
.SYNOPSIS
    NOA Alpine Linux VM Image Builder for Windows (B134-B138)

.DESCRIPTION
    Builds a minimal Alpine Linux image for NOA VM mode on Windows.
    Creates VHDX format for Hyper-V.

.PARAMETER Output
    Output image path

.PARAMETER Size
    Image size (default: 2GB)

.EXAMPLE
    .\build-alpine.ps1 -Output .\noa-linux.vhdx
#>
[CmdletBinding()]
param(
    [string]$Output,
    [string]$Size = "2GB"
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
    Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))
}

# Default output
if (-not $Output) {
    $Output = Join-Path $PSScriptRoot "noa-linux.vhdx"
}

$AlpineVersion = "3.19"
$AlpineArch = "x86_64"
$AlpineMirror = "https://dl-cdn.alpinelinux.org/alpine"

Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║        NOA Alpine Linux VM Image Builder (Windows)        ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "Format:       VHDX (Hyper-V)" -ForegroundColor Gray
Write-Host "Size:         $Size" -ForegroundColor Gray
Write-Host "Output:       $Output" -ForegroundColor Gray
Write-Host ""

# Check if Hyper-V is available
$hyperv = Get-WindowsOptionalFeature -FeatureName Microsoft-Hyper-V-All -Online -ErrorAction SilentlyContinue
if (-not $hyperv -or $hyperv.State -ne "Enabled") {
    Write-Host "[WARN] Hyper-V not enabled. Image can still be created." -ForegroundColor Yellow
}

Write-Host "Creating VHDX image..." -ForegroundColor Cyan

# Create VHDX using Hyper-V cmdlets if available
try {
    New-VHD -Path $Output -SizeBytes ([int64]($Size -replace "GB", "") * 1GB) -Dynamic
    Write-Host "  [OK] Created VHDX using Hyper-V" -ForegroundColor Green
} catch {
    # Fallback: Create using diskpart or qemu-img
    Write-Host "  [INFO] Hyper-V cmdlets not available, trying qemu-img..." -ForegroundColor Yellow

    $qemuImg = Get-Command "qemu-img" -ErrorAction SilentlyContinue
    if ($qemuImg) {
        & qemu-img create -f vhdx $Output $Size
        Write-Host "  [OK] Created VHDX using qemu-img" -ForegroundColor Green
    } else {
        Write-Host "  [ERROR] Neither Hyper-V nor qemu-img available" -ForegroundColor Red
        Write-Host "  Install QEMU or enable Hyper-V" -ForegroundColor Gray
        exit 1
    }
}

Write-Host ""
Write-Host "Downloading Alpine Linux ISO..." -ForegroundColor Cyan

$isoUrl = "$AlpineMirror/v$AlpineVersion/releases/$AlpineArch/alpine-virt-$AlpineVersion.0-$AlpineArch.iso"
$isoPath = Join-Path $PSScriptRoot "alpine-virt-$AlpineVersion.0-$AlpineArch.iso"

if (-not (Test-Path $isoPath)) {
    Invoke-WebRequest -Uri $isoUrl -OutFile $isoPath -UseBasicParsing
    Write-Host "  [OK] Downloaded Alpine ISO" -ForegroundColor Green
} else {
    Write-Host "  [SKIP] ISO already exists" -ForegroundColor Gray
}

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║                Image Build Complete!                      ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "Output: $Output" -ForegroundColor Gray
Write-Host "ISO:    $isoPath" -ForegroundColor Gray
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Create a VM with the VHDX and boot from ISO" -ForegroundColor Gray
Write-Host "  2. Run setup-alpine and configsure the system" -ForegroundColor Gray
Write-Host "  3. Reboot and remove ISO" -ForegroundColor Gray

