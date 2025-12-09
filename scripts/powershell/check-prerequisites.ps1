<#
.SYNOPSIS
    Shim wrapper to maintain backward-compatible path for prereq checks.

.DESCRIPTION
    Delegates to scripts/setup/check-prereqs.ps1 with all passed arguments.
    Supports: -Json, -PathsOnly, -AllowGlobal, -NoaRoot
#>
[CmdletBinding()]
param(
    [switch]$Json,
    [switch]$PathsOnly,
    [switch]$AllowGlobal,
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
if ($NoaRoot) { $params['NoaRoot'] = $NoaRoot }

# Forward parameters
& $target @params
exit $LASTEXITCODE
