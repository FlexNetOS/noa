<#
.SYNOPSIS
    NOA Environment Configuration for Windows PowerShell
    
.DESCRIPTION
    Sets up NOA environment variables and aliases for Windows.
    Source this file in your PowerShell profile for persistent configuration.
    
.EXAMPLE
    . .\noa-env.ps1
#>

# Auto-detect NOA_ROOT from the location of this script
$script:ScriptPath = $MyInvocation.MyCommand.Path
if ($ScriptPath) {
    $env:NOA_ROOT = Split-Path -Parent $ScriptPath
} else {
    $env:NOA_ROOT = $PWD.Path
}

# Core directories
$env:NOA_REPOS = Join-Path $env:NOA_ROOT "repos"
$env:NOA_CONTAINERS = Join-Path $env:NOA_ROOT "containers"
$env:NOA_WORKSPACE = Join-Path $env:NOA_ROOT "workspace"
$env:NOA_CONFIG = Join-Path $env:NOA_ROOT "config"
$env:NOA_SCRIPTS = Join-Path $env:NOA_ROOT "scripts"
$env:NOA_LOGS = Join-Path $env:NOA_ROOT "logs"
$env:NOA_TMP = Join-Path $env:NOA_ROOT "tmp"
$env:NOA_P2P = Join-Path $env:NOA_ROOT "p2p"

# AI directories
$env:NOA_AI = Join-Path $env:NOA_ROOT "ai"
$env:NOA_AI_SHARED = Join-Path $env:NOA_AI "shared"
$env:NOA_AI_PROVIDERS = Join-Path $env:NOA_AI "providers"
$env:NOA_AI_DEVICES = Join-Path $env:NOA_AI "devices"
$env:NOA_AI_ORCHESTRATION = Join-Path $env:NOA_AI "orchestration"

# Git directories
$env:NOA_GIT = Join-Path $env:NOA_ROOT "git"
$env:NOA_GIT_REPOS = Join-Path $env:NOA_GIT "repos"
$env:NOA_GIT_PRS = Join-Path $env:NOA_GIT "prs"
$env:NOA_GIT_CONFLICTS = Join-Path $env:NOA_GIT "conflicts"
$env:NOA_GIT_CI_CD = Join-Path $env:NOA_GIT "ci-cd"
$env:NOA_GIT_MIRRORS = Join-Path $env:NOA_GIT "mirrors"

# System directories
$env:NOA_BIN = Join-Path $env:NOA_ROOT "bin"
$env:NOA_ETC = Join-Path $env:NOA_ROOT "etc"
$env:NOA_LIB = Join-Path $env:NOA_ROOT "lib"
$env:NOA_OPT = Join-Path $env:NOA_ROOT "opt"
$env:NOA_SYS = Join-Path $env:NOA_ROOT "sys"
$env:NOA_INIT = Join-Path $env:NOA_ROOT "init"

# Kernel-level isolation paths
$env:NOA_NAMESPACE = Join-Path $env:NOA_SYS "namespace"
$env:NOA_CGROUP = Join-Path $env:NOA_SYS "cgroup"
$env:NOA_KERNEL = Join-Path $env:NOA_SYS "kernel"

# Add NOA directories to PATH
function Add-NoaPath {
    param([string]$Dir)
    if ((Test-Path $Dir) -and ($env:PATH -notlike "*$Dir*")) {
        $env:PATH = "$Dir;$env:PATH"
    }
}

Add-NoaPath $env:NOA_BIN
Add-NoaPath $env:NOA_SCRIPTS

# Navigation aliases
function cda { Set-Location $env:NOA_ROOT }
function cdr { Set-Location $env:NOA_REPOS }
function cdc { Set-Location $env:NOA_CONTAINERS }
function cdw { Set-Location $env:NOA_WORKSPACE }
function cdp { Set-Location $env:NOA_P2P }
function cdai { Set-Location $env:NOA_AI }
function cdgit { Set-Location $env:NOA_GIT }
function cds { Set-Location $env:NOA_SCRIPTS }
function cdl { Set-Location $env:NOA_LOGS }

# NOA CLI wrapper
function noa {
    param([Parameter(ValueFromRemainingArguments)]$Args)
    $noaScript = Join-Path $env:NOA_SCRIPTS "noa.ps1"
    if (Test-Path $noaScript) {
        & $noaScript @Args
    } else {
        Write-Host "NOA CLI not found at: $noaScript" -ForegroundColor Red
    }
}

# Git workflow helpers
function git-sync {
    param([string]$Message = "")
    & (Join-Path $env:NOA_SCRIPTS "sync.ps1") $Message
}

function git-push-dev {
    param([string]$Message = "")
    & (Join-Path $env:NOA_SCRIPTS "git-push.ps1") -Message $Message
}

# Environment validation
function Test-NoaEnv {
    $required = @(
        $env:NOA_ROOT,
        $env:NOA_SCRIPTS,
        $env:NOA_CONFIG
    )
    
    $missing = @()
    foreach ($var in $required) {
        if (-not $var -or -not (Test-Path $var)) {
            $missing += $var
        }
    }
    
    if ($missing.Count -gt 0) {
        Write-Host "Missing or invalid NOA directories:" -ForegroundColor Red
        $missing | ForEach-Object { Write-Host "  - $_" -ForegroundColor Yellow }
        return $false
    }
    
    Write-Host "NOA environment validated successfully" -ForegroundColor Green
    Write-Host "  NOA_ROOT: $env:NOA_ROOT" -ForegroundColor Cyan
    return $true
}

# Display confirmation
Write-Host "NOA environment loaded: $env:NOA_ROOT" -ForegroundColor Green
