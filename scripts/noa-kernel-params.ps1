<#
.SYNOPSIS
    NOA Kernel Parameter Management for Windows

.DESCRIPTION
    Manages kernel-like parameters for NOA on Windows.
    Windows equivalent of scripts/noa-kernel-params (bash)

    On Windows, this manages:
    - Network forwarding settings
    - Firewall rules for P2P
    - WSL kernel parameters (if in WSL)
    - Hyper-V isolation settings (if using NOA VM)

.PARAMETER Action
    Action: set, get, list, apply

.PARAMETER Param
    Parameter name (for set/get)

.PARAMETER Value
    Parameter value (for set)

.PARAMETER NoaRoot
    NOA root directory

.EXAMPLE
    .\noa-kernel-params.ps1 -Action list
    .\noa-kernel-params.ps1 -Action set -Param "ip_forward" -Value "1"
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("set", "get", "list", "apply", "list-modes", "check-availability", "select-mode")]
    [string]$Action,

    [string]$Param,
    [string]$Value,
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$NOA_KERNEL = Join-Path $NoaRoot "sys/kernel"
$PARAMS_FILE = Join-Path $NOA_KERNEL "params/current.json"

# Ensure directories exist
$paramsDir = Join-Path $NOA_KERNEL "params"
if (-not (Test-Path $paramsDir)) {
    New-Item -ItemType Directory -Path $paramsDir -Force | Out-Null
}

# Load current parameters
function Get-NoaParams {
    if (Test-Path $PARAMS_FILE) {
        return Get-Content $PARAMS_FILE -Raw | ConvertFrom-Json
    }
    return @{}
}

function Save-NoaParams {
    param($Params)
    $Params | ConvertTo-Json -Depth 5 | Set-Content -Path $PARAMS_FILE
}

# Load kernel selection policy
function Get-KernelSelectionPolicy {
    $policyPath = Join-Path $NoaRoot "configs/kernel-selection-policy.json"
    if (Test-Path $policyPath) {
        return Get-Content $policyPath -Raw | ConvertFrom-Json
    }
    return $null
}

# Check availability of kernel modes (FR-160)
function Get-AvailableModes {
    $modes = @()

    # Native is always available
    $modes += @{
        Mode = "native"
        Available = $true
        Reason = "Always available"
    }

    # Check Hyper-V availability
    try {
        $hyperV = Get-WindowsOptionalFeature -FeatureName Microsoft-Hyper-V-All -Online -ErrorAction SilentlyContinue
        if ($hyperV -and $hyperV.State -eq "Enabled") {
            $modes += @{ Mode = "vm"; Available = $true; Reason = "Hyper-V enabled" }
        } else {
            $modes += @{ Mode = "vm"; Available = $false; Reason = "Hyper-V not enabled" }
        }
    } catch {
        $modes += @{ Mode = "vm"; Available = $false; Reason = "Cannot check Hyper-V: $_" }
    }

    # Check Docker availability
    try {
        $dockerInfo = docker info 2>$null
        if ($LASTEXITCODE -eq 0) {
            $modes += @{ Mode = "container"; Available = $true; Reason = "Docker available" }
        } else {
            $modes += @{ Mode = "container"; Available = $false; Reason = "Docker not running" }
        }
    } catch {
        $modes += @{ Mode = "container"; Available = $false; Reason = "Docker not installed" }
    }

    # Check Windows Sandbox availability
    try {
        $sandbox = Get-WindowsOptionalFeature -FeatureName Containers-DisposableClientVM -Online -ErrorAction SilentlyContinue
        if ($sandbox -and $sandbox.State -eq "Enabled") {
            $modes += @{ Mode = "sandbox"; Available = $true; Reason = "Windows Sandbox enabled" }
        } else {
            $modes += @{ Mode = "sandbox"; Available = $false; Reason = "Windows Sandbox not enabled" }
        }
    } catch {
        $modes += @{ Mode = "sandbox"; Available = $false; Reason = "Cannot check Sandbox: $_" }
    }

    return $modes
}

