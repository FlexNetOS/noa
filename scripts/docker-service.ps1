<#
.SYNOPSIS
    NOA Docker Service Manager for Windows

.DESCRIPTION
    Manages Docker daemon contained within NOA root.
    Windows equivalent of scripts/docker-service (bash)

.PARAMETER Action
    Action to perform: start, stop, status

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\docker-service.ps1 -Action start
    .\docker-service.ps1 -Action status
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("start", "stop", "status")]
    [string]$Action,

    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$DOCKER_BIN = Join-Path $NoaRoot "bin/docker.exe"
$DOCKERD_BIN = Join-Path $NoaRoot "bin/dockerd.exe"
$DOCKER_DATA = Join-Path $NoaRoot "containers/docker-data"
$DOCKER_configs = Join-Path $NoaRoot "etc/docker/daemon.json"
$DOCKER_PID = Join-Path $NoaRoot "init/run/dockerd.pid"
$DOCKER_PIPE = "\\.\pipe\noa-docker"

switch ($Action) {
    "start" {
        Write-Host "Starting Docker (contained in $NoaRoot)..." -ForegroundColor Cyan

        # Create directories
        @($DOCKER_DATA, (Join-Path $NoaRoot "init/run")) | ForEach-Object {
            if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ -Force | Out-Null }
        }

        # Check if already running
        if (Test-Path $DOCKER_PID) {
            $pid = Get-Content $DOCKER_PID
            if (Get-Process -Id $pid -ErrorAction SilentlyContinue) {
                Write-Host "Docker is already running (PID $pid)" -ForegroundColor Yellow
                exit 0
            }
        }

        # Start dockerd
        if (Test-Path $DOCKERD_BIN) {
            $args = @(
                "--data-root", $DOCKER_DATA,
                "--host", "npipe://$DOCKER_PIPE"
            )
            if (Test-Path $DOCKER_configs) {
                $args += "--configs-file", $DOCKER_configs
            }

            $process = Start-Process -FilePath $DOCKERD_BIN -ArgumentList $args -PassThru -WindowStyle Hidden
            $process.Id | Set-Content -Path $DOCKER_PID
            Write-Host "Docker daemon started (PID $($process.Id))" -ForegroundColor Green
        } else {
            Write-Host "Docker daemon not found. Using Docker Desktop or system Docker." -ForegroundColor Yellow
            Write-Host "To use contained Docker, install dockerd to $DOCKERD_BIN" -ForegroundColor Gray
        }
    }

    "stop" {
        Write-Host "Stopping Docker..." -ForegroundColor Cyan

        if (Test-Path $DOCKER_PID) {
            $pid = Get-Content $DOCKER_PID
            $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
            if ($process) {
                Stop-Process -Id $pid -Force
                Write-Host "Docker daemon stopped" -ForegroundColor Green
            }
            Remove-Item -Path $DOCKER_PID -Force
        } else {
            Write-Host "Docker PID file not found" -ForegroundColor Yellow
        }
    }

    "status" {
        if (Test-Path $DOCKER_PID) {
            $pid = Get-Content $DOCKER_PID
            $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
            if ($process) {
                Write-Host "Docker is running (PID $pid)" -ForegroundColor Green

                # Try to get container list
                if (Test-Path $DOCKER_BIN) {
                    & $DOCKER_BIN --host "npipe://$DOCKER_PIPE" ps 2>$null
                }
            } else {
                Write-Host "Docker is not running (stale PID file)" -ForegroundColor Yellow
            }
        } else {
            Write-Host "Docker is not running" -ForegroundColor Yellow
        }
    }
}

