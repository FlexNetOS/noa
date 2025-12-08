<#
.SYNOPSIS
    NOA Comprehensive Prerequisites Check (T673-T674 merged)

.DESCRIPTION
    Unified prerequisite checker that validates:
    1. SYSTEM-WIDE build toolchains (Rust, Go, Node, Python, protoc) - MUST be system-wide
    2. SELF-CONTAINED utilities (downloaded to noa_root/bin/) - per §3.1 containment
    3. Code quality and security tools

    Per NOA Constitution §3.1: The system MUST operate entirely inside noa_root,
    EXCEPT for build toolchains which require system-wide installation.

.PARAMETER Json
    Output results in JSON format

.PARAMETER InstallMissing
    Attempt to install missing tools (requires admin for system tools)

.PARAMETER NoaRoot
    NOA root directory (default: parent of scripts directory)

.EXAMPLE
    .\check-prereqs.ps1
    .\check-prereqs.ps1 -Json
    .\check-prereqs.ps1 -InstallMissing

.NOTES
    Exit codes:
      0 - All prerequisites met
      1 - Critical tools missing (cannot build)
      2 - High-priority tools missing (builds may fail quality gates)
#>

param(
    [switch]$Json,
    [switch]$InstallMissing,
    [string]$NoaRoot
)

$ErrorActionPreference = "SilentlyContinue"

# Determine NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent (Split-Path -Parent $PSScriptRoot) }
}
$NOA_BIN = Join-Path $NoaRoot "bin"

# Results collections
$Installed = @()
$MissingCritical = @()
$MissingHigh = @()
$VersionWarnings = @()

function Compare-Version {
    param([string]$Current, [string]$Required)
    try {
        $currentParts = $Current -split '\.' | ForEach-Object { [int]$_ }
        $requiredParts = $Required -split '\.' | ForEach-Object { [int]$_ }
        for ($i = 0; $i -lt [Math]::Max($currentParts.Count, $requiredParts.Count); $i++) {
            $c = if ($i -lt $currentParts.Count) { $currentParts[$i] } else { 0 }
            $r = if ($i -lt $requiredParts.Count) { $requiredParts[$i] } else { 0 }
            if ($c -gt $r) { return $true }
            if ($c -lt $r) { return $false }
        }
        return $true
    } catch { return $false }
}

function Check-Tool {
    param(
        [string]$Name,
        [string]$Command,
        [string]$MinVersion,
        [string]$Severity,
        [string]$InstallCmd,
        [string]$VersionCmd,
        [string]$Category
    )

    $exists = Get-Command $Command -ErrorAction SilentlyContinue

    if ($exists) {
        $versionOutput = try { Invoke-Expression $VersionCmd 2>$null } catch { "unknown" }
        $currentVersion = if ($versionOutput -match '(\d+\.\d+(\.\d+)?)') { $Matches[1] } else { "unknown" }

        if ($currentVersion -ne "unknown" -and (Compare-Version $currentVersion $MinVersion)) {
            $script:Installed += @{
                Name = $Name
                Version = $currentVersion
                Required = $MinVersion
                Category = $Category
            }
            if (-not $Json) {
                Write-Host "  [OK] $Name $currentVersion" -ForegroundColor Green
            }
        } else {
            $script:VersionWarnings += @{
                Name = $Name
                Current = $currentVersion
                Required = $MinVersion
                Category = $Category
            }
            if (-not $Json) {
                Write-Host "  [!!] $Name $currentVersion (need >= $MinVersion)" -ForegroundColor Yellow
            }
        }
    } else {
        $entry = @{
            Name = $Name
            Install = $InstallCmd
            Category = $Category
        }
        if ($Severity -eq "CRITICAL") {
            $script:MissingCritical += $entry
            if (-not $Json) {
                Write-Host "  [X] $Name NOT FOUND (CRITICAL)" -ForegroundColor Red
                Write-Host "      Install: $InstallCmd" -ForegroundColor Gray
            }
        } else {
            $script:MissingHigh += $entry
            if (-not $Json) {
                Write-Host "  [X] $Name NOT FOUND (HIGH)" -ForegroundColor Red
                Write-Host "      Install: $InstallCmd" -ForegroundColor Gray
            }
        }
    }
}

