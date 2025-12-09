<#
.SYNOPSIS
    Centralized logging utilities for NOA bootstrap

.DESCRIPTION
    Provides consistent logging functions for all bootstrap scripts.
    Per NOA Constitution §3.1: Self-contained logging to noa_root/logs/
#>

# Logging levels
$script:LogLevels = @{
    "Debug"   = 0
    "Info"    = 1
    "Success" = 2
    "Warning" = 3
    "Error"   = 4
}

# Current log level (can be set via $env:NOA_LOG_LEVEL)
$_logLevel = if ($env:NOA_LOG_LEVEL) { $env:NOA_LOG_LEVEL } else { "Info" }
$script:CurrentLogLevel = $LogLevels[[string]$_logLevel]

# Log file path
$script:LogFile = $null

function Initialize-Logging {
    <#
    .SYNOPSIS
        Initialize logging with optional file output
    #>
    param(
        [string]$NoaRoot,
        [string]$LogName = "bootstrap"
    )

    if (-not $NoaRoot) {
        if ($env:NOA_ROOT) { $NoaRoot = $env:NOA_ROOT }
        else { $NoaRoot = Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)) }
    }

    $logDir = Join-Path $NoaRoot "logs"
    if (-not (Test-Path $logDir)) {
        New-Item -ItemType Directory -Path $logDir -Force | Out-Null
    }

    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $script:LogFile = Join-Path $logDir "$LogName-$timestamp.log"

    Write-LogInternal "Info" "Logging initialized: $($script:LogFile)"
}

function Write-LogInternal {
    param(
        [string]$Level,
        [string]$Message
    )

    $levelNum = $script:LogLevels[$Level]
    if ($levelNum -lt $script:CurrentLogLevel) { return }

    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logLine = "[$timestamp] [$Level] $Message"

    # Write to file if initialized
    if ($script:LogFile) {
        Add-Content -Path $script:LogFile -Value $logLine -ErrorAction SilentlyContinue
    }

    # Write to console with colors
    $color = switch ($Level) {
        "Debug"   { "Gray" }
        "Info"    { "White" }
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error"   { "Red" }
        default   { "White" }
    }

    $prefix = switch ($Level) {
        "Debug"   { "[D]" }
        "Info"    { "[i]" }
        "Success" { "[OK]" }
        "Warning" { "[!!]" }
        "Error"   { "[XX]" }
        default   { "[ ]" }
    }

    Write-Host "$prefix $Message" -ForegroundColor $color
}

function Write-Log {
    <#
    .SYNOPSIS
        Write a log message with specified level

    .PARAMETER Message
        The message to log

    .PARAMETER Level
        Log level: Debug, Info, Success, Warning, Error
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Message,

        [ValidateSet("Debug", "Info", "Success", "Warning", "Error")]
        [string]$Level = "Info"
    )

    Write-LogInternal -Level $Level -Message $Message
}

function Write-LogDebug   { param([string]$Message) Write-LogInternal "Debug" $Message }
function Write-LogInfo    { param([string]$Message) Write-LogInternal "Info" $Message }
function Write-LogSuccess { param([string]$Message) Write-LogInternal "Success" $Message }
function Write-LogWarning { param([string]$Message) Write-LogInternal "Warning" $Message }
function Write-LogError   { param([string]$Message) Write-LogInternal "Error" $Message }

function Write-LogSection {
    <#
    .SYNOPSIS
        Write a section header to the log
    #>
    param([string]$Title)

    Write-Host ""
    Write-Host ("=" * 60) -ForegroundColor Cyan
    Write-Host " $Title" -ForegroundColor Cyan
    Write-Host ("=" * 60) -ForegroundColor Cyan

    if ($script:LogFile) {
        Add-Content -Path $script:LogFile -Value "`n$("=" * 60)`n $Title`n$("=" * 60)" -ErrorAction SilentlyContinue
    }
}

function Write-LogStep {
    <#
    .SYNOPSIS
        Write a numbered step to the log
    #>
    param(
        [int]$Step,
        [int]$Total,
        [string]$Description
    )

    Write-Log "Step $Step/$Total`: $Description" -Level Info
}

# Functions exported when dot-sourced:
# Initialize-Logging, Write-Log, Write-LogDebug, Write-LogInfo,
# Write-LogSuccess, Write-LogWarning, Write-LogError, Write-LogSection, Write-LogStep

