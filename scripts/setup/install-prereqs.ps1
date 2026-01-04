<#
.SYNOPSIS
    NOA Portable Prerequisites Installer

.DESCRIPTION
    Installs all missing prerequisites using portable installers.
    This script:
    1. Runs check-prereqs.ps1 to identify missing tools
    2. Invokes the appropriate portable installer for each missing tool
    3. Updates noa-env.ps1 with portable toolchain paths
    4. Re-verifies installation

    All tools are installed to noa_root/opt/ per Constitution §3.1 (Self-Contained).

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER All
    Install all portable toolchains, even if system versions exist

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\install-prereqs.ps1
    .\install-prereqs.ps1 -All -Force
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$All,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) {
        $env:NOA_ROOT
    } else {
        Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    }
}

$NOA_OPT = Join-Path $NoaRoot "opt"
$InstallerDir = Join-Path $NoaRoot "scripts/bootstrap/installers"

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

# Define all portable installers
$PortableInstallers = [ordered]@{
    "Rust"   = @{
        Script     = "rust-portable.ps1"
        CheckPath  = "rust/cargo/bin/rustc.exe"
        Priority   = 1
    }
    "Go"     = @{
        Script     = "go-portable.ps1"
        CheckPath  = "go/bin/go.exe"
        Priority   = 2
    }
    "Node"   = @{
        Script     = "node-portable.ps1"
        CheckPath  = "node/node.exe"
        Priority   = 3
    }
    "Python" = @{
        Script     = "python-portable.ps1"
        CheckPath  = "python/python.exe"
        Priority   = 4
    }
    "protoc" = @{
        Script     = "protoc-portable.ps1"
        CheckPath  = "protobuf/bin/protoc.exe"
        Priority   = 5
    }
}

function Test-PortableInstalled {
    param([string]$CheckPath)
    $fullPath = Join-Path $NOA_OPT $CheckPath
    return Test-Path $fullPath
}

function Get-MissingToolchains {
    $missing = @()
    foreach ($name in $PortableInstallers.Keys) {
        $info = $PortableInstallers[$name]
        if (-not (Test-PortableInstalled $info.CheckPath)) {
            $missing += $name
        }
    }
    return $missing
}

function Install-PortableToolchain {
    param(
        [string]$Name,
        [hashtable]$Info,
        [switch]$Force
    )

    $scriptPath = Join-Path $InstallerDir $Info.Script

    if (-not (Test-Path $scriptPath)) {
        Write-Log "Installer not found: $scriptPath" -Level Error
        return $false
    }

    Write-Log "Installing $Name to $NOA_OPT..." -Level Info

    $args = @("-NoaRoot", $NoaRoot)
    if ($Force) { $args += "-Force" }

    try {
        & $scriptPath @args
        if ($LASTEXITCODE -eq 0) {
            Write-Log "$Name installed successfully" -Level Success
            return $true
        } else {
            Write-Log "$Name installer exited with code $LASTEXITCODE" -Level Warning
            return $false
        }
    } catch {
        Write-Log "Failed to install $Name : $_" -Level Error
        return $false
    }
}

