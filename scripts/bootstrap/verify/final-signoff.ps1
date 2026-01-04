<#
.SYNOPSIS
    Final sign-off verification for bootstrap (B100).

.DESCRIPTION
    Comprehensive check that all tools are working and caches configsured.
    This is the final verification before declaring bootstrap complete.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER GenerateReport
    Generate a report file

.EXAMPLE
    .\final-signoff.ps1 -GenerateReport
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$GenerateReport
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

Write-Host "╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║                                                              ║" -ForegroundColor Cyan
Write-Host "║         NOA Bootstrap Final Sign-Off                         ║" -ForegroundColor Cyan
Write-Host "║                                                              ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "Timestamp: $timestamp" -ForegroundColor Gray
Write-Host "NOA Root:  $NoaRoot" -ForegroundColor Gray
Write-Host ""

$results = @{
    Toolchains = @{}
    Caches = @{}
    Directories = @{}
    configsuration = @{}
    Constitution = @{}
}

$totalPassed = 0
$totalFailed = 0

# ===== 1. Toolchain Verification =====
Write-Host "1. Verifying Toolchains" -ForegroundColor Yellow
Write-Host "   ─────────────────────" -ForegroundColor Gray

$toolchains = @(
    @{ Name = "Rust"; Cmd = "rustc"; Args = "--version"; Path = (Join-Path $NoaRoot "opt/rust/cargo/bin/rustc.exe") },
    @{ Name = "Go"; Cmd = "go"; Args = "version"; Path = (Join-Path $NoaRoot "opt/go/bin/go.exe") },
    @{ Name = "Node.js"; Cmd = "node"; Args = "--version"; Path = (Join-Path $NoaRoot "opt/node/node.exe") },
    @{ Name = "Python"; Cmd = "python"; Args = "--version"; Path = (Join-Path $NoaRoot "opt/python/python.exe") },
    @{ Name = "protoc"; Cmd = "protoc"; Args = "--version"; Path = (Join-Path $NoaRoot "opt/protobuf/bin/protoc.exe") }
)

foreach ($tc in $toolchains) {
    $exists = Test-Path $tc.Path
    if ($exists) {
        try {
            $version = & $tc.Path $tc.Args 2>&1
            Write-Host "   [OK] $($tc.Name): $version" -ForegroundColor Green
            $results.Toolchains[$tc.Name] = @{ Status = "OK"; Version = $version }
            $totalPassed++
        } catch {
            Write-Host "   [!!] $($tc.Name): installed but failed to run" -ForegroundColor Yellow
            $results.Toolchains[$tc.Name] = @{ Status = "ERROR"; Version = "N/A" }
            $totalFailed++
        }
    } else {
        Write-Host "   [--] $($tc.Name): not installed" -ForegroundColor Gray
        $results.Toolchains[$tc.Name] = @{ Status = "MISSING"; Version = "N/A" }
    }
}

# ===== 2. Cache configsuration =====
Write-Host ""
Write-Host "2. Verifying Caches" -ForegroundColor Yellow
Write-Host "   ─────────────────" -ForegroundColor Gray

$caches = @("rust", "go", "npm", "pip", "models", "ollama", "huggingface", "downloads")
foreach ($cache in $caches) {
    $cachePath = Join-Path $NoaRoot "cache/$cache"
    if (Test-Path $cachePath) {
        Write-Host "   [OK] cache/$cache/" -ForegroundColor Green
        $results.Caches[$cache] = "OK"
        $totalPassed++
    } else {
        Write-Host "   [!!] cache/$cache/ - MISSING" -ForegroundColor Red
        $results.Caches[$cache] = "MISSING"
        $totalFailed++
    }
}

# ===== 3. Directory Structure =====
Write-Host ""
Write-Host "3. Verifying Directory Structure" -ForegroundColor Yellow
Write-Host "   ──────────────────────────────" -ForegroundColor Gray

$requiredDirs = @("bin", "opt", "lib", "configs", "logs", "ai/shared", "sys/kernel")
foreach ($dir in $requiredDirs) {
    $dirPath = Join-Path $NoaRoot $dir
    if (Test-Path $dirPath) {
        Write-Host "   [OK] $dir/" -ForegroundColor Green
        $results.Directories[$dir] = "OK"
        $totalPassed++
    } else {
        Write-Host "   [!!] $dir/ - MISSING" -ForegroundColor Red
        $results.Directories[$dir] = "MISSING"
        $totalFailed++
    }
}

# ===== 4. configsuration Files =====
Write-Host ""
Write-Host "4. Verifying configsuration" -ForegroundColor Yellow
Write-Host "   ────────────────────────" -ForegroundColor Gray

