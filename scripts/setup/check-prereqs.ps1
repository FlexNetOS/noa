<#
.SYNOPSIS
    NOA Comprehensive Prerequisites Check (containment-first)

.DESCRIPTION
    - Prefers contained installs under NOA_ROOT/bin (portable)
    - Optional system-wide fallback only when -AllowGlobal is used
    - Install hints point to scripts/setup/install-all-tools.ps1

.PARAMETER Json
    Output results in JSON format

.PARAMETER PathsOnly
    Output only feature directory paths (for spec-kit integration)

.PARAMETER NoaRoot
    NOA root directory (default: parent of scripts directory or env:NOA_ROOT)

.PARAMETER AllowGlobal
    Permit detection of system-wide tools. Default: false (contained only).

.EXAMPLE
    .\check-prereqs.ps1
    .\check-prereqs.ps1 -Json
    .\check-prereqs.ps1 -Json -PathsOnly
    .\check-prereqs.ps1 -AllowGlobal
#>

[CmdletBinding()]
param(
    [switch]$Json,
    [switch]$PathsOnly,
    [string]$NoaRoot,
    [switch]$AllowGlobal
)

$ErrorActionPreference = "SilentlyContinue"

if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent (Split-Path -Parent $PSScriptRoot) }
}
$NOA_BIN = Join-Path $NoaRoot "bin"

# Handle -PathsOnly mode for spec-kit integration (/clarify, /plan, /tasks commands)
if ($PathsOnly) {
    # Find the active feature directory
    $specsDir = Join-Path $NoaRoot "specs"
    $featureDir = $null
    $featureSpec = $null
    $implPlan = $null
    $tasksFile = $null
    $availableDocs = @()

    # Look for feature directories (prefer 001-noa-seed-foundation if exists)
    if (Test-Path $specsDir) {
        $features = Get-ChildItem -Path $specsDir -Directory | Where-Object { $_.Name -match '^\d{3}-' } | Sort-Object Name
        foreach ($feature in $features) {
            $specPath = Join-Path $feature.FullName "spec.md"
            if (Test-Path $specPath) {
                $featureDir = $feature.FullName
                $featureSpec = $specPath

                # Check for optional docs
                $planPath = Join-Path $feature.FullName "plan.md"
                $tasksPath = Join-Path $feature.FullName "tasks.md"
                $dataModelPath = Join-Path $feature.FullName "data-model.md"
                $researchPath = Join-Path $feature.FullName "research.md"
                $quickstartPath = Join-Path $feature.FullName "quickstart.md"
                $contractsDir = Join-Path $feature.FullName "contracts"

                if (Test-Path $planPath) { $implPlan = $planPath; $availableDocs += "plan.md" }
                if (Test-Path $tasksPath) { $tasksFile = $tasksPath; $availableDocs += "tasks.md" }
                if (Test-Path $dataModelPath) { $availableDocs += "data-model.md" }
                if (Test-Path $researchPath) { $availableDocs += "research.md" }
                if (Test-Path $quickstartPath) { $availableDocs += "quickstart.md" }
                if (Test-Path $contractsDir) { $availableDocs += "contracts/" }

                break  # Use first valid feature
            }
        }
    }

    if ($Json) {
        $result = @{
            NOA_ROOT = $NoaRoot
            FEATURE_DIR = $featureDir
            FEATURE_SPEC = $featureSpec
            IMPL_PLAN = $implPlan
            TASKS = $tasksFile
            AVAILABLE_DOCS = $availableDocs
        }
        $result | ConvertTo-Json -Depth 3
    } else {
        Write-Host "NOA_ROOT=$NoaRoot"
        Write-Host "FEATURE_DIR=$featureDir"
        Write-Host "FEATURE_SPEC=$featureSpec"
        Write-Host "IMPL_PLAN=$implPlan"
        Write-Host "TASKS=$tasksFile"
        Write-Host "AVAILABLE_DOCS=$($availableDocs -join ',')"
    }
    exit 0
}

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

function Resolve-InBin {
    param([string[]]$Names)
    foreach ($n in $Names) {
        $p = Join-Path $NOA_BIN $n
        if (Test-Path $p) { return $p }
    }
    return $null
}

