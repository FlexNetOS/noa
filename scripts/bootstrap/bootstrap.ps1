<#
.SYNOPSIS
    NOA Unified Bootstrap for Windows
    Single entry point for complete NOA environment setup

.DESCRIPTION
    Per NOA Constitution 3.1: Self-contained installation to noa_root

    This script:
    1. Creates directory structure
    2. Checks and installs prerequisites (portable toolchains)
    3. Downloads self-contained utilities
    4. Configures kernel parameters (if applicable)
    5. Generates environment configuration

    Mirrors scripts/bootstrap/bootstrap.sh for cross-platform parity.

.PARAMETER NoaRoot
    NOA root directory (default: N:\noa or auto-detect)

.PARAMETER SkipKernel
    Skip kernel/networking configuration

.PARAMETER SkipServices
    Skip service setup

.PARAMETER Force
    Force reinstall all tools

.EXAMPLE
    .\bootstrap.ps1
    .\bootstrap.ps1 -NoaRoot "C:\noa" -Force
#>

param(
    [string]$NoaRoot,
    [switch]$SkipKernel,
    [switch]$SkipServices,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) {
        $env:NOA_ROOT
    } elseif (Test-Path "N:\noa") {
        "N:\noa"
    } else {
        Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    }
}

$NOA_BIN = Join-Path $NoaRoot "bin"
$NOA_OPT = Join-Path $NoaRoot "opt"
$NOA_LIB = Join-Path $NoaRoot "lib"
$NOA_LOGS = Join-Path $NoaRoot "logs"
$NOA_SCRIPTS = Join-Path $NoaRoot "scripts"

# Detect platform
$Platform = "windows"
$IsWSL = $false
if (Test-Path "/proc/version") {
    $procVersion = Get-Content "/proc/version" -ErrorAction SilentlyContinue
    if ($procVersion -match "microsoft") {
        $IsWSL = $true
        $Platform = if (Test-Path "/run/WSL") { "wsl2" } else { "wsl1" }
    }
}

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

# ============================================
# Banner
# ============================================

Write-Host ""
Write-Host "+============================================================+" -ForegroundColor Cyan
Write-Host "|                                                            |" -ForegroundColor Cyan
Write-Host "|           NOA Bootstrap for Windows                        |" -ForegroundColor Cyan
Write-Host "|           Constitution 3.1 Compliant                       |" -ForegroundColor Cyan
Write-Host "|                                                            |" -ForegroundColor Cyan
Write-Host "+============================================================+" -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host "Platform: $Platform" -ForegroundColor White
Write-Host ""

# ============================================
# Phase 1: Directory Structure
# ============================================

Write-Log "Phase 1: Creating directory structure..." -Level Info

$directories = @(
    "bin", "opt", "lib", "etc", "tmp", "logs",
    "config", "repos", "workspace", "containers",
    "p2p/compute", "p2p/network", "p2p/storage", "p2p/nodes",
    "ai/providers", "ai/devices", "ai/shared/models",
    "sys/kernel/modules", "sys/kernel/params", "sys/namespace",
    "git", "init/run", "init/services"
)

foreach ($dir in $directories) {
    $dirPath = Join-Path $NoaRoot $dir
    if (-not (Test-Path $dirPath)) {
        New-Item -ItemType Directory -Path $dirPath -Force | Out-Null
        Write-Log "  Created: $dir" -Level Success
    }
}

# ============================================
# Phase 2: Check Prerequisites
# ============================================

Write-Log "Phase 2: Checking prerequisites..." -Level Info

$prereqScript = Join-Path $NOA_SCRIPTS "setup/check-prereqs.ps1"
if (Test-Path $prereqScript) {
    & $prereqScript
    $prereqExit = $LASTEXITCODE

    if ($prereqExit -eq 1) {
        Write-Log "Critical prerequisites missing. Running installer..." -Level Warning

        $installScript = Join-Path $NOA_SCRIPTS "setup/install-prereqs.ps1"
        if (Test-Path $installScript) {
            $installArgs = @("-NoaRoot", $NoaRoot)
            if ($Force) { $installArgs += "-Force" }
            & $installScript @installArgs
        }
    }
} else {
    Write-Log "Prereqs script not found: $prereqScript" -Level Warning
}

