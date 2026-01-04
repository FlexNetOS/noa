<#
.SYNOPSIS
    NOA Uninstaller - Remove NOA installation

.DESCRIPTION
    Removes NOA installation from the system.
    Per NOA Constitution §3.1: Self-contained means clean uninstall.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from NOA_ROOT env var)

.PARAMETER Keepconfigs
    Keep configsuration files

.PARAMETER KeepLogs
    Keep log files

.PARAMETER DryRun
    Show what would be removed without actually removing

.PARAMETER Force
    Skip confirmation prompt

.EXAMPLE
    .\uninstall.ps1
    .\uninstall.ps1 -DryRun
    .\uninstall.ps1 -Keepconfigs -KeepLogs
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Keepconfigs,
    [switch]$KeepLogs,
    [switch]$DryRun,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) {
        $env:NOA_ROOT
    } else {
        Split-Path -Parent $PSScriptRoot
    }
}

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        "DryRun" { "Cyan" }
        default { "White" }
    }
    $prefix = switch ($Level) {
        "Success" { "[OK]" }
        "Warning" { "[!!]" }
        "Error" { "[XX]" }
        "DryRun" { "[DR]" }
        default { "[..]" }
    }
    Write-Host "$prefix $Message" -ForegroundColor $color
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Uninstaller" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
if ($DryRun) {
    Write-Host "MODE: DRY RUN (no changes will be made)" -ForegroundColor Cyan
}
Write-Host ""

# Verify NOA installation
if (-not (Test-Path $NoaRoot)) {
    Write-Log "NOA installation not found at: $NoaRoot" -Level Error
    exit 1
}

# Directories to remove
$DirsToRemove = @(
    "opt/rust",
    "opt/go",
    "opt/node",
    "opt/python",
    "opt/venv",
    "opt/protobuf",
    "opt/dev-tools",
    "opt/npm-cache",
    "opt/cache",
    "opt/cursor-cli",
    "opt/claude-code",
    "opt/codex",
    "cache",
    "tmp",
    "lib/shared",
    "init/run",
    "ai/shared/resources/*.db"
)

if (-not $Keepconfigs) {
    $DirsToRemove += @(
        "configs/bootstrap-state.json"
    )
}

if (-not $KeepLogs) {
    $DirsToRemove += @(
        "logs"
    )
}

# Generated files to remove
$FilesToRemove = @(
    "noa-env.ps1",
    "noa-env.sh",
    ".env.local"
)

# Confirm
if (-not $Force -and -not $DryRun) {
    Write-Host "This will remove the following from $NoaRoot :" -ForegroundColor Yellow
    foreach ($dir in $DirsToRemove) {
        Write-Host "  - $dir" -ForegroundColor Gray
    }
    foreach ($file in $FilesToRemove) {
        Write-Host "  - $file" -ForegroundColor Gray
    }
    Write-Host ""
    $confirm = Read-Host "Continue? (y/N)"
    if ($confirm -ne "y" -and $confirm -ne "Y") {
        Write-Log "Uninstall cancelled" -Level Warning
        exit 0
    }
}

# Remove directories
foreach ($dir in $DirsToRemove) {
    $fullPath = Join-Path $NoaRoot $dir

    # Handle wildcard patterns
    if ($dir -like "*\**" -or $dir -like "*/*") {
        $items = Get-ChildItem -Path (Join-Path $NoaRoot (Split-Path $dir -Parent)) -Filter (Split-Path $dir -Leaf) -ErrorAction SilentlyContinue
        foreach ($item in $items) {
            if ($DryRun) {
                Write-Log "Would remove: $($item.FullName)" -Level DryRun
            } else {
                Remove-Item $item.FullName -Recurse -Force -ErrorAction SilentlyContinue
                Write-Log "Removed: $($item.FullName)" -Level Success
            }
        }
    } elseif (Test-Path $fullPath) {
        if ($DryRun) {
            Write-Log "Would remove: $fullPath" -Level DryRun
        } else {
            Remove-Item $fullPath -Recurse -Force -ErrorAction SilentlyContinue
            Write-Log "Removed: $fullPath" -Level Success
        }
    }
}

# Remove files
foreach ($file in $FilesToRemove) {
    $fullPath = Join-Path $NoaRoot $file
    if (Test-Path $fullPath) {
        if ($DryRun) {
            Write-Log "Would remove: $fullPath" -Level DryRun
        } else {
            Remove-Item $fullPath -Force -ErrorAction SilentlyContinue
            Write-Log "Removed: $fullPath" -Level Success
        }
    }
}

# Remove symlinks from bin/
$BinDir = Join-Path $NoaRoot "bin"
if (Test-Path $BinDir) {
    $symlinks = Get-ChildItem -Path $BinDir | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }
    foreach ($link in $symlinks) {
        if ($DryRun) {
            Write-Log "Would remove symlink: $($link.FullName)" -Level DryRun
        } else {
            Remove-Item $link.FullName -Force -ErrorAction SilentlyContinue
            Write-Log "Removed symlink: $($link.Name)" -Level Success
        }
    }
}

# Clean environment variables (inform user)
Write-Host ""
Write-Log "Manual cleanup required:" -Level Warning
Write-Host "  Remove NOA entries from your PowerShell profile:" -ForegroundColor Gray
Write-Host "    - \$env:NOA_ROOT" -ForegroundColor Gray
Write-Host "    - \$env:RUSTUP_HOME, \$env:CARGO_HOME" -ForegroundColor Gray
Write-Host "    - \$env:GOROOT, \$env:GOPATH" -ForegroundColor Gray
Write-Host "    - PATH entries pointing to $NoaRoot" -ForegroundColor Gray

Write-Host ""
if ($DryRun) {
    Write-Log "DRY RUN complete - no changes were made" -Level DryRun
} else {
    Write-Log "NOA uninstall complete" -Level Success
    Write-Log "Core repository files are preserved" -Level Info
}

