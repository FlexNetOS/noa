<#
.SYNOPSIS
  Bootstrap micromamba + a conda-forge env for NOA (Windows)

.DESCRIPTION
  Creates a self-contained conda runtime under N:\noa\opt\conda.
  This is intended for notebook use (Jupyter) and cross-platform parity.

  Directory layout:
    $env:NOA_OPT\conda\micromamba.exe
    $env:NOA_OPT\conda\envs\noa\

.NOTES
  This script does not download micromamba automatically.
  Place micromamba.exe at $env:NOA_OPT\conda\micromamba.exe, then run.

.EXAMPLE
  . N:\noa\noa-env.ps1
  .\scripts\conda\bootstrap-micromamba.ps1
#>

[CmdletBinding()]
param(
  [string]$EnvName = "noa",
  [string]$PythonVersion = "3.12",
  [string[]]$Packages = @("jupyterlab", "ipykernel")
)

$ErrorActionPreference = "Stop"

if (-not $env:NOA_ROOT) {
  Write-Error "NOA_ROOT is not set. Run: . N:\\noa\\noa-env.ps1"
}

$condaRoot = Join-Path $env:NOA_OPT "conda"
$micromamba = Join-Path $condaRoot "micromamba.exe"
$envPath = Join-Path $condaRoot ("envs\\" + $EnvName)

New-Item -ItemType Directory -Path $condaRoot -Force | Out-Null

if (-not (Test-Path $micromamba)) {
  Write-Host "micromamba.exe not found at: $micromamba" -ForegroundColor Yellow
  Write-Host "Place micromamba.exe there, then re-run this script." -ForegroundColor Gray
  exit 1
}

$pkgArgs = @()
foreach ($p in $Packages) { $pkgArgs += $p }

& $micromamba create -y -p $envPath -c conda-forge ("python=" + $PythonVersion) @pkgArgs

Write-Host "[OK] conda-forge env ready: $envPath" -ForegroundColor Green
Write-Host "To activate:" -ForegroundColor Cyan
Write-Host "  . N:\\noa\\noa-env.ps1" -ForegroundColor Gray
Write-Host "  noa-conda-activate" -ForegroundColor Gray
