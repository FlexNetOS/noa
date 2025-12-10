# Generate SHA-256 hashes for all key NOA files
# Per Universal Task Execution Policy §9 - HASHES.txt

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

# Key file patterns to include
$includePatterns = @(
    "*.md",
    "*.toml",
    "*.json",
    "*.yaml",
    "*.yml",
    "*.rs",
    "*.go",
    "*.ts",
    "*.tsx",
    "*.ps1",
    "*.sh",
    "*.sql",
    "*.py"
)

# Directories to include
$includeDirs = @(
    "specs",
    "sys",
    "p2p",
    "init",
    "config",
    "scripts",
    ".github"
)

# Directories to exclude
$excludeDirs = @(
    ".git",
    "node_modules",
    "target",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
    ".idea",
    ".vscode"
)

$hashes = @()

# Get all files matching patterns in include directories
foreach ($dir in $includeDirs) {
    $dirPath = Join-Path $noaRoot $dir
    if (Test-Path $dirPath) {
        $files = Get-ChildItem -Path $dirPath -Recurse -File | Where-Object {
            $includePatterns -contains "*$($_.Extension)" -or
            $includePatterns -contains $_.Extension -or
            $includePatterns | ForEach-Object { $_.Replace("*", "") -eq $_.Extension }
        } | Where-Object {
            $exclude = $false
            foreach ($excludeDir in $excludeDirs) {
                if ($_.FullName -like "*\$excludeDir\*") {
                    $exclude = $true
                    break
                }
            }
            -not $exclude
        }

        foreach ($file in $files) {
            $hash = Get-FileHash $file.FullName -Algorithm SHA256
            $relativePath = $file.FullName.Replace($noaRoot + '\', '').Replace('\', '/')
            $hashes += "$($hash.Hash)  $relativePath"
        }
    }
}

# Sort hashes
$hashes = $hashes | Sort-Object

# Write output
$hashes | Out-File -FilePath $outputFile -Encoding utf8

Write-Host "Generated HASHES.txt with $($hashes.Count) files" -ForegroundColor Green
Write-Host "Output: $outputFile" -ForegroundColor Cyan

