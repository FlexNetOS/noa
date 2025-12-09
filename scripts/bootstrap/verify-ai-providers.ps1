<#
.SYNOPSIS
    Verify all AI provider CLIs are installed and functional.

.DESCRIPTION
    Checks each configured AI provider CLI for availability and version.
    Reports status of local, cloud, hybrid, and IDE providers.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Json
    Output results in JSON format

.EXAMPLE
    .\verify-ai-providers.ps1
    .\verify-ai-providers.ps1 -Json
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Json
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    }
}

$PROVIDERS_DIR = Join-Path $NoaRoot "ai/providers"

# Provider definitions with CLI commands
$providerChecks = @(
    @{ Name = "git-cli"; Type = "local"; Command = "git"; VersionArg = "--version" },
    @{ Name = "ollama"; Type = "local"; Command = "ollama"; VersionArg = "--version"; Optional = $true },  # Optional - llama.cpp is primary
    @{ Name = "llama-server"; Type = "local"; Command = "llama-server"; VersionArg = "--version"; BinPath = "bin/llama-server"; Optional = $true },  # Optional - built on demand
    @{ Name = "cursor"; Type = "hybrid"; Command = "cursor"; VersionArg = "--version"; Optional = $true },
    @{ Name = "claude-code"; Type = "cloud"; Command = "claude"; VersionArg = "--version"; Optional = $true },
    @{ Name = "codex"; Type = "cloud"; Command = "codex"; VersionArg = "--version"; Optional = $true },
    @{ Name = "abacus"; Type = "cloud"; Command = "abacusai"; VersionArg = "--version"; Optional = $true },
    @{ Name = "gh-copilot"; Type = "ide"; Command = "gh"; VersionArg = "copilot --version"; Optional = $true }
)

$results = @{
    timestamp = (Get-Date -Format "o")
    noa_root = $NoaRoot
    providers = @{}
    summary = @{
        total = 0
        available = 0
        missing = 0
        optional_missing = 0
    }
}

if (-not $Json) {
    Write-Host "Verifying AI Provider CLIs..." -ForegroundColor Cyan
    Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
    Write-Host ""
}

foreach ($provider in $providerChecks) {
    $results.summary.total++

    $status = @{
        name = $provider.Name
        type = $provider.Type
        command = $provider.Command
        optional = [bool]$provider.Optional
        available = $false
        version = $null
        path = $null
        error = $null
    }

    try {
        # Check if command is in NOA bin first
        $noaBinPath = if ($provider.BinPath) {
            Join-Path $NoaRoot $provider.BinPath
        } else {
            Join-Path $NoaRoot "bin/$($provider.Command).exe"
        }

        $cmdInfo = $null
        if (Test-Path $noaBinPath) {
            $cmdInfo = @{ Source = $noaBinPath }
        } else {
            $cmdInfo = Get-Command $provider.Command -ErrorAction SilentlyContinue
        }

        if ($cmdInfo) {
            $status.path = if ($cmdInfo -is [hashtable]) { $cmdInfo.Source } else { $cmdInfo.Source }

            # Try to get version
            try {
                $versionOutput = if ($provider.VersionArg -match ' ') {
                    # Multi-word version arg (like "copilot --version")
                    $args = $provider.VersionArg -split ' '
                    & $provider.Command @args 2>&1 | Select-Object -First 1
                } else {
                    & $provider.Command $provider.VersionArg 2>&1 | Select-Object -First 1
                }
                $status.version = $versionOutput -replace '^\s*', ''
            } catch {
                $status.version = "unknown"
            }

            $status.available = $true
            $results.summary.available++

            if (-not $Json) {
                Write-Host "  [OK] $($provider.Name) ($($provider.Type)): $($status.version)" -ForegroundColor Green
            }
        } else {
            throw "Command not found"
        }
    } catch {
        $status.error = $_.Exception.Message

        if ($provider.Optional) {
            $results.summary.optional_missing++
            if (-not $Json) {
                Write-Host "  [SKIP] $($provider.Name) ($($provider.Type)): Not installed (optional)" -ForegroundColor Yellow
            }
        } else {
            $results.summary.missing++
            if (-not $Json) {
                Write-Host "  [MISS] $($provider.Name) ($($provider.Type)): Not installed" -ForegroundColor Red
            }
        }
    }

    $results.providers[$provider.Name] = $status
}

# Check provider config files
$configDirs = @("local", "cloud", "hybrid", "ide")
$configuredProviders = @()

foreach ($dir in $configDirs) {
    $dirPath = Join-Path $PROVIDERS_DIR $dir
    if (Test-Path $dirPath) {
        $configs = Get-ChildItem -Path $dirPath -Filter "config.json" -Recurse
        foreach ($config in $configs) {
            $providerName = Split-Path -Parent $config.FullName | Split-Path -Leaf
            $configuredProviders += @{
                name = $providerName
                type = $dir
                config_path = $config.FullName
            }
        }
    }
}

$results.configured_providers = $configuredProviders

if (-not $Json) {
    Write-Host ""
    Write-Host "Summary:" -ForegroundColor Cyan
    Write-Host "  Total providers checked: $($results.summary.total)" -ForegroundColor Gray
    Write-Host "  Available: $($results.summary.available)" -ForegroundColor Green
    Write-Host "  Missing (required): $($results.summary.missing)" -ForegroundColor $(if ($results.summary.missing -gt 0) { "Red" } else { "Gray" })
    Write-Host "  Missing (optional): $($results.summary.optional_missing)" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Configured provider configs found: $($configuredProviders.Count)" -ForegroundColor Gray
}

if ($Json) {
    $results | ConvertTo-Json -Depth 5
}

# Exit with error if required providers are missing
if ($results.summary.missing -gt 0) {
    exit 1
}

