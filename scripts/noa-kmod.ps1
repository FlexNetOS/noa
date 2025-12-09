<#
.SYNOPSIS
    NOA Kernel Module Management for Windows

.DESCRIPTION
    Manages kernel-level features for NOA on Windows.
    Windows equivalent of scripts/noa-kmod (bash)

    On Windows, this manages:
    - Network drivers for P2P (TAP adapter)
    - Hyper-V virtual switch
    - WinDivert for packet capture
    - WSL kernel modules (if applicable)

.PARAMETER Action
    Action: load, unload, list, required, check

.PARAMETER Module
    Module name (for load/unload)

.PARAMETER NoaRoot
    NOA root directory

.EXAMPLE
    .\noa-kmod.ps1 -Action check
    .\noa-kmod.ps1 -Action load -Module tap
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("load", "unload", "list", "required", "check")]
    [string]$Action,

    [string]$Module,
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$NOA_KERNEL = Join-Path $NoaRoot "sys/kernel"
$MODULES_FILE = Join-Path $NOA_KERNEL "loaded-modules.json"

# Ensure directories exist
if (-not (Test-Path $NOA_KERNEL)) {
    New-Item -ItemType Directory -Path $NOA_KERNEL -Force | Out-Null
}

# Windows "modules" mapping
$WindowsModules = @{
    "tap" = @{
        Description = "TAP-Windows Adapter (VPN/P2P)"
        Service = "tap0901"
        Driver = "tap0901.sys"
        InstallCmd = "choco install openvpn -y"
    }
    "hyperv_switch" = @{
        Description = "Hyper-V Virtual Switch"
        Feature = "Microsoft-Hyper-V"
        InstallCmd = "Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All"
    }
    "windivert" = @{
        Description = "WinDivert Packet Capture"
        Service = "WinDivert"
        InstallCmd = "Download from https://reqrypt.org/windivert.html"
    }
    "nat" = @{
        Description = "Windows NAT"
        Feature = "Routing and Remote Access"
        InstallCmd = "New-NetNat -Name 'NOANAT' -InternalIPInterfaceAddressPrefix '10.0.0.0/24'"
    }
    "bridge" = @{
        Description = "Network Bridge"
        Feature = "Built-in"
        InstallCmd = "New-VMSwitch -Name 'NOABridge' -SwitchType Internal"
    }
}

function Test-ModuleLoaded {
    param([string]$Name)

    $info = $WindowsModules[$Name]
    if (-not $info) { return $false }

    if ($info.Service) {
        $service = Get-Service -Name $info.Service -ErrorAction SilentlyContinue
        return ($service -and $service.Status -eq "Running")
    }

    if ($info.Feature) {
        if ($info.Feature -eq "Built-in") { return $true }
        $feature = Get-WindowsOptionalFeature -FeatureName $info.Feature -Online -ErrorAction SilentlyContinue
        return ($feature -and $feature.State -eq "Enabled")
    }

    return $false
}

function Get-LoadedModules {
    if (Test-Path $MODULES_FILE) {
        return Get-Content $MODULES_FILE -Raw | ConvertFrom-Json
    }
    return @()
}

function Save-LoadedModules {
    param($Modules)
    $Modules | ConvertTo-Json | Set-Content -Path $MODULES_FILE
}

switch ($Action) {
    "load" {
        if (-not $Module) {
            Write-Error "Usage: noa-kmod.ps1 -Action load -Module <name>"
        }

        $info = $WindowsModules[$Module]
        if (-not $info) {
            Write-Error "Unknown module: $Module. Use -Action list to see available modules."
        }

        Write-Host "Loading module: $Module ($($info.Description))..." -ForegroundColor Cyan

        if (Test-ModuleLoaded $Module) {
            Write-Host "  Already loaded" -ForegroundColor Green
            exit 0
        }

        if ($info.Service) {
            $service = Get-Service -Name $info.Service -ErrorAction SilentlyContinue
            if ($service) {
                Start-Service -Name $info.Service
                Write-Host "  Started service: $($info.Service)" -ForegroundColor Green
            } else {
                Write-Host "  Service not installed. Install with:" -ForegroundColor Yellow
                Write-Host "    $($info.InstallCmd)" -ForegroundColor Gray
            }
        }

        if ($info.Feature -and $info.Feature -ne "Built-in") {
            Write-Host "  Requires Windows Feature. Enable with:" -ForegroundColor Yellow
            Write-Host "    $($info.InstallCmd)" -ForegroundColor Gray
        }

        # Track loaded module
        $loaded = Get-LoadedModules
        if ($Module -notin $loaded) {
            $loaded += $Module
            Save-LoadedModules $loaded
        }
    }

    "unload" {
        if (-not $Module) {
            Write-Error "Usage: noa-kmod.ps1 -Action unload -Module <name>"
        }

        $info = $WindowsModules[$Module]
        if (-not $info) {
            Write-Error "Unknown module: $Module"
        }

        Write-Host "Unloading module: $Module..." -ForegroundColor Cyan

        if ($info.Service) {
            Stop-Service -Name $info.Service -ErrorAction SilentlyContinue
            Write-Host "  Stopped service: $($info.Service)" -ForegroundColor Green
        }

        # Update tracked modules
        $loaded = Get-LoadedModules
        $loaded = $loaded | Where-Object { $_ -ne $Module }
        Save-LoadedModules $loaded
    }

    "list" {
        Write-Host "NOA Kernel Modules (Windows):" -ForegroundColor Cyan

        foreach ($name in $WindowsModules.Keys) {
            $info = $WindowsModules[$name]
            $loaded = Test-ModuleLoaded $name
            $status = if ($loaded) { "[OK]" } else { "[--]" }
            $color = if ($loaded) { "Green" } else { "Yellow" }

            Write-Host "  $status $name - $($info.Description)" -ForegroundColor $color
        }
    }

    "required" {
        Write-Host "Windows components required for NOA P2P:" -ForegroundColor Cyan
        Write-Host "  - tap         : TAP-Windows Adapter (VPN/P2P tunneling)"
        Write-Host "  - hyperv_switch: Hyper-V Virtual Switch (VM networking)"
        Write-Host "  - windivert   : WinDivert (packet capture/modification)"
        Write-Host "  - nat         : Windows NAT (network translation)"
        Write-Host "  - bridge      : Network Bridge (internal networking)"
    }

    "check" {
        Write-Host "Checking Windows kernel component availability..." -ForegroundColor Cyan

        foreach ($name in $WindowsModules.Keys) {
            $info = $WindowsModules[$name]
            $loaded = Test-ModuleLoaded $name

            if ($loaded) {
                Write-Host "  [OK] $name : loaded/enabled" -ForegroundColor Green
            } else {
                # Check if available but not loaded
                if ($info.Service) {
                    $service = Get-Service -Name $info.Service -ErrorAction SilentlyContinue
                    if ($service) {
                        Write-Host "  [--] $name : available (not running)" -ForegroundColor Yellow
                    } else {
                        Write-Host "  [X] $name : not installed" -ForegroundColor Red
                    }
                } elseif ($info.Feature -and $info.Feature -ne "Built-in") {
                    $feature = Get-WindowsOptionalFeature -FeatureName $info.Feature -Online -ErrorAction SilentlyContinue
                    if ($feature) {
                        Write-Host "  [--] $name : available (not enabled)" -ForegroundColor Yellow
                    } else {
                        Write-Host "  [X] $name : not available" -ForegroundColor Red
                    }
                } else {
                    Write-Host "  [--] $name : needs configuration" -ForegroundColor Yellow
                }
            }
        }
    }
}