# Select kernel mode based on policy (FR-159, FR-160)
function Select-KernelMode {
    param(
        [string]$RequestedMode = "",
        [switch]$PreferIsolated,
        [string]$UseCase = ""
    )

    $policy = Get-KernelSelectionPolicy
    $availableModes = Get-AvailableModes

    # If specific mode requested, validate and use it
    if ($RequestedMode) {
        $modeInfo = $availableModes | Where-Object { $_.Mode -eq $RequestedMode }
        if ($modeInfo -and $modeInfo.Available) {
            return @{
                SelectedMode = $RequestedMode
                Reason = "User requested"
                Available = $true
            }
        } else {
            Write-Host "  Requested mode '$RequestedMode' not available: $($modeInfo.Reason)" -ForegroundColor Yellow
        }
    }

    # Apply policy precedence (VM > Container > Sandbox > Native)
    if ($PreferIsolated -and $policy) {
        foreach ($precedence in $policy.precedenceOrder) {
            $modeInfo = $availableModes | Where-Object { $_.Mode -eq $precedence.mode }
            if ($modeInfo -and $modeInfo.Available) {
                return @{
                    SelectedMode = $precedence.mode
                    Reason = "Policy precedence (isolation preferred)"
                    Available = $true
                    Priority = $precedence.priority
                }
            }
        }
    }

    # Default to native mode
    return @{
        SelectedMode = "native"
        Reason = "Default mode (performance)"
        Available = $true
    }
}

# Windows-specific parameter application
function Apply-WindowsParam {
    param([string]$Name, [string]$Value)

    switch ($Name) {
        "ip_forward" {
            # Enable IP forwarding
            if ($Value -eq "1") {
                Write-Host "  Enabling IP forwarding..." -ForegroundColor Gray
                Set-NetIPInterface -Forwarding Enabled -ErrorAction SilentlyContinue
                # Also via registry for persistence
                Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters" -Name "IPEnableRouter" -Value 1 -ErrorAction SilentlyContinue
            } else {
                Set-NetIPInterface -Forwarding Disabled -ErrorAction SilentlyContinue
            }
        }

        "ipv6_forward" {
            # Enable IPv6 forwarding
            if ($Value -eq "1") {
                Write-Host "  Enabling IPv6 forwarding..." -ForegroundColor Gray
                netsh interface ipv6 set global forwarding=enabled 2>$null
            } else {
                netsh interface ipv6 set global forwarding=disabled 2>$null
            }
        }

        "p2p_firewall" {
            # configsure firewall for P2P
            if ($Value -eq "1") {
                Write-Host "  configsuring P2P firewall rules..." -ForegroundColor Gray
                # Allow NOA P2P ports
                New-NetFirewallRule -DisplayName "NOA P2P" -Direction Inbound -Protocol TCP -LocalPort 4001,4002,9000 -Action Allow -ErrorAction SilentlyContinue
                New-NetFirewallRule -DisplayName "NOA P2P UDP" -Direction Inbound -Protocol UDP -LocalPort 4001,4002 -Action Allow -ErrorAction SilentlyContinue
            }
        }

        "hyper_v_isolation" {
            # Hyper-V isolation for NOA VM
            Write-Host "  Hyper-V isolation setting stored (apply via NOA VM manager)" -ForegroundColor Gray
        }

        default {
            Write-Host "  Unknown parameter: $Name (stored for reference)" -ForegroundColor Yellow
        }
    }
}

