<#
Shim: consolidated into setup-noa.ps1. Use that as the single entrypoint.
Examples:
  pwsh -File scripts/setup/setup-noa.ps1 -InstallPrereqs
  pwsh -File scripts/setup/setup-noa.ps1 -NoaRoot "N:\noa" -InstallPrereqs
#>

[CmdletBinding()]
param()

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$setup = Join-Path $repoRoot 'scripts/setup/setup-noa.ps1'

if (-not (Test-Path $setup)) {
    Write-Error "Missing setup entrypoint: $setup"
    exit 1
}

& $setup -InstallPrereqs
exit $LASTEXITCODE
