<#
.SYNOPSIS
  Single-command repo verification (config + security + quality).

.DESCRIPTION
  Wrapper intended for both humans and CI.
  Defaults to staged/report-only behavior in the underlying scripts unless -Strict is passed.
#>

[CmdletBinding()]
param(
  [string]$NoaRoot,
  [switch]$Strict
)

$ErrorActionPreference = "Stop"

if (-not $NoaRoot) {
  $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { (Resolve-Path (Join-Path $PSScriptRoot "..")).Path }
}

$validateConfigs = Join-Path $NoaRoot "scripts/validate/validate-configs.ps1"
$securityGates = Join-Path $NoaRoot "scripts/validate/security-gates.ps1"
$qualityGates = Join-Path $NoaRoot "scripts/validate/quality-gates.ps1"

Write-Host "check-all for: $NoaRoot" -ForegroundColor Cyan

if ($Strict) {
  & pwsh -NoLogo -NoProfile -File $validateConfigs -NoaRoot $NoaRoot -Strict
  & pwsh -NoLogo -NoProfile -File $securityGates -NoaRoot $NoaRoot -Strict
  & pwsh -NoLogo -NoProfile -File $qualityGates -NoaRoot $NoaRoot -Strict
} else {
  & pwsh -NoLogo -NoProfile -File $validateConfigs -NoaRoot $NoaRoot
  & pwsh -NoLogo -NoProfile -File $securityGates -NoaRoot $NoaRoot
  & pwsh -NoLogo -NoProfile -File $qualityGates -NoaRoot $NoaRoot
}

Write-Host "Done." -ForegroundColor Cyan


