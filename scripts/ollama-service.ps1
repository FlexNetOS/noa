<#
.SYNOPSIS
    NOA Ollama Service Manager for Windows

.DESCRIPTION
    Manages Ollama AI backend contained within NOA root.
    Windows equivalent of scripts/ollama-service (bash)

.PARAMETER Action
    Action to perform: start, stop, status

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\ollama-service.ps1 -Action start
    .\ollama-service.ps1 -Action status
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

$OLLAMA_BIN = Join-Path $NoaRoot "bin/ollama.exe"
$OLLAMA_MODELS = Join-Path $NoaRoot "ai/shared/models/ollama"
$OLLAMA_PID = Join-Path $NoaRoot "init/run/ollama.pid"
$OLLAMA_HOST = "127.0.0.1:11434"

switch ($Action) {
    "start" {
        Write-Host "Starting Ollama (contained in $NoaRoot)..." -ForegroundColor Cyan

        # Create directories
        @($OLLAMA_MODELS, (Join-Path $NoaRoot "init/run")) | ForEach-Object {
            if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ -Force | Out-Null }
        }

        # Set environment
        $env:OLLAMA_MODELS = $OLLAMA_MODELS
        $env:OLLAMA_HOST = $OLLAMA_HOST

        # Check if already running
        if (Test-Path $OLLAMA_PID) {
            $pid = Get-Content $OLLAMA_PID
            if (Get-Process -Id $pid -ErrorAction SilentlyContinue) {
                Write-Host "Ollama is already running (PID $pid)" -ForegroundColor Yellow
                exit 0
            }
        }

        # Start Ollama
        if (Test-Path $OLLAMA_BIN) {
            $process = Start-Process -FilePath $OLLAMA_BIN -ArgumentList "serve" -PassThru -WindowStyle Hidden
            $process.Id | Set-Content -Path $OLLAMA_PID
            Write-Host "Ollama started (PID $($process.Id))" -ForegroundColor Green
            Write-Host "  OLLAMA_MODELS: $OLLAMA_MODELS" -ForegroundColor Gray
            Write-Host "  OLLAMA_HOST: $OLLAMA_HOST" -ForegroundColor Gray
        } else {
            # Try system ollama
            $systemOllama = Get-Command "ollama" -ErrorAction SilentlyContinue
            if ($systemOllama) {
                $process = Start-Process -FilePath "ollama" -ArgumentList "serve" -PassThru -WindowStyle Hidden
                $process.Id | Set-Content -Path $OLLAMA_PID
                Write-Host "Ollama (system) started (PID $($process.Id))" -ForegroundColor Green
            } else {
                Write-Error "Ollama not found. Install to $OLLAMA_BIN or system PATH."
            }
        }
    }

    "stop" {
        Write-Host "Stopping Ollama..." -ForegroundColor Cyan

        if (Test-Path $OLLAMA_PID) {
            $pid = Get-Content $OLLAMA_PID
            $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
            if ($process) {
                Stop-Process -Id $pid -Force
                Write-Host "Ollama stopped" -ForegroundColor Green
            }
            Remove-Item -Path $OLLAMA_PID -Force
        } else {
            # Try to stop by name
            Get-Process -Name "ollama" -ErrorAction SilentlyContinue | Stop-Process -Force
            Write-Host "Ollama stopped" -ForegroundColor Green
        }
    }

    "status" {
        try {
            $response = Invoke-WebRequest -Uri "http://$OLLAMA_HOST/api/tags" -TimeoutSec 2 -ErrorAction Stop
            Write-Host "Ollama is running" -ForegroundColor Green

            $tags = $response.Content | ConvertFrom-Json
            if ($tags.models) {
                Write-Host "  Available models:" -ForegroundColor Gray
                foreach ($model in $tags.models) {
                    Write-Host "    - $($model.name)" -ForegroundColor Cyan
                }
            }
        } catch {
            Write-Host "Ollama is not running" -ForegroundColor Yellow
        }
    }
}

