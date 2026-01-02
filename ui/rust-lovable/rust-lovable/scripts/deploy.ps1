# Rust Lovable - Dynamic Deployment Script for Windows
# Integrates with hardware detection for platform-aware deployment

param(
    [int]$Port = 8080,
    [string]$ServerHost = '127.0.0.1',
    [string]$DeployDir = '',
    [string]$ConfigDir = '',
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

if (-not $DeployDir) { $DeployDir = Join-Path $ProjectRoot 'deployed' }
if (-not $ConfigDir) { $ConfigDir = Join-Path $ProjectRoot 'config' }
$LogDir = Join-Path $ProjectRoot 'logs'

function Write-Header {
    Write-Host '========================================' -ForegroundColor Cyan
    Write-Host '  Rust Lovable - Dynamic Deployment' -ForegroundColor Cyan
    Write-Host "  $(Get-Date)" -ForegroundColor Cyan
    Write-Host '========================================' -ForegroundColor Cyan
}

function Write-Status($Message) { Write-Host "[INFO] $Message" -ForegroundColor Blue }
function Write-Success($Message) { Write-Host "[SUCCESS] $Message" -ForegroundColor Green }
function Write-Warning($Message) { Write-Host "[WARNING] $Message" -ForegroundColor Yellow }
function Write-Error($Message) { Write-Host "[ERROR] $Message" -ForegroundColor Red }

if ($Help) {
    Write-Host @'
Usage: .\deploy.ps1 [OPTIONS]

Options:
  -Port PORT       Server port (default: 8080)
  -Host HOST       Server host (default: 127.0.0.1)
  -DeployDir DIR   Deployment directory
  -ConfigDir DIR   Configuration directory
  -Help            Show this help message
'@
    exit 0
}

# Step 1: Detect hardware
function Get-HardwareInfo {
    Write-Status 'Detecting hardware configuration...'
    
    $CpuInfo = Get-CimInstance -ClassName Win32_Processor | Select-Object -First 1
    $MemoryInfo = Get-CimInstance -ClassName Win32_ComputerSystem
    $GpuInfo = Get-CimInstance -ClassName Win32_VideoController | Select-Object -First 1
    $DiskInfo = Get-CimInstance -ClassName Win32_LogicalDisk -Filter "DeviceID='C:'"
    
    $GpuMemoryMB = 0
    if ($GpuInfo -and $GpuInfo.AdapterRAM) {
        $GpuMemoryMB = [math]::Round($GpuInfo.AdapterRAM / 1MB, 0)
    }
    
    $AvailableMemGB = 0
    try {
        $AvailableMemGB = [math]::Round((Get-Counter '\Memory\Available MBytes' -ErrorAction SilentlyContinue).CounterSamples[0].CookedValue / 1024, 2)
    }
    catch {
        $AvailableMemGB = 0
    }
    
    $Hardware = @{
        cpu       = @{
            model         = $CpuInfo.Name
            cores         = $CpuInfo.NumberOfCores
            threads       = $CpuInfo.NumberOfLogicalProcessors
            architecture  = $CpuInfo.AddressWidth
            max_clock_mhz = $CpuInfo.MaxClockSpeed
        }
        memory    = @{
            total_gb     = [math]::Round($MemoryInfo.TotalPhysicalMemory / 1GB, 2)
            available_gb = $AvailableMemGB
        }
        gpu       = @{
            available = ($null -ne $GpuInfo)
            model     = if ($GpuInfo) { $GpuInfo.Name } else { 'None' }
            memory_mb = $GpuMemoryMB
        }
        storage   = @{
            total_gb      = [math]::Round($DiskInfo.Size / 1GB, 2)
            free_gb       = [math]::Round($DiskInfo.FreeSpace / 1GB, 2)
            usage_percent = [math]::Round((($DiskInfo.Size - $DiskInfo.FreeSpace) / $DiskInfo.Size) * 100, 1)
        }
        os        = @{
            name     = 'Windows'
            version  = [System.Environment]::OSVersion.Version.ToString()
            hostname = $env:COMPUTERNAME
        }
        timestamp = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
    }
    
    Write-Success 'Hardware detected:'
    Write-Host "  - CPU: $($Hardware.cpu.model)"
    Write-Host "  - Cores: $($Hardware.cpu.cores) (Threads: $($Hardware.cpu.threads))"
    Write-Host "  - Memory: $($Hardware.memory.total_gb) GB"
    Write-Host "  - GPU: $($Hardware.gpu.model)"
    
    # Save hardware info
    New-Item -ItemType Directory -Force -Path $ConfigDir | Out-Null
    $Hardware | ConvertTo-Json -Depth 5 | Set-Content (Join-Path $ConfigDir 'hardware.json')
    Write-Success "Hardware info saved to $ConfigDir\hardware.json"
    
    return $Hardware
}

# Step 2: Configure based on hardware
function Set-DeploymentConfig {
    param($Hardware)
    
    Write-Status 'Configuring deployment based on hardware...'
    
    $CpuCores = $Hardware.cpu.cores
    $MemoryGB = $Hardware.memory.total_gb
    $GpuAvailable = $Hardware.gpu.available
    
    # Determine optimal configuration
    if ($CpuCores -ge 8 -and $MemoryGB -ge 16) {
        $Profile = 'high-performance'
        $WorkerThreads = [int]($CpuCores / 2)
        $MaxConnections = 10000
        $CacheSizeMB = [int]($MemoryGB * 100)
    }
    elseif ($CpuCores -ge 4 -and $MemoryGB -ge 8) {
        $Profile = 'standard'
        $WorkerThreads = 4
        $MaxConnections = 5000
        $CacheSizeMB = 500
    }
    else {
        $Profile = 'minimal'
        $WorkerThreads = 2
        $MaxConnections = 1000
        $CacheSizeMB = 256
    }
    
    Write-Success "Deployment profile: $Profile"
    Write-Host "  - Worker threads: $WorkerThreads"
    Write-Host "  - Max connections: $MaxConnections"
    Write-Host "  - Cache size: ${CacheSizeMB}MB"
    
    $RuntimeConfig = @{
        profile   = $Profile
        server    = @{
            host            = $ServerHost
            port            = $Port
            worker_threads  = $WorkerThreads
            max_connections = $MaxConnections
        }
        cache     = @{
            size_mb = $CacheSizeMB
            enabled = $true
        }
        gpu       = @{
            enabled      = $GpuAvailable
            acceleration = $GpuAvailable
        }
        logging   = @{
            level       = 'info'
            directory   = $LogDir
            max_size_mb = 100
            max_files   = 10
        }
        timestamp = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
    }
    
    $RuntimeConfig | ConvertTo-Json -Depth 5 | Set-Content (Join-Path $ConfigDir 'runtime.json')
    Write-Success "Runtime configuration saved to $ConfigDir\runtime.json"
    
    return $RuntimeConfig
}

# Step 3: Prepare deployment directory
function Initialize-Deployment {
    Write-Status 'Preparing deployment directory...'
    
    New-Item -ItemType Directory -Force -Path $DeployDir | Out-Null
    New-Item -ItemType Directory -Force -Path $LogDir | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $DeployDir 'config') | Out-Null
    
    # Find and copy binary
    $BinaryPaths = @(
        (Join-Path $ProjectRoot 'dist\rust-lovable.exe'),
        (Join-Path $ProjectRoot 'target\release\rust-lovable.exe')
    )
    
    $BinaryFound = $false
    foreach ($Path in $BinaryPaths) {
        if (Test-Path $Path) {
            Copy-Item $Path (Join-Path $DeployDir 'rust-lovable.exe') -Force
            $BinaryFound = $true
            Write-Success "Binary copied from $Path"
            break
        }
    }
    
    if (-not $BinaryFound) {
        Write-Error 'Binary not found! Run prepare-deployment.sh first.'
        exit 1
    }
    
    # Copy configuration files
    if (Test-Path (Join-Path $ConfigDir 'runtime.json')) {
        Copy-Item (Join-Path $ConfigDir 'runtime.json') (Join-Path $DeployDir 'config\') -Force
    }
    if (Test-Path (Join-Path $ConfigDir 'hardware.json')) {
        Copy-Item (Join-Path $ConfigDir 'hardware.json') (Join-Path $DeployDir 'config\') -Force
    }
    
    Write-Success "Deployment directory prepared: $DeployDir"
}

# Step 4: Create launcher scripts
function New-LauncherScripts {
    Write-Status 'Creating launcher scripts...'
    
    # PowerShell start script
    $StartScript = @'
# Rust Lovable - Start Script
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$env:RUST_LOVABLE_CONFIG_DIR = "$ScriptDir\config"
$env:RUST_LOG = if ($env:RUST_LOG) { $env:RUST_LOG } else { "info" }

$RuntimeConfig = "$ScriptDir\config\runtime.json"
if (Test-Path $RuntimeConfig) {
    $Config = Get-Content $RuntimeConfig | ConvertFrom-Json
    $env:RUST_LOVABLE_ADDRESS = "$($Config.server.host):$($Config.server.port)"
}

Write-Host "Starting Rust Lovable..." -ForegroundColor Cyan
Write-Host "Server: $env:RUST_LOVABLE_ADDRESS" -ForegroundColor Green
Write-Host "Config: $env:RUST_LOVABLE_CONFIG_DIR" -ForegroundColor Green

& "$ScriptDir\rust-lovable.exe" @args
'@
    Set-Content (Join-Path $DeployDir 'start.ps1') $StartScript
    
    # Stop script
    $StopScript = @'
# Rust Lovable - Stop Script
Write-Host "Stopping Rust Lovable..." -ForegroundColor Yellow
Get-Process -Name "rust-lovable" -ErrorAction SilentlyContinue | Stop-Process -Force
Write-Host "Stopped." -ForegroundColor Green
'@
    Set-Content (Join-Path $DeployDir 'stop.ps1') $StopScript
    
    # Status script
    $StatusScript = @'
# Rust Lovable - Status Script
$Process = Get-Process -Name "rust-lovable" -ErrorAction SilentlyContinue
if ($Process) {
    Write-Host "Rust Lovable is running (PID: $($Process.Id))" -ForegroundColor Green
    Write-Host "Memory: $([math]::Round($Process.WorkingSet64 / 1MB, 2)) MB" -ForegroundColor Cyan
    Write-Host "CPU Time: $($Process.TotalProcessorTime)" -ForegroundColor Cyan
} else {
    Write-Host "Rust Lovable is not running" -ForegroundColor Yellow
}
'@
    Set-Content (Join-Path $DeployDir 'status.ps1') $StatusScript
    
    Write-Success 'Launcher scripts created'
}

# Step 5: Deploy and start
function Start-Deployment {
    Write-Status 'Starting application...'
    
    $env:RUST_LOVABLE_ADDRESS = "${Host}:${Port}"
    $env:RUST_LOVABLE_CONFIG_DIR = Join-Path $DeployDir 'config'
    $env:RUST_LOG = 'info'
    
    $Binary = Join-Path $DeployDir 'rust-lovable.exe'
    $LogFile = Join-Path $LogDir 'rust-lovable.log'
    
    # Start as background process
    $ProcessInfo = Start-Process -FilePath $Binary `
        -WorkingDirectory $DeployDir `
        -RedirectStandardOutput $LogFile `
        -RedirectStandardError (Join-Path $LogDir 'rust-lovable-error.log') `
        -PassThru `
        -WindowStyle Hidden
    
    # Save PID
    $ProcessInfo.Id | Set-Content (Join-Path $DeployDir 'rust-lovable.pid')
    
    Start-Sleep -Seconds 2
    
    if (-not $ProcessInfo.HasExited) {
        Write-Success "Application started successfully (PID: $($ProcessInfo.Id))"
        Write-Success "Server running at http://${Host}:${Port}"
    }
    else {
        Write-Error "Failed to start application. Check logs at $LogFile"
        exit 1
    }
}

# Main
Write-Header

$Hardware = Get-HardwareInfo
Write-Host ''

$Config = Set-DeploymentConfig -Hardware $Hardware
Write-Host ''

Initialize-Deployment
Write-Host ''

New-LauncherScripts
Write-Host ''

Start-Deployment

Write-Host ''
Write-Host '========================================' -ForegroundColor Green
Write-Host '  Deployment Complete!' -ForegroundColor Green
Write-Host '========================================' -ForegroundColor Green
Write-Host ''
Write-Host "Deployment directory: $DeployDir"
Write-Host "Log directory: $LogDir"
Write-Host "Configuration: $DeployDir\config\"
Write-Host ''
Write-Host 'Useful commands:'
Write-Host "  Start:  $DeployDir\start.ps1"
Write-Host "  Stop:   $DeployDir\stop.ps1"
Write-Host "  Status: $DeployDir\status.ps1"
Write-Host "  Logs:   Get-Content $LogDir\rust-lovable.log -Wait"
Write-Host ''
Write-Host "Server URL: http://${Host}:${Port}" -ForegroundColor Cyan

