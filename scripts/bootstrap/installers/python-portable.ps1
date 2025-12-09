<#
.SYNOPSIS
    Install Python embeddable to noa_root/opt/python/ with pip support

.DESCRIPTION
    Installs Python embeddable distribution with pip enabled.
    Creates a venv in noa_root/opt/venv/ for package installations.

    Package manager works normally:
    - 'pip install <package>' installs to noa_root/opt/venv/
    - Use the venv activation script before running pip commands

    Per NOA Constitution §3.1: Self-contained but fully functional.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from script location)

.PARAMETER Version
    Python version to install (default: 3.12.8)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\python-portable.ps1
    .\python-portable.ps1 -Version "3.12.8" -Force
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "3.12.8",
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

# Paths - All within noa_root
$PythonRoot = Join-Path $NoaRoot "opt/python"
$VenvPath = Join-Path $NoaRoot "opt/venv"
$TempDir = Join-Path $NoaRoot "tmp"
$StateFile = Join-Path $PythonRoot ".installed.json"

# Version parsing (3.12.8 -> 3128 for URL)
$VersionParts = $Version -split '\.'
$MajorMinor = "$($VersionParts[0])$($VersionParts[1])"
$VersionNoDots = "$($VersionParts[0])$($VersionParts[1])$($VersionParts[2])"

# Download URLs
$EmbedUrl = "https://www.python.org/ftp/python/${Version}/python-${Version}-embed-amd64.zip"
$GetPipUrl = "https://bootstrap.pypa.io/get-pip.py"
$ArchiveName = "python-${Version}-embed-amd64.zip"
$ArchivePath = Join-Path $TempDir $ArchiveName
$GetPipPath = Join-Path $TempDir "get-pip.py"

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        default { "White" }
    }
    $prefix = switch ($Level) {
        "Success" { "[✓]" }
        "Warning" { "[!]" }
        "Error" { "[✗]" }
        default { "[i]" }
    }
    Write-Host "$prefix $Message" -ForegroundColor $color
}

function Test-PythonInstalled {
    if (-not (Test-Path $StateFile)) { return $false }

    $state = Get-Content $StateFile -Raw | ConvertFrom-Json
    if ($state.version -ne $Version) { return $false }

    $pythonBinary = Join-Path $PythonRoot "python.exe"
    return Test-Path $pythonBinary
}

function Install-PortablePython {
    Write-Log "Installing Python $Version to $PythonRoot" -Level Info

    # Create directories
    foreach ($dir in @($PythonRoot, $TempDir)) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Log "Created directory: $dir" -Level Success
        }
    }

    # Download embeddable Python if not cached
    if (-not (Test-Path $ArchivePath)) {
        Write-Log "Downloading Python $Version from $EmbedUrl..." -Level Info
        try {
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $EmbedUrl -OutFile $ArchivePath -UseBasicParsing
            Write-Log "Downloaded: $ArchiveName" -Level Success
        } catch {
            Write-Log "Failed to download Python: $_" -Level Error
            throw
        }
    } else {
        Write-Log "Using cached archive: $ArchivePath" -Level Info
    }

    # Remove existing installation
    if (Test-Path $PythonRoot) {
        Write-Log "Removing existing Python installation..." -Level Info
        Remove-Item -Path $PythonRoot -Recurse -Force
        New-Item -ItemType Directory -Path $PythonRoot -Force | Out-Null
    }

    # Extract archive
    Write-Log "Extracting Python to $PythonRoot..." -Level Info
    try {
        Expand-Archive -Path $ArchivePath -DestinationPath $PythonRoot -Force
        Write-Log "Extracted Python successfully" -Level Success
    } catch {
        Write-Log "Failed to extract Python: $_" -Level Error
        throw
    }

    # Enable pip by modifying python*._pth file
    $PthFile = Get-ChildItem -Path $PythonRoot -Filter "python*._pth" | Select-Object -First 1
    if ($PthFile) {
        $PthContent = Get-Content $PthFile.FullName
        # Uncomment the import site line to enable pip
        $NewContent = $PthContent -replace '#import site', 'import site'
        $NewContent | Set-Content $PthFile.FullName
        Write-Log "Enabled site-packages in $($PthFile.Name)" -Level Success
    }

    # Verify installation
    $PythonBinary = Join-Path $PythonRoot "python.exe"
    if (-not (Test-Path $PythonBinary)) {
        Write-Log "Python binary not found at expected path: $PythonBinary" -Level Error
        throw "Installation failed"
    }

    # Set environment
    $env:PATH = "$PythonRoot;$PythonRoot\Scripts;$env:PATH"

    # Verify version
    $InstalledVersion = & $PythonBinary --version 2>&1
    Write-Log "Installed: $InstalledVersion" -Level Success

    # Download and run get-pip.py
    if (-not (Test-Path $GetPipPath)) {
        Write-Log "Downloading get-pip.py..." -Level Info
        Invoke-WebRequest -Uri $GetPipUrl -OutFile $GetPipPath -UseBasicParsing
    }

    Write-Log "Installing pip..." -Level Info
    & $PythonBinary $GetPipPath --no-warn-script-location 2>&1 | Out-Null
    Write-Log "pip installed" -Level Success

    # Create venv in noa_root
    Write-Log "Creating virtual environment at $VenvPath..." -Level Info
    if (Test-Path $VenvPath) {
        Remove-Item -Path $VenvPath -Recurse -Force
    }
    & $PythonBinary -m venv $VenvPath 2>&1 | Out-Null
    Write-Log "Virtual environment created" -Level Success

    # Save state
    $state = @{
        version = $Version
        installed_at = (Get-Date -Format "o")
        python_root = $PythonRoot
        venv_path = $VenvPath
        note = "Use $VenvPath/Scripts/Activate.ps1 before pip install"
    }
    $state | ConvertTo-Json | Set-Content -Path $StateFile -Encoding UTF8

    Write-Log "Installation state saved to $StateFile" -Level Success
}

function Get-EnvironmentSetup {
    @"

# Add these to your noa-env.ps1 or shell profile:
`$env:PATH = "$PythonRoot;$PythonRoot\Scripts;`$env:PATH"

# Activate virtual environment for package installations:
& "$VenvPath\Scripts\Activate.ps1"

# After activation, 'pip install <package>' will install to:
#   $VenvPath\Lib\site-packages\

"@
}

# Main execution
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable Python Installer" -ForegroundColor Cyan
Write-Host "Constitution §3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host "Target:   $PythonRoot" -ForegroundColor White
Write-Host "VEnv:     $VenvPath" -ForegroundColor White
Write-Host "Version:  $Version" -ForegroundColor White
Write-Host ""

# Check if already installed
if ((Test-PythonInstalled) -and -not $Force) {
    Write-Log "Python $Version is already installed" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info

    Write-Host (Get-EnvironmentSetup)
    exit 0
}

# Install
try {
    Install-PortablePython

    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host "Python $Version installed successfully!" -ForegroundColor Green
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host (Get-EnvironmentSetup)

    exit 0
} catch {
    Write-Log "Installation failed: $_" -Level Error
    exit 1
}

