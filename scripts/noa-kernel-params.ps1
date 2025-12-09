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
    [ValidateSet("set", "get", "list", "apply")]
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
            # Configure firewall for P2P
            if ($Value -eq "1") {
                Write-Host "  Configuring P2P firewall rules..." -ForegroundColor Gray
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
        Write-Host "  p2p_firewall    - Configure P2P firewall rules (0/1)"
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
}

