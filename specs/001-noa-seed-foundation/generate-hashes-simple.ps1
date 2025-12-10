# Generate SHA-256 hashes for all key NOA files
# Simplified version

$noaRoot = $env:NOA_ROOT
if (-not $noaRoot) {
    $noaRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
}

$outputFile = Join-Path $noaRoot "test-results\HASHES.txt"
$testResultsDir = Join-Path $noaRoot "test-results"

# Create test-results directory if it doesn't exist
if (-not (Test-Path $testResultsDir)) {
    New-Item -ItemType Directory -Force -Path $testResultsDir | Out-Null
}

$hashes = @()

# Get all files in key directories, excluding build artifacts
$keyDirs = @("specs", "sys", "p2p", "init", "config", "scripts", ".github")
$excludeDirs = @(".git", "node_modules", "target", "__pycache__", ".venv", "venv", "dist", "build", ".idea", ".vscode")

foreach ($dir in $keyDirs) {
    $dirPath = Join-Path $noaRoot $dir
    if (Test-Path $dirPath) {
        $files = Get-ChildItem -Path $dirPath -Recurse -File | Where-Object {
            $shouldExclude = $false
            foreach ($excludeDir in $excludeDirs) {
                if ($_.FullName -like "*\$excludeDir\*") {
                    $shouldExclude = $true
                    break
                }
            }
            -not $shouldExclude
        }

        foreach ($file in $files) {
            try {
                $hash = Get-FileHash $file.FullName -Algorithm SHA256
                $relativePath = $file.FullName.Replace($noaRoot + '\', '').Replace('\', '/')
                $hashes += "$($hash.Hash)  $relativePath"
            } catch {
                Write-Warning "Failed to hash: $($file.FullName)"
            }
        }
    }
}

# Sort hashes
$hashes = $hashes | Sort-Object

# Write output
$hashes | Out-File -FilePath $outputFile -Encoding utf8

Write-Host "Generated HASHES.txt with $($hashes.Count) files" -ForegroundColor Green
Write-Host "Output: $outputFile" -ForegroundColor Cyan