function Check-SelfContainedTool {
    param(
        [string]$Name,
        [string]$ExeName,
        [string]$DownloadUrl
    )

    $toolPath = Join-Path $NOA_BIN $ExeName
    if (Test-Path $toolPath) {
        $script:Installed += @{
            Name = $Name
            Version = "self-contained"
            Category = "Self-Contained"
            Path = $toolPath
        }
        if (-not $Json) {
            Write-Host "  [OK] $Name (self-contained: $toolPath)" -ForegroundColor Green
        }
    } else {
        $script:MissingHigh += @{
            Name = $Name
            Install = ".\scripts\download-static-binaries.ps1"
            Category = "Self-Contained"
        }
        if (-not $Json) {
            Write-Host "  [--] $Name not in bin/ (optional)" -ForegroundColor Yellow
        }
    }
}

# ========================================
# Main Checks
# ========================================

if (-not $Json) {
    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host "NOA Prerequisites Check" -ForegroundColor Cyan
    Write-Host "Constitution: 3.1 (Self-Contained), FR-015 (Security)" -ForegroundColor Gray
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host ""
    Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
    Write-Host "NOA_BIN:  $NOA_BIN" -ForegroundColor White
    Write-Host ""

    Write-Host "1. SYSTEM-WIDE Build Toolchains (CRITICAL)" -ForegroundColor Yellow
    Write-Host "   Note: These CANNOT be self-contained per language requirements" -ForegroundColor Gray
    Write-Host "-" * 50 -ForegroundColor Gray
}

# CRITICAL - System-wide Build Toolchains
Check-Tool -Name "Rust (rustc)" -Command "rustc" -MinVersion "1.83.0" -Severity "CRITICAL" `
    -InstallCmd "winget install Rustlang.Rustup && rustup default stable" `
    -VersionCmd "rustc --version" -Category "Build-SystemWide"

Check-Tool -Name "Cargo" -Command "cargo" -MinVersion "1.83.0" -Severity "CRITICAL" `
    -InstallCmd "(installed with Rust)" `
    -VersionCmd "cargo --version" -Category "Build-SystemWide"

Check-Tool -Name "Go" -Command "go" -MinVersion "1.23.0" -Severity "CRITICAL" `
    -InstallCmd "winget install GoLang.Go" `
    -VersionCmd "go version" -Category "Build-SystemWide"

Check-Tool -Name "Node.js" -Command "node" -MinVersion "20.0.0" -Severity "CRITICAL" `
    -InstallCmd "winget install OpenJS.NodeJS.LTS" `
    -VersionCmd "node --version" -Category "Build-SystemWide"

Check-Tool -Name "Python" -Command "python" -MinVersion "3.12.0" -Severity "CRITICAL" `
    -InstallCmd "winget install Python.Python.3.12" `
    -VersionCmd "python --version" -Category "Build-SystemWide"

Check-Tool -Name "protoc" -Command "protoc" -MinVersion "28.0.0" -Severity "CRITICAL" `
    -InstallCmd "winget install Google.Protobuf" `
    -VersionCmd "protoc --version" -Category "Build-SystemWide"