# ============================================
# Phase 3: Download Static Binaries
# ============================================

Write-Log "Phase 3: Downloading self-contained utilities..." -Level Info

$staticScript = Join-Path $NOA_SCRIPTS "download-static-binaries.ps1"
if (Test-Path $staticScript) {
    & $staticScript -NoaRoot $NoaRoot
} else {
    Write-Log "Static binaries script not found (skipping)" -Level Warning
}

# ============================================
# Phase 4: Bundle Libraries
# ============================================

Write-Log "Phase 4: Bundling shared libraries..." -Level Info

$bundleScript = Join-Path $NOA_SCRIPTS "bundle-all-libs.ps1"
if (Test-Path $bundleScript) {
    & $bundleScript -NoaRoot $NoaRoot
} else {
    Write-Log "Bundle script not found (skipping)" -Level Warning
}

# ============================================
# Phase 5: Kernel/Network Setup
# ============================================

if (-not $SkipKernel) {
    Write-Log "Phase 5: Kernel/Network setup..." -Level Info

    $kmodScript = Join-Path $NOA_SCRIPTS "noa-kmod.ps1"
    if (Test-Path $kmodScript) {
        & $kmodScript -Action check -NoaRoot $NoaRoot
    }

    # Check if running as admin for kernel params
    $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

    if ($isAdmin) {
        $kparamScript = Join-Path $NOA_SCRIPTS "noa-kernel-params.ps1"
        if (Test-Path $kparamScript) {
            Write-Log "  Setting kernel parameters..." -Level Info
            & $kparamScript -Action set -Param "ip_forward" -Value "1" -NoaRoot $NoaRoot
            & $kparamScript -Action set -Param "p2p_firewall" -Value "1" -NoaRoot $NoaRoot
        }
    } else {
        Write-Log "  Skipping kernel params (not Administrator)" -Level Warning
        Write-Log "  Run as Administrator for full kernel setup" -Level Warning
    }
} else {
    Write-Log "Phase 5: Kernel setup SKIPPED" -Level Info
}

# ============================================
# Phase 6: Generate Environment File
# ============================================

Write-Log "Phase 6: Generating environment configuration..." -Level Info

$envPath = Join-Path $NoaRoot "noa-env.ps1"

$envContent = @"
# NOA Environment Configuration
# Auto-generated by bootstrap.ps1
# Last Updated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

# NOA Root Directory
`$env:NOA_ROOT = "$NoaRoot"
`$env:NOA_BIN = "$NOA_BIN"
`$env:NOA_OPT = "$NOA_OPT"
`$env:NOA_LIB = "$NOA_LIB"
`$env:NOA_CONFIG = "$(Join-Path $NoaRoot 'config')"
`$env:NOA_LOGS = "$NOA_LOGS"
`$env:NOA_TMP = "$(Join-Path $NoaRoot 'tmp')"

"@

# Add portable toolchain paths if they exist
if (Test-Path (Join-Path $NOA_OPT "rust/cargo/bin")) {
    $envContent += @"

# Rust (portable)
`$env:RUSTUP_HOME = "$(Join-Path $NOA_OPT 'rust/rustup')"
`$env:CARGO_HOME = "$(Join-Path $NOA_OPT 'rust/cargo')"

"@
}

if (Test-Path (Join-Path $NOA_OPT "go/bin")) {
    $envContent += @"

# Go (portable)
`$env:GOROOT = "$(Join-Path $NOA_OPT 'go')"
`$env:GOPATH = "$(Join-Path $NOA_OPT 'go/workspace')"
`$env:GOBIN = "$(Join-Path $NOA_OPT 'go/workspace/bin')"
`$env:GOCACHE = "$(Join-Path $NOA_OPT 'go/cache')"
`$env:GOMODCACHE = "$(Join-Path $NOA_OPT 'go/pkg/mod')"

"@
}

if (Test-Path (Join-Path $NOA_OPT "node")) {
    $envContent += @"

# Node.js (portable)
`$env:npm_config_prefix = "$(Join-Path $NOA_OPT 'node')"
`$env:npm_config_cache = "$(Join-Path $NOA_OPT 'npm-cache')"

"@
}

