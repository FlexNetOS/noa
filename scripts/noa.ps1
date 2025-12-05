<#
.SYNOPSIS
    NOA CLI Tool - P2P Server & Multi-Provider AI Management (Windows)
    
.DESCRIPTION
    Main NOA command-line interface for Windows.
    
.PARAMETER Command
    The command to execute (start, stop, status, ai, device, etc.)
    
.EXAMPLE
    .\noa.ps1 status
    .\noa.ps1 ai providers
#>

param(
    [Parameter(Position=0)]
    [string]$Command,
    
    [Parameter(Position=1)]
    [string]$SubCommand,
    
    [Parameter(Position=2)]
    [string]$Arg1
)

$NOA_ROOT = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
$NOA_CONFIG = Join-Path $NOA_ROOT "config"
$NOA_CONFIG_FILE = Join-Path $NOA_CONFIG "noa-server.json"
$AI_PROVIDERS_CONFIG = Join-Path $NOA_CONFIG "ai-providers.json"
$DEVICE_CONFIG = Join-Path $NOA_CONFIG "device-orchestration.json"

function Show-Usage {
    Write-Host "NOA CLI - P2P Server & AI Management" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: noa <command> [subcommand] [args]" -ForegroundColor White
    Write-Host ""
    Write-Host "Commands:" -ForegroundColor Yellow
    Write-Host "  start       Start NOA P2P server"
    Write-Host "  stop        Stop NOA P2P server"
    Write-Host "  status      Show server status"
    Write-Host "  nodes       List connected nodes"
    Write-Host "  storage     Show storage information"
    Write-Host "  compute     Show compute resources"
    Write-Host "  ai          AI provider management"
    Write-Host "  device      Device management"
    Write-Host "  env         Environment information"
    Write-Host "  validate    Validate environment setup"
    Write-Host ""
    Write-Host "AI Subcommands:" -ForegroundColor Yellow
    Write-Host "  ai providers    List available AI providers"
    Write-Host "  ai devices      List registered devices"
    Write-Host "  ai shared       Show shared AI resources"
    Write-Host "  ai switch <p>   Switch to provider <p>"
    Write-Host ""
    Write-Host "Device Subcommands:" -ForegroundColor Yellow
    Write-Host "  device register <name>   Register a device"
    Write-Host "  device list              List devices"
    Write-Host "  device capabilities      Show device capabilities"
}

