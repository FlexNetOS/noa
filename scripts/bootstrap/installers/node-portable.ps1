<#
.SYNOPSIS
    Install FULL Node.js to noa_root/opt/node/

.DESCRIPTION
    Installs a complete, fully-functional Node.js with npm.
    The toolchain works exactly like a system-wide installation, but everything lives in noa_root.

    Package manager works normally:
    - 'npm install -g <package>' installs to noa_root/opt/node/bin/
    - npm cache is stored in noa_root/opt/npm-cache/

    Per NOA Constitution 3.1: Self-contained but fully functional.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from script location)

.PARAMETER Version
    Node.js version to install (default: 20.18.1)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\node-portable.ps1
    .\node-portable.ps1 -Version "20.18.1" -Force
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "20.18.1",
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
$NodeRoot = Join-Path $NoaRoot "opt/node"
$NpmCache = Join-Path $NoaRoot "opt/npm-cache"
$NpmPrefix = $NodeRoot  # Global installs go here
$TempDir = Join-Path $NoaRoot "tmp"
$StateFile = Join-Path $NodeRoot ".installed.json"

# Platform detection
$Platform = "win"
$Arch = "x64"

# Download URL (Node provides pre-built binaries)
$DownloadUrl = "https://nodejs.org/dist/v${Version}/node-v${Version}-${Platform}-${Arch}.zip"
$ArchiveName = "node-v${Version}-${Platform}-${Arch}.zip"
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
        "Success" { "[OK]" }
        "Warning" { "[!!]" }
        "Error" { "[XX]" }
        default { "[i]" }
    }
    Write-Host "$prefix $Message" -ForegroundColor $color
}

function Test-NodeInstalled {
    if (-not (Test-Path $StateFile)) { return $false }

    $state = Get-Content $StateFile -Raw | ConvertFrom-Json
    if ($state.version -ne $Version) { return $false }

    $nodeBinary = Join-Path $NodeRoot "node.exe"
    return Test-Path $nodeBinary
}

function Install-PortableNode {
    Write-Log "Installing Node.js $Version to $NodeRoot" -Level Info

    # Create directories
    foreach ($dir in @($NodeRoot, $NpmCache, $TempDir)) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Log "Created directory: $dir" -Level Success
        }
    }

    # Download if not cached
    if (-not (Test-Path $ArchivePath)) {
        Write-Log "Downloading Node.js $Version from $DownloadUrl..." -Level Info
        try {
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath -UseBasicParsing
            Write-Log "Downloaded: $ArchiveName" -Level Success
        } catch {
            Write-Log "Failed to download Node.js: $_" -Level Error
            throw
        }
    } else {
        Write-Log "Using cached archive: $ArchivePath" -Level Info
    }

    # Remove existing installation
    if (Test-Path $NodeRoot) {
        Write-Log "Removing existing Node.js installation..." -Level Info
        Remove-Item -Path $NodeRoot -Recurse -Force
        New-Item -ItemType Directory -Path $NodeRoot -Force | Out-Null
    }

    # Extract archive
    Write-Log "Extracting Node.js to $NodeRoot..." -Level Info
    try {
        # Node archive extracts to 'node-vX.X.X-win-x64/' folder
        $ExtractTemp = Join-Path $TempDir "node-extract"
        if (Test-Path $ExtractTemp) { Remove-Item $ExtractTemp -Recurse -Force }

        Expand-Archive -Path $ArchivePath -DestinationPath $ExtractTemp -Force

        # Find extracted folder and move contents
        $ExtractedNode = Get-ChildItem -Path $ExtractTemp -Directory | Select-Object -First 1
        if ($ExtractedNode) {
            Get-ChildItem -Path $ExtractedNode.FullName | Move-Item -Destination $NodeRoot -Force
        }

        # Cleanup
        Remove-Item $ExtractTemp -Recurse -Force

        Write-Log "Extracted Node.js successfully" -Level Success
    } catch {
        Write-Log "Failed to extract Node.js: $_" -Level Error
        throw
    }

    # Verify installation
    $NodeBinary = Join-Path $NodeRoot "node.exe"
    if (-not (Test-Path $NodeBinary)) {
        Write-Log "Node binary not found at expected path: $NodeBinary" -Level Error
        throw "Installation failed"
    }

    # Set environment for verification
    $env:PATH = "$NodeRoot;$env:PATH"
    $env:npm_config_prefix = $NpmPrefix
    $env:npm_config_cache = $NpmCache

    # Verify version
    $InstalledVersion = & $NodeBinary --version 2>&1
    Write-Log "Installed: Node.js $InstalledVersion" -Level Success

    # Configure npm to use noa_root directories
    $NpmBinary = Join-Path $NodeRoot "npm.cmd"
    if (Test-Path $NpmBinary) {
        & $NpmBinary config set prefix $NpmPrefix 2>$null
        & $NpmBinary config set cache $NpmCache 2>$null
        Write-Log "Configured npm prefix: $NpmPrefix" -Level Success
        Write-Log "Configured npm cache: $NpmCache" -Level Success
    }

    # Save state
    $state = @{
        version = $Version
        installed_at = (Get-Date -Format "o")
        node_root = $NodeRoot
        npm_prefix = $NpmPrefix
        npm_cache = $NpmCache
        note = "npm -g installs to $NodeRoot"
    }
    $state | ConvertTo-Json | Set-Content -Path $StateFile -Encoding UTF8

    Write-Log "Installation state saved to $StateFile" -Level Success
}

function Get-EnvironmentSetup {
    @"

# Add these to your noa-env.ps1 or shell profile:
`$env:PATH = "$NodeRoot;`$env:PATH"
`$env:npm_config_prefix = "$NpmPrefix"
`$env:npm_config_cache = "$NpmCache"

# After this, 'npm install -g <package>' will install to:
#   $NodeRoot (e.g., N:\noa\opt\node\)

"@
}

# Main execution
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable Node.js Installer" -ForegroundColor Cyan
Write-Host "Constitution 3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host "Target:   $NodeRoot" -ForegroundColor White
Write-Host "Version:  $Version" -ForegroundColor White
Write-Host ""

# Check if already installed
if ((Test-NodeInstalled) -and -not $Force) {
    Write-Log "Node.js $Version is already installed" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info

    Write-Host (Get-EnvironmentSetup)
    exit 0
}

# Install
try {
    Install-PortableNode

    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host "Node.js $Version installed successfully!" -ForegroundColor Green
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host (Get-EnvironmentSetup)

    exit 0
} catch {
    Write-Log "Installation failed: $_" -Level Error
    exit 1
}

