<#
.SYNOPSIS
    Verify all actions are logged to logs/bootstrap/ (§3.5 compliance).

.DESCRIPTION
    Checks that logging is properly configured and bootstrap actions are auditable.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\verify-logging.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         Logging Verification (§3.5)" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

$LOGS_DIR = Join-Path $NoaRoot "logs"
$BOOTSTRAP_LOGS = Join-Path $LOGS_DIR "bootstrap"

$passed = 0
$failed = 0

# Check log directories exist
Write-Host "Checking log directories..." -ForegroundColor Yellow

$requiredLogDirs = @(
    "bootstrap",
    "providers",
    "agents",
    "workflows",
    "system",
    "audit",
    "errors"
)

foreach ($dir in $requiredLogDirs) {
    $dirPath = Join-Path $LOGS_DIR $dir
    if (Test-Path $dirPath -PathType Container) {
        Write-Host "  [OK] logs/$dir/" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  [!!] logs/$dir/ - MISSING" -ForegroundColor Red
        $failed++
    }
}

# Check log configuration exists
Write-Host ""
Write-Host "Checking log configuration..." -ForegroundColor Yellow

$logConfigPath = Join-Path $LOGS_DIR "log-config.json"
if (Test-Path $logConfigPath) {
    Write-Host "  [OK] log-config.json exists" -ForegroundColor Green
    $passed++

    # Validate config structure
    try {
        $logConfig = Get-Content $logConfigPath -Raw | ConvertFrom-Json

        if ($logConfig.rotation.enabled) {
            Write-Host "  [OK] Log rotation enabled" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "  [!!] Log rotation disabled" -ForegroundColor Yellow
        }

        if ($logConfig.retention) {
            Write-Host "  [OK] Retention policy configured" -ForegroundColor Green
            $passed++
        }
    } catch {
        Write-Host "  [!!] Invalid log-config.json" -ForegroundColor Red
        $failed++
    }
} else {
    Write-Host "  [!!] log-config.json - MISSING" -ForegroundColor Red
    $failed++
}

# Check .gitignore in logs
Write-Host ""
Write-Host "Checking log privacy..." -ForegroundColor Yellow

$logsGitignore = Join-Path $LOGS_DIR ".gitignore"
if (Test-Path $logsGitignore) {
    Write-Host "  [OK] logs/.gitignore exists (logs not committed)" -ForegroundColor Green
    $passed++
} else {
    Write-Host "  [!!] logs/.gitignore missing - logs may be committed!" -ForegroundColor Yellow
}

# Check recent bootstrap logs
Write-Host ""
Write-Host "Checking recent activity..." -ForegroundColor Yellow

if (Test-Path $BOOTSTRAP_LOGS) {
    $recentLogs = Get-ChildItem -Path $BOOTSTRAP_LOGS -Filter "*.log" -ErrorAction SilentlyContinue |
        Sort-Object LastWriteTime -Descending |
        Select-Object -First 5

    if ($recentLogs) {
        Write-Host "  Recent bootstrap logs:" -ForegroundColor Gray
        foreach ($log in $recentLogs) {
            Write-Host "    - $($log.Name) ($($log.LastWriteTime))" -ForegroundColor Gray
        }
    } else {
        Write-Host "  [INFO] No bootstrap logs found yet" -ForegroundColor Gray
    }
}

# Summary
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "                     Verification Summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Passed: $passed" -ForegroundColor Green
Write-Host "  Failed: $failed" -ForegroundColor $(if ($failed -gt 0) { "Red" } else { "Gray" })
Write-Host ""

if ($failed -eq 0) {
    Write-Host "✓ Logging is properly configured - §3.5 COMPLIANT" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ Logging configuration incomplete" -ForegroundColor Red
    Write-Host "Run: .\scripts\bootstrap\config\log-setup.ps1" -ForegroundColor Yellow
    exit 1
}

