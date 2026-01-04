<#
.SYNOPSIS
    Verify all actions are logged to logs/bootstrap/ (§3.5 compliance).

.DESCRIPTION
    Checks that logging is properly configsured and bootstrap actions are auditable.

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

# Check log configsuration exists
Write-Host ""
Write-Host "Checking log configsuration..." -ForegroundColor Yellow

$logconfigsPath = Join-Path $LOGS_DIR "log-configs.json"
if (Test-Path $logconfigsPath) {
    Write-Host "  [OK] log-configs.json exists" -ForegroundColor Green
    $passed++

    # Validate configs structure
    try {
        $logconfigs = Get-Content $logconfigsPath -Raw | ConvertFrom-Json

        if ($logconfigs.rotation.enabled) {
            Write-Host "  [OK] Log rotation enabled" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "  [!!] Log rotation disabled" -ForegroundColor Yellow
        }

        if ($logconfigs.retention) {
            Write-Host "  [OK] Retention policy configsured" -ForegroundColor Green
            $passed++
        }
    } catch {
        Write-Host "  [!!] Invalid log-configs.json" -ForegroundColor Red
        $failed++
    }
} else {
    Write-Host "  [!!] log-configs.json - MISSING" -ForegroundColor Red
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
    Write-Host "✓ Logging is properly configsured - §3.5 COMPLIANT" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ Logging configsuration incomplete" -ForegroundColor Red
    Write-Host "Run: .\scripts\bootstrap\configs\log-setup.ps1" -ForegroundColor Yellow
    exit 1
}

