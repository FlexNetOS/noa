<#
.SYNOPSIS
  Run staged quality gates (report-only by default).

.DESCRIPTION
  This script is intentionally non-blocking by default to support staged rollout.
  Use -Strict to fail on any check failure or missing tool.
#>

[CmdletBinding()]
param(
  [string]$NoaRoot,
  [switch]$Strict
)

$ErrorActionPreference = "Stop"

if (-not $NoaRoot) {
  $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { (Resolve-Path (Join-Path $PSScriptRoot "../..")).Path }
}

function Try-Run {
  param(
    [string]$Name,
    [scriptblock]$Action
  )

  try {
    & $Action
    if ($LASTEXITCODE -ne 0) {
      $msg = "$Name failed (exit $LASTEXITCODE)"
      if ($Strict) { throw $msg } else { Write-Host "WARN: $msg" -ForegroundColor Yellow }
    } else {
      Write-Host "OK: $Name" -ForegroundColor Green
    }
  } catch {
    $msg = "$Name failed: $($_.Exception.Message)"
    if ($Strict) { throw $msg } else { Write-Host "WARN: $msg" -ForegroundColor Yellow }
  }
}

Write-Host "Quality gates (staged) for: $NoaRoot" -ForegroundColor Cyan

Try-Run -Name "config-validate" -Action {
  & pwsh -NoLogo -NoProfile -File (Join-Path $NoaRoot "scripts/validate/validate-configs.ps1") -NoaRoot $NoaRoot
}

Try-Run -Name "provider-config-validate" -Action {
  & pwsh -NoLogo -NoProfile -File (Join-Path $NoaRoot "scripts/bootstrap/verify/validate-provider-configs.ps1") -NoaRoot $NoaRoot
}

# Rust fmt/clippy (only if cargo exists; report-only default)
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargo) {
  Try-Run -Name "rustfmt(sys/core)" -Action {
    Push-Location (Join-Path $NoaRoot "sys/core")
    try { cargo fmt --check } finally { Pop-Location }
  }
  Try-Run -Name "clippy(sys/core)" -Action {
    Push-Location (Join-Path $NoaRoot "sys/core")
    try { cargo clippy -- -D warnings } finally { Pop-Location }
  }
} else {
  $msg = "cargo not found; skipping rust fmt/clippy"
  if ($Strict) { throw $msg } else { Write-Host "WARN: $msg" -ForegroundColor Yellow }
}

Write-Host "Done." -ForegroundColor Cyan


