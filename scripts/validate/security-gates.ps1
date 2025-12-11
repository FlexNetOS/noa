<#
.SYNOPSIS
  Run staged security gates (report-only by default).

.DESCRIPTION
  Intended as a local companion to CI workflows.
  Use -Strict to fail the process on any failing gate.
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

function Fail-OrWarn([string]$msg) {
  if ($Strict) { throw $msg } else { Write-Host "WARN: $msg" -ForegroundColor Yellow }
}

function Try-Run {
  param([string]$Name, [scriptblock]$Action)
  try {
    & $Action
    if ($LASTEXITCODE -ne 0) {
      Fail-OrWarn "$Name failed (exit $LASTEXITCODE)"
    } else {
      Write-Host "OK: $Name" -ForegroundColor Green
    }
  } catch {
    Fail-OrWarn "$Name failed: $($_.Exception.Message)"
  }
}

Write-Host "Security gates (staged) for: $NoaRoot" -ForegroundColor Cyan

if (Get-Command gitleaks -ErrorAction SilentlyContinue) {
  Try-Run -Name "gitleaks" -Action {
    Push-Location $NoaRoot
    try { gitleaks detect --source . --redact --no-banner } finally { Pop-Location }
  }
} else {
  Fail-OrWarn "gitleaks not found; install via scripts/setup/install-all-tools.ps1"
}

if (Get-Command trivy -ErrorAction SilentlyContinue) {
  Try-Run -Name "trivy(fs)" -Action {
    Push-Location $NoaRoot
    try { trivy fs --severity HIGH,CRITICAL --ignore-unfixed --exit-code 1 . } finally { Pop-Location }
  }
} else {
  Fail-OrWarn "trivy not found; install via scripts/setup/install-all-tools.ps1"
}

if (Get-Command grype -ErrorAction SilentlyContinue) {
  Try-Run -Name "grype(dir)" -Action {
    Push-Location $NoaRoot
    try { grype dir:. --fail-on high } finally { Pop-Location }
  }
} else {
  Fail-OrWarn "grype not found; install via scripts/setup/install-all-tools.ps1"
}

if (Get-Command semgrep -ErrorAction SilentlyContinue) {
  Try-Run -Name "semgrep(p/default)" -Action {
    Push-Location $NoaRoot
    try { semgrep --config p/default } finally { Pop-Location }
  }
} else {
  Fail-OrWarn "semgrep not found; install via scripts/setup/install-all-tools.ps1"
}

Write-Host "Done." -ForegroundColor Cyan