$configsFiles = @(
    "configs/ai-providers.json",
    "configs/bootstrap-tools.json",
    ".noa",
    "noa-env.ps1"
)

foreach ($configs in $configsFiles) {
    $configsPath = Join-Path $NoaRoot $configs
    if (Test-Path $configsPath) {
        Write-Host "   [OK] $configs" -ForegroundColor Green
        $results.configsuration[$configs] = "OK"
        $totalPassed++
    } else {
        Write-Host "   [!!] $configs - MISSING" -ForegroundColor Red
        $results.configsuration[$configs] = "MISSING"
        $totalFailed++
    }
}

# ===== 5. Constitutional Compliance =====
Write-Host ""
Write-Host "5. Constitutional Compliance Checks" -ForegroundColor Yellow
Write-Host "   ─────────────────────────────────" -ForegroundColor Gray

# §3.1 - Self-contained
$selfContained = -not (Get-ChildItem -Path $NoaRoot -Recurse -Force -ErrorAction SilentlyContinue |
    Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint } |
    Where-Object { $_.Target -and -not $_.Target.StartsWith($NoaRoot) })

if ($selfContained) {
    Write-Host "   [OK] §3.1 Self-Contained: All paths within noa_root" -ForegroundColor Green
    $results.Constitution["§3.1"] = "COMPLIANT"
    $totalPassed++
} else {
    Write-Host "   [!!] §3.1 Self-Contained: External symlinks detected" -ForegroundColor Red
    $results.Constitution["§3.1"] = "VIOLATION"
    $totalFailed++
}

# §3.2 - Offline capable (check cache exists)
$offlineReady = (Test-Path (Join-Path $NoaRoot "cache/downloads"))
if ($offlineReady) {
    Write-Host "   [OK] §3.2 Offline Capable: Download cache exists" -ForegroundColor Green
    $results.Constitution["§3.2"] = "READY"
    $totalPassed++
} else {
    Write-Host "   [--] §3.2 Offline Capable: Download cache missing" -ForegroundColor Yellow
    $results.Constitution["§3.2"] = "NOT_READY"
}

# §3.5 - Logging
$loggingOk = (Test-Path (Join-Path $NoaRoot "logs/bootstrap"))
if ($loggingOk) {
    Write-Host "   [OK] §3.5 Logging: Bootstrap logs directory exists" -ForegroundColor Green
    $results.Constitution["§3.5"] = "COMPLIANT"
    $totalPassed++
} else {
    Write-Host "   [!!] §3.5 Logging: logs/bootstrap/ missing" -ForegroundColor Red
    $results.Constitution["§3.5"] = "VIOLATION"
    $totalFailed++
}

# ===== Summary =====
Write-Host ""
Write-Host "╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║                     Final Sign-Off Summary                   ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "   Passed: $totalPassed" -ForegroundColor Green
Write-Host "   Failed: $totalFailed" -ForegroundColor $(if ($totalFailed -gt 0) { "Red" } else { "Gray" })
Write-Host ""

if ($totalFailed -eq 0) {
    Write-Host "   ╔════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "   ║  ✓ BOOTSTRAP COMPLETE - READY FOR PRODUCTION          ║" -ForegroundColor Green
    Write-Host "   ╚════════════════════════════════════════════════════════╝" -ForegroundColor Green
} else {
    Write-Host "   ╔════════════════════════════════════════════════════════╗" -ForegroundColor Yellow
    Write-Host "   ║  ⚠ BOOTSTRAP INCOMPLETE - $totalFailed issues remaining" -ForegroundColor Yellow
    Write-Host "   ╚════════════════════════════════════════════════════════╝" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "   Run bootstrap with missing components:" -ForegroundColor Gray
    Write-Host "   .\scripts\bootstrap\bootstrap.ps1 -InstallAllTools" -ForegroundColor Cyan
}

# Generate report if requested
if ($GenerateReport) {
    $reportPath = Join-Path $NoaRoot "logs/bootstrap/signoff-$(Get-Date -Format 'yyyyMMdd-HHmmss').json"
    $report = @{
        timestamp = $timestamp
        noaRoot = $NoaRoot
        passed = $totalPassed
        failed = $totalFailed
        results = $results
    }

    # Ensure directory exists
    $reportDir = Split-Path -Parent $reportPath
    if (-not (Test-Path $reportDir)) {
        New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
    }

    $report | ConvertTo-Json -Depth 5 | Set-Content -Path $reportPath -Encoding UTF8
    Write-Host ""
    Write-Host "   Report saved: $reportPath" -ForegroundColor Gray
}

exit $totalFailed

