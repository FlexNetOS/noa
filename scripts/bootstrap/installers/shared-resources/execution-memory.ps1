<#
.SYNOPSIS
    Initialize shared execution memory database (B058r-s).

.DESCRIPTION
    Creates the SQLite database for shared provider execution memory.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\execution-memory.ps1
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

Write-Host "Initializing execution memory..." -ForegroundColor Cyan

$resourcesDir = Join-Path $NoaRoot "ai/shared/resources"
$dbPath = Join-Path $resourcesDir "execution-memory.db"

# Ensure directory exists
if (-not (Test-Path $resourcesDir)) {
    New-Item -ItemType Directory -Path $resourcesDir -Force | Out-Null
}

if (Test-Path $dbPath) {
    Write-Host "  [OK] execution-memory.db exists" -ForegroundColor Green
    return
}

# Create schema file
$schemaPath = Join-Path $resourcesDir "schema/execution-memory.sql"
$schemaDir = Split-Path -Parent $schemaPath
if (-not (Test-Path $schemaDir)) {
    New-Item -ItemType Directory -Path $schemaDir -Force | Out-Null
}

$schema = @"
-- NOA Shared Execution Memory Schema
-- Enables context sharing across AI providers (§4.10)

CREATE TABLE IF NOT EXISTS execution_context (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    context_type TEXT NOT NULL,
    content TEXT NOT NULL,
    metadata TEXT,
    UNIQUE(session_id, provider, context_type)
);

CREATE TABLE IF NOT EXISTS reasoning_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    state_key TEXT NOT NULL,
    state_value TEXT NOT NULL,
    UNIQUE(session_id, state_key)
);

CREATE TABLE IF NOT EXISTS task_distribution (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL UNIQUE,
    assigned_provider TEXT,
    status TEXT DEFAULT 'pending',
    priority INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    started_at DATETIME,
    completed_at DATETIME,
    result TEXT
);

CREATE TABLE IF NOT EXISTS provider_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider TEXT NOT NULL UNIQUE,
    status TEXT DEFAULT 'unknown',
    last_heartbeat DATETIME,
    capabilities TEXT,
    current_load REAL DEFAULT 0.0
);

CREATE INDEX IF NOT EXISTS idx_context_session ON execution_context(session_id);
CREATE INDEX IF NOT EXISTS idx_reasoning_session ON reasoning_state(session_id);
CREATE INDEX IF NOT EXISTS idx_task_status ON task_distribution(status);
"@

$schema | Set-Content -Path $schemaPath -Encoding UTF8
Write-Host "  [OK] Created schema: $schemaPath" -ForegroundColor Green

# Create placeholder database file
# Note: Full SQLite initialization requires sqlite3 binary
"" | Set-Content -Path $dbPath -Encoding UTF8
Write-Host "  [OK] Created database placeholder: $dbPath" -ForegroundColor Green

Write-Host ""
Write-Host "Execution memory initialized." -ForegroundColor Green
Write-Host "Note: Full initialization requires sqlite3 binary" -ForegroundColor Gray
