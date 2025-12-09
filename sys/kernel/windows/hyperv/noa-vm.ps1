<#
.SYNOPSIS
    NOA Hyper-V VM Management (B123)

.DESCRIPTION
    Manages NOA VM instances on Windows Hyper-V.
    Supports create, start, stop, status, and destroy operations.

.PARAMETER Action
    Action to perform: create, start, stop, status, destroy, list

.PARAMETER VmName
    Name of the VM (default: noa-vm)

.PARAMETER ImagePath
    Path to the VHDX image

.PARAMETER Memory
    VM memory in MB (default: 512)

.PARAMETER Cpus
    Number of virtual CPUs (default: 2)

.EXAMPLE
    .\noa-vm.ps1 -Action create
    .\noa-vm.ps1 -Action start
    .\noa-vm.ps1 -Action status
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidateSet("create", "start", "stop", "status", "destroy", "list", "console")]
    [string]$Action,

    [string]$VmName = "noa-vm",
    [string]$ImagePath,
    [int]$Memory = 512,
    [int]$Cpus = 2,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
    Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
}

# Default image path
if (-not $ImagePath) {
    $ImagePath = Join-Path $NoaRoot "sys/kernel/images/noa-linux.vhdx"
}

# VM configuration
$VmSwitch = "NOA-NAT-Switch"
$VmGeneration = 2

function Test-HyperVEnabled {
    $feature = Get-WindowsOptionalFeature -FeatureName Microsoft-Hyper-V-All -Online -ErrorAction SilentlyContinue
    return $feature -and $feature.State -eq "Enabled"
}

function Ensure-VmSwitch {
    $switch = Get-VMSwitch -Name $VmSwitch -ErrorAction SilentlyContinue
    if (-not $switch) {
        Write-Host "Creating NAT switch: $VmSwitch" -ForegroundColor Yellow

        # Create internal switch
        New-VMSwitch -Name $VmSwitch -SwitchType Internal

        # Configure NAT
        $ifIndex = (Get-NetAdapter | Where-Object Name -like "*$VmSwitch*").ifIndex
        New-NetIPAddress -IPAddress 192.168.100.1 -PrefixLength 24 -InterfaceIndex $ifIndex
        New-NetNat -Name "NOA-NAT" -InternalIPInterfaceAddressPrefix 192.168.100.0/24

        Write-Host "  [OK] NAT switch created" -ForegroundColor Green
    }
}

