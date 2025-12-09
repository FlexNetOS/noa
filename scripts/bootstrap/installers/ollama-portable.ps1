<#
.SYNOPSIS
    Install Ollama to NOA opt directory (self-contained per FR-001, §3.1).

.DESCRIPTION
    Downloads and installs Ollama to noa_root/opt/ollama/.
    Creates wrapper script in noa_root/bin/ for CLI access.
    This is the Priority 1 local AI provider alongside llama.cpp.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\ollama-portable.ps1
    .\ollama-portable.ps1 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        # Walk up from script location to find repo root (contains .git)
        $current = $PSScriptRoot
        while ($current -and -not (Test-Path (Join-Path $current ".git"))) {
            $current = Split-Path -Parent $current
        }
        if ($current) { $current } else { Get-Location }
    }
}

$OPT_DIR = Join-Path $NoaRoot "opt"
$BIN_DIR = Join-Path $NoaRoot "bin"
$INSTALL_DIR = Join-Path $OPT_DIR "ollama"
$OLLAMA_EXE = Join-Path $INSTALL_DIR "ollama.exe"
$WRAPPER_PATH = Join-Path $BIN_DIR "ollama.cmd"
$MODELS_DIR = Join-Path $NoaRoot "ai\shared\models\ollama"

# Download URL - Windows release
$OLLAMA_VERSION = "latest"
$DOWNLOAD_URL = "https://github.com/ollama/ollama/releases/latest/download/ollama-windows-amd64.zip"

Write-Host "NOA Ollama Installer (Self-Contained)" -ForegroundColor Cyan
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor Gray
Write-Host "Target:   $INSTALL_DIR" -ForegroundColor Gray
Write-Host ""

# Create directories
New-Item -ItemType Directory -Path $OPT_DIR -Force -ErrorAction SilentlyContinue | Out-Null
New-Item -ItemType Directory -Path $BIN_DIR -Force -ErrorAction SilentlyContinue | Out-Null
New-Item -ItemType Directory -Path $INSTALL_DIR -Force -ErrorAction SilentlyContinue | Out-Null
New-Item -ItemType Directory -Path $MODELS_DIR -Force -ErrorAction SilentlyContinue | Out-Null

# Check if already installed
if ((Test-Path $OLLAMA_EXE) -and -not $Force) {
    try {
        $version = & $OLLAMA_EXE --version 2>&1 | Select-Object -First 1
        Write-Host "  [OK] Ollama already installed: $version" -ForegroundColor Green
        Write-Host "  Location: $INSTALL_DIR" -ForegroundColor Gray
        Write-Host "  Use -Force to reinstall" -ForegroundColor Gray
        exit 0
    } catch {
        Write-Host "  [INFO] Existing installation found but version check failed" -ForegroundColor Yellow
    }
}

# Download Ollama
$TEMP_ZIP = Join-Path $env:TEMP "ollama-windows-amd64.zip"

Write-Host "  [INFO] Downloading Ollama..." -ForegroundColor Yellow
Write-Host "  URL: $DOWNLOAD_URL" -ForegroundColor Gray

try {
    # Use TLS 1.2
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    
    $webClient = New-Object System.Net.WebClient
    $webClient.DownloadFile($DOWNLOAD_URL, $TEMP_ZIP)
    Write-Host "  [OK] Downloaded: $TEMP_ZIP" -ForegroundColor Green
} catch {
    Write-Host "  [ERROR] Download failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "  Manual installation:" -ForegroundColor Yellow
    Write-Host "    1. Download from: https://ollama.com/download" -ForegroundColor Gray
    Write-Host "    2. Extract to: $INSTALL_DIR" -ForegroundColor Gray
    Write-Host "    3. Run this script again" -ForegroundColor Gray
    exit 1
}

# Extract Ollama
Write-Host "  [INFO] Extracting to $INSTALL_DIR..." -ForegroundColor Yellow

try {
    # Remove old installation
    if (Test-Path $INSTALL_DIR) {
        Remove-Item -Path (Join-Path $INSTALL_DIR "*") -Force -Recurse -ErrorAction SilentlyContinue
    }
    
    Expand-Archive -Path $TEMP_ZIP -DestinationPath $INSTALL_DIR -Force
    Write-Host "  [OK] Extracted successfully" -ForegroundColor Green
} catch {
    Write-Host "  [ERROR] Extraction failed: $_" -ForegroundColor Red
    exit 1
} finally {
    # Cleanup temp file
    Remove-Item -Path $TEMP_ZIP -Force -ErrorAction SilentlyContinue
}

