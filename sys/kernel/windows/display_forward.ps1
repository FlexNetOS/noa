<#
.SYNOPSIS
    Configure display forwarding for NDCL in Windows VM/Hyper-V scenarios (Phase 19 T876).

.DESCRIPTION
    Enables Enhanced Session Mode/RDP for the NOA desktop VM and prints the GPU-PV
    configuration steps. Intended as a helper, not a one-click configurator.
#>
[CmdletBinding()]
param(
    [string]$VmName = "NOA-Desktop",
    [switch]$EnableEnhancedSession,
    [switch]$EnableGpuPv
)

$ErrorActionPreference = "Stop"

Write-Host "[NDCL] Display forwarding helper" -ForegroundColor Cyan
Write-Host "VM: $VmName" -ForegroundColor Gray

if ($EnableEnhancedSession) {
    Write-Host "  [INFO] Enabling Enhanced Session Mode..." -ForegroundColor Yellow
    Enable-VMIntegrationService -VMName $VmName -Name "Guest Service Interface" -ErrorAction SilentlyContinue | Out-Null
    Set-VM -Name $VmName -EnhancedSessionTransportType HvSocket -ErrorAction SilentlyContinue
    Write-Host "  [OK] Enhanced Session Mode enabled (RDP over Hyper-V socket)" -ForegroundColor Green
}

if ($EnableGpuPv) {
    Write-Host "  [INFO] Configuring GPU partitioning..." -ForegroundColor Yellow
    try {
        Set-VMGpuPartitionAdapter -VMName $VmName -MinPartitionVRAM 1073741824 -MaxPartitionVRAM 2147483648 -ErrorAction Stop
        Write-Host "  [OK] GPU-PV partition applied (1-2GB VRAM slice)" -ForegroundColor Green
    } catch {
        Write-Host "  [WARN] GPU-PV configuration failed: $_" -ForegroundColor Yellow
        Write-Host "        Ensure your GPU/driver supports GPU partitioning." -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "Next steps:" -ForegroundColor Gray
Write-Host " 1) Connect via mstsc to the VM or use vmconnect.exe for Enhanced Session." -ForegroundColor Gray
Write-Host " 2) Ensure NOA wrappers launch desktop apps with APPDATA redirected to data/apps/." -ForegroundColor Gray
Write-Host " 3) For WSL-based containers, forward RDP/VNC ports from the VM as needed." -ForegroundColor Gray
