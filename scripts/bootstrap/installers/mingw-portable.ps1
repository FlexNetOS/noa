<#
.SYNOPSIS
    Installs portable MinGW-w64 to NOA_ROOT/opt/mingw/

.DESCRIPTION
    Downloads and installs MinGW-w64 as a portable C/C++ compiler within the NOA contained environment.
    Creates symlinks in NOA_ROOT/bin/ for gcc, g++, mingw32-make, and other tools.

    MinGW-w64 provides GCC compiler for Windows without requiring Visual Studio.
    Useful for building C/C++ projects that prefer GCC toolchain.

    Constitutional Compliance: §3.1 Self-Contained & Autonomous

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Version
    MinGW-w64 version to install (default: 14.2.0)

.PARAMETER Force
    Force reinstallation even if already installed

.EXAMPLE
    .\mingw-portable.ps1
    .\mingw-portable.ps1 -Version 14.2.0 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "14.2.0",
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
$MINGW_DIR = Join-Path $NOA_OPT "mingw"

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
Write-Host "NOA Portable MinGW-w64 Installer" -ForegroundColor Cyan
Write-Host "Version: $Version | Target: $MINGW_DIR" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$gccExe = Join-Path $MINGW_DIR "bin\gcc.exe"
if ((Test-Path $gccExe) -and -not $Force) {
    $currentVersion = & $gccExe --version 2>&1 | Select-Object -First 1
    Write-Log "MinGW-w64 already installed: $currentVersion" -Level Success
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

# Download MinGW-w64 - Using niXman's builds (popular, well-maintained)
# Release format: x86_64-{version}-release-posix-seh-ucrt-rt_v{rt_version}-rev{revision}.7z
$rtVersion = "12"
$revision = "0"
$downloadUrl = "https://github.com/niXman/mingw-builds-binaries/releases/download/$Version-rt_v$rtVersion-rev$revision/x86_64-$Version-release-posix-seh-ucrt-rt_v$rtVersion-rev$revision.7z"
$archiveFileName = "mingw-$Version-x86_64.7z"

Write-Log "Downloading MinGW-w64 $Version (~200MB)..." -Level Info
try {
    if (Test-Path $downloadLib) {
        # Use Get-NoaDownload for checksum support (when available)
        $archiveFile = Get-NoaDownload -Url $downloadUrl -DestinationName $archiveFileName -UseCache
    } else {
        # Fallback to direct download
        $archiveFile = Join-Path $NOA_CACHE $archiveFileName
        if (-not (Test-Path $archiveFile)) {
            Invoke-WebRequest -Uri $downloadUrl -OutFile $archiveFile -UseBasicParsing
        }
    }
    Write-Log "Downloaded: $([math]::Round((Get-Item $archiveFile).Length / 1MB, 1)) MB" -Level Success
} catch {
    Write-Log "Download failed: $_" -Level Error
    Write-Log "Trying alternative download URL..." -Level Warning

    # Alternative: Try winlibs.com builds
    $altUrl = "https://github.com/brechtsanders/winlibs_mingw/releases/download/$Version-16.0.6-11.0.0-ucrt-r1/winlibs-x86_64-posix-seh-gcc-$Version-mingw-w64ucrt-11.0.0-r1.7z"
    try {
        Invoke-WebRequest -Uri $altUrl -OutFile $archiveFile -UseBasicParsing
        Write-Log "Downloaded from alternative source" -Level Success
    } catch {
        Write-Log "Alternative download also failed: $_" -Level Error
        exit 1
    }
}

# Extract using 7-Zip
Write-Log "Extracting MinGW-w64..." -Level Info

# Find 7-Zip
$sevenZip = Get-Command 7z -ErrorAction SilentlyContinue
if (-not $sevenZip) {
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

if (-not $sevenZip) {
    Write-Log "7-Zip not found. MinGW-w64 requires 7-Zip for extraction." -Level Error
    Write-Log "Install 7-Zip from https://www.7-zip.org/ or use portable 7-Zip" -Level Info
    exit 1
}

try {
    # Remove old installation
    if (Test-Path $MINGW_DIR) {
        Remove-Item -Recurse -Force $MINGW_DIR
    }

    # Extract - MinGW builds are typically flat (bin/, lib/, include/, etc. at root)
    $tempExtract = Join-Path $NOA_OPT "mingw-temp"
    Remove-Item -Recurse -Force $tempExtract -ErrorAction SilentlyContinue
    New-Item -ItemType Directory -Path $tempExtract -Force | Out-Null

    & $sevenZip.FullName x $archiveFile -o"$tempExtract" -y | Out-Null

    # Find the extracted directory (usually named like "mingw64" or "x86_64-...")
    $extractedRoot = Get-ChildItem $tempExtract -Directory | Select-Object -First 1
    if ($extractedRoot) {
        Move-Item $extractedRoot.FullName $MINGW_DIR -Force
    } else {
        # Might be flat structure already
        Move-Item (Join-Path $tempExtract "*") $MINGW_DIR -Force
    }
    Remove-Item -Recurse -Force $tempExtract -ErrorAction SilentlyContinue

    Write-Log "Extracted to $MINGW_DIR" -Level Success
} catch {
    Write-Log "Extraction failed: $_" -Level Error
    exit 1
}

# Create symlinks in bin/
Write-Log "Creating symlinks in $NOA_BIN..." -Level Info
$mingwTools = @(
    "gcc.exe",
    "g++.exe",
    "gfortran.exe",
    "ar.exe",
    "as.exe",
    "ld.exe",
    "nm.exe",
    "objdump.exe",
    "ranlib.exe",
    "strip.exe",
    "windres.exe",
    "dlltool.exe",
    "mingw32-make.exe"
)

foreach ($tool in $mingwTools) {
    $target = Join-Path $MINGW_DIR "bin\$tool"
    $link = Join-Path $NOA_BIN $tool
    if (Test-Path $target) {
        Remove-Item $link -Force -ErrorAction SilentlyContinue
        New-Item -ItemType SymbolicLink -Path $link -Target $target -Force | Out-Null
        Write-Log "Linked $tool" -Level Success
    }
}

# Also create 'make.exe' symlink pointing to mingw32-make.exe
$makeLink = Join-Path $NOA_BIN "make.exe"
$mingwMake = Join-Path $MINGW_DIR "bin\mingw32-make.exe"
if (Test-Path $mingwMake) {
    Remove-Item $makeLink -Force -ErrorAction SilentlyContinue
    New-Item -ItemType SymbolicLink -Path $makeLink -Target $mingwMake -Force | Out-Null
    Write-Log "Linked make.exe -> mingw32-make.exe" -Level Success
}

# Verify installation
Write-Log "Verifying installation..." -Level Info
if (Test-Path $gccExe) {
    $installedVersion = & $gccExe --version 2>&1 | Select-Object -First 1
    Write-Log "MinGW-w64 installed successfully: $installedVersion" -Level Success
} else {
    Write-Log "Installation verification failed - gcc.exe not found" -Level Error
    exit 1
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "MinGW-w64 $Version installed successfully!" -ForegroundColor Green
Write-Host "Location: $MINGW_DIR" -ForegroundColor Gray
Write-Host "Symlinks: gcc, g++, mingw32-make, make, etc." -ForegroundColor Gray
Write-Host "" -ForegroundColor Gray
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  gcc --version          # C compiler" -ForegroundColor Gray
Write-Host "  g++ --version          # C++ compiler" -ForegroundColor Gray
Write-Host "  make --version         # Build tool (via mingw32-make)" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Green

exit 0
