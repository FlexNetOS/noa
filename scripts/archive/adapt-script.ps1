# Script to adapt setup-agentic-os.ps1 for the noa repo
# This reads the source script and adapts it for n:\noa

param(
    [string]$SourceFile = '',
    [string]$TargetFile = ''
)

$ErrorActionPreference = 'Stop'

# Use parameter or default to looking for the source file in common locations
if ([string]::IsNullOrWhiteSpace($SourceFile)) {
    # Try to find the source file in the current directory or parent directories
    $possiblePaths = @(
        Join-Path $PSScriptRoot 'setup-agentic-os.ps1',
        Join-Path (Split-Path $PSScriptRoot -Parent) 'setup-agentic-os.ps1',
        'E:\Backups\WSL\setup-agentic-os.ps1'  # Fallback to original location
    )

    foreach ($path in $possiblePaths) {
        if (Test-Path $path) {
            $SourceFile = $path
            Write-Host "Found source file at: $SourceFile" -ForegroundColor Green
            break
        }
    }

    if ([string]::IsNullOrWhiteSpace($SourceFile)) {
        Write-Error "Source file not found. Please specify -SourceFile parameter or place setup-agentic-os.ps1 in the script directory."
        exit 1
    }
}

# Use parameter or default to current directory
if ([string]::IsNullOrWhiteSpace($TargetFile)) {
    $TargetFile = Join-Path $PSScriptRoot 'setup-noa.ps1'
}

if (-not (Test-Path $SourceFile)) {
    Write-Error "Source file not found: $SourceFile"
    exit 1
}

Write-Host "Reading source file: $SourceFile" -ForegroundColor Cyan
$content = [System.IO.File]::ReadAllText($SourceFile, [System.Text.Encoding]::UTF8)
Write-Host "  Read $($content.Length) characters" -ForegroundColor Gray

Write-Host "Making path replacements..." -ForegroundColor Cyan
# Note: Using escaped backslashes for regex pattern matching in both pattern and replacement
# The replacement string must also escape backslashes to prevent interpretation as escape sequences
# IMPORTANT: Do more specific replacements FIRST before general replacements
# Replace specific paths (e.g., W:\WSL\arkos) before general paths (W:\WSL)
$content = $content -replace 'W:\\WSL\\arkos', 'n:\\noa\\images'
# Replace all remaining instances of W:\WSL with n:\noa
$content = $content -replace 'W:\\WSL', 'n:\\noa'
# Replace script name references
$content = $content -replace 'setup-agentic-os\.ps1', 'setup-noa.ps1'

Write-Host "Writing adapted script: $TargetFile" -ForegroundColor Cyan
[System.IO.File]::WriteAllText($TargetFile, $content, [System.Text.Encoding]::UTF8)

if (Test-Path $TargetFile) {
    $fileInfo = Get-Item $TargetFile
    Write-Host "SUCCESS!" -ForegroundColor Green
    Write-Host "  File: $($fileInfo.Name)" -ForegroundColor White
    Write-Host "  Size: $($fileInfo.Length) bytes" -ForegroundColor White
    Write-Host "  Path: $($fileInfo.FullName)" -ForegroundColor White
} else {
    Write-Error "Failed to create file: $TargetFile"
    exit 1
}