function Create-NoaVm {
    Write-Host "Creating NOA VM: $VmName" -ForegroundColor Cyan

    # Check if VM exists
    $existingVm = Get-VM -Name $VmName -ErrorAction SilentlyContinue
    if ($existingVm) {
        if ($Force) {
            Write-Host "  Removing existing VM..." -ForegroundColor Yellow
            Stop-VM -Name $VmName -Force -ErrorAction SilentlyContinue
            Remove-VM -Name $VmName -Force
        } else {
            Write-Error "VM '$VmName' already exists. Use -Force to replace."
            return
        }
    }

    # Check image
    if (-not (Test-Path $ImagePath)) {
        Write-Error "VM image not found: $ImagePath"
        Write-Host "Build the image first with: .\sys\kernel\images\build-alpine.ps1" -ForegroundColor Yellow
        return
    }

    # Ensure switch exists
    Ensure-VmSwitch

    # Create VM
    $vmPath = Join-Path $NoaRoot "sys/kernel/vms/$VmName"
    if (-not (Test-Path $vmPath)) {
        New-Item -ItemType Directory -Path $vmPath -Force | Out-Null
    }

    # Copy image to VM directory
    $vmDisk = Join-Path $vmPath "$VmName.vhdx"
    Write-Host "  Copying VM disk..." -ForegroundColor Gray
    Copy-Item -Path $ImagePath -Destination $vmDisk -Force

    # Create the VM
    New-VM -Name $VmName `
        -Generation $VmGeneration `
        -MemoryStartupBytes ($Memory * 1MB) `
        -VHDPath $vmDisk `
        -SwitchName $VmSwitch `
        -Path $vmPath

    # Configure VM
    Set-VM -Name $VmName `
        -ProcessorCount $Cpus `
        -DynamicMemory `
        -MemoryMinimumBytes (256MB) `
        -MemoryMaximumBytes ($Memory * 2MB) `
        -AutomaticStartAction Nothing `
        -AutomaticStopAction ShutDown

    # Disable secure boot for Linux
    Set-VMFirmware -VMName $VmName -EnableSecureBoot Off

    # Enable guest services
    Enable-VMIntegrationService -VMName $VmName -Name "Guest Service Interface"

    Write-Host "  [OK] VM created: $VmName" -ForegroundColor Green
    Write-Host "       Memory: ${Memory}MB (dynamic)" -ForegroundColor Gray
    Write-Host "       CPUs: $Cpus" -ForegroundColor Gray
    Write-Host "       Disk: $vmDisk" -ForegroundColor Gray
}

function Start-NoaVm {
    $vm = Get-VM -Name $VmName -ErrorAction SilentlyContinue
    if (-not $vm) {
        Write-Error "VM '$VmName' not found. Create it first with -Action create"
        return
    }

    if ($vm.State -eq "Running") {
        Write-Host "VM '$VmName' is already running" -ForegroundColor Yellow
        return
    }

    Write-Host "Starting VM: $VmName" -ForegroundColor Cyan
    $startTime = Get-Date

    Start-VM -Name $VmName

    # Wait for VM to boot (check heartbeat)
    Write-Host "  Waiting for boot..." -ForegroundColor Gray
    $timeout = 30
    $elapsed = 0
    while ($elapsed -lt $timeout) {
        $heartbeat = (Get-VMIntegrationService -VMName $VmName | Where-Object Name -eq "Heartbeat").PrimaryStatusDescription
        if ($heartbeat -eq "OK") {
            break
        }
        Start-Sleep -Seconds 1
        $elapsed++
    }

    $bootTime = ((Get-Date) - $startTime).TotalSeconds

    if ($elapsed -ge $timeout) {
        Write-Host "  [WARN] Boot timeout - VM may still be starting" -ForegroundColor Yellow
    } else {
        Write-Host "  [OK] VM started in ${bootTime}s" -ForegroundColor Green
    }

    # Get IP address
    $ip = (Get-VMNetworkAdapter -VMName $VmName).IPAddresses | Where-Object { $_ -match '^\d+\.\d+\.\d+\.\d+$' } | Select-Object -First 1
    if ($ip) {
        Write-Host "  IP: $ip" -ForegroundColor Gray
    }
}

function Stop-NoaVm {
    $vm = Get-VM -Name $VmName -ErrorAction SilentlyContinue
    if (-not $vm) {
        Write-Error "VM '$VmName' not found"
        return
    }

    if ($vm.State -eq "Off") {
        Write-Host "VM '$VmName' is already stopped" -ForegroundColor Yellow
        return
    }

    Write-Host "Stopping VM: $VmName" -ForegroundColor Cyan

    if ($Force) {
        Stop-VM -Name $VmName -Force
    } else {
        Stop-VM -Name $VmName -Save:$false
    }

    Write-Host "  [OK] VM stopped" -ForegroundColor Green
}

function Get-NoaVmStatus {
    $vm = Get-VM -Name $VmName -ErrorAction SilentlyContinue
    if (-not $vm) {
        Write-Host "VM '$VmName' not found" -ForegroundColor Yellow
        return
    }

    Write-Host "NOA VM Status: $VmName" -ForegroundColor Cyan
    Write-Host "  State: $($vm.State)" -ForegroundColor $(if ($vm.State -eq "Running") { "Green" } else { "Yellow" })
    Write-Host "  Uptime: $($vm.Uptime)" -ForegroundColor Gray
    Write-Host "  Memory: $([math]::Round($vm.MemoryAssigned / 1MB))MB assigned" -ForegroundColor Gray
    Write-Host "  CPUs: $($vm.ProcessorCount)" -ForegroundColor Gray

    if ($vm.State -eq "Running") {
        $ip = (Get-VMNetworkAdapter -VMName $VmName).IPAddresses | Where-Object { $_ -match '^\d+\.\d+\.\d+\.\d+$' } | Select-Object -First 1
        if ($ip) {
            Write-Host "  IP: $ip" -ForegroundColor Gray
        }

        $heartbeat = (Get-VMIntegrationService -VMName $VmName | Where-Object Name -eq "Heartbeat").PrimaryStatusDescription
        Write-Host "  Heartbeat: $heartbeat" -ForegroundColor $(if ($heartbeat -eq "OK") { "Green" } else { "Yellow" })
    }
}

function Remove-NoaVm {
    $vm = Get-VM -Name $VmName -ErrorAction SilentlyContinue
    if (-not $vm) {
        Write-Host "VM '$VmName' not found" -ForegroundColor Yellow
        return
    }

    Write-Host "Destroying VM: $VmName" -ForegroundColor Red

    if ($vm.State -ne "Off") {
        Write-Host "  Stopping VM first..." -ForegroundColor Gray
        Stop-VM -Name $VmName -Force
    }

    Remove-VM -Name $VmName -Force

    # Remove VM directory
    $vmPath = Join-Path $NoaRoot "sys/kernel/vms/$VmName"
    if (Test-Path $vmPath) {
        Remove-Item -Path $vmPath -Recurse -Force
    }

    Write-Host "  [OK] VM destroyed" -ForegroundColor Green
}

function Get-NoaVmList {
    Write-Host "NOA VMs:" -ForegroundColor Cyan

    $vms = Get-VM | Where-Object { $_.Name -like "noa*" }
    if ($vms) {
        $vms | Format-Table Name, State, @{N="Memory(MB)";E={[math]::Round($_.MemoryAssigned/1MB)}}, ProcessorCount, Uptime -AutoSize
    } else {
        Write-Host "  No NOA VMs found" -ForegroundColor Gray
    }
}

function Connect-NoaVmConsole {
    $vm = Get-VM -Name $VmName -ErrorAction SilentlyContinue
    if (-not $vm) {
        Write-Error "VM '$VmName' not found"
        return
    }

    if ($vm.State -ne "Running") {
        Write-Error "VM must be running to connect. Start it first."
        return
    }

    Write-Host "Connecting to VM console: $VmName" -ForegroundColor Cyan
    vmconnect.exe localhost $VmName
}

# Check Hyper-V
if (-not (Test-HyperVEnabled)) {
    Write-Error "Hyper-V is not enabled. Enable it with: Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All"
    exit 1
}

# Execute action
switch ($Action) {
    "create" { Create-NoaVm }
    "start" { Start-NoaVm }
    "stop" { Stop-NoaVm }
    "status" { Get-NoaVmStatus }
    "destroy" { Remove-NoaVm }
    "list" { Get-NoaVmList }
    "console" { Connect-NoaVmConsole }
}

