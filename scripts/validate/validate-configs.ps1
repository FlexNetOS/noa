<#
.SYNOPSIS
  Validate committed NOA config files (parse + basic policy + schema when available).

.DESCRIPTION
  - Parses JSON/YAML configs under config/
  - Requires version field
  - If $schema is present and a matching local schema exists under config/schemas/, validates via python jsonschema
  - Also validates AI provider configs via scripts/bootstrap/verify/validate-provider-configs.ps1

.PARAMETER NoaRoot
  NOA root directory (defaults to env:NOA_ROOT or repo root).

.PARAMETER Strict
  Fail when a config declares $schema but the local schema cannot be resolved.
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

$configDir = Join-Path $NoaRoot "config"
$schemasDir = Join-Path $configDir "schemas"
$pythonValidator = Join-Path $NoaRoot "scripts/validate/validate_jsonschema.py"
$providerValidator = Join-Path $NoaRoot "scripts/bootstrap/verify/validate-provider-configs.ps1"

if (-not (Test-Path $configDir)) { throw "config/ not found at: $configDir" }
if (-not (Test-Path $schemasDir)) { throw "config/schemas/ not found at: $schemasDir" }
if (-not (Test-Path $pythonValidator)) { throw "Missing validator: $pythonValidator" }

$errors = @()
$warnings = @()

function Resolve-LocalSchemaPath {
  param([string]$schemaUri)

  if (-not $schemaUri) { return $null }
  # Accept URL form: https://noa.local/schemas/<name>.(json|yaml|yml)
  $file = [System.IO.Path]::GetFileName($schemaUri)
  if (-not $file) { return $null }
  $candidate = Join-Path $schemasDir $file
  if (Test-Path $candidate) { return $candidate }
  return $null
}

function Load-JsonFile {
  param([string]$path)
  return (Get-Content -Path $path -Raw -Encoding UTF8 | ConvertFrom-Json)
}

function Load-YamlFile {
  param([string]$path)
  # Prefer python for YAML parsing to avoid module dependency on Windows.
  $py = @"
import sys, yaml
from pathlib import Path
p = Path(sys.argv[1])
obj = yaml.safe_load(p.read_text(encoding='utf-8'))
print('OK')
"@
  & python -c $py $path | Out-Null
}

# Collect committed config files
$files = Get-ChildItem -Path $configDir -File -Recurse |
  Where-Object {
    $_.FullName -notlike "$schemasDir*" -and
    $_.Name -notin @("README.md","requirements.txt") -and
    $_.Extension -in @(".json",".yaml",".yml")
  }

foreach ($f in $files) {
  try {
    if ($f.Extension -eq ".json") {
      $obj = Load-JsonFile -path $f.FullName
      if (-not $obj.version) { $errors += "CFG-001 missing version: $($f.FullName)" }

      $schemaUri = $obj.'$schema'
      if ($schemaUri) {
        $schemaPath = Resolve-LocalSchemaPath -schemaUri $schemaUri
        if (-not $schemaPath) {
          $msg = "CFG-002 $($f.Name) declares `$schema ($schemaUri) but no local schema found under config/schemas/"
          if ($Strict) { $errors += $msg } else { $warnings += $msg }
        } else {
          & python $pythonValidator $f.FullName $schemaPath | Out-Null
          if ($LASTEXITCODE -ne 0) { $errors += "Schema validation failed: $($f.FullName) vs $schemaPath" }
        }
      }
    } else {
      # YAML parseability check (schema mapping is opt-in; many YAML configs don't declare $schema)
      Load-YamlFile -path $f.FullName
    }
  } catch {
    $errors += "Parse/validation error for $($f.FullName): $($_.Exception.Message)"
  }
}

# Provider config validation (PowerShell version is the source-of-truth; bash variant had historical drift)
if (Test-Path $providerValidator) {
  try {
    & pwsh -NoLogo -NoProfile -File $providerValidator -NoaRoot $NoaRoot | Out-Null
    if ($LASTEXITCODE -ne 0) { $errors += "Provider config validation failed (exit $LASTEXITCODE)" }
  } catch {
    $errors += "Provider config validator failed to run: $($_.Exception.Message)"
  }
} else {
  $warnings += "Provider config validator not found: $providerValidator"
}

Write-Host "Config validation completed for: $NoaRoot" -ForegroundColor Cyan
if ($warnings.Count -gt 0) {
  Write-Host "Warnings:" -ForegroundColor Yellow
  $warnings | ForEach-Object { Write-Host "  - $_" -ForegroundColor Yellow }
}
if ($errors.Count -gt 0) {
  Write-Host "Errors:" -ForegroundColor Red
  $errors | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
  exit 1
}

Write-Host "OK: configs validated" -ForegroundColor Green
exit 0


