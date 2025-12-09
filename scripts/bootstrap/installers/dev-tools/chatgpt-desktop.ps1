<#
.SYNOPSIS
    Detect and configure ChatGPT Desktop for NOA integration.

.DESCRIPTION
    Detects ChatGPT Desktop installation and creates configuration notes.
    ChatGPT Desktop must be manually installed from OpenAI.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\chatgpt-desktop.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

Write-Host "Checking ChatGPT Desktop installation..." -ForegroundColor Cyan

# Common ChatGPT Desktop installation paths
$chatgptPaths = @(
    "$env:LOCALAPPDATA\Programs\chatgpt\ChatGPT.exe"
    "$env:LOCALAPPDATA\ChatGPT\ChatGPT.exe"
    "C:\Program Files\ChatGPT\ChatGPT.exe"
    "$env:USERPROFILE\AppData\Local\Programs\chatgpt\ChatGPT.exe"
)

$chatgptExe = $null
foreach ($path in $chatgptPaths) {
    if (Test-Path $path) {
        $chatgptExe = $path
        break
    }
}

if ($chatgptExe) {
    Write-Host "  [OK] ChatGPT Desktop found: $chatgptExe" -ForegroundColor Green

    # Check if it's running
    $process = Get-Process -Name "ChatGPT" -ErrorAction SilentlyContinue
    if ($process) {
        Write-Host "  [OK] ChatGPT Desktop is running" -ForegroundColor Green
    } else {
        Write-Host "  [INFO] ChatGPT Desktop is not currently running" -ForegroundColor Gray
    }
} else {
    Write-Host "  [SKIP] ChatGPT Desktop not found" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  To install ChatGPT Desktop:" -ForegroundColor Gray
    Write-Host "    1. Visit https://openai.com/chatgpt/desktop/" -ForegroundColor Gray
    Write-Host "    2. Download the Windows installer" -ForegroundColor Gray
    Write-Host "    3. Run the installer" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  Note: ChatGPT Desktop requires a ChatGPT Plus subscription" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "ChatGPT Desktop check complete." -ForegroundColor Green

