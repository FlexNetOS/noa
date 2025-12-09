<#
.SYNOPSIS
    Install FULL Rust toolchain to noa_root/opt/rust/

.DESCRIPTION
    Installs a complete, fully-functional Rust toolchain with rustup, cargo, rustc, etc.
    The toolchain works exactly like a system-wide installation, but everything lives in noa_root.

    Package managers work normally:
    - 'cargo install <crate>' installs to noa_root/opt/rust/cargo/bin/
    - 'rustup component add <component>' installs to noa_root

    This is NOT a static binary download - it's a real, working Rust installation.
    Per NOA Constitution §3.1: Self-contained but fully functional.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from script location)

.PARAMETER Toolchain
    Rust toolchain to install (default: stable)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\rust-portable.ps1
    .\rust-portable.ps1 -Toolchain "stable" -Force

    # After installation, cargo works normally:
    # cargo install ripgrep  --> installs to N:\noa\opt\rust\cargo\bin\rg.exe
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$Toolchain = "stable",
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

# Paths - ALL within noa_root
$RustRoot = Join-Path $NoaRoot "opt/rust"
$RustupHome = Join-Path $RustRoot "rustup"
$CargoHome = Join-Path $RustRoot "cargo"
$CargoBin = Join-Path $CargoHome "bin"
$TempDir = Join-Path $NoaRoot "tmp"
$StateFile = Join-Path $RustRoot ".installed.json"

# Download URL for rustup-init
$RustupInitUrl = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
$RustupInitPath = Join-Path $TempDir "rustup-init.exe"

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

function Test-RustInstalled {
    if (-not (Test-Path $StateFile)) { return $false }

    $rustcBinary = Join-Path $CargoBin "rustc.exe"
    return Test-Path $rustcBinary
}

function Get-InstalledRustVersion {
    $rustcBinary = Join-Path $CargoBin "rustc.exe"
    if (Test-Path $rustcBinary) {
        $env:RUSTUP_HOME = $RustupHome
        $env:CARGO_HOME = $CargoHome
        $version = & $rustcBinary --version 2>&1
        return $version
    }
    return $null
}

function Install-PortableRust {
    Write-Log "Installing Rust ($Toolchain) to $RustRoot" -Level Info
    Write-Log "RUSTUP_HOME: $RustupHome" -Level Info
    Write-Log "CARGO_HOME:  $CargoHome" -Level Info

    # Create directories
    foreach ($dir in @($RustRoot, $RustupHome, $CargoHome, $CargoBin, $TempDir)) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Log "Created directory: $dir" -Level Success
        }
    }

    # Download rustup-init if not cached
    if (-not (Test-Path $RustupInitPath)) {
        Write-Log "Downloading rustup-init from $RustupInitUrl..." -Level Info
        try {
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $RustupInitUrl -OutFile $RustupInitPath -UseBasicParsing
            Write-Log "Downloaded: rustup-init.exe" -Level Success
        } catch {
            Write-Log "Failed to download rustup-init: $_" -Level Error
            throw
        }
    } else {
        Write-Log "Using cached rustup-init: $RustupInitPath" -Level Info
    }

    # Set environment BEFORE running rustup-init
    # This is critical - rustup-init reads these to determine install location
    $env:RUSTUP_HOME = $RustupHome
    $env:CARGO_HOME = $CargoHome

    # Remove existing installation if forcing
    if ($Force -and (Test-Path $RustupHome)) {
        Write-Log "Removing existing Rust installation..." -Level Info
        Remove-Item -Path $RustupHome -Recurse -Force -ErrorAction SilentlyContinue
    }
    if ($Force -and (Test-Path $CargoHome)) {
        Remove-Item -Path $CargoHome -Recurse -Force -ErrorAction SilentlyContinue
    }

    # Run rustup-init
    Write-Log "Running rustup-init (this may take a few minutes)..." -Level Info

    $rustupArgs = @(
        "-y",                       # Non-interactive
        "--default-toolchain", $Toolchain,
        "--no-modify-path"          # Don't touch system PATH
    )

    try {
        $process = Start-Process -FilePath $RustupInitPath -ArgumentList $rustupArgs -Wait -PassThru -NoNewWindow
        if ($process.ExitCode -ne 0) {
            Write-Log "rustup-init exited with code $($process.ExitCode)" -Level Error
            throw "rustup-init failed"
        }
        Write-Log "Rust toolchain installed" -Level Success
    } catch {
        Write-Log "Failed to run rustup-init: $_" -Level Error
        throw
    }

    # Verify installation
    $RustcBinary = Join-Path $CargoBin "rustc.exe"
    $CargoBinary = Join-Path $CargoBin "cargo.exe"
    $RustupBinary = Join-Path $CargoBin "rustup.exe"

    if (-not (Test-Path $RustcBinary)) {
        Write-Log "rustc not found at expected path: $RustcBinary" -Level Error
        throw "Installation failed - rustc not found"
    }

    if (-not (Test-Path $CargoBinary)) {
        Write-Log "cargo not found at expected path: $CargoBinary" -Level Error
        throw "Installation failed - cargo not found"
    }

    # Get installed version
    $InstalledVersion = & $RustcBinary --version 2>&1
    Write-Log "Installed: $InstalledVersion" -Level Success

    # Install additional components
    Write-Log "Installing rustfmt component..." -Level Info
    & $RustupBinary component add rustfmt 2>&1 | Out-Null
    Write-Log "Installed: rustfmt" -Level Success

    Write-Log "Installing clippy component..." -Level Info
    & $RustupBinary component add clippy 2>&1 | Out-Null
    Write-Log "Installed: clippy" -Level Success

    # Save state
    $state = @{
        toolchain = $Toolchain
        installed_at = (Get-Date -Format "o")
        rustup_home = $RustupHome
        cargo_home = $CargoHome
        version = $InstalledVersion
        components = @("rustfmt", "clippy")
    }
    $state | ConvertTo-Json | Set-Content -Path $StateFile -Encoding UTF8

    Write-Log "Installation state saved to $StateFile" -Level Success
}

function Get-EnvironmentSetup {
    @"

# Add these to your noa-env.ps1 or shell profile:
`$env:RUSTUP_HOME = "$RustupHome"
`$env:CARGO_HOME = "$CargoHome"
`$env:PATH = "$CargoBin;`$env:PATH"

"@
}

# Main execution
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable Rust Installer" -ForegroundColor Cyan
Write-Host "Constitution §3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT:    $NoaRoot" -ForegroundColor White
Write-Host "RUSTUP_HOME: $RustupHome" -ForegroundColor White
Write-Host "CARGO_HOME:  $CargoHome" -ForegroundColor White
Write-Host "Toolchain:   $Toolchain" -ForegroundColor White
Write-Host ""

# Check if already installed
if ((Test-RustInstalled) -and -not $Force) {
    $version = Get-InstalledRustVersion
    Write-Log "Rust is already installed: $version" -Level Success
    Write-Log "Use -Force to reinstall" -Level Info

    Write-Host (Get-EnvironmentSetup)
    exit 0
}

# Install
try {
    Install-PortableRust

    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host "Rust toolchain installed successfully!" -ForegroundColor Green
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host (Get-EnvironmentSetup)

    exit 0
} catch {
    Write-Log "Installation failed: $_" -Level Error
    exit 1
}

