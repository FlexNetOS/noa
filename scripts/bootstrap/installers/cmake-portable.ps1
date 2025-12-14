<#
.SYNOPSIS
    Installs portable CMake to NOA_ROOT/opt/cmake/

.DESCRIPTION
    Downloads and installs CMake as a portable tool within the NOA contained environment.
    Creates symlinks in NOA_ROOT/bin/ for easy access.

    Constitutional Compliance: §3.1 Self-Contained & Autonomous

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from NOA_ROOT env or script location)

.PARAMETER Version
    CMake version to install (default: 3.31.3)

.PARAMETER Force
    Force reinstallation even if already installed

.EXAMPLE
    .\cmake-portable.ps1
    .\cmake-portable.ps1 -Version 3.31.3 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "3.31.3",
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) {
        $env:NOA_ROOT
    } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$NOA_BIN = Join-Path $NoaRoot "bin"
$NOA_OPT = Join-Path $NoaRoot "opt"
$NOA_CACHE = Join-Path $NoaRoot "cache"
$CMAKE_DIR = Join-Path $NOA_OPT "cmake"

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        default { "White" }
    }
    $prefix = switch ($Level) {
        "Success" { "[OK]" }
        "Warning" { "[!!]" }
        "Error" { "[XX]" }
        default { "[..]" }
    }
    Write-Host "$prefix $Message" -ForegroundColor $color
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable CMake Installer" -ForegroundColor Cyan
Write-Host "Version: $Version | Target: $CMAKE_DIR" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$cmakeExe = Join-Path $CMAKE_DIR "bin\cmake.exe"
if ((Test-Path $cmakeExe) -and -not $Force) {
    $currentVersion = & $cmakeExe --version 2>&1 | Select-Object -First 1
    Write-Log "CMake already installed: $currentVersion" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info
    exit 0
}

# Ensure directories exist
New-Item -ItemType Directory -Path $NOA_CACHE -Force | Out-Null
New-Item -ItemType Directory -Path $NOA_OPT -Force | Out-Null
New-Item -ItemType Directory -Path $NOA_BIN -Force | Out-Null

# Download CMake
$downloadUrl = "https://github.com/Kitware/CMake/releases/download/v$Version/cmake-$Version-windows-x86_64.zip"
$zipFile = Join-Path $NOA_CACHE "cmake-$Version-windows-x86_64.zip"
$extractDir = Join-Path $NOA_OPT "cmake-$Version-windows-x86_64"

Write-Log "Downloading CMake $Version..." -Level Info
try {
    if (-not (Test-Path $zipFile)) {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipFile -UseBasicParsing
        Write-Log "Downloaded: $([math]::Round((Get-Item $zipFile).Length / 1MB, 1)) MB" -Level Success
    } else {
        Write-Log "Using cached download" -Level Info
    }
} catch {
    Write-Log "Download failed: $_" -Level Error
    exit 1
}

# Extract
Write-Log "Extracting to $NOA_OPT..." -Level Info
try {
    # Remove old extraction if exists
    if (Test-Path $extractDir) {
        Remove-Item -Recurse -Force $extractDir
    }
    if (Test-Path $CMAKE_DIR) {
        Remove-Item -Recurse -Force $CMAKE_DIR
    }

    Expand-Archive -Path $zipFile -DestinationPath $NOA_OPT -Force
    Rename-Item $extractDir $CMAKE_DIR
    Write-Log "Extracted to $CMAKE_DIR" -Level Success
} catch {
    Write-Log "Extraction failed: $_" -Level Error
    exit 1
}

# Create symlinks in bin/
Write-Log "Creating symlinks in $NOA_BIN..." -Level Info
$cmakeTools = @("cmake.exe", "ctest.exe", "cpack.exe")
foreach ($tool in $cmakeTools) {
    $target = Join-Path $CMAKE_DIR "bin\$tool"
    $link = Join-Path $NOA_BIN $tool
    if (Test-Path $target) {
        Remove-Item $link -Force -ErrorAction SilentlyContinue
        New-Item -ItemType SymbolicLink -Path $link -Target $target -Force | Out-Null
        Write-Log "Linked $tool" -Level Success
    }
}

# Verify installation
Write-Log "Verifying installation..." -Level Info
$installedVersion = & $cmakeExe --version 2>&1 | Select-Object -First 1
if ($installedVersion -match "cmake version") {
    Write-Log "CMake installed successfully: $installedVersion" -Level Success
} else {
    Write-Log "Installation verification failed" -Level Error
    exit 1
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "CMake $Version installed successfully!" -ForegroundColor Green
Write-Host "Location: $CMAKE_DIR" -ForegroundColor Gray
Write-Host "Symlinks: $NOA_BIN\cmake.exe, ctest.exe, cpack.exe" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Green

exit 0

