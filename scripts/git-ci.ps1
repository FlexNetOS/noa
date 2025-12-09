<#
.SYNOPSIS
    NOA Local Git CI/CD Tool for Windows

.DESCRIPTION
    Windows equivalent of scripts/git-ci (bash)
    Runs local CI/CD pipelines

.PARAMETER Action
    Action: run, status, logs, artifacts

.PARAMETER Pipeline
    Pipeline name (for run)

.PARAMETER NoaRoot
    NOA root directory

.EXAMPLE
    .\git-ci.ps1 -Action status
    .\git-ci.ps1 -Action run -Pipeline "build"
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("run", "status", "logs", "artifacts")]
    [string]$Action,

    [string]$Pipeline,
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$NOA_GIT = Join-Path $NoaRoot "git"
$CI_CONFIG = Join-Path $NoaRoot "config/git-local-cicd.json"
$CI_LOGS = Join-Path $NOA_GIT "ci-cd/logs"
$CI_ARTIFACTS = Join-Path $NOA_GIT "ci-cd/artifacts"

# Ensure directories
@($CI_LOGS, $CI_ARTIFACTS) | ForEach-Object {
    if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ -Force | Out-Null }
}

switch ($Action) {
    "run" {
        Write-Host "Running local CI/CD pipeline: $Pipeline" -ForegroundColor Cyan

        if (-not $Pipeline) {
            Write-Error "Usage: git-ci.ps1 -Action run -Pipeline <name>"
        }

        # Load config if exists
        if (Test-Path $CI_CONFIG) {
            $config = Get-Content $CI_CONFIG | ConvertFrom-Json
            $pipelineConfig = $config.pipelines | Where-Object { $_.name -eq $Pipeline }

            if ($pipelineConfig) {
                Write-Host "  Running steps..." -ForegroundColor Gray
                foreach ($step in $pipelineConfig.steps) {
                    Write-Host "    [$($step.name)] $($step.command)" -ForegroundColor Yellow
                    # TODO: Execute step
                }
            } else {
                Write-Host "  Pipeline '$Pipeline' not found in config" -ForegroundColor Yellow
            }
        } else {
            Write-Host "  CI config not found: $CI_CONFIG" -ForegroundColor Yellow
            Write-Host "  Create config or use standard pipelines" -ForegroundColor Gray
        }
    }

    "status" {
        Write-Host "CI/CD Pipeline Status:" -ForegroundColor Cyan

        # Check for recent runs
        $logs = Get-ChildItem -Path $CI_LOGS -Filter "*.log" -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | Select-Object -First 5
        if ($logs) {
            foreach ($log in $logs) {
                Write-Host "  $($log.Name) - $($log.LastWriteTime)" -ForegroundColor Gray
            }
        } else {
            Write-Host "  No recent CI runs" -ForegroundColor Yellow
        }
    }

    "logs" {
        Write-Host "CI/CD Logs:" -ForegroundColor Cyan

        if (Test-Path $CI_LOGS) {
            Get-ChildItem -Path $CI_LOGS | ForEach-Object {
                Write-Host "  $($_.Name)" -ForegroundColor Gray
            }
        } else {
            Write-Host "  No logs found" -ForegroundColor Yellow
        }
    }

    "artifacts" {
        Write-Host "CI/CD Artifacts:" -ForegroundColor Cyan

        if (Test-Path $CI_ARTIFACTS) {
            Get-ChildItem -Path $CI_ARTIFACTS | ForEach-Object {
                Write-Host "  $($_.Name)" -ForegroundColor Gray
            }
        } else {
            Write-Host "  No artifacts found" -ForegroundColor Yellow
        }
    }
}

