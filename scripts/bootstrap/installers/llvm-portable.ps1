<#
.SYNOPSIS
    Installs portable LLVM/Clang to NOA_ROOT/opt/llvm/

.DESCRIPTION
    Downloads and installs LLVM/Clang as a portable C/C++ compiler within the NOA contained environment.
    Creates symlinks in NOA_ROOT/bin/ for clang, clang++, lld, llvm-* tools.

    Use cases:
    - C/C++ compilation without Visual Studio
    - CUDA host compiler (with CUDA toolkit)
    - Cross-compilation

    Constitutional Compliance: §3.1 Self-Contained & Autonomous

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Version
    LLVM version to install (default: 19.1.6)

.PARAMETER Force
    Force reinstallation even if already installed

.EXAMPLE
    .\llvm-portable.ps1
    .\llvm-portable.ps1 -Version 19.1.6 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "19.1.6",
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
$LLVM_DIR = Join-Path $NOA_OPT "llvm"

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
Write-Host "NOA Portable LLVM/Clang Installer" -ForegroundColor Cyan
Write-Host "Version: $Version | Target: $LLVM_DIR" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$clangExe = Join-Path $LLVM_DIR "bin\clang.exe"
if ((Test-Path $clangExe) -and -not $Force) {
    $currentVersion = & $clangExe --version 2>&1 | Select-Object -First 1
    Write-Log "LLVM already installed: $currentVersion" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info
    exit 0
}

# Ensure directories exist
New-Item -ItemType Directory -Path $NOA_CACHE -Force | Out-Null
New-Item -ItemType Directory -Path $NOA_OPT -Force | Out-Null
New-Item -ItemType Directory -Path $NOA_BIN -Force | Out-Null

# Source download library for checksum support
$downloadLib = Join-Path $PSScriptRoot "..\lib\download.ps1"
if (Test-Path $downloadLib) {
    . $downloadLib
}

# Download LLVM - Windows uses NSIS installer (.exe)
$downloadUrl = "https://github.com/llvm/llvm-project/releases/download/llvmorg-$Version/LLVM-$Version-win64.exe"
$installerFileName = "LLVM-$Version-win64.exe"

Write-Log "Downloading LLVM $Version (this is ~500MB)..." -Level Info
try {
    if (Test-Path $downloadLib) {
        # Use Get-NoaDownload for checksum support (when available)
        $installerFile = Get-NoaDownload -Url $downloadUrl -DestinationName $installerFileName -UseCache
    } else {
        # Fallback to direct download
        $installerFile = Join-Path $NOA_CACHE $installerFileName
        if (-not (Test-Path $installerFile)) {
            Invoke-WebRequest -Uri $downloadUrl -OutFile $installerFile -UseBasicParsing
        }
    }
    Write-Log "Downloaded: $([math]::Round((Get-Item $installerFile).Length / 1MB, 1)) MB" -Level Success
} catch {
    Write-Log "Download failed: $_" -Level Error
    exit 1
}

# Extract using 7-Zip if available, otherwise fall back to the installer
Write-Log "Extracting LLVM..." -Level Info

# Try 7-Zip first (NSIS installers are 7z archives)
$sevenZip = Get-Command 7z -ErrorAction SilentlyContinue
if (-not $sevenZip) {
    # Check common locations
    $sevenZipPaths = @(
        "$env:ProgramFiles\7-Zip\7z.exe",
        "${env:ProgramFiles(x86)}\7-Zip\7z.exe",
        (Join-Path $NoaRoot "bin\7z.exe")
    )
    foreach ($path in $sevenZipPaths) {
        if (Test-Path $path) {
            $sevenZip = Get-Item $path
            break
        }
    }
}

if ($sevenZip) {
    # Extract with 7-Zip (faster, no admin)
    Write-Log "Using 7-Zip for extraction..." -Level Info
    try {
        # Remove old installation
        if (Test-Path $LLVM_DIR) {
            Remove-Item -Recurse -Force $LLVM_DIR
        }

        # Extract - NSIS creates a folder structure inside
        $tempExtract = Join-Path $NOA_OPT "llvm-temp"
        Remove-Item -Recurse -Force $tempExtract -ErrorAction SilentlyContinue
        & $sevenZip.FullName x $installerFile -o"$tempExtract" -y | Out-Null

        # Move the extracted content
        $extractedBin = Get-ChildItem $tempExtract -Filter "bin" -Directory -Recurse | Select-Object -First 1
        if ($extractedBin) {
            $extractedRoot = $extractedBin.Parent.FullName
            Move-Item $extractedRoot $LLVM_DIR -Force
        } else {
            # Might be flat structure
            Move-Item $tempExtract $LLVM_DIR -Force
        }
        Remove-Item -Recurse -Force $tempExtract -ErrorAction SilentlyContinue

        Write-Log "Extracted to $LLVM_DIR" -Level Success
    } catch {
        Write-Log "7-Zip extraction failed: $_" -Level Warning
        Write-Log "Falling back to installer (requires admin)..." -Level Info
    }
}

if (-not (Test-Path $clangExe)) {
    # Fall back to NSIS installer (may require admin)
    Write-Log "Running LLVM installer (silent mode)..." -Level Info
    Write-Log "Note: This may require administrator privileges" -Level Warning
    try {
        Start-Process -FilePath $installerFile -ArgumentList "/S", "/D=$LLVM_DIR" -Wait -NoNewWindow
        Write-Log "Installed to $LLVM_DIR" -Level Success
    } catch {
        Write-Log "Installation failed: $_" -Level Error
        Write-Log "Try running as Administrator or install 7-Zip first" -Level Info
        exit 1
    }
}

# Create symlinks in bin/
Write-Log "Creating symlinks in $NOA_BIN..." -Level Info
$llvmTools = @(
    "clang.exe",
    "clang++.exe",
    "clang-cl.exe",     # MSVC-compatible driver
    "lld.exe",          # Fast linker
    "lld-link.exe",     # MSVC-compatible linker
    "llvm-ar.exe",
    "llvm-nm.exe",
    "llvm-objdump.exe",
    "llvm-ranlib.exe",
    "llvm-size.exe",
    "clang-format.exe",
    "clang-tidy.exe"
)

foreach ($tool in $llvmTools) {
    $target = Join-Path $LLVM_DIR "bin\$tool"
    $link = Join-Path $NOA_BIN $tool
    if (Test-Path $target) {
        Remove-Item $link -Force -ErrorAction SilentlyContinue
        New-Item -ItemType SymbolicLink -Path $link -Target $target -Force | Out-Null
        Write-Log "Linked $tool" -Level Success
    }
}

# Verify installation
Write-Log "Verifying installation..." -Level Info
if (Test-Path $clangExe) {
    $installedVersion = & $clangExe --version 2>&1 | Select-Object -First 1
    Write-Log "LLVM installed successfully: $installedVersion" -Level Success
} else {
    Write-Log "Installation verification failed - clang.exe not found" -Level Error
    exit 1
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "LLVM $Version installed successfully!" -ForegroundColor Green
Write-Host "Location: $LLVM_DIR" -ForegroundColor Gray
Write-Host "Symlinks: clang, clang++, clang-cl, lld, etc." -ForegroundColor Gray
Write-Host "" -ForegroundColor Gray
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  clang --version          # C compiler" -ForegroundColor Gray
Write-Host "  clang++ --version        # C++ compiler" -ForegroundColor Gray
Write-Host "  clang-cl --version       # MSVC-compatible driver" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Green

exit 0