switch ($Command) {
    "start" {
        Write-Host "Starting NOA P2P server..." -ForegroundColor Cyan
        # TODO: Implement server start
        Write-Host "Server start not yet implemented" -ForegroundColor Yellow
    }
    
    "stop" {
        Write-Host "Stopping NOA P2P server..." -ForegroundColor Cyan
        # TODO: Implement server stop
        Write-Host "Server stop not yet implemented" -ForegroundColor Yellow
    }
    
    "status" {
        Write-Host "NOA P2P Server Status" -ForegroundColor Cyan
        Write-Host "  Root: $NOA_ROOT"
        Write-Host "  Config: $(if (Test-Path $NOA_CONFIG_FILE) { 'Found' } else { 'Not found' })"
        # TODO: Implement full status check
    }
    
    "nodes" {
        Write-Host "Connected nodes..." -ForegroundColor Cyan
        # TODO: Implement node listing
        Write-Host "Node listing not yet implemented" -ForegroundColor Yellow
    }
    
    "storage" {
        Write-Host "Storage Information" -ForegroundColor Cyan
        $storagePath = Join-Path $NOA_ROOT "p2p/storage"
        if (Test-Path $storagePath) {
            Get-ChildItem $storagePath -Recurse | Measure-Object -Property Length -Sum | 
                ForEach-Object { Write-Host "  Total size: $([math]::Round($_.Sum / 1MB, 2)) MB" }
        } else {
            Write-Host "  Storage directory not found: $storagePath" -ForegroundColor Yellow
        }
        
        # Show drive info
        $drive = (Get-Item $NOA_ROOT).PSDrive
        Write-Host "  Drive $($drive.Name): $([math]::Round($drive.Free / 1GB, 2)) GB free"
    }
    
    "compute" {
        Write-Host "Compute Resources" -ForegroundColor Cyan
        $cpu = Get-CimInstance Win32_Processor
        $mem = Get-CimInstance Win32_OperatingSystem
        Write-Host "  CPU: $($cpu.Name)"
        Write-Host "  Cores: $($cpu.NumberOfCores) ($($cpu.NumberOfLogicalProcessors) logical)"
        Write-Host "  RAM: $([math]::Round($mem.TotalVisibleMemorySize / 1MB, 2)) GB total"
        Write-Host "  RAM Free: $([math]::Round($mem.FreePhysicalMemory / 1MB, 2)) GB"
        
        # Check for GPU
        $gpu = Get-CimInstance Win32_VideoController | Where-Object { $_.Name -match "NVIDIA|AMD|Intel" }
        if ($gpu) {
            Write-Host "  GPU: $($gpu.Name)"
        }
    }
    
    "ai" {
        switch ($SubCommand) {
            "providers" {
                Write-Host "Available AI Providers:" -ForegroundColor Cyan
                if (Test-Path $AI_PROVIDERS_CONFIG) {
                    Get-Content $AI_PROVIDERS_CONFIG | ConvertFrom-Json | 
                        Select-Object -ExpandProperty providers -ErrorAction SilentlyContinue |
                        ForEach-Object { Write-Host "  - $_" }
                } else {
                    Write-Host "  Config not found: $AI_PROVIDERS_CONFIG" -ForegroundColor Yellow
                }
            }
            "devices" {
                Write-Host "Registered Devices:" -ForegroundColor Cyan
                if (Test-Path $DEVICE_CONFIG) {
                    Get-Content $DEVICE_CONFIG | ConvertFrom-Json |
                        Select-Object -ExpandProperty deviceTypes -ErrorAction SilentlyContinue |
                        ForEach-Object { Write-Host "  - $_" }
                } else {
                    Write-Host "  Config not found: $DEVICE_CONFIG" -ForegroundColor Yellow
                }
            }
            "shared" {
                Write-Host "Shared AI Resources:" -ForegroundColor Cyan
                $sharedPath = Join-Path $NOA_ROOT "ai/shared"
                if (Test-Path $sharedPath) {
                    Get-ChildItem $sharedPath | ForEach-Object { Write-Host "  - $($_.Name)" }
                } else {
                    Write-Host "  Shared directory not found" -ForegroundColor Yellow
                }
            }
            "switch" {
                if ($Arg1) {
                    Write-Host "Switching AI provider to: $Arg1" -ForegroundColor Cyan
                    # TODO: Implement provider switching
                } else {
                    Write-Host "Usage: noa ai switch <provider>" -ForegroundColor Yellow
                }
            }
            default {
                Write-Host "Usage: noa ai {providers|devices|shared|switch <provider>}" -ForegroundColor Yellow
            }
        }
    }
    
    "device" {
        switch ($SubCommand) {
            "register" {
                if ($Arg1) {
                    Write-Host "Registering device: $Arg1" -ForegroundColor Cyan
                    # TODO: Implement device registration
                } else {
                    Write-Host "Usage: noa device register <name>" -ForegroundColor Yellow
                }
            }
            "list" {
                Write-Host "Registered Devices:" -ForegroundColor Cyan
                $devicesPath = Join-Path $NOA_ROOT "ai/devices/registered"
                if (Test-Path $devicesPath) {
                    Get-ChildItem $devicesPath | ForEach-Object { Write-Host "  - $($_.Name)" }
                } else {
                    Write-Host "  No devices registered" -ForegroundColor Yellow
                }
            }
            "capabilities" {
                Write-Host "Device Capabilities:" -ForegroundColor Cyan
                if (Test-Path $DEVICE_CONFIG) {
                    Get-Content $DEVICE_CONFIG -Raw
                } else {
                    Write-Host "  Config not found" -ForegroundColor Yellow
                }
            }
            default {
                Write-Host "Usage: noa device {register <name>|list|capabilities}" -ForegroundColor Yellow
            }
        }
    }
    
    "env" {
        Write-Host "NOA Environment" -ForegroundColor Cyan
        Write-Host "  NOA_ROOT: $env:NOA_ROOT"
        Write-Host "  NOA_SCRIPTS: $env:NOA_SCRIPTS"
        Write-Host "  NOA_CONFIG: $env:NOA_CONFIG"
        Write-Host "  NOA_AI: $env:NOA_AI"
        Write-Host "  NOA_BIN: $env:NOA_BIN"
        Write-Host ""
        Write-Host "Platform: Windows $([System.Environment]::OSVersion.Version)" -ForegroundColor Gray
        Write-Host "PowerShell: $($PSVersionTable.PSVersion)" -ForegroundColor Gray
    }
    
    "validate" {
        Write-Host "Validating NOA Environment..." -ForegroundColor Cyan
        $errors = 0
        
        $requiredDirs = @(
            @{ Path = $NOA_ROOT; Name = "NOA_ROOT" },
            @{ Path = (Join-Path $NOA_ROOT "scripts"); Name = "scripts" },
            @{ Path = (Join-Path $NOA_ROOT "config"); Name = "config" },
            @{ Path = (Join-Path $NOA_ROOT "ai"); Name = "ai" },
            @{ Path = (Join-Path $NOA_ROOT "bin"); Name = "bin" }
        )
        
        foreach ($dir in $requiredDirs) {
            if (Test-Path $dir.Path) {
                Write-Host "  [OK] $($dir.Name): $($dir.Path)" -ForegroundColor Green
            } else {
                Write-Host "  [MISSING] $($dir.Name): $($dir.Path)" -ForegroundColor Red
                $errors++
            }
        }
        
        # Check for required tools
        Write-Host ""
        Write-Host "Checking tools..." -ForegroundColor Cyan
        $tools = @("git", "gh", "node", "python")
        foreach ($tool in $tools) {
            $cmd = Get-Command $tool -ErrorAction SilentlyContinue
            if ($cmd) {
                Write-Host "  [OK] $tool" -ForegroundColor Green
            } else {
                Write-Host "  [MISSING] $tool" -ForegroundColor Yellow
            }
        }
        
        if ($errors -eq 0) {
            Write-Host ""
            Write-Host "Environment validated successfully!" -ForegroundColor Green
        } else {
            Write-Host ""
            Write-Host "$errors error(s) found" -ForegroundColor Red
        }
    }
    
    default {
        Show-Usage
    }
}