function Check-Tool {
    param(
        [string]$Name,
        [string[]]$Commands,
        [string]$MinVersion,
        [string]$Severity,
        [string]$InstallCmd,
        [string]$VersionCmd,
        [string]$Category
    )

    $binCandidate = Resolve-InBin $Commands
    $cmd = $null
    if ($binCandidate) { $cmd = $binCandidate }
    elseif ($AllowGlobal) { $cmd = (Get-Command $Commands[0] -ErrorAction SilentlyContinue) }

    if ($cmd) {
        $versionOutput = try {
            if ($VersionCmd) { Invoke-Expression $VersionCmd } else { & $cmd --version }
        } catch { "unknown" }
        $currentVersion = if ($versionOutput -match '(\d+\.\d+(\.\d+)?)') { $Matches[1] } else { "unknown" }

        if ($currentVersion -ne "unknown" -and (Compare-Version $currentVersion $MinVersion)) {
            $Installed += @{
                Name = $Name
                Version = $currentVersion
                Required = $MinVersion
                Category = $Category
                Path = $cmd
            }
            if (-not $Json) { Write-Host "  [OK] $Name $currentVersion ($cmd)" -ForegroundColor Green }
        } else {
            $VersionWarnings += @{
                Name = $Name
                Current = $currentVersion
                Required = $MinVersion
                Category = $Category
            }
            if (-not $Json) { Write-Host "  [!!] $Name $currentVersion (need >= $MinVersion)" -ForegroundColor Yellow }
        }
        return
    }

    $entry = @{ Name = $Name; Install = $InstallCmd; Category = $Category }
    if ($Severity -eq "CRITICAL") {
        $MissingCritical += $entry
        if (-not $Json) {
            Write-Host "  [X] $Name NOT FOUND (CRITICAL)" -ForegroundColor Red
            Write-Host "      Install: $InstallCmd" -ForegroundColor Gray
        }
    } else {
        $MissingHigh += $entry
        if (-not $Json) {
            Write-Host "  [X] $Name NOT FOUND (HIGH)" -ForegroundColor Red
            Write-Host "      Install: $InstallCmd" -ForegroundColor Gray
        }
    }
}

function Check-SelfContainedTool {
    param(
        [string]$Name,
        [string]$ExeName
    )

    $toolPath = Join-Path $NOA_BIN $ExeName
    if (Test-Path $toolPath) {
        $Installed += @{
            Name = $Name
            Version = "self-contained"
            Category = "Self-Contained"
            Path = $toolPath
        }
        if (-not $Json) { Write-Host "  [OK] $Name (self-contained: $toolPath)" -ForegroundColor Green }
    } else {
        $MissingHigh += @{
            Name = $Name
            Install = ".\scripts\setup\install-all-tools.ps1 -Tool $Name"
            Category = "Self-Contained"
        }
        if (-not $Json) { Write-Host "  [--] $Name not in bin/ (optional)" -ForegroundColor Yellow }
    }
}

