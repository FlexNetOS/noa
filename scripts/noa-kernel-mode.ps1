<#
.SYNOPSIS
    NOA Kernel Mode Control (B139-B141)

.DESCRIPTION
    Set and manage kernel isolation mode for NOA.
    Implements noa-kernel-params set kernel_mode {native|vm|container|sandbox}

.PARAMETER Action
    Action to perform: get, set, detect, status

.PARAMETER Mode
    Kernel mode: native, vm, container, sandbox

.EXAMPLE
    .\noa-kernel-mode.ps1 -Action get
    .\noa-kernel-mode.ps1 -Action set -Mode vm
    .\noa-kernel-mode.ps1 -Action detect
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidateSet("get", "set", "detect", "status")]
    [string]$Action,

    [ValidateSet("native", "vm", "container", "sandbox", "auto")]
    [string]$Mode = "auto"
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
    Split-Path -Parent $PSScriptRoot
}

$ConfigPath = Join-Path $NoaRoot "config/kernel-mode.json"

function Get-CurrentMode {
    if (Test-Path $ConfigPath) {
        $config = Get-Content $ConfigPath -Raw | ConvertFrom-Json
        return $config.mode
    }
    return "native"
}

function Test-HyperV {
    try {
        # Try checking via Hyper-V cmdlets first (doesn't require admin)
        $hyperv = Get-Command "Get-VM" -ErrorAction SilentlyContinue
        if ($hyperv) { return $true }

        # Fallback: check registry (doesn't require admin)
        $regPath = "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Virtualization"
        if (Test-Path $regPath) { return $true }

        # If we can, check the feature (requires admin)
        $feature = Get-WindowsOptionalFeature -FeatureName Microsoft-Hyper-V-All -Online -ErrorAction SilentlyContinue
        return $feature -and $feature.State -eq "Enabled"
    } catch {
        return $false
    }
}

function Test-WindowsSandbox {
    try {
        # Check if WindowsSandbox.exe exists
        $sandbox = Get-Command "WindowsSandbox.exe" -ErrorAction SilentlyContinue
        if ($sandbox) { return $true }

        # Fallback: check feature (requires admin)
        $feature = Get-WindowsOptionalFeature -FeatureName Containers-DisposableClientVM -Online -ErrorAction SilentlyContinue
        return $feature -and $feature.State -eq "Enabled"
    } catch {
        return $false
    }
}

function Test-Docker {
    $docker = Get-Command "docker" -ErrorAction SilentlyContinue
    if ($docker) {
        $info = docker info 2>&1
        return $LASTEXITCODE -eq 0
    }
    return $false
}

function Test-Podman {
    $podman = Get-Command "podman" -ErrorAction SilentlyContinue
    return $null -ne $podman
}

function Detect-BestMode {
    # Check in order of isolation level
    if (Test-HyperV) {
        return "vm"
    }
    if (Test-Docker -or Test-Podman) {
        return "container"
    }
    if (Test-WindowsSandbox) {
        return "sandbox"
    }
    return "native"
}

function Get-ModeCapabilities {
    $caps = @{
        native = $true
        vm = Test-HyperV
        container = (Test-Docker -or Test-Podman)
        sandbox = Test-WindowsSandbox
    }
    return $caps
}

function Set-KernelMode {
    param([string]$NewMode)

    $caps = Get-ModeCapabilities

    if ($NewMode -eq "auto") {
        $NewMode = Detect-BestMode
        Write-Host "Auto-detected mode: $NewMode" -ForegroundColor Cyan
    }

    if (-not $caps[$NewMode]) {
        Write-Host "[ERROR] Mode '$NewMode' is not available on this system" -ForegroundColor Red
        Write-Host "Available modes:" -ForegroundColor Yellow
        $caps.GetEnumerator() | Where-Object { $_.Value } | ForEach-Object {
            Write-Host "  - $($_.Key)" -ForegroundColor Gray
        }
        exit 1
    }

    # Save configuration
    $configDir = Split-Path -Parent $ConfigPath
    if (-not (Test-Path $configDir)) {
        New-Item -ItemType Directory -Path $configDir -Force | Out-Null
    }

    $config = @{
        mode = $NewMode
        setAt = (Get-Date -Format "o")
        capabilities = $caps
    }

    $config | ConvertTo-Json -Depth 3 | Set-Content -Path $ConfigPath -Encoding UTF8

    # Also set environment variable
    $env:NOA_KERNEL_MODE = $NewMode

    Write-Host "[OK] Kernel mode set to: $NewMode" -ForegroundColor Green

    # Initialize mode if needed
    switch ($NewMode) {
        "vm" {
            Write-Host ""
            Write-Host "VM mode active. Use these commands:" -ForegroundColor Yellow
            Write-Host "  Create VM:  .\sys\kernel\windows\hyperv\noa-vm.ps1 -Action create" -ForegroundColor Gray
            Write-Host "  Start VM:   .\sys\kernel\windows\hyperv\noa-vm.ps1 -Action start" -ForegroundColor Gray
            Write-Host "  VM Status:  .\sys\kernel\windows\hyperv\noa-vm.ps1 -Action status" -ForegroundColor Gray
        }
        "container" {
            Write-Host ""
            Write-Host "Container mode active. Ensure Docker/Podman is running." -ForegroundColor Yellow
        }
        "sandbox" {
            Write-Host ""
            Write-Host "Sandbox mode active. Use Windows Sandbox profile:" -ForegroundColor Yellow
            Write-Host "  .\sys\kernel\windows\sandbox\noa.wsb" -ForegroundColor Gray
        }
    }
}

function Show-Status {
    Write-Host "NOA Kernel Mode Status" -ForegroundColor Cyan
    Write-Host "═══════════════════════" -ForegroundColor Cyan
    Write-Host ""

    $currentMode = Get-CurrentMode
    Write-Host "Current Mode: $currentMode" -ForegroundColor $(if ($currentMode -eq "native") { "Yellow" } else { "Green" })
    Write-Host ""

    Write-Host "Available Modes:" -ForegroundColor Yellow
    $caps = Get-ModeCapabilities

    foreach ($mode in @("native", "vm", "container", "sandbox")) {
        $available = $caps[$mode]
        $status = if ($available) { "[OK]" } else { "[--]" }
        $color = if ($available) { "Green" } else { "Gray" }
        $current = if ($mode -eq $currentMode) { " (current)" } else { "" }

        $details = switch ($mode) {
            "native" { "Direct host execution" }
            "vm" { "Hyper-V isolation" }
            "container" { "Docker/Podman containers" }
            "sandbox" { "Windows Sandbox" }
        }

        Write-Host "  $status $mode$current" -ForegroundColor $color
        Write-Host "      $details" -ForegroundColor Gray
    }

    Write-Host ""
    Write-Host "Best Available: $(Detect-BestMode)" -ForegroundColor Cyan
}

# Main
switch ($Action) {
    "get" {
        Write-Host (Get-CurrentMode)
    }
    "set" {
        Set-KernelMode -NewMode $Mode
    }
    "detect" {
        $best = Detect-BestMode
        Write-Host "Best available mode: $best" -ForegroundColor Cyan

        $caps = Get-ModeCapabilities
        Write-Host ""
        Write-Host "Capabilities detected:" -ForegroundColor Yellow
        foreach ($cap in $caps.GetEnumerator()) {
            $status = if ($cap.Value) { "[OK]" } else { "[--]" }
            $color = if ($cap.Value) { "Green" } else { "Gray" }
            Write-Host "  $status $($cap.Key)" -ForegroundColor $color
        }
    }
    "status" {
        Show-Status
    }
}

