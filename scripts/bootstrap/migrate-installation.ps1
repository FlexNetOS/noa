<#
.SYNOPSIS
    Migrate existing NOA installation to new bootstrap structure.

.DESCRIPTION
    Updates an existing NOA installation to use the new unified bootstrap system.
    Preserves existing data while updating scripts and configuration.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER DryRun
    Show what would be done without making changes

.PARAMETER BackupFirst
    Create a backup before migrating

.EXAMPLE
    .\migrate-installation.ps1 -DryRun
    .\migrate-installation.ps1 -BackupFirst
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$DryRun,
    [switch]$BackupFirst
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    }
}

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         NOA Installation Migration" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray

if ($DryRun) {
    Write-Host "[DRY RUN MODE - No changes will be made]" -ForegroundColor Yellow
}
Write-Host ""

# Check if this is an existing installation
$markerFile = Join-Path $NoaRoot ".noa"
$isExisting = Test-Path $markerFile

if (-not $isExisting) {
    Write-Host "No existing NOA installation found at $NoaRoot" -ForegroundColor Yellow
    Write-Host "Run bootstrap.ps1 for fresh installation" -ForegroundColor Gray
    exit 0
}

# Read current marker
$currentMarker = Get-Content $markerFile -Raw
Write-Host "Current installation:" -ForegroundColor Cyan
Write-Host $currentMarker -ForegroundColor Gray
Write-Host ""

# Backup if requested
if ($BackupFirst -and -not $DryRun) {
    $backupDir = Join-Path $NoaRoot "backups/migration-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
    Write-Host "Creating backup: $backupDir" -ForegroundColor Yellow

    New-Item -ItemType Directory -Path $backupDir -Force | Out-Null

    # Backup key files
    $filesToBackup = @(
        ".noa",
        "noa-env.ps1",
        "config/ai-providers.json",
        "config/bootstrap-state.json"
    )

    foreach ($file in $filesToBackup) {
        $srcPath = Join-Path $NoaRoot $file
        if (Test-Path $srcPath) {
            $dstPath = Join-Path $backupDir $file
            $dstDir = Split-Path -Parent $dstPath
            if (-not (Test-Path $dstDir)) {
                New-Item -ItemType Directory -Path $dstDir -Force | Out-Null
            }
            Copy-Item -Path $srcPath -Destination $dstPath -Force
            Write-Host "  Backed up: $file" -ForegroundColor Green
        }
    }
    Write-Host ""
}

# Migration tasks
Write-Host "Migration Tasks:" -ForegroundColor Cyan
Write-Host ""

# 1. Create new directories
$newDirs = @(
    "ai/shared/agents",
    "ai/shared/workflows",
    "ai/shared/prompts",
    "ai/shared/tools",
    "ai/shared/skills",
    "ai/shared/models",
    "ai/shared/commands",
    "ai/shared/resources/schema",
    "cache/rust",
    "cache/go",
    "cache/npm",
    "cache/pip",
    "cache/models",
    "cache/ollama",
    "cache/huggingface",
    "cache/downloads",
    "logs/bootstrap",
    "logs/providers",
    "logs/agents",
    "logs/workflows",
    "logs/system",
    "logs/audit",
    "logs/errors"
)

Write-Host "1. Creating new directories..." -ForegroundColor Yellow
foreach ($dir in $newDirs) {
    $dirPath = Join-Path $NoaRoot $dir
    if (-not (Test-Path $dirPath)) {
        if ($DryRun) {
            Write-Host "  [DRY] Would create: $dir" -ForegroundColor Gray
        } else {
            New-Item -ItemType Directory -Path $dirPath -Force | Out-Null
            Write-Host "  [OK] Created: $dir" -ForegroundColor Green
        }
    }
}

# 2. Update configuration files
Write-Host ""
Write-Host "2. Updating configuration files..." -ForegroundColor Yellow

$configUpdates = @{
    "config/shared-resources.json" = $true
    "ai/shared/resources/resource-registry.json" = $true
}

foreach ($config in $configUpdates.Keys) {
    $configPath = Join-Path $NoaRoot $config
    if (-not (Test-Path $configPath)) {
        if ($DryRun) {
            Write-Host "  [DRY] Would create: $config" -ForegroundColor Gray
        } else {
            Write-Host "  [INFO] Missing: $config (run bootstrap to create)" -ForegroundColor Yellow
        }
    } else {
        Write-Host "  [OK] Exists: $config" -ForegroundColor Green
    }
}

# 3. Update marker file
Write-Host ""
Write-Host "3. Updating marker file..." -ForegroundColor Yellow

$newMarker = @"
# NOA Root Directory Marker
# Migrated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
version=2.0.0
platform=windows
root=$NoaRoot
migrated=true
"@

if ($DryRun) {
    Write-Host "  [DRY] Would update .noa marker" -ForegroundColor Gray
} else {
    $newMarker | Set-Content -Path $markerFile -Encoding UTF8
    Write-Host "  [OK] Updated .noa marker" -ForegroundColor Green
}

# Summary
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "                     Migration Summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

if ($DryRun) {
    Write-Host "DRY RUN COMPLETE - No changes were made" -ForegroundColor Yellow
    Write-Host "Run without -DryRun to apply changes" -ForegroundColor Gray
} else {
    Write-Host "Migration complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "  1. Run: .\scripts\bootstrap\bootstrap.ps1 -InstallSharedResources" -ForegroundColor Cyan
    Write-Host "  2. Verify: .\scripts\bootstrap\verify\verify-all.ps1" -ForegroundColor Cyan
}