function Update-NoaEnv {
    <#
    .SYNOPSIS
        Updates noa-env.ps1 with all portable toolchain environment variables
    #>

    $envPath = Join-Path $NoaRoot "noa-env.ps1"

    Write-Log "Generating noa-env.ps1 with portable toolchain configsuration..." -Level Info

    $pathAdditions = @()
    $envVars = @()

    # Rust
    $rustCargoHome = Join-Path $NOA_OPT "rust/cargo"
    $rustupHome = Join-Path $NOA_OPT "rust/rustup"
    if (Test-Path (Join-Path $rustCargoHome "bin/rustc.exe")) {
        $envVars += "`$env:RUSTUP_HOME = `"$rustupHome`""
        $envVars += "`$env:CARGO_HOME = `"$rustCargoHome`""
        $pathAdditions += "$rustCargoHome\bin"
    }

    # Go
    $goRoot = Join-Path $NOA_OPT "go"
    $goPath = Join-Path $NOA_OPT "go/workspace"
    $goBin = Join-Path $goPath "bin"
    $goCache = Join-Path $NOA_OPT "go/cache"
    $goModCache = Join-Path $NOA_OPT "go/pkg/mod"
    if (Test-Path (Join-Path $goRoot "bin/go.exe")) {
        $envVars += "`$env:GOROOT = `"$goRoot`""
        $envVars += "`$env:GOPATH = `"$goPath`""
        $envVars += "`$env:GOBIN = `"$goBin`""
        $envVars += "`$env:GOCACHE = `"$goCache`""
        $envVars += "`$env:GOMODCACHE = `"$goModCache`""
        $pathAdditions += "$goRoot\bin"
        $pathAdditions += $goBin
    }

    # Node.js
    $nodeRoot = Join-Path $NOA_OPT "node"
    $npmCache = Join-Path $NOA_OPT "npm-cache"
    if (Test-Path (Join-Path $nodeRoot "node.exe")) {
        $envVars += "`$env:npm_configs_prefix = `"$nodeRoot`""
        $envVars += "`$env:npm_configs_cache = `"$npmCache`""
        $pathAdditions += $nodeRoot
    }

    # Python
    $pythonRoot = Join-Path $NOA_OPT "python"
    $venvPath = Join-Path $NOA_OPT "venv"
    if (Test-Path (Join-Path $pythonRoot "python.exe")) {
        $pathAdditions += $pythonRoot
        $pathAdditions += "$pythonRoot\Scripts"
        if (Test-Path $venvPath) {
            $envVars += "# To activate Python venv: & `"$venvPath\Scripts\Activate.ps1`""
        }
    }

    # protoc
    $protobufBin = Join-Path $NOA_OPT "protobuf/bin"
    if (Test-Path (Join-Path $protobufBin "protoc.exe")) {
        $pathAdditions += $protobufBin
    }

    # NOA bin directory (self-contained utilities like jq, rg, fd, bat)
    $noaBin = Join-Path $NoaRoot "bin"
    $pathAdditions += $noaBin

    # Build the noa-env.ps1 content
    $content = @(
        "# NOA Environment configsuration"
        "# Auto-generated by install-prereqs.ps1"
        "# Portable Toolchains per Constitution §3.1 (Self-Contained)"
        "# Last Updated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
        ""
        "# NOA Root Directory"
        "`$env:NOA_ROOT = `"$NoaRoot`""
        ""
        "# NOA Directory Shortcuts"
        "`$env:NOA_BIN = `"$(Join-Path $NoaRoot 'bin')`""
        "`$env:NOA_OPT = `"$NOA_OPT`""
        "`$env:NOA_configs = `"$(Join-Path $NoaRoot 'configs')`""
        "`$env:NOA_LOGS = `"$(Join-Path $NoaRoot 'logs')`""
        "`$env:NOA_TMP = `"$(Join-Path $NoaRoot 'tmp')`""
        ""
        "# ============================================"
        "# Portable Toolchain Environment Variables"
        "# ============================================"
    )

    if ($envVars.Count -gt 0) {
        $content += $envVars
    }

    $content += @(
        ""
        "# ============================================"
        "# PATH configsuration (prepend portable tools)"
        "# ============================================"
    )

    if ($pathAdditions.Count -gt 0) {
        $pathString = ($pathAdditions -join ";")
        $content += "`$env:PATH = `"$pathString;`$env:PATH`""
    }

    $content += @(
        ""
        "# ============================================"
        "# Helper Functions"
        "# ============================================"
        ""
        "# Navigation"
        "function cda { Set-Location `$env:NOA_ROOT }"
        "function cdopt { Set-Location `$env:NOA_OPT }"
        "function cdbin { Set-Location `$env:NOA_BIN }"
        ""
        "# Toolchain status"
        "function Get-NoaToolchains {"
        "    Write-Host 'NOA Portable Toolchains:' -ForegroundColor Cyan"
        "    @("
        "        @{ Name = 'Rust'; Path = `"`$env:NOA_OPT/rust/cargo/bin/rustc.exe`" },"
        "        @{ Name = 'Go'; Path = `"`$env:NOA_OPT/go/bin/go.exe`" },"
        "        @{ Name = 'Node'; Path = `"`$env:NOA_OPT/node/node.exe`" },"
        "        @{ Name = 'Python'; Path = `"`$env:NOA_OPT/python/python.exe`" },"
        "        @{ Name = 'protoc'; Path = `"`$env:NOA_OPT/protobuf/bin/protoc.exe`" }"
        "    ) | ForEach-Object {"
        "        `$exists = Test-Path `$_.Path"
        "        `$status = if (`$exists) { '[OK]' } else { '[--]' }"
        "        `$color = if (`$exists) { 'Green' } else { 'Yellow' }"
        "        Write-Host `"  `$status `$(`$_.Name)`" -ForegroundColor `$color"
        "    }"
        "}"
        ""
        "# Activation message"
        "Write-Host 'NOA environment loaded (portable toolchains)' -ForegroundColor Green"
        "Write-Host `"  NOA_ROOT: `$env:NOA_ROOT`" -ForegroundColor Gray"
    )

    $content -join "`r`n" | Set-Content -Path $envPath -Encoding UTF8
    Write-Log "Created: $envPath" -Level Success
}

