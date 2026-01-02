# Rust Lovable - Log Monitor
# Real-time monitoring of application logs

param(
    [string]$LogDir = "",
    [switch]$Tail,
    [int]$Lines = 50,
    [switch]$Errors,
    [switch]$Watch,
    [switch]$Stats,
    [switch]$Help
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

if (-not $LogDir) { $LogDir = Join-Path $ProjectRoot "logs" }

if ($Help) {
    Write-Host @"
Rust Lovable - Log Monitor

Usage: .\monitor-logs.ps1 [OPTIONS]

Options:
  -LogDir DIR   Log directory (default: logs/)
  -Tail         Show last N lines of log
  -Lines N      Number of lines to show (default: 50)
  -Errors       Show only errors
  -Watch        Continuously watch logs (like tail -f)
  -Stats        Show log statistics
  -Help         Show this help message

Examples:
  .\monitor-logs.ps1 -Tail
  .\monitor-logs.ps1 -Watch
  .\monitor-logs.ps1 -Errors -Lines 100
  .\monitor-logs.ps1 -Stats
"@
    exit 0
}

function Write-Header {
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  Rust Lovable - Log Monitor" -ForegroundColor Cyan
    Write-Host "  $(Get-Date)" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
}

function Get-LogStats {
    param($LogFile)
    
    if (-not (Test-Path $LogFile)) {
        Write-Host "Log file not found: $LogFile" -ForegroundColor Yellow
        return
    }
    
    $Content = Get-Content $LogFile
    $TotalLines = $Content.Count
    $InfoCount = ($Content | Select-String -Pattern "INFO" -SimpleMatch).Count
    $WarnCount = ($Content | Select-String -Pattern "WARN" -SimpleMatch).Count
    $ErrorCount = ($Content | Select-String -Pattern "ERROR" -SimpleMatch).Count
    $DebugCount = ($Content | Select-String -Pattern "DEBUG" -SimpleMatch).Count
    
    $FileInfo = Get-Item $LogFile
    $FileSizeMB = [math]::Round($FileInfo.Length / 1MB, 2)
    
    Write-Host ""
    Write-Host "Log Statistics" -ForegroundColor Cyan
    Write-Host "─────────────────────────────────────────" -ForegroundColor Gray
    Write-Host "File: $($FileInfo.Name)"
    Write-Host "Size: $FileSizeMB MB"
    Write-Host "Total Lines: $TotalLines"
    Write-Host ""
    Write-Host "Log Levels:" -ForegroundColor Cyan
    Write-Host "  INFO:  $InfoCount" -ForegroundColor Blue
    Write-Host "  WARN:  $WarnCount" -ForegroundColor Yellow
    Write-Host "  ERROR: $ErrorCount" -ForegroundColor Red
    Write-Host "  DEBUG: $DebugCount" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Last Modified: $($FileInfo.LastWriteTime)"
}

function Show-ColoredLog {
    param($Line)
    
    if ($Line -match "ERROR") {
        Write-Host $Line -ForegroundColor Red
    } elseif ($Line -match "WARN") {
        Write-Host $Line -ForegroundColor Yellow
    } elseif ($Line -match "INFO") {
        Write-Host $Line -ForegroundColor Green
    } elseif ($Line -match "DEBUG") {
        Write-Host $Line -ForegroundColor Gray
    } else {
        Write-Host $Line
    }
}

function Watch-Logs {
    param($LogFile)
    
    Write-Host "Watching logs (Ctrl+C to stop)..." -ForegroundColor Cyan
    Write-Host "─────────────────────────────────────────" -ForegroundColor Gray
    
    Get-Content $LogFile -Wait | ForEach-Object {
        Show-ColoredLog $_
    }
}

function Show-TailLogs {
    param($LogFile, $LineCount, $ErrorsOnly)
    
    if (-not (Test-Path $LogFile)) {
        Write-Host "Log file not found: $LogFile" -ForegroundColor Yellow
        return
    }
    
    $Content = Get-Content $LogFile -Tail $LineCount
    
    if ($ErrorsOnly) {
        $Content = $Content | Select-String -Pattern "ERROR|WARN" -SimpleMatch
        Write-Host "Showing errors and warnings only:" -ForegroundColor Yellow
    } else {
        Write-Host "Last $LineCount lines:" -ForegroundColor Cyan
    }
    
    Write-Host "─────────────────────────────────────────" -ForegroundColor Gray
    
    foreach ($Line in $Content) {
        Show-ColoredLog $Line.ToString()
    }
}

function Get-ProcessStatus {
    $Process = Get-Process -Name "rust-lovable" -ErrorAction SilentlyContinue
    
    Write-Host ""
    Write-Host "Process Status" -ForegroundColor Cyan
    Write-Host "─────────────────────────────────────────" -ForegroundColor Gray
    
    if ($Process) {
        Write-Host "Status: Running" -ForegroundColor Green
        Write-Host "PID: $($Process.Id)"
        Write-Host "Memory: $([math]::Round($Process.WorkingSet64 / 1MB, 2)) MB"
        Write-Host "CPU Time: $($Process.TotalProcessorTime)"
        Write-Host "Start Time: $($Process.StartTime)"
        Write-Host "Uptime: $((Get-Date) - $Process.StartTime)"
    } else {
        Write-Host "Status: Not Running" -ForegroundColor Red
    }
}

# Main
Write-Header

$MainLog = Join-Path $LogDir "rust-lovable.log"
$ErrorLog = Join-Path $LogDir "rust-lovable-error.log"

# Show process status
Get-ProcessStatus

if ($Stats) {
    Get-LogStats -LogFile $MainLog
    if (Test-Path $ErrorLog) {
        Write-Host ""
        Write-Host "Error Log:" -ForegroundColor Red
        Get-LogStats -LogFile $ErrorLog
    }
} elseif ($Watch) {
    Watch-Logs -LogFile $MainLog
} elseif ($Tail -or $true) {
    Show-TailLogs -LogFile $MainLog -LineCount $Lines -ErrorsOnly:$Errors
    
    if ($Errors -and (Test-Path $ErrorLog)) {
        Write-Host ""
        Write-Host "Error Log:" -ForegroundColor Red
        Write-Host "─────────────────────────────────────────" -ForegroundColor Gray
        Get-Content $ErrorLog | ForEach-Object { Write-Host $_ -ForegroundColor Red }
    }
}

Write-Host ""
Write-Host "─────────────────────────────────────────" -ForegroundColor Gray
Write-Host "Log directory: $LogDir" -ForegroundColor Gray
