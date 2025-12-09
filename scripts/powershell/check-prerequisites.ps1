<#
.SYNOPSIS
    Shim wrapper to maintain backward-compatible path for prereq checks.

.DESCRIPTION
    Delegates to scripts/setup/check-prereqs.ps1 with all passed arguments.
    Supports: -Json, -PathsOnly, -AllowGlobal, -NoaRoot, -RequireTasks, -IncludeTasks

.PARAMETER Json
    Output results in JSON format

.PARAMETER PathsOnly
    Output only feature directory paths (for spec-kit integration)
    NOTE: For /tasks and /analyze commands, this is REQUIRED to get artifact paths

.PARAMETER RequireTasks
    Fail if tasks.md does not exist (for /analyze command)

.PARAMETER IncludeTasks
    Include tasks.md in output (implied by PathsOnly, kept for compatibility)

.PARAMETER AllowGlobal
    Permit detection of system-wide tools

.PARAMETER NoaRoot
    NOA root directory override
#>
[CmdletBinding()]
param(
    [switch]$Json,
    [switch]$PathsOnly,
    [switch]$AllowGlobal,
    [switch]$RequireTasks,
    [switch]$IncludeTasks,
    [string]$NoaRoot
)

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$target = Join-Path $repoRoot 'scripts/setup/check-prereqs.ps1'

if (-not (Test-Path $target)) {
    Write-Error "Missing target script: $target"
    exit 1
}

# Build parameter hashtable for splatting
$params = @{}
if ($Json) { $params['Json'] = $true }
if ($PathsOnly) { $params['PathsOnly'] = $true }
if ($AllowGlobal) { $params['AllowGlobal'] = $true }
if ($RequireTasks) { $params['RequireTasks'] = $true }
if ($IncludeTasks) { $params['IncludeTasks'] = $true }
if ($NoaRoot) { $params['NoaRoot'] = $NoaRoot }

# Forward parameters
& $target @params
exit $LASTEXITCODE
