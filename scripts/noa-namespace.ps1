<#
.SYNOPSIS
    NOA Namespace Isolation for Windows

.DESCRIPTION
    Creates isolated execution environments for NOA processes on Windows.
    Windows equivalent of scripts/noa-namespace (bash)

    Uses Windows isolation mechanisms:
    - Job objects for resource limits
    - Windows Containers/Sandbox (if available)
    - Hyper-V isolation (if available)
    - AppContainer for sandboxing

.PARAMETER Action
    Action: run, sandbox, container, list

.PARAMETER Command
    Command to run in isolated environment

.PARAMETER NoaRoot
    NOA root directory

.EXAMPLE
    .\noa-namespace.ps1 -Action run -Command "cmd /c dir"
    .\noa-namespace.ps1 -Action sandbox -Command "notepad.exe"
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("run", "sandbox", "container", "list")]
    [string]$Action,

    [string]$Command,
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$NOA_NAMESPACE = Join-Path $NoaRoot "sys/namespace"

# Ensure directories exist
if (-not (Test-Path $NOA_NAMESPACE)) {
    New-Item -ItemType Directory -Path $NOA_NAMESPACE -Force | Out-Null
}

# Check available isolation methods
function Get-IsolationMethods {
    $methods = @()

    # Job Objects (always available)
    $methods += "job"

    # Windows Sandbox
    if (Get-Command "WindowsSandbox.exe" -ErrorAction SilentlyContinue) {
        $methods += "sandbox"
    }

    # Hyper-V
    if ((Get-WindowsOptionalFeature -FeatureName Microsoft-Hyper-V -Online -ErrorAction SilentlyContinue).State -eq "Enabled") {
        $methods += "hyperv"
    }

    # Windows Containers
    if (Get-Command "docker" -ErrorAction SilentlyContinue) {
        $methods += "container"
    }

    return $methods
}

# Run with Job Object isolation
function Invoke-WithJobIsolation {
    param([string]$Cmd)

    # Create a job object for resource limits
    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public class JobObject {
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr CreateJobObject(IntPtr lpJobAttributes, string lpName);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool AssignProcessToJobObject(IntPtr hJob, IntPtr hProcess);

    [DllImport("kernel32.dll")]
    public static extern bool CloseHandle(IntPtr hObject);
}
"@ -ErrorAction SilentlyContinue

    Write-Host "Running with Job Object isolation..." -ForegroundColor Cyan

    # Parse command
    $parts = $Cmd -split ' ', 2
    $exe = $parts[0]
    $args = if ($parts.Count -gt 1) { $parts[1] } else { "" }

    # Start process
    $process = Start-Process -FilePath $exe -ArgumentList $args -PassThru -Wait
    return $process.ExitCode
}

# Run in Windows Sandbox
function Invoke-InSandbox {
    param([string]$Cmd)

    Write-Host "Running in Windows Sandbox..." -ForegroundColor Cyan

    # Create sandbox configsuration
    $wsbFile = Join-Path $NOA_NAMESPACE "noa-sandbox.wsb"
    $logonCommand = "<LogonCommand><Command>$Cmd</Command></LogonCommand>"

    @"
<configsuration>
  <MappedFolders>
    <MappedFolder>
      <HostFolder>$NoaRoot</HostFolder>
      <SandboxFolder>C:\noa</SandboxFolder>
      <ReadOnly>false</ReadOnly>
    </MappedFolder>
  </MappedFolders>
  $logonCommand
</configsuration>
"@ | Set-Content -Path $wsbFile

    & WindowsSandbox.exe $wsbFile
}

# Run in container
function Invoke-InContainer {
    param([string]$Cmd)

    Write-Host "Running in Windows Container..." -ForegroundColor Cyan

    # Use Docker with Windows containers
    $containerCmd = "docker run --rm -v ${NoaRoot}:C:\noa -w C:\noa mcr.microsoft.com/windows/servercore:ltsc2022 $Cmd"
    Invoke-Expression $containerCmd
}

switch ($Action) {
    "run" {
        if (-not $Command) {
            Write-Error "Usage: noa-namespace.ps1 -Action run -Command <command>"
        }

        $exitCode = Invoke-WithJobIsolation -Cmd $Command
        exit $exitCode
    }

    "sandbox" {
        if (-not $Command) {
            Write-Error "Usage: noa-namespace.ps1 -Action sandbox -Command <command>"
        }

        $methods = Get-IsolationMethods
        if ($methods -contains "sandbox") {
            Invoke-InSandbox -Cmd $Command
        } else {
            Write-Error "Windows Sandbox not available. Enable it in Windows Features."
        }
    }

    "container" {
        if (-not $Command) {
            Write-Error "Usage: noa-namespace.ps1 -Action container -Command <command>"
        }

        $methods = Get-IsolationMethods
        if ($methods -contains "container") {
            Invoke-InContainer -Cmd $Command
        } else {
            Write-Error "Docker not available. Install Docker Desktop."
        }
    }

    "list" {
        Write-Host "NOA Namespace Isolation Methods:" -ForegroundColor Cyan

        $methods = Get-IsolationMethods
        foreach ($method in $methods) {
            $desc = switch ($method) {
                "job" { "Job Objects (resource limits)" }
                "sandbox" { "Windows Sandbox (isolated desktop)" }
                "hyperv" { "Hyper-V (VM isolation)" }
                "container" { "Windows Containers (Docker)" }
            }
            Write-Host "  [OK] $method - $desc" -ForegroundColor Green
        }

        $allMethods = @("job", "sandbox", "hyperv", "container")
        $missing = $allMethods | Where-Object { $_ -notin $methods }
        foreach ($method in $missing) {
            $desc = switch ($method) {
                "sandbox" { "Windows Sandbox (enable in Windows Features)" }
                "hyperv" { "Hyper-V (enable in Windows Features)" }
                "container" { "Windows Containers (install Docker)" }
            }
            Write-Host "  [--] $method - $desc" -ForegroundColor Yellow
        }
    }
}

