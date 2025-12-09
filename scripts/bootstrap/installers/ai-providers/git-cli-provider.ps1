<#
.SYNOPSIS
    Configure git as a local AI provider for NOA.

.DESCRIPTION
    Verifies git CLI is available and configures it as a local provider.
    Git provides agentic capabilities through git hooks, diff analysis, and commit workflows.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\git-cli-provider.ps1
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$PROVIDER_DIR = Join-Path $NoaRoot "ai/providers/local/git-cli"
$CONFIG_FILE = Join-Path $PROVIDER_DIR "config.json"

Write-Host "Configuring Git CLI as local AI provider..." -ForegroundColor Cyan

# Check if git is available
$gitPath = Get-Command git -ErrorAction SilentlyContinue
if (-not $gitPath) {
    Write-Error "Git is not installed or not in PATH. Please install git first."
    exit 1
}

$gitVersion = & git --version 2>&1 | Select-Object -First 1
Write-Host "  Found: $gitVersion" -ForegroundColor Green

# Ensure provider directory exists
if (-not (Test-Path $PROVIDER_DIR)) {
    New-Item -ItemType Directory -Path $PROVIDER_DIR -Force | Out-Null
    Write-Host "  Created provider directory: $PROVIDER_DIR" -ForegroundColor Green
}

# Check if config already exists
if (Test-Path $CONFIG_FILE) {
    Write-Host "  [EXISTS] Git CLI provider config already configured" -ForegroundColor Gray
    return
}

# Create provider config
$config = @{
    name = "git-cli"
    type = "local"
    priority = 8
    enabled = $true
    description = "Git CLI for version control and diff analysis"
    cli = @{
        command = "git"
        version = $gitVersion -replace 'git version ', ''
        path = $gitPath.Source
    }
    capabilities = @{
        versionControl = $true
        diffAnalysis = $true
        commitManagement = $true
        branchOperations = $true
        hookIntegration = $true
        mergeConflictResolution = $true
    }
    agentic_features = @{
        commit_message_generation = $true
        diff_explanation = $true
        conflict_resolution_hints = $true
        pr_description_generation = $true
    }
    sharedResourcePath = "`${NOA_ROOT}/ai/shared"
    sharedResources = @{
        prompts = "`${NOA_ROOT}/ai/shared/prompts"
        agents = "`${NOA_ROOT}/ai/shared/agents"
        tools = "`${NOA_ROOT}/ai/shared/tools"
        commands = "`${NOA_ROOT}/ai/shared/commands"
        executionMemory = "`${NOA_ROOT}/ai/shared/resources/execution-memory.db"
    }
}

$config | ConvertTo-Json -Depth 4 | Set-Content -Path $CONFIG_FILE -Encoding UTF8
Write-Host "  [OK] Created Git CLI provider config: $CONFIG_FILE" -ForegroundColor Green

