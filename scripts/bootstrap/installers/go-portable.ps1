<#
.SYNOPSIS
    Install FULL Go toolchain to noa_root/opt/go/

.DESCRIPTION
    Installs a complete, fully-functional Go toolchain with go, gofmt, etc.
    The toolchain works exactly like a system-wide installation, but everything lives in noa_root.

    Package manager works normally:
    - 'go install github.com/...' installs to noa_root/opt/go/workspace/bin/
    - 'go mod download' caches modules to noa_root/opt/go/pkg/mod/

    This is NOT a static binary download - it's a real, working Go installation.
    Per NOA Constitution 3.1: Self-contained but fully functional.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from script location)

.PARAMETER Version
    Go version to install (default: 1.23.4)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\go-portable.ps1
    .\go-portable.ps1 -Version "1.23.4" -Force

    # After installation, go install works normally:
    # go install golang.org/x/tools/gopls@latest  --> installs to N:\noa\opt\go\workspace\bin\gopls.exe
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Version = "1.23.4",
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
$GoRoot = Join-Path $NoaRoot "opt/go"
$GoPath = Join-Path $NoaRoot "opt/go/workspace"
$GoBin = Join-Path $GoPath "bin"
$GoCache = Join-Path $NoaRoot "opt/go/cache"
$GoModCache = Join-Path $NoaRoot "opt/go/pkg/mod"
$TempDir = Join-Path $NoaRoot "tmp"
$StateFile = Join-Path $GoRoot ".installed.json"

# Platform detection
$Platform = "windows"
$Arch = "amd64"
$Extension = "zip"

# Download URL
$DownloadUrl = "https://go.dev/dl/go${Version}.${Platform}-${Arch}.${Extension}"
$ArchiveName = "go${Version}.${Platform}-${Arch}.${Extension}"
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

function Test-GoInstalled {
    if (-not (Test-Path $StateFile)) { return $false }

    $state = Get-Content $StateFile -Raw | ConvertFrom-Json
    if ($state.version -ne $Version) { return $false }

    $goBinary = Join-Path $GoRoot "bin/go.exe"
    return Test-Path $goBinary
}

function Install-PortableGo {
    Write-Log "Installing Go $Version to $GoRoot" -Level Info

    # Create directories
    foreach ($dir in @($GoRoot, $GoPath, $GoBin, $GoCache, $TempDir)) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Log "Created directory: $dir" -Level Success
        }
    }

    # Download if not cached
    if (-not (Test-Path $ArchivePath)) {
        Write-Log "Downloading Go $Version from $DownloadUrl..." -Level Info
        try {
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath -UseBasicParsing
            Write-Log "Downloaded: $ArchiveName" -Level Success
        } catch {
            Write-Log "Failed to download Go: $_" -Level Error
            throw
        }
    } else {
        Write-Log "Using cached archive: $ArchivePath" -Level Info
    }

    # Remove existing installation
    if (Test-Path $GoRoot) {
        Write-Log "Removing existing Go installation..." -Level Info
        Remove-Item -Path $GoRoot -Recurse -Force
        New-Item -ItemType Directory -Path $GoRoot -Force | Out-Null
    }

    # Extract archive
    Write-Log "Extracting Go to $GoRoot..." -Level Info
    try {
        # Go archive extracts to 'go/' folder, we need to extract to parent and rename
        $ExtractTemp = Join-Path $TempDir "go-extract"
        if (Test-Path $ExtractTemp) { Remove-Item $ExtractTemp -Recurse -Force }

        Expand-Archive -Path $ArchivePath -DestinationPath $ExtractTemp -Force

        # Move contents from go/ to GoRoot
        $ExtractedGo = Join-Path $ExtractTemp "go"
        if (Test-Path $ExtractedGo) {
            Get-ChildItem -Path $ExtractedGo | Move-Item -Destination $GoRoot -Force
        }

        # Cleanup
        Remove-Item $ExtractTemp -Recurse -Force

        Write-Log "Extracted Go successfully" -Level Success
    } catch {
        Write-Log "Failed to extract Go: $_" -Level Error
        throw
    }

    # Recreate workspace directories (these persist across Go versions)
    foreach ($dir in @($GoPath, $GoBin, $GoCache, $GoModCache)) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
        }
    }

    # Verify installation
    $GoBinary = Join-Path $GoRoot "bin/go.exe"
    if (-not (Test-Path $GoBinary)) {
        Write-Log "Go binary not found at expected path: $GoBinary" -Level Error
        throw "Installation failed"
    }

    # Set environment for verification - all pointing to noa_root
    $env:GOROOT = $GoRoot
    $env:GOPATH = $GoPath
    $env:GOBIN = $GoBin
    $env:GOCACHE = $GoCache
    $env:GOMODCACHE = $GoModCache
    $env:PATH = "$GoRoot\bin;$GoBin;$env:PATH"

    # Verify version
    $InstalledVersion = & $GoBinary version 2>&1
    Write-Log "Installed: $InstalledVersion" -Level Success

    # Save state
    $state = @{
        version = $Version
        installed_at = (Get-Date -Format "o")
        goroot = $GoRoot
        gopath = $GoPath
        gobin = $GoBin
        gocache = $GoCache
        gomodcache = $GoModCache
        note = "go install packages will be installed to $GoBin"
    }
    $state | ConvertTo-Json | Set-Content -Path $StateFile -Encoding UTF8

    Write-Log "Installation state saved to $StateFile" -Level Success
}

function Get-EnvironmentSetup {
    @"

# Add these to your noa-env.ps1 or shell profile:
`$env:GOROOT = "$GoRoot"
`$env:GOPATH = "$GoPath"
`$env:GOBIN = "$GoBin"
`$env:GOCACHE = "$GoCache"
`$env:GOMODCACHE = "$GoModCache"
`$env:PATH = "$GoRoot\bin;$GoBin;`$env:PATH"

# After this, 'go install github.com/...' will install to:
#   $GoBin (e.g., N:\noa\opt\go\workspace\bin\)

"@
}

# Main execution
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable Go Installer" -ForegroundColor Cyan
Write-Host "Constitution 3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host "Target:   $GoRoot" -ForegroundColor White
Write-Host "Version:  $Version" -ForegroundColor White
Write-Host ""

# Check if already installed
if ((Test-GoInstalled) -and -not $Force) {
    Write-Log "Go $Version is already installed" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info

    # Still output environment setup
    Write-Host (Get-EnvironmentSetup)
    exit 0
}

# Install
try {
    Install-PortableGo

    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host "Go $Version installed successfully!" -ForegroundColor Green
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host (Get-EnvironmentSetup)

    exit 0
} catch {
    Write-Log "Installation failed: $_" -Level Error
    exit 1
}

