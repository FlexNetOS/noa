<#
.SYNOPSIS
    NOA Gitea Service Manager for Windows

.DESCRIPTION
    Manages Gitea Git server contained within NOA root.
    Windows equivalent of scripts/gitea-service (bash)

.PARAMETER Action
    Action to perform: start, stop, status

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\gitea-service.ps1 -Action start
    .\gitea-service.ps1 -Action status
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

$GITEA_BIN = Join-Path $NoaRoot "bin/gitea.exe"
$GITEA_ROOT = Join-Path $NoaRoot "git/gitea"
$GITEA_configs = Join-Path $GITEA_ROOT "configs/app.ini"
$GITEA_PID = Join-Path $NoaRoot "init/run/gitea.pid"
$GITEA_PORT = 3000

switch ($Action) {
    "start" {
        Write-Host "Starting Gitea (contained in $NoaRoot)..." -ForegroundColor Cyan

        # Create directories
        $dirs = @(
            $GITEA_ROOT,
            (Join-Path $GITEA_ROOT "configs"),
            (Join-Path $GITEA_ROOT "data"),
            (Join-Path $GITEA_ROOT "repos"),
            (Join-Path $NoaRoot "init/run")
        )
        foreach ($dir in $dirs) {
            if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
        }

        # Check if already running
        if (Test-Path $GITEA_PID) {
            $pid = Get-Content $GITEA_PID
            if (Get-Process -Id $pid -ErrorAction SilentlyContinue) {
                Write-Host "Gitea is already running (PID $pid)" -ForegroundColor Yellow
                exit 0
            }
        }

        if (-not (Test-Path $GITEA_BIN)) {
            Write-Error "Gitea not found at $GITEA_BIN. Download from https://gitea.io/downloads"
        }

        # Set working directory and start
        Push-Location $GITEA_ROOT
        try {
            $args = @("web")
            if (Test-Path $GITEA_configs) {
                $args += "--configs", $GITEA_configs
            }

            $process = Start-Process -FilePath $GITEA_BIN -ArgumentList $args -PassThru -WindowStyle Hidden
            $process.Id | Set-Content -Path $GITEA_PID
            Write-Host "Gitea started (PID $($process.Id))" -ForegroundColor Green
            Write-Host "  URL: http://localhost:$GITEA_PORT" -ForegroundColor Gray
        } finally {
            Pop-Location
        }
    }

    "stop" {
        Write-Host "Stopping Gitea..." -ForegroundColor Cyan

        if (Test-Path $GITEA_PID) {
            $pid = Get-Content $GITEA_PID
            $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
            if ($process) {
                Stop-Process -Id $pid -Force
                Write-Host "Gitea stopped" -ForegroundColor Green
            }
            Remove-Item -Path $GITEA_PID -Force
        } else {
            # Try to stop by name
            Get-Process -Name "gitea" -ErrorAction SilentlyContinue | Stop-Process -Force
            Write-Host "Gitea stopped" -ForegroundColor Green
        }
    }

    "status" {
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:$GITEA_PORT" -TimeoutSec 2 -ErrorAction Stop
            Write-Host "Gitea is running" -ForegroundColor Green
            Write-Host "  URL: http://localhost:$GITEA_PORT" -ForegroundColor Gray
        } catch {
            Write-Host "Gitea is not running" -ForegroundColor Yellow
        }
    }
}

