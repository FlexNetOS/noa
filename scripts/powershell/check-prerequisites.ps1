<#
Shim wrapper to maintain backward-compatible path for prereq checks.
Delegates to scripts/setup/check-prereqs.ps1 with all passed arguments.
#>
[CmdletBinding()]
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $Args
)

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$target = Join-Path $repoRoot 'scripts/setup/check-prereqs.ps1'

if (-not (Test-Path $target)) {
    Write-Error "Missing target script: $target"
    exit 1
}

# Forward all arguments verbatim
& $target @Args
exit $LASTEXITCODE