# ============================================
# Main Execution
# ============================================

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable Prerequisites Installer" -ForegroundColor Cyan
Write-Host "Constitution §3.1 Compliant - Self-Contained" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT:     $NoaRoot" -ForegroundColor White
Write-Host "Installer Dir: $InstallerDir" -ForegroundColor White
Write-Host ""

# Ensure installer directory exists
if (-not (Test-Path $InstallerDir)) {
    Write-Log "Installer directory not found: $InstallerDir" -Level Error
    exit 1
}

# Determine what to install
$toInstall = @()

if ($All) {
    Write-Log "Installing ALL portable toolchains (--All specified)..." -Level Info
    $toInstall = $PortableInstallers.Keys
} else {
    Write-Log "Checking for missing portable toolchains..." -Level Info
    $toInstall = Get-MissingToolchains

    if ($toInstall.Count -eq 0) {
        Write-Log "All portable toolchains are already installed!" -Level Success
        Write-Host ""
        Write-Host "Installed toolchains:" -ForegroundColor Green
        foreach ($name in $PortableInstallers.Keys) {
            $info = $PortableInstallers[$name]
            $path = Join-Path $NOA_OPT $info.CheckPath
            Write-Host "  [✓] $name : $path" -ForegroundColor Green
        }
        Write-Host ""

        # Still regenerate noa-env.ps1 to ensure it's up to date
        Update-NoaEnv
        exit 0
    }
}

Write-Host ""
Write-Host "Toolchains to install:" -ForegroundColor Yellow
foreach ($name in $toInstall) {
    Write-Host "  - $name" -ForegroundColor White
}
Write-Host ""

# Install each missing toolchain in priority order
$installed = 0
$failed = 0

foreach ($name in $toInstall) {
    $info = $PortableInstallers[$name]
    Write-Host ""
    Write-Host "-" * 50 -ForegroundColor Gray

    $forceParam = if ($Force) { @{ Force = $true } } else { @{} }

    if (Install-PortableToolchain -Name $name -Info $info @forceParam) {
        $installed++
    } else {
        $failed++
    }
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan

# Update noa-env.ps1 with all installed toolchains
Update-NoaEnv

# Summary
Write-Host ""
Write-Host "=" * 60 -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Yellow" })
Write-Host "Installation Summary" -ForegroundColor White
Write-Host "=" * 60 -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Yellow" })
Write-Host "  Installed: $installed" -ForegroundColor Green
Write-Host "  Failed:    $failed" -ForegroundColor $(if ($failed -gt 0) { "Red" } else { "Gray" })
Write-Host ""

# Detect WSL
$isWSL = $false
$wslVersion = ""
if (Test-Path "/proc/version") {
    $procVersion = Get-Content "/proc/version" -ErrorAction SilentlyContinue
    if ($procVersion -match "microsoft") {
        $isWSL = $true
        $wslVersion = if (Test-Path "/run/WSL") { "WSL2" } else { "WSL1" }
    }
}

if ($failed -eq 0) {
    Write-Host "All prerequisites installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "  1. Load environment:  . `"$(Join-Path $NoaRoot 'noa-env.ps1')`"" -ForegroundColor Cyan
    Write-Host "  2. Verify tools:      Get-NoaToolchains" -ForegroundColor Cyan
    Write-Host "  3. Check prereqs:     .\scripts\setup\check-prereqs.ps1" -ForegroundColor Cyan

    # WSL/Linux kernel setup guidance
    if ($isWSL -and $wslVersion -eq "WSL2") {
        Write-Host ""
        Write-Host "WSL2 Detected - Kernel Setup:" -ForegroundColor Yellow
        Write-Host "  For P2P networking, configsure kernel modules:" -ForegroundColor White
        Write-Host "    wsl -d <distro> -u root $NoaRoot/scripts/noa-kmod check" -ForegroundColor Cyan
        Write-Host "    wsl -d <distro> -u root $NoaRoot/scripts/noa-kernel-params set net.ipv4.ip_forward 1" -ForegroundColor Cyan
    } elseif ($isWSL -and $wslVersion -eq "WSL1") {
        Write-Host ""
        Write-Host "WSL1 Detected - Limited kernel access (P2P may be restricted)" -ForegroundColor Yellow
    }

    Write-Host ""
    exit 0
} else {
    Write-Host "Some installations failed. See output above for details." -ForegroundColor Yellow
    Write-Host "You may need to run failed installers manually." -ForegroundColor Yellow
    exit 1
}
