<#
.SYNOPSIS
    Installs portable Make (or uses MinGW's mingw32-make) to NOA_ROOT/opt/make/

.DESCRIPTION
    On Windows, Make is typically provided by MinGW-w64 as mingw32-make.exe.
    This script:
    1. Checks if MinGW is installed and uses its mingw32-make
    2. If MinGW not found, downloads standalone GnuWin32 Make
    3. Creates symlink in NOA_ROOT/bin/ for make.exe

    Constitutional Compliance: §3.1 Self-Contained & Autonomous

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstallation even if already installed

.EXAMPLE
    .\make-portable.ps1
    .\make-portable.ps1 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
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
$MAKE_DIR = Join-Path $NOA_OPT "make"

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
Write-Host "NOA Portable Make Installer" -ForegroundColor Cyan
Write-Host "Target: $NOA_BIN\make.exe" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$makeExe = Join-Path $NOA_BIN "make.exe"
if ((Test-Path $makeExe) -and -not $Force) {
    $currentVersion = & $makeExe --version 2>&1 | Select-Object -First 1
    Write-Log "Make already installed: $currentVersion" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info
    exit 0
}

# Ensure directories exist
New-Item -ItemType Directory -Path $NOA_CACHE -Force | Out-Null
New-Item -ItemType Directory -Path $NOA_OPT -Force | Out-Null
New-Item -ItemType Directory -Path $NOA_BIN -Force | Out-Null

# Strategy 1: Check if MinGW is installed (preferred - mingw32-make)
$mingwMake = Join-Path $NOA_OPT "mingw\bin\mingw32-make.exe"
if (Test-Path $mingwMake) {
    Write-Log "Found MinGW-w64 installation" -Level Info
    Write-Log "Using mingw32-make from MinGW" -Level Success

    Remove-Item $makeExe -Force -ErrorAction SilentlyContinue
    New-Item -ItemType SymbolicLink -Path $makeExe -Target $mingwMake -Force | Out-Null
    Write-Log "Created symlink: make.exe -> mingw32-make.exe" -Level Success

    # Verify
    $version = & $makeExe --version 2>&1 | Select-Object -First 1
    Write-Log "Make installed: $version" -Level Success

    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host "Make installed successfully (via MinGW)!" -ForegroundColor Green
    Write-Host "Location: $makeExe -> $mingwMake" -ForegroundColor Gray
    Write-Host "=" * 60 -ForegroundColor Green
    exit 0
}

# Strategy 2: Download standalone GnuWin32 Make
Write-Log "MinGW not found. Installing standalone GnuWin32 Make..." -Level Info

$makeVersion = "3.81"
$downloadUrl = "https://sourceforge.net/projects/gnuwin32/files/make/3.81/make-3.81-bin.zip/download"
$zipFile = Join-Path $NOA_CACHE "make-3.81-bin.zip"

Write-Log "Downloading GnuWin32 Make $makeVersion..." -Level Info
try {
    if (-not (Test-Path $zipFile)) {
        # SourceForge redirects, so we need to follow redirects
        $response = Invoke-WebRequest -Uri $downloadUrl -MaximumRedirection 0 -ErrorAction SilentlyContinue
        if ($response.StatusCode -eq 302 -or $response.StatusCode -eq 301) {
            $actualUrl = $response.Headers.Location
            Invoke-WebRequest -Uri $actualUrl -OutFile $zipFile -UseBasicParsing
        } else {
            Invoke-WebRequest -Uri $downloadUrl -OutFile $zipFile -UseBasicParsing
        }
        Write-Log "Downloaded: $([math]::Round((Get-Item $zipFile).Length / 1KB, 1)) KB" -Level Success
    } else {
        Write-Log "Using cached download" -Level Info
    }
} catch {
    Write-Log "Download failed: $_" -Level Error
    Write-Log "Alternative: Install MinGW-w64 first (provides mingw32-make)" -Level Info
    Write-Log "Run: .\scripts\bootstrap\installers\mingw-portable.ps1" -Level Info
    exit 1
}

# Extract
Write-Log "Extracting to $MAKE_DIR..." -Level Info
try {
    if (Test-Path $MAKE_DIR) {
        Remove-Item -Recurse -Force $MAKE_DIR
    }

    Expand-Archive -Path $zipFile -DestinationPath $MAKE_DIR -Force

    # GnuWin32 structure: bin/, share/, etc.
    # Find make.exe
    $extractedMake = Get-ChildItem $MAKE_DIR -Filter "make.exe" -Recurse | Select-Object -First 1
    if ($extractedMake) {
        # Move to a consistent location
        $makeBinDir = Join-Path $MAKE_DIR "bin"
        New-Item -ItemType Directory -Path $makeBinDir -Force | Out-Null
        Copy-Item $extractedMake.FullName (Join-Path $makeBinDir "make.exe") -Force
        Write-Log "Extracted to $makeBinDir" -Level Success
    } else {
        Write-Log "make.exe not found in archive" -Level Error
        exit 1
    }
} catch {
    Write-Log "Extraction failed: $_" -Level Error
    exit 1
}

# Create symlink
Write-Log "Creating symlink in $NOA_BIN..." -Level Info
$target = Join-Path $MAKE_DIR "bin\make.exe"
if (Test-Path $target) {
    Remove-Item $makeExe -Force -ErrorAction SilentlyContinue
    New-Item -ItemType SymbolicLink -Path $makeExe -Target $target -Force | Out-Null
    Write-Log "Linked make.exe" -Level Success
}

# Verify installation
Write-Log "Verifying installation..." -Level Info
if (Test-Path $makeExe) {
    $installedVersion = & $makeExe --version 2>&1 | Select-Object -First 1
    Write-Log "Make installed successfully: $installedVersion" -Level Success
} else {
    Write-Log "Installation verification failed" -Level Error
    exit 1
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "Make installed successfully!" -ForegroundColor Green
Write-Host "Location: $makeExe" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Green

exit 0

