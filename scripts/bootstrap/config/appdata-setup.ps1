<#
.SYNOPSIS
    Initialize NOA AppData directory structure.

.DESCRIPTION
    Creates all necessary AppData directories within noa_root for FR-001 compliance.
    Ensures all application data stays contained within NOA.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\appdata-setup.ps1
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

Write-Host "NOA AppData Directory Initialization" -ForegroundColor Cyan
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor Gray
Write-Host ""

# Define all AppData directories (FR-001: Self-contained operation)
$directories = @(
    # Windows AppData structure
    "data/appdata/roaming"              # $env:APPDATA
    "data/appdata/local"                # $env:LOCALAPPDATA

    # Unix XDG directories
    "data"                              # XDG_DATA_HOME
    "etc"                               # XDG_CONFIG_HOME
    "data/cache"                        # XDG_CACHE_HOME
    "data/state"                        # XDG_STATE_HOME

    # Temp directories
    "tmp"                               # TEMP, TMP, TMPDIR
    "tmp/runtime"                       # XDG_RUNTIME_DIR

    # NOA-specific data directories
    "data/memory"                       # Memory store
    "data/knowledge"                    # Knowledge graphs
    "data/embeddings"                   # Vector embeddings
    "data/artifacts"                    # CAS artifact store
    "data/backups"                      # Backup storage
    "data/archives"                     # Archived data

    # Log directories
    "logs"                              # Application logs
    "logs/bootstrap"                    # Bootstrap logs
    "logs/audit"                        # Audit logs

    # Common app-specific directories (will be created on-demand, but pre-create for clarity)
    "data/appdata/roaming/Claude"       # Claude Desktop
    "data/appdata/roaming/Abacus"       # Abacus Desktop
    "data/appdata/roaming/OpenAI"       # ChatGPT Desktop
    "data/appdata/local/Claude"         # Claude cache
    "data/appdata/local/Abacus"         # Abacus cache
    "data/appdata/local/OpenAI"         # ChatGPT cache
)

$created = 0
$existed = 0

foreach ($dir in $directories) {
    $fullPath = Join-Path $NoaRoot $dir

    if (Test-Path $fullPath) {
        Write-Host "  [EXISTS] $dir" -ForegroundColor Gray
        $existed++
    } else {
        try {
            New-Item -ItemType Directory -Path $fullPath -Force | Out-Null
            Write-Host "  [CREATED] $dir" -ForegroundColor Green
            $created++
        } catch {
            Write-Host "  [ERROR] Failed to create $dir : $_" -ForegroundColor Red
        }
    }
}

Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "  Created: $created directories" -ForegroundColor Green
Write-Host "  Existed: $existed directories" -ForegroundColor Gray
Write-Host "  Total:   $($created + $existed) directories" -ForegroundColor White
Write-Host ""
Write-Host "AppData directory structure initialized!" -ForegroundColor Green
Write-Host "All application data will be contained in: $NoaRoot\data\" -ForegroundColor Yellow