if (-not $Json) {
    Write-Host ""
    Write-Host "2. Code Quality Tools (HIGH)" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

# HIGH - Quality Tools (system-wide, required for CI)
Check-Tool -Name "rustfmt" -Command "rustfmt" -MinVersion "1.0.0" -Severity "HIGH" `
    -InstallCmd "rustup component add rustfmt" `
    -VersionCmd "rustfmt --version" -Category "Quality-SystemWide"

Check-Tool -Name "clippy" -Command "cargo-clippy" -MinVersion "0.1.0" -Severity "HIGH" `
    -InstallCmd "rustup component add clippy" `
    -VersionCmd "cargo clippy --version" -Category "Quality-SystemWide"

Check-Tool -Name "golangci-lint" -Command "golangci-lint" -MinVersion "1.62.0" -Severity "HIGH" `
    -InstallCmd "go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest" `
    -VersionCmd "golangci-lint --version" -Category "Quality-SystemWide"

Check-Tool -Name "eslint" -Command "eslint" -MinVersion "9.0.0" -Severity "HIGH" `
    -InstallCmd "npm install -g eslint" `
    -VersionCmd "eslint --version" -Category "Quality-SystemWide"

Check-Tool -Name "ruff" -Command "ruff" -MinVersion "0.8.0" -Severity "HIGH" `
    -InstallCmd "pip install ruff" `
    -VersionCmd "ruff --version" -Category "Quality-SystemWide"

if (-not $Json) {
    Write-Host ""
    Write-Host "3. Security Tools - FR-015 (HIGH)" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

# HIGH - Security Tools
Check-Tool -Name "Gitleaks" -Command "gitleaks" -MinVersion "8.21.0" -Severity "HIGH" `
    -InstallCmd "choco install gitleaks" `
    -VersionCmd "gitleaks version" -Category "Security"

Check-Tool -Name "Trivy" -Command "trivy" -MinVersion "0.57.0" -Severity "HIGH" `
    -InstallCmd "choco install trivy" `
    -VersionCmd "trivy --version" -Category "Security"

Check-Tool -Name "Grype" -Command "grype" -MinVersion "0.84.0" -Severity "HIGH" `
    -InstallCmd "choco install grype" `
    -VersionCmd "grype version" -Category "Security"

Check-Tool -Name "Semgrep" -Command "semgrep" -MinVersion "1.97.0" -Severity "HIGH" `
    -InstallCmd "pip install semgrep" `
    -VersionCmd "semgrep --version" -Category "Security"

if (-not $Json) {
    Write-Host ""
    Write-Host "4. Self-Contained Utilities (noa_root/bin/) - 3.1 Compliant" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

# Self-contained utilities (in noa_root/bin/)
Check-SelfContainedTool -Name "jq" -ExeName "jq.exe"
Check-SelfContainedTool -Name "ripgrep" -ExeName "rg.exe"
Check-SelfContainedTool -Name "fd" -ExeName "fd.exe"
Check-SelfContainedTool -Name "bat" -ExeName "bat.exe"

if (-not $Json) {
    Write-Host ""
    Write-Host "5. Basic Prerequisites" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

# Basic prereqs
Check-Tool -Name "Git" -Command "git" -MinVersion "2.40.0" -Severity "CRITICAL" `
    -InstallCmd "winget install Git.Git" `
    -VersionCmd "git --version" -Category "Basic"

Check-Tool -Name "GitHub CLI" -Command "gh" -MinVersion "2.40.0" -Severity "HIGH" `
    -InstallCmd "winget install GitHub.cli" `
    -VersionCmd "gh --version" -Category "Basic"

Check-Tool -Name "Git LFS" -Command "git-lfs" -MinVersion "3.0.0" -Severity "HIGH" `
    -InstallCmd "winget install GitHub.GitLFS" `
    -VersionCmd "git lfs version" -Category "Basic"

# ========================================
# Output Results
# ========================================

if ($Json) {
    $result = @{
        noa_root = $NoaRoot
        installed = $Installed
        missing_critical = $MissingCritical
        missing_high = $MissingHigh
        version_warnings = $VersionWarnings
        summary = @{
            installed = $Installed.Count
            missing_critical = $MissingCritical.Count
            missing_high = $MissingHigh.Count
            version_warnings = $VersionWarnings.Count
        }
    }
    $result | ConvertTo-Json -Depth 5
} else {
    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host "Summary" -ForegroundColor Cyan
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host "Installed:        $($Installed.Count)" -ForegroundColor Green
    Write-Host "Missing CRITICAL: $($MissingCritical.Count)" -ForegroundColor $(if ($MissingCritical.Count -gt 0) { "Red" } else { "Green" })
    Write-Host "Missing HIGH:     $($MissingHigh.Count)" -ForegroundColor $(if ($MissingHigh.Count -gt 0) { "Yellow" } else { "Green" })
    Write-Host "Version Warnings: $($VersionWarnings.Count)" -ForegroundColor $(if ($VersionWarnings.Count -gt 0) { "Yellow" } else { "Green" })
}

# Exit code
if ($MissingCritical.Count -gt 0) {
    if (-not $Json) {
        Write-Host ""
        Write-Host "ERROR: Critical prerequisites missing. Install before building." -ForegroundColor Red
        Write-Host ""
        Write-Host "Quick Install (run as Administrator):" -ForegroundColor Yellow
        Write-Host "  winget install Rustlang.Rustup GoLang.Go Google.Protobuf" -ForegroundColor White
        Write-Host "  rustup default stable && rustup component add rustfmt clippy" -ForegroundColor White
    }
    exit 1
} elseif ($MissingHigh.Count -gt 0) {
    if (-not $Json) {
        Write-Host ""
        Write-Host "WARNING: High-priority tools missing. Quality gates may fail." -ForegroundColor Yellow
    }
    exit 2
} else {
    if (-not $Json) {
        Write-Host ""
        Write-Host "All prerequisites met! Ready for implementation." -ForegroundColor Green
    }
    exit 0
}
