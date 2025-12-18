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

# Add NOA directories to PATH
function Add-NoaPath {
    param([string]$Dir)
    if ((Test-Path $Dir) -and ($env:PATH -notlike "*$Dir*")) {
        $env:PATH = "$Dir;$env:PATH"
    }
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

# Data directories (FR-001: All data within noa_root)
$env:NOA_DATA = Join-Path $env:NOA_ROOT "data"
$env:NOA_CACHE = Join-Path $env:NOA_ROOT "data/cache"
$env:NOA_CONFIG_HOME = Join-Path $env:NOA_ROOT "etc"

# AppData redirection (FR-001: Self-contained operation)
# Override Windows AppData paths to keep all application data within NOA
$env:APPDATA = Join-Path $env:NOA_ROOT "data/appdata/roaming"
$env:LOCALAPPDATA = Join-Path $env:NOA_ROOT "data/appdata/local"
$env:TEMP = Join-Path $env:NOA_ROOT "tmp"
$env:TMP = Join-Path $env:NOA_ROOT "tmp"

# XDG Base Directory specification (Unix compatibility)
$env:XDG_DATA_HOME = Join-Path $env:NOA_ROOT "data"
$env:XDG_CONFIG_HOME = Join-Path $env:NOA_ROOT "etc"
$env:XDG_CACHE_HOME = Join-Path $env:NOA_ROOT "data/cache"
$env:XDG_STATE_HOME = Join-Path $env:NOA_ROOT "data/state"

# Kernel-level isolation paths
$env:NOA_NAMESPACE = Join-Path $env:NOA_SYS "namespace"
$env:NOA_CGROUP = Join-Path $env:NOA_SYS "cgroup"
$env:NOA_KERNEL = Join-Path $env:NOA_SYS "kernel"

# Add kernel tools to PATH
Add-NoaPath (Join-Path $env:NOA_KERNEL "windows")
Add-NoaPath (Join-Path $env:NOA_KERNEL "windows\hyperv")
Add-NoaPath (Join-Path $env:NOA_KERNEL "windows\sandbox")

# Go environment (portable installation)
$env:GOROOT = Join-Path $env:NOA_OPT "go"
$env:GOPATH = Join-Path $env:NOA_OPT "go\workspace"
$env:GOBIN = Join-Path $env:NOA_OPT "go\workspace\bin"
$env:GOCACHE = Join-Path $env:NOA_OPT "go\cache"
$env:GOMODCACHE = Join-Path $env:NOA_OPT "go\pkg\mod"

# Add NOA directories to PATH
function Add-NoaPath {
    param([string]$Dir)
    if ((Test-Path $Dir) -and ($env:PATH -notlike "*$Dir*")) {
        $env:PATH = "$Dir;$env:PATH"
    }
}

Add-NoaPath $env:NOA_BIN
Add-NoaPath $env:NOA_SCRIPTS

# Add Go to PATH
Add-NoaPath (Join-Path $env:GOROOT "bin")
Add-NoaPath $env:GOBIN

# Add Node.js and npm global modules to PATH
$env:NOA_NODE = Join-Path $env:NOA_OPT "node"
$env:npm_config_prefix = $env:NOA_NODE
$env:npm_config_cache = Join-Path $env:NOA_OPT "npm-cache"
Add-NoaPath $env:NOA_NODE
Add-NoaPath (Join-Path $env:NOA_NODE "node_modules\.bin")

# Add Rust to PATH
$env:RUSTUP_HOME = Join-Path $env:NOA_OPT "rust\rustup"
$env:CARGO_HOME = Join-Path $env:NOA_OPT "rust\cargo"
Add-NoaPath (Join-Path $env:CARGO_HOME "bin")

# Add Python to PATH
Add-NoaPath (Join-Path $env:NOA_OPT "python")

# Add protobuf to PATH
Add-NoaPath (Join-Path $env:NOA_OPT "protobuf\bin")

# Add CMake to PATH
Add-NoaPath (Join-Path $env:NOA_OPT "cmake\bin")

# Add llama.cpp build to PATH
Add-NoaPath (Join-Path $env:NOA_OPT "llama.cpp\build\bin\Release")

# Conda-forge environment (for notebooks / cross-platform Python)
# Preferred runtime is a self-contained micromamba environment under $env:NOA_OPT\conda
$env:NOA_CONDA = Join-Path $env:NOA_OPT "conda"
$env:NOA_CONDA_ENV = Join-Path $env:NOA_CONDA "envs\noa"

# If the env exists, prepend it so python/jupyter resolve from it
$condaPython = Join-Path $env:NOA_CONDA_ENV "python.exe"
if (Test-Path $condaPython) {
    Add-NoaPath (Join-Path $env:NOA_CONDA_ENV "Scripts")
    Add-NoaPath $env:NOA_CONDA_ENV
}

function noa-conda-init {
    if (-not (Test-Path $env:NOA_CONDA)) {
        New-Item -ItemType Directory -Path $env:NOA_CONDA -Force | Out-Null
    }

    $micromamba = Join-Path $env:NOA_CONDA "micromamba.exe"
    if (-not (Test-Path $micromamba)) {
        Write-Host "micromamba not found at: $micromamba" -ForegroundColor Yellow
        Write-Host "Install or place micromamba.exe there to enable conda-forge envs." -ForegroundColor Gray
        return
    }

    & $micromamba create -y -p $env:NOA_CONDA_ENV -c conda-forge python=3.12 jupyterlab ipykernel
    Write-Host "Created/updated conda-forge env: $env:NOA_CONDA_ENV" -ForegroundColor Green
}

function noa-conda-activate {
    $micromamba = Join-Path $env:NOA_CONDA "micromamba.exe"
    if (-not (Test-Path $micromamba)) {
        Write-Host "micromamba not found at: $micromamba" -ForegroundColor Yellow
        return
    }

    & $micromamba shell hook -s powershell | Out-String | Invoke-Expression
    micromamba activate -p $env:NOA_CONDA_ENV
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
