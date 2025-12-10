<#
.SYNOPSIS
    Generate SHA-256 hashes for all Phase 0 bootstrap scripts

.DESCRIPTION
    Creates a hash file for quality verification evidence.
#>

$ErrorActionPreference = "Stop"

$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
    Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
}

$OutputFile = Join-Path $NoaRoot "specs/001-noa-seed-foundation/checklists/phase0-hashes.txt"
$BootstrapDir = Join-Path $NoaRoot "scripts/bootstrap"

Write-Host "Generating SHA-256 hashes for Phase 0 scripts..." -ForegroundColor Cyan

$files = Get-ChildItem -Path $BootstrapDir -Recurse -File -Include "*.ps1","*.sh" |
    Where-Object { $_.FullName -notlike "*test*" -and $_.FullName -notlike "*verify*" }

$hashes = @()
foreach ($f in $files) {
    $hash = (Get-FileHash $f.FullName -Algorithm SHA256).Hash
    $relPath = $f.FullName.Replace("$NoaRoot\", "").Replace("\", "/")
    $hashes += "$hash  $relPath"
    Write-Host "  [OK] $relPath" -ForegroundColor Gray
}

# Ensure output directory exists
$outputDir = Split-Path $OutputFile -Parent
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

$hashes | Out-File -FilePath $OutputFile -Encoding utf8
Write-Host ""
Write-Host "Generated $($hashes.Count) hashes" -ForegroundColor Green
Write-Host "Output: $OutputFile" -ForegroundColor Green