# Build PATH
$pathAdditions = @($NOA_BIN)
if (Test-Path (Join-Path $NOA_OPT "rust/cargo/bin")) { $pathAdditions += Join-Path $NOA_OPT "rust/cargo/bin" }
if (Test-Path (Join-Path $NOA_OPT "go/bin")) { $pathAdditions += Join-Path $NOA_OPT "go/bin"; $pathAdditions += Join-Path $NOA_OPT "go/workspace/bin" }
if (Test-Path (Join-Path $NOA_OPT "node")) { $pathAdditions += Join-Path $NOA_OPT "node" }
if (Test-Path (Join-Path $NOA_OPT "python")) { $pathAdditions += Join-Path $NOA_OPT "python" }
if (Test-Path (Join-Path $NOA_OPT "protobuf/bin")) { $pathAdditions += Join-Path $NOA_OPT "protobuf/bin" }

$pathString = ($pathAdditions -join ";")

$envContent += @"

# PATH Configuration
`$env:PATH = "$pathString;`$env:PATH"

# Helper Functions
function cda { Set-Location `$env:NOA_ROOT }
function cdopt { Set-Location `$env:NOA_OPT }
function cdbin { Set-Location `$env:NOA_BIN }

# Toolchain status
function Get-NoaToolchains {
    Write-Host "NOA Portable Toolchains:" -ForegroundColor Cyan
    @(
        @{ Name = "Rust"; Path = "`$env:NOA_OPT/rust/cargo/bin/rustc.exe" },
        @{ Name = "Go"; Path = "`$env:NOA_OPT/go/bin/go.exe" },
        @{ Name = "Node"; Path = "`$env:NOA_OPT/node/node.exe" },
        @{ Name = "Python"; Path = "`$env:NOA_OPT/python/python.exe" },
        @{ Name = "protoc"; Path = "`$env:NOA_OPT/protobuf/bin/protoc.exe" }
    ) | ForEach-Object {
        `$exists = Test-Path `$_.Path
        `$status = if (`$exists) { "[OK]" } else { "[--]" }
        `$color = if (`$exists) { "Green" } else { "Yellow" }
        Write-Host "  `$status `$(`$_.Name)" -ForegroundColor `$color
    }
}

Write-Host "NOA environment loaded: `$env:NOA_ROOT" -ForegroundColor Green
"@

$envContent | Set-Content -Path $envPath -Encoding UTF8
Write-Log "Created: noa-env.ps1" -Level Success

# ============================================
# Phase 7: Create Marker File
# ============================================

Write-Log "Phase 7: Creating marker file..." -Level Info

$markerPath = Join-Path $NoaRoot ".noa"
@"
# NOA Root Directory Marker
# Created: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
version=2.0.0
platform=$Platform
root=$NoaRoot
"@ | Set-Content -Path $markerPath -Encoding UTF8

Write-Log "Created: .noa" -Level Success

# ============================================
# Summary
# ============================================

Write-Host ""
Write-Host "+============================================================+" -ForegroundColor Green
Write-Host "|                                                            |" -ForegroundColor Green
Write-Host "|              Bootstrap Completed Successfully!             |" -ForegroundColor Green
Write-Host "|                                                            |" -ForegroundColor Green
Write-Host "+============================================================+" -ForegroundColor Green
Write-Host ""

Write-Log "Summary:" -Level Info
Write-Host "  NOA_ROOT:    $NoaRoot" -ForegroundColor White
Write-Host "  Platform:    $Platform" -ForegroundColor White
Write-Host "  Env file:    $envPath" -ForegroundColor White
Write-Host ""

Write-Log "Next steps:" -Level Info
Write-Host "  1. Load environment: " -NoNewline -ForegroundColor White
Write-Host ('. "{0}"' -f $envPath) -ForegroundColor Cyan
Write-Host "  2. Verify prereqs:   " -NoNewline -ForegroundColor White
Write-Host 'scripts\setup\check-prereqs.ps1' -ForegroundColor Cyan
Write-Host "  3. Check toolchains: " -NoNewline -ForegroundColor White
Write-Host "Get-NoaToolchains" -ForegroundColor Cyan
Write-Host ""

Write-Log "To auto-load NOA, add to your PowerShell profile:" -Level Info
Write-Host ('  . "{0}"' -f $envPath) -ForegroundColor Cyan
Write-Host ""

exit 0

