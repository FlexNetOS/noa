<#
.SYNOPSIS
    Configure centralized logging for NOA.

.DESCRIPTION
    Sets up log directories and rotation for all NOA components.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\log-setup.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))
    }
}

$LOGS_DIR = Join-Path $NoaRoot "logs"

Write-Host "Configuring centralized logging..." -ForegroundColor Cyan
Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
Write-Host "Logs Dir: $LOGS_DIR" -ForegroundColor Gray
Write-Host ""

# Log subdirectories by component
$logDirs = @(
    @{ Name = "bootstrap"; Description = "Bootstrap and setup logs" },
    @{ Name = "providers"; Description = "AI provider logs" },
    @{ Name = "agents"; Description = "Agent execution logs" },
    @{ Name = "workflows"; Description = "Workflow execution logs" },
    @{ Name = "system"; Description = "System and runtime logs" },
    @{ Name = "audit"; Description = "Audit trail logs" },
    @{ Name = "errors"; Description = "Error logs" }
)

foreach ($logDir in $logDirs) {
    $logPath = Join-Path $LOGS_DIR $logDir.Name
    if (-not (Test-Path $logPath)) {
        New-Item -ItemType Directory -Path $logPath -Force | Out-Null
        Write-Host "  [CREATE] $($logDir.Name)/ ($($logDir.Description))" -ForegroundColor Green
    } else {
        Write-Host "  [EXISTS] $($logDir.Name)/" -ForegroundColor Gray
    }
}

# Create log configuration file
$logConfigPath = Join-Path $LOGS_DIR "log-config.json"
$logConfig = @{
    '$schema' = "https://noa.local/schemas/log-config.json"
    version = "1.0.0"
    log_root = $LOGS_DIR
    directories = @{}
    rotation = @{
        enabled = $true
        max_size_mb = 100
        max_files = 10
        compress = $true
    }
    retention = @{
        default_days = 30
        audit_days = 365
        error_days = 90
    }
    format = @{
        timestamp = "yyyy-MM-ddTHH:mm:ss.fffzzz"
        include_process_id = $true
        include_thread_id = $false
    }
}

foreach ($logDir in $logDirs) {
    $logConfig.directories[$logDir.Name] = @{
        path = Join-Path $LOGS_DIR $logDir.Name
        description = $logDir.Description
    }
}

$logConfig | ConvertTo-Json -Depth 4 | Set-Content -Path $logConfigPath -Encoding UTF8
Write-Host ""
Write-Host "Log configuration saved to: $logConfigPath" -ForegroundColor Green

# Create .gitignore for logs
$gitignorePath = Join-Path $LOGS_DIR ".gitignore"
if (-not (Test-Path $gitignorePath)) {
    @"
# Ignore all log files
*.log
*.log.*
*.gz

# Keep directory structure
!.gitignore
"@ | Set-Content -Path $gitignorePath -Encoding UTF8
    Write-Host "Created .gitignore for logs directory" -ForegroundColor Green
}

Write-Host ""
Write-Host "Log setup complete." -ForegroundColor Green