switch ($Action) {
    "set" {
        if (-not $Param -or $Value -eq $null) {
            Write-Error "Usage: noa-kernel-params.ps1 -Action set -Param <name> -Value <value>"
        }

        Write-Host "Setting $Param = $Value" -ForegroundColor Cyan

        # Store parameter
        $params = Get-NoaParams
        $params | Add-Member -NotePropertyName $Param -NotePropertyValue $Value -Force
        Save-NoaParams $params

        # Apply to Windows
        if ([Security.Principal.WindowsIdentity]::GetCurrent().Groups -match 'S-1-5-32-544') {
            Apply-WindowsParam -Name $Param -Value $Value
            Write-Host "  Applied to Windows" -ForegroundColor Green
        } else {
            Write-Host "  Stored (run as Administrator to apply to Windows)" -ForegroundColor Yellow
        }
    }

    "get" {
        if (-not $Param) {
            Write-Error "Usage: noa-kernel-params.ps1 -Action get -Param <name>"
        }

        $params = Get-NoaParams
        $value = $params.$Param
        if ($value) {
            Write-Host "$Param = $value"
        } else {
            Write-Host "Parameter not found: $Param" -ForegroundColor Yellow
        }
    }

    "list" {
        Write-Host "NOA Kernel Parameters:" -ForegroundColor Cyan

        $params = Get-NoaParams
        if ($params.PSObject.Properties.Count -gt 0) {
            $params.PSObject.Properties | ForEach-Object {
                Write-Host "  $($_.Name) = $($_.Value)"
            }
        } else {
            Write-Host "  No parameters set" -ForegroundColor Gray
        }

        Write-Host ""
        Write-Host "Available parameters:" -ForegroundColor Yellow
        Write-Host "  ip_forward      - Enable IPv4 forwarding (0/1)"
        Write-Host "  ipv6_forward    - Enable IPv6 forwarding (0/1)"
        Write-Host "  p2p_firewall    - configsure P2P firewall rules (0/1)"
        Write-Host "  hyper_v_isolation - Hyper-V isolation mode"
    }

    "apply" {
        Write-Host "Applying all stored kernel parameters..." -ForegroundColor Cyan

        $params = Get-NoaParams
        $params.PSObject.Properties | ForEach-Object {
            Apply-WindowsParam -Name $_.Name -Value $_.Value
        }

        Write-Host "Parameters applied" -ForegroundColor Green
    }

    "list-modes" {
        Write-Host "NOA Kernel Modes (FR-160 Precedence Order):" -ForegroundColor Cyan
        Write-Host ""

        $policy = Get-KernelSelectionPolicy
        $availableModes = Get-AvailableModes

        if ($policy) {
            foreach ($precedence in $policy.precedenceOrder) {
                $modeInfo = $availableModes | Where-Object { $_.Mode -eq $precedence.mode }
                $status = if ($modeInfo -and $modeInfo.Available) { "[OK]" } else { "[--]" }
                $color = if ($modeInfo -and $modeInfo.Available) { "Green" } else { "Yellow" }

                Write-Host "  Priority $($precedence.priority): $($precedence.displayName) ($($precedence.mode))" -ForegroundColor $color
                Write-Host "    Description: $($precedence.description)" -ForegroundColor Gray
                Write-Host "    Use Cases: $($precedence.useCases -join ', ')" -ForegroundColor Gray
                Write-Host "    Status: $status $($modeInfo.Reason)" -ForegroundColor $color
                Write-Host ""
            }
        } else {
            Write-Host "  kernel-selection-policy.json not found" -ForegroundColor Yellow
            Write-Host "  Default modes: native (always available)" -ForegroundColor Gray
        }

        # Show current mode
        $currentParams = Get-NoaParams
        $currentMode = if ($currentParams.kernel_mode) { $currentParams.kernel_mode } else { "native" }
        Write-Host "Current mode: $currentMode" -ForegroundColor Cyan
    }

    "check-availability" {
        Write-Host "Checking kernel mode availability..." -ForegroundColor Cyan
        Write-Host ""

        $availableModes = Get-AvailableModes
        foreach ($mode in $availableModes) {
            $status = if ($mode.Available) { "[OK]" } else { "[--]" }
            $color = if ($mode.Available) { "Green" } else { "Yellow" }
            Write-Host "  $status $($mode.Mode): $($mode.Reason)" -ForegroundColor $color
        }
    }

    "select-mode" {
        if (-not $Value) {
            Write-Host "Usage: noa-kernel-params.ps1 -Action select-mode -Value <mode>" -ForegroundColor Yellow
            Write-Host "  Available modes: native, vm, container, sandbox" -ForegroundColor Gray
            Write-Host "  Or use -Value 'auto' for automatic selection based on policy" -ForegroundColor Gray
            exit 1
        }

        $preferIsolated = ($Value -eq "auto-isolated")
        $requestedMode = if ($Value -eq "auto" -or $Value -eq "auto-isolated") { "" } else { $Value }

        $selection = Select-KernelMode -RequestedMode $requestedMode -PreferIsolated:$preferIsolated
        Write-Host "Selected kernel mode: $($selection.SelectedMode)" -ForegroundColor Cyan
        Write-Host "  Reason: $($selection.Reason)" -ForegroundColor Gray

        # Store the selected mode
        $params = Get-NoaParams
        $params | Add-Member -NotePropertyName "kernel_mode" -NotePropertyValue $selection.SelectedMode -Force
        Save-NoaParams $params
        Write-Host "  Stored to kernel parameters" -ForegroundColor Green
    }
}

