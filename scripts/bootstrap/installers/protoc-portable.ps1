<#
.SYNOPSIS
    Install protoc (Protocol Buffers compiler) to noa_root/opt/protobuf/

.DESCRIPTION
    Installs the Protocol Buffers compiler (protoc) as a self-contained binary.
    Per NOA Constitution §3.1: Self-contained but fully functional.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from script location)

.PARAMETER Version
    protoc version to install (default: 28.3)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\protoc-portable.ps1
    .\protoc-portable.ps1 -Version "28.3" -Force
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "28.3",
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
$ProtobufRoot = Join-Path $NoaRoot "opt/protobuf"
$ProtobufBin = Join-Path $ProtobufRoot "bin"
$ProtobufInclude = Join-Path $ProtobufRoot "include"
$TempDir = Join-Path $NoaRoot "tmp"
$StateFile = Join-Path $ProtobufRoot ".installed.json"

# Download URL (GitHub releases)
$DownloadUrl = "https://github.com/protocolbuffers/protobuf/releases/download/v${Version}/protoc-${Version}-win64.zip"
$ArchiveName = "protoc-${Version}-win64.zip"
$ArchivePath = Join-Path $TempDir $ArchiveName

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

function Test-ProtocInstalled {
    if (-not (Test-Path $StateFile)) { return $false }

    $state = Get-Content $StateFile -Raw | ConvertFrom-Json
    if ($state.version -ne $Version) { return $false }

    $protocBinary = Join-Path $ProtobufBin "protoc.exe"
    return Test-Path $protocBinary
}

function Install-PortableProtoc {
    Write-Log "Installing protoc $Version to $ProtobufRoot" -Level Info

    # Create directories
    foreach ($dir in @($ProtobufRoot, $ProtobufBin, $TempDir)) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Log "Created directory: $dir" -Level Success
        }
    }

    # Download if not cached
    if (-not (Test-Path $ArchivePath)) {
        Write-Log "Downloading protoc $Version from $DownloadUrl..." -Level Info
        try {
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath -UseBasicParsing
            Write-Log "Downloaded: $ArchiveName" -Level Success
        } catch {
            Write-Log "Failed to download protoc: $_" -Level Error
            throw
        }
    } else {
        Write-Log "Using cached archive: $ArchivePath" -Level Info
    }

    # Remove existing installation
    if (Test-Path $ProtobufRoot) {
        Write-Log "Removing existing protoc installation..." -Level Info
        Remove-Item -Path $ProtobufRoot -Recurse -Force
        New-Item -ItemType Directory -Path $ProtobufRoot -Force | Out-Null
    }

    # Extract archive
    Write-Log "Extracting protoc to $ProtobufRoot..." -Level Info
    try {
        Expand-Archive -Path $ArchivePath -DestinationPath $ProtobufRoot -Force
        Write-Log "Extracted protoc successfully" -Level Success
    } catch {
        Write-Log "Failed to extract protoc: $_" -Level Error
        throw
    }

    # Verify installation
    $ProtocBinary = Join-Path $ProtobufBin "protoc.exe"
    if (-not (Test-Path $ProtocBinary)) {
        Write-Log "protoc binary not found at expected path: $ProtocBinary" -Level Error
        throw "Installation failed"
    }

    # Set environment
    $env:PATH = "$ProtobufBin;$env:PATH"

    # Verify version
    $InstalledVersion = & $ProtocBinary --version 2>&1
    Write-Log "Installed: $InstalledVersion" -Level Success

    # Save state
    $state = @{
        version = $Version
        installed_at = (Get-Date -Format "o")
        protobuf_root = $ProtobufRoot
        protobuf_bin = $ProtobufBin
        protobuf_include = $ProtobufInclude
    }
    $state | ConvertTo-Json | Set-Content -Path $StateFile -Encoding UTF8

    Write-Log "Installation state saved to $StateFile" -Level Success
}

function Get-EnvironmentSetup {
    @"

# Add these to your noa-env.ps1 or shell profile:
`$env:PATH = "$ProtobufBin;`$env:PATH"

# Include files are at:
#   $ProtobufInclude

"@
}

# Main execution
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable protoc Installer" -ForegroundColor Cyan
Write-Host "Constitution §3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host "Target:   $ProtobufRoot" -ForegroundColor White
Write-Host "Version:  $Version" -ForegroundColor White
Write-Host ""

# Check if already installed
if ((Test-ProtocInstalled) -and -not $Force) {
    Write-Log "protoc $Version is already installed" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info

    Write-Host (Get-EnvironmentSetup)
    exit 0
}

# Install
try {
    Install-PortableProtoc

    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host "protoc $Version installed successfully!" -ForegroundColor Green
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host (Get-EnvironmentSetup)

    exit 0
} catch {
    Write-Log "Installation failed: $_" -Level Error
    exit 1
}

