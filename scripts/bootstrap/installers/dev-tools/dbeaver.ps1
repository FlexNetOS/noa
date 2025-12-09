<#
.SYNOPSIS
    Detect DBeaver database tool for NOA integration.

.DESCRIPTION
    Detects DBeaver installation and creates connection templates.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\dbeaver.ps1
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

Write-Host "Checking DBeaver installation..." -ForegroundColor Cyan

# Common DBeaver installation paths
$dbeaverPaths = @(
    "C:\Program Files\DBeaver\dbeaver.exe"
    "$env:LOCALAPPDATA\DBeaver\dbeaver.exe"
    "$env:LOCALAPPDATA\Programs\DBeaver\dbeaver.exe"
    "C:\Program Files\DBeaverCommunity\dbeaver.exe"
)

$dbeaverExe = $null
foreach ($path in $dbeaverPaths) {
    if (Test-Path $path) {
        $dbeaverExe = $path
        break
    }
}

if ($dbeaverExe) {
    Write-Host "  [OK] DBeaver found: $dbeaverExe" -ForegroundColor Green

    # Create NOA database connections template
    $workspaceDir = Join-Path $NoaRoot "workspace/tools/dbeaver"
    if (-not (Test-Path $workspaceDir)) {
        New-Item -ItemType Directory -Path $workspaceDir -Force | Out-Null
        Write-Host "  [OK] Created DBeaver workspace directory: $workspaceDir" -ForegroundColor Green
    }

    # Create connection template documentation
    $connectionDoc = Join-Path $workspaceDir "connections.md"
    if (-not (Test-Path $connectionDoc)) {
        $docContent = @"
# NOA Database Connections for DBeaver

## SQLite Databases

### Execution Memory Database
- **Driver**: SQLite
- **Database Path**: `$NoaRoot/ai/shared/resources/execution-memory.db`
- **Description**: Shared provider execution memory

### Metrics Database (when observability enabled)
- **Driver**: SQLite
- **Database Path**: `$NoaRoot/data/metrics.db`
- **Description**: Local metrics storage

## PostgreSQL (Production)

### Main Database
- **Driver**: PostgreSQL
- **Host**: localhost
- **Port**: 5432
- **Database**: noa
- **Username**: noa_user
- **Password**: (from environment)

## Qdrant Vector Database

- **Host**: localhost
- **Port**: 6333
- **REST API**: http://localhost:6333

## Import Instructions

1. Open DBeaver
2. File → Import → DBeaver Project
3. Select this directory
4. Configure credentials from environment
"@

        $docContent | Set-Content -Path $connectionDoc -Encoding UTF8
        Write-Host "  [OK] Created connection documentation: $connectionDoc" -ForegroundColor Green
    }
} else {
    Write-Host "  [SKIP] DBeaver not found" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  To install DBeaver:" -ForegroundColor Gray
    Write-Host "    1. Visit https://dbeaver.io/download/" -ForegroundColor Gray
    Write-Host "    2. Download DBeaver Community Edition" -ForegroundColor Gray
    Write-Host "    3. Run the installer" -ForegroundColor Gray
}

Write-Host ""
Write-Host "DBeaver check complete." -ForegroundColor Green

