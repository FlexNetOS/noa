<#
.SYNOPSIS
    Install Rust quality tools (rustfmt, clippy) via rustup

.DESCRIPTION
    Installs Rust components via rustup to the portable Rust installation.
    Per NOA Constitution 3.1: Self-contained installation.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall even if already installed
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT }
    else { Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))) }
}

$RustRoot = Join-Path $NoaRoot "opt/rust"
$CargoBin = Join-Path $RustRoot "cargo/bin"

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
Write-Host "NOA Rust Tools Installer" -ForegroundColor Cyan
Write-Host "Installing: rustfmt, clippy" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if rustup is available
$rustupBin = Join-Path $CargoBin "rustup.exe"
if (-not (Test-Path $rustupBin)) {
    # Try system rustup
    $rustupBin = Get-Command rustup -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
    if (-not $rustupBin) {
        Write-Log "rustup not found. Please install Rust first." -Level Error
        Write-Log "Run: scripts/bootstrap/installers/rust-portable.ps1" -Level Info
        exit 1
    }
}

# Set environment for portable rust
$env:RUSTUP_HOME = Join-Path $RustRoot "rustup"
$env:CARGO_HOME = Join-Path $RustRoot "cargo"

# Install components
$components = @("rustfmt", "clippy")
foreach ($component in $components) {
    Write-Log "Installing $component..." -Level Info
    # Temporarily allow errors so we can capture rustup's stderr (info messages go there)
    $oldErrorAction = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $output = & $rustupBin component add $component 2>&1 | Out-String
        $exitCode = $LASTEXITCODE
        $ErrorActionPreference = $oldErrorAction

        if ($exitCode -eq 0) {
            # "up to date" is a success condition, not an error
            if ($output -match "up to date") {
                Write-Log "$component is already up to date" -Level Success
            } else {
                Write-Log "Installed: $component" -Level Success
            }
        } else {
            Write-Log "Failed to install ${component}: $output" -Level Warning
        }
    } catch {
        $ErrorActionPreference = $oldErrorAction
        Write-Log "Error installing $component : $_" -Level Warning
    }
}

# Verify installations
Write-Host ""
Write-Log "Verifying installations..." -Level Info

$rustfmtBin = Join-Path $CargoBin "rustfmt.exe"
$clippyBin = Join-Path $CargoBin "cargo-clippy.exe"

if (Test-Path $rustfmtBin) {
    $version = & $rustfmtBin --version 2>&1
    Write-Log "rustfmt: $version" -Level Success
} else {
    Write-Log "rustfmt not found" -Level Warning
}

if (Test-Path $clippyBin) {
    Write-Log "clippy: installed" -Level Success
} else {
    Write-Log "clippy not found" -Level Warning
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "Rust tools installation complete!" -ForegroundColor Green
Write-Host "=" * 60 -ForegroundColor Green

exit 0