if (-not $Json) {
    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host "NOA Prerequisites Check" -ForegroundColor Cyan
    Write-Host "Constitution: §3.1 (Self-Contained), FR-015 (Security)" -ForegroundColor Gray
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host ""
    Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
    Write-Host "NOA_BIN:  $NOA_BIN" -ForegroundColor White
    Write-Host ""
    Write-Host "Mode: Contained-first (AllowGlobal = $AllowGlobal)" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

# CRITICAL build toolchains
Check-Tool -Name "Rust (rustc)" -Commands @("rustc.exe","rustc") -MinVersion "1.83.0" -Severity "CRITICAL" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool rust" `
    -VersionCmd "rustc --version" -Category "Build"

Check-Tool -Name "Cargo" -Commands @("cargo.exe","cargo") -MinVersion "1.83.0" -Severity "CRITICAL" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool rust" `
    -VersionCmd "cargo --version" -Category "Build"

Check-Tool -Name "Go" -Commands @("go.exe","go") -MinVersion "1.23.0" -Severity "CRITICAL" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool go" `
    -VersionCmd "go version" -Category "Build"

Check-Tool -Name "Node.js" -Commands @("node.exe","node") -MinVersion "20.0.0" -Severity "CRITICAL" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool node" `
    -VersionCmd "node --version" -Category "Build"

Check-Tool -Name "Python" -Commands @("python.exe","python") -MinVersion "3.12.0" -Severity "CRITICAL" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool python" `
    -VersionCmd "python --version" -Category "Build"

Check-Tool -Name "protoc" -Commands @("protoc.exe","protoc") -MinVersion "28.0.0" -Severity "CRITICAL" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool protoc" `
    -VersionCmd "protoc --version" -Category "Build"

if (-not $Json) {
    Write-Host ""
    Write-Host "2. Code Quality Tools (HIGH)" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

Check-Tool -Name "rustfmt" -Commands @("rustfmt.exe","rustfmt") -MinVersion "1.0.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool rust" `
    -VersionCmd "rustfmt --version" -Category "Quality"

Check-Tool -Name "clippy" -Commands @("cargo-clippy.exe","cargo-clippy") -MinVersion "0.1.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool rust" `
    -VersionCmd "cargo clippy --version" -Category "Quality"

Check-Tool -Name "golangci-lint" -Commands @("golangci-lint.exe","golangci-lint") -MinVersion "1.62.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool golangci-lint" `
    -VersionCmd "golangci-lint --version" -Category "Quality"

Check-Tool -Name "eslint" -Commands @("eslint.cmd","eslint") -MinVersion "9.0.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool eslint" `
    -VersionCmd "eslint --version" -Category "Quality"

Check-Tool -Name "ruff" -Commands @("ruff.exe","ruff") -MinVersion "0.8.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool ruff" `
    -VersionCmd "ruff --version" -Category "Quality"

if (-not $Json) {
    Write-Host ""
    Write-Host "3. Security Tools (HIGH)" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

Check-Tool -Name "Gitleaks" -Commands @("gitleaks.exe","gitleaks") -MinVersion "8.21.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool gitleaks" `
    -VersionCmd "gitleaks version" -Category "Security"

Check-Tool -Name "Trivy" -Commands @("trivy.exe","trivy") -MinVersion "0.57.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool trivy" `
    -VersionCmd "trivy --version" -Category "Security"

Check-Tool -Name "Grype" -Commands @("grype.exe","grype") -MinVersion "0.84.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool grype" `
    -VersionCmd "grype version" -Category "Security"

Check-Tool -Name "Semgrep" -Commands @("semgrep.exe","semgrep") -MinVersion "1.97.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool semgrep" `
    -VersionCmd "semgrep --version" -Category "Security"

if (-not $Json) {
    Write-Host ""
    Write-Host "4. Self-Contained Utilities (noa_root/bin)" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

Check-SelfContainedTool -Name "jq" -ExeName "jq.exe"
Check-SelfContainedTool -Name "ripgrep" -ExeName "rg.exe"
Check-SelfContainedTool -Name "fd" -ExeName "fd.exe"
Check-SelfContainedTool -Name "bat" -ExeName "bat.exe"

if (-not $Json) {
    Write-Host ""
    Write-Host "5. Basic Prerequisites" -ForegroundColor Yellow
    Write-Host "-" * 50 -ForegroundColor Gray
}

Check-Tool -Name "Git" -Commands @("git.exe","git") -MinVersion "2.40.0" -Severity "CRITICAL" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool git" `
    -VersionCmd "git --version" -Category "Basic"

Check-Tool -Name "GitHub CLI" -Commands @("gh.exe","gh") -MinVersion "2.40.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool gh" `
    -VersionCmd "gh --version" -Category "Basic"

Check-Tool -Name "Git LFS" -Commands @("git-lfs.exe","git-lfs") -MinVersion "3.0.0" -Severity "HIGH" `
    -InstallCmd ".\scripts\setup\install-all-tools.ps1 -Tool gitlfs" `
    -VersionCmd "git lfs version" -Category "Basic"

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

if ($MissingCritical.Count -gt 0) {
    if (-not $Json) {
        Write-Host ""
        Write-Host "ERROR: Critical prerequisites missing. Install before building." -ForegroundColor Red
        Write-Host "Run: pwsh -File scripts/setup/install-all-tools.ps1" -ForegroundColor Gray
    }
    exit 1
} elseif ($MissingHigh.Count -gt 0) {
    if (-not $Json) {
        Write-Host ""
        Write-Host "WARNING: High-priority tools missing. Quality gates may fail." -ForegroundColor Yellow
        Write-Host "Run: pwsh -File scripts/setup/install-all-tools.ps1" -ForegroundColor Gray
    }
    exit 2
} else {
    if (-not $Json) {
        Write-Host ""
        Write-Host "All prerequisites met! Ready for implementation." -ForegroundColor Green
    }
    exit 0
}

