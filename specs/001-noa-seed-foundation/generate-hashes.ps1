# Generate SHA-256 hashes for Phase 1 key files
$noaRoot = $env:NOA_ROOT
if (-not $noaRoot) {
    $noaRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
}

$specDir = Join-Path $noaRoot "specs\001-noa-seed-foundation"
$outputFile = Join-Path $specDir "HASHES.txt"

$extensions = @('.md', '.toml', '.json', '.yaml', '.yml', '.rs', '.go', '.ts', '.tsx', '.ps1', '.sh')
$files = Get-ChildItem -Path $specDir -Recurse -File | Where-Object { $extensions -contains $_.Extension }

$hashes = @()
foreach ($file in $files) {
    $hash = Get-FileHash $file.FullName -Algorithm SHA256
    $relativePath = $file.FullName.Replace($noaRoot + '\', '').Replace('\', '/')
    $hashes += "$($hash.Hash)  $relativePath"
}

$hashes | Sort-Object | Out-File -FilePath $outputFile -Encoding utf8
Write-Host "Generated HASHES.txt with $($hashes.Count) files" -ForegroundColor Green