# Find ollama.exe (might be in a subdirectory)
$OLLAMA_FOUND = Get-ChildItem -Path $INSTALL_DIR -Filter "ollama.exe" -Recurse | Select-Object -First 1
if (-not $OLLAMA_FOUND) {
    Write-Host "  [ERROR] ollama.exe not found after extraction" -ForegroundColor Red
    exit 1
}

# Move to root if in subdirectory
if ($OLLAMA_FOUND.DirectoryName -ne $INSTALL_DIR) {
    Move-Item -Path (Join-Path $OLLAMA_FOUND.DirectoryName "*") -Destination $INSTALL_DIR -Force
    Remove-Item -Path $OLLAMA_FOUND.DirectoryName -Force -Recurse -ErrorAction SilentlyContinue
}

$OLLAMA_EXE = Join-Path $INSTALL_DIR "ollama.exe"
Write-Host "  [OK] Found: $OLLAMA_EXE" -ForegroundColor Green

# Verify installation
try {
    $version = & $OLLAMA_EXE --version 2>&1 | Select-Object -First 1
    Write-Host "  [OK] Version: $version" -ForegroundColor Green
} catch {
    Write-Host "  [WARN] Version check failed, but binary exists" -ForegroundColor Yellow
}

# Create wrapper script in bin/
Write-Host "  [INFO] Creating wrapper script..." -ForegroundColor Yellow

$wrapperContent = @"
@ECHO off
REM Ollama Wrapper - Generated by NOA bootstrap
REM Self-contained installation per FR-001, Constitution §3.1
REM Runs in CURRENT terminal (no new window)
SETLOCAL EnableDelayedExpansion

SET "NOA_ROOT=%~dp0.."
SET "OLLAMA_EXE=%NOA_ROOT%\opt\ollama\ollama.exe"
SET "OLLAMA_MODELS=%NOA_ROOT%\ai\shared\models\ollama"

REM Set Ollama environment for self-containment
SET "OLLAMA_MODELS=%OLLAMA_MODELS%"

IF NOT EXIST "%OLLAMA_EXE%" (
    echo [ERROR] Ollama not found at %OLLAMA_EXE%
    echo Run: pwsh -File scripts\bootstrap\installers\ollama-portable.ps1
    exit /b 1
)

"%OLLAMA_EXE%" %*
"@

$wrapperContent | Set-Content -Path $WRAPPER_PATH -Encoding ASCII
Write-Host "  [OK] Created wrapper: $WRAPPER_PATH" -ForegroundColor Green

# Update provider config
$providerConfig = Join-Path $NoaRoot "ai\providers\local\ollama\config.json"
if (Test-Path $providerConfig) {
    Write-Host "  [INFO] Updating provider configuration..." -ForegroundColor Yellow
    
    try {
        $config = Get-Content $providerConfig -Raw | ConvertFrom-Json
        
        # Add binary paths
        if (-not $config.PSObject.Properties['cli']) {
            $config | Add-Member -MemberType NoteProperty -Name 'cli' -Value @{} -Force
        }
        
        $config.cli = @{
            command = "ollama"
            binaryPath = @{
                windows = "`${NOA_ROOT}/opt/ollama/ollama.exe"
                unix = "`${NOA_ROOT}/opt/ollama/ollama"
            }
            wrapper = @{
                windows = "`${NOA_ROOT}/bin/ollama.cmd"
                unix = "`${NOA_ROOT}/bin/ollama"
            }
        }
        
        $config | ConvertTo-Json -Depth 10 | Set-Content $providerConfig -Encoding UTF8
        Write-Host "  [OK] Updated provider config" -ForegroundColor Green
    } catch {
        Write-Host "  [WARN] Failed to update provider config: $_" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Ollama installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Location: $INSTALL_DIR" -ForegroundColor Gray
Write-Host "Wrapper:  $WRAPPER_PATH" -ForegroundColor Gray
Write-Host "Models:   $MODELS_DIR" -ForegroundColor Gray
Write-Host ""
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  ollama serve              # Start Ollama server" -ForegroundColor Gray
Write-Host "  ollama pull llama2        # Download a model" -ForegroundColor Gray
Write-Host "  ollama run llama2         # Run a model" -ForegroundColor Gray
Write-Host ""
Write-Host "Note: Models are stored in $MODELS_DIR (self-contained)" -ForegroundColor Yellow

