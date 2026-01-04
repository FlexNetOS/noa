<#
.SYNOPSIS
    Generate bootstrap installation report.

.DESCRIPTION
    Creates a detailed report of the NOA bootstrap installation including
    all installed tools, versions, and configsuration status.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER OutputPath
    Path to save the report (default: logs/bootstrap/report.md)

.EXAMPLE
    .\generate-report.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [string]$OutputPath
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

if (-not $OutputPath) {
    $OutputPath = Join-Path $NoaRoot "logs/bootstrap/report.md"
}

# Ensure output directory exists
$outputDir = Split-Path -Parent $OutputPath
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

Write-Host "Generating NOA Bootstrap Report..." -ForegroundColor Cyan

$report = @"
# NOA Bootstrap Installation Report

**Generated**: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
**NOA Root**: ``$NoaRoot``
**Platform**: $([System.Environment]::OSVersion.Platform) ($([System.Environment]::OSVersion.VersionString))

---

## Environment Summary

| Component | Status | Details |
|-----------|--------|---------|
"@

# Check toolchains
$toolchecks = @(
    @{ Name = "Git"; Cmd = "git --version" },
    @{ Name = "Rust"; Cmd = "rustc --version" },
    @{ Name = "Go"; Cmd = "go version" },
    @{ Name = "Node.js"; Cmd = "node --version" },
    @{ Name = "Python"; Cmd = "python --version" },
    @{ Name = "jq"; Cmd = "jq --version" },
    @{ Name = "ripgrep"; Cmd = "rg --version" }
)

foreach ($tool in $toolchecks) {
    try {
        $version = Invoke-Expression $tool.Cmd 2>&1 | Select-Object -First 1
        $report += "`n| $($tool.Name) | ✅ Installed | ``$version`` |"
    } catch {
        $report += "`n| $($tool.Name) | ❌ Not Found | - |"
    }
}

$report += @"


---

## Directory Structure

| Directory | Exists | Contents |
|-----------|--------|----------|
"@

$dirs = @("bin", "configs", "ai", "ai/shared", "ai/providers", "logs", "specs", "cache")
foreach ($dir in $dirs) {
    $path = Join-Path $NoaRoot $dir
    if (Test-Path $path) {
        $count = (Get-ChildItem -Path $path -Recurse -File -ErrorAction SilentlyContinue | Measure-Object).Count
        $report += "`n| ``$dir`` | ✅ | $count files |"
    } else {
        $report += "`n| ``$dir`` | ❌ | Not created |"
    }
}

$report += @"


---

## AI Providers

| Provider | Type | configs Exists |
|----------|------|---------------|
"@

$providerTypes = @("local", "cloud", "hybrid", "ide")
foreach ($type in $providerTypes) {
    $providerDir = Join-Path $NoaRoot "ai/providers/$type"
    if (Test-Path $providerDir) {
        $configss = Get-ChildItem -Path $providerDir -Filter "configs.json" -Recurse -ErrorAction SilentlyContinue
        foreach ($configs in $configss) {
            $providerName = Split-Path -Parent $configs.FullName | Split-Path -Leaf
            $report += "`n| $providerName | $type | ✅ |"
        }
    }
}

$report += @"


---

## Shared Resources

| Resource | Path | Status |
|----------|------|--------|
"@

$resources = @(
    @{ Name = "Agent definitions"; Path = "ai/shared/agents" },
    @{ Name = "Workflow definitions"; Path = "ai/shared/workflows" },
    @{ Name = "Prompts"; Path = "ai/shared/prompts" },
    @{ Name = "Tools"; Path = "ai/shared/tools" },
    @{ Name = "Skills"; Path = "ai/shared/skills" },
    @{ Name = "Models"; Path = "ai/shared/models" },
    @{ Name = "Commands"; Path = "ai/shared/commands" },
    @{ Name = "Execution Memory DB"; Path = "ai/shared/resources/execution-memory.db" },
    @{ Name = "Resource Registry"; Path = "ai/shared/resources/resource-registry.json" }
)

foreach ($res in $resources) {
    $path = Join-Path $NoaRoot $res.Path
    if (Test-Path $path) {
        $report += "`n| $($res.Name) | ``$($res.Path)`` | ✅ |"
    } else {
        $report += "`n| $($res.Name) | ``$($res.Path)`` | ❌ |"
    }
}

$report += @"


---

## configsuration Files

| configs | Path | Valid |
|--------|------|-------|
"@

$configss = @(
    "configs/ai-providers.json",
    "configs/shared-resources.json",
    "configs/bootstrap-state.json",
    "configs/bootstrap-tools.json"
)

foreach ($cfg in $configss) {
    $path = Join-Path $NoaRoot $cfg
    if (Test-Path $path) {
        try {
            Get-Content $path -Raw | ConvertFrom-Json | Out-Null
            $report += "`n| $(Split-Path -Leaf $cfg) | ``$cfg`` | ✅ Valid JSON |"
        } catch {
            $report += "`n| $(Split-Path -Leaf $cfg) | ``$cfg`` | ⚠️ Invalid JSON |"
        }
    } else {
        $report += "`n| $(Split-Path -Leaf $cfg) | ``$cfg`` | ❌ Not found |"
    }
}

$report += @"


---

## Next Steps

1. Run ``./scripts/bootstrap/verify/verify-all.ps1`` to verify installation
2. Run ``./scripts/bootstrap/verify/smoke-test.ps1`` to test toolchains
3. Source the environment: ``. ./noa-env.ps1``
4. Start using NOA commands

---

*Report generated by NOA Bootstrap*
"@

# Save report
$report | Set-Content -Path $OutputPath -Encoding UTF8

Write-Host "Report saved to: $OutputPath" -ForegroundColor Green
Write-Host ""
Write-Host "Preview:" -ForegroundColor Yellow
Write-Host $report.Substring(0, [Math]::Min(1000, $report.Length))
Write-Host "..."

