<#
.SYNOPSIS
    Install Abacus Desktop to NOA opt directory.

.DESCRIPTION
    Downloads and installs Abacus Desktop to noa_root/opt/abacus-desktop/.
    Creates wrapper script in noa_root/bin/ for easy access.
    Abacus Desktop is required for Abacus CLI authentication (AI provider).

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\abacus-desktop.ps1
    .\abacus-desktop.ps1 -Force
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
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$OPT_DIR = Join-Path $NoaRoot "opt"
$BIN_DIR = Join-Path $NoaRoot "bin"
$INSTALL_DIR = Join-Path $OPT_DIR "abacus-desktop"
$INSTALLER_PATH = Join-Path $OPT_DIR "AbacusAISetup-x64-latest.exe"
$WRAPPER_PATH = Join-Path $BIN_DIR "abacus-desktop.cmd"

# Download URL (update with actual URL when available)
$DOWNLOAD_URL = "https://desktop.abacus.ai/download/windows/x64/latest"

Write-Host "NOA Abacus Desktop Installer" -ForegroundColor Cyan
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor Gray
Write-Host ""

# Create directories
New-Item -ItemType Directory -Path $OPT_DIR -Force -ErrorAction SilentlyContinue | Out-Null
New-Item -ItemType Directory -Path $BIN_DIR -Force -ErrorAction SilentlyContinue | Out-Null

# Check if already installed
$existingExe = Join-Path $INSTALL_DIR "Abacus.exe"
if ((Test-Path $existingExe) -and -not $Force) {
    try {
        $version = (Get-Item $existingExe).VersionInfo.ProductVersion
        Write-Host "  [OK] Abacus Desktop already installed: v$version" -ForegroundColor Green
        Write-Host "  Location: $INSTALL_DIR" -ForegroundColor Gray
        Write-Host "  Use -Force to reinstall" -ForegroundColor Gray
        exit 0
    } catch {
        Write-Host "  [INFO] Existing installation found but version unknown" -ForegroundColor Yellow
    }
}

# Check if installer already downloaded
if (-not (Test-Path $INSTALLER_PATH)) {
    Write-Host "  [INFO] Downloading Abacus Desktop installer..." -ForegroundColor Yellow
    Write-Host "  URL: $DOWNLOAD_URL" -ForegroundColor Gray

    try {
        # Use .NET WebClient for download with progress
        $webClient = New-Object System.Net.WebClient
        $webClient.DownloadFile($DOWNLOAD_URL, $INSTALLER_PATH)
        Write-Host "  [OK] Downloaded: $INSTALLER_PATH" -ForegroundColor Green
    } catch {
        Write-Host "  [ERROR] Download failed: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "  Manual installation:" -ForegroundColor Yellow
        Write-Host "    1. Download from: https://desktop.abacus.ai/" -ForegroundColor Gray
        Write-Host "    2. Save to: $INSTALLER_PATH" -ForegroundColor Gray
        Write-Host "    3. Run this script again" -ForegroundColor Gray
        exit 1
    }
} else {
    Write-Host "  [OK] Installer already downloaded: $INSTALLER_PATH" -ForegroundColor Green
}

# Install Abacus Desktop to opt/abacus-desktop/
Write-Host "  [INFO] Installing Abacus Desktop to NOA opt directory..." -ForegroundColor Yellow
Write-Host "  Target: $INSTALL_DIR" -ForegroundColor Gray

try {
    # Try silent install with common flags
    # Note: Adjust flags based on actual installer type (NSIS, Inno Setup, MSI, etc.)
    $installArgs = @(
        "/S",                          # Silent install (NSIS)
        "/D=$INSTALL_DIR"              # Installation directory
    )

    $process = Start-Process -FilePath $INSTALLER_PATH -ArgumentList $installArgs -Wait -PassThru -NoNewWindow

    if ($process.ExitCode -eq 0) {
        Write-Host "  [OK] Abacus Desktop installed successfully" -ForegroundColor Green
    } else {
        Write-Host "  [WARN] Installer exited with code: $($process.ExitCode)" -ForegroundColor Yellow
        Write-Host "  [INFO] Checking if installation succeeded anyway..." -ForegroundColor Gray
    }
} catch {
    Write-Host "  [ERROR] Installation failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "  Manual installation:" -ForegroundColor Yellow
    Write-Host "    Run: $INSTALLER_PATH" -ForegroundColor Gray
    Write-Host "    Install to: $INSTALL_DIR" -ForegroundColor Gray
    exit 1
}

# Verify installation
$abacusExe = Get-ChildItem -Path $INSTALL_DIR -Filter "*.exe" -Recurse -ErrorAction SilentlyContinue |
    Where-Object { $_.Name -match "Abacus|abacus" } |
    Select-Object -First 1

if (-not $abacusExe) {
    Write-Host "  [ERROR] Installation verification failed - Abacus.exe not found" -ForegroundColor Red
    Write-Host "  Expected location: $INSTALL_DIR" -ForegroundColor Gray
    exit 1
}

Write-Host "  [OK] Found: $($abacusExe.FullName)" -ForegroundColor Green

# Create wrapper script in bin/
Write-Host "  [INFO] Creating wrapper script..." -ForegroundColor Yellow

$wrapperContent = @"
@echo off
REM Abacus Desktop Wrapper - Generated by NOA bootstrap
REM Launches Abacus Desktop from NOA opt directory
REM Required for Abacus CLI authentication (AI provider)

"$($abacusExe.FullName)" %*
"@

$wrapperContent | Set-Content -Path $WRAPPER_PATH -Encoding ASCII
Write-Host "  [OK] Created wrapper: $WRAPPER_PATH" -ForegroundColor Green

# Update provider configs
$providerconfigs = Join-Path $NoaRoot "ai\providers\cloud\abacus\configs.json"
if (Test-Path $providerconfigs) {
    Write-Host "  [INFO] Updating provider configsuration..." -ForegroundColor Yellow

    try {
        $configs = Get-Content $providerconfigs -Raw | ConvertFrom-Json

        # Add desktop binary path
        if (-not $configs.PSObject.Properties['desktop']) {
            $configs | Add-Member -MemberType NoteProperty -Name 'desktop' -Value @{} -Force
        }

        $configs.desktop = @{
            binaryPath = @{
                windows = "`${NOA_ROOT}/opt/abacus-desktop/$($abacusExe.Name)"
                unix = "`${NOA_ROOT}/opt/abacus-desktop/bin/abacus"
            }
            wrapper = @{
                windows = "`${NOA_ROOT}/bin/abacus-desktop.cmd"
                unix = "`${NOA_ROOT}/bin/abacus-desktop"
            }
        }

        $configs | ConvertTo-Json -Depth 10 | Set-Content $providerconfigs -Encoding UTF8
        Write-Host "  [OK] Updated provider configs: $providerconfigs" -ForegroundColor Green
    } catch {
        Write-Host "  [WARN] Failed to update provider configs: $_" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Abacus Desktop installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Location: $INSTALL_DIR" -ForegroundColor Gray
Write-Host "Wrapper:  $WRAPPER_PATH" -ForegroundColor Gray
Write-Host ""
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  abacus-desktop        # Launch Abacus Desktop" -ForegroundColor Gray
Write-Host ""
Write-Host "Note: Abacus Desktop authentication is required for Abacus CLI" -ForegroundColor Yellow
Write-Host "      Sign in to Abacus Desktop before using 'abacusai' CLI commands" -ForegroundColor Yellow
