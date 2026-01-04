<#
.SYNOPSIS
    configsure centralized cache directories for all NOA toolchains.

.DESCRIPTION
    Creates cache directory structure and configsures symlinks/junctions
    to centralize all tool caches under noa_root/cache/.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect from NOA_ROOT env or script location)

.EXAMPLE
    .\cache-setup.ps1
    .\cache-setup.ps1 -NoaRoot "C:\noa"
#>
[CmdletBinding()]
param(
    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))
    }
}

$CACHE_DIR = Join-Path $NoaRoot "cache"

Write-Host "Setting up centralized cache directories..." -ForegroundColor Cyan
Write-Host "NOA Root: $NoaRoot" -ForegroundColor Gray
Write-Host "Cache Dir: $CACHE_DIR" -ForegroundColor Gray
Write-Host ""

# Create main cache directory
if (-not (Test-Path $CACHE_DIR)) {
    New-Item -ItemType Directory -Path $CACHE_DIR -Force | Out-Null
    Write-Host "  [CREATE] $CACHE_DIR" -ForegroundColor Green
}

# Define cache subdirectories
$cacheDirs = @(
    @{ Name = "rust"; Description = "Rust/Cargo registry cache" },
    @{ Name = "go"; Description = "Go module cache" },
    @{ Name = "npm"; Description = "npm package cache" },
    @{ Name = "pip"; Description = "pip package cache" },
    @{ Name = "models"; Description = "AI model cache (llama.cpp, HuggingFace)" },
    @{ Name = "ollama"; Description = "Ollama model cache" },
    @{ Name = "huggingface"; Description = "HuggingFace Hub cache" },
    @{ Name = "downloads"; Description = "Downloaded archives/installers" }
)

foreach ($cacheInfo in $cacheDirs) {
    $cachePath = Join-Path $CACHE_DIR $cacheInfo.Name
    if (-not (Test-Path $cachePath)) {
        New-Item -ItemType Directory -Path $cachePath -Force | Out-Null
        Write-Host "  [CREATE] $cachePath ($($cacheInfo.Description))" -ForegroundColor Green
    } else {
        Write-Host "  [EXISTS] $cachePath" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "Cache directory setup complete." -ForegroundColor Green
Write-Host ""

# configsure toolchain cache environment variables
Write-Host "Recommended environment variables for cache centralization:" -ForegroundColor Yellow
Write-Host ""
Write-Host "  # Rust/Cargo" -ForegroundColor Cyan
Write-Host "  `$env:CARGO_HOME = `"$NoaRoot\opt\rust\cargo`""
Write-Host "  # Note: Registry cache is at `$env:CARGO_HOME\registry"
Write-Host ""
Write-Host "  # Go" -ForegroundColor Cyan
Write-Host "  `$env:GOCACHE = `"$CACHE_DIR\go`""
Write-Host "  `$env:GOMODCACHE = `"$NoaRoot\opt\go\pkg\mod`""
Write-Host ""
Write-Host "  # npm" -ForegroundColor Cyan
Write-Host "  `$env:npm_configs_cache = `"$CACHE_DIR\npm`""
Write-Host ""
Write-Host "  # pip" -ForegroundColor Cyan
Write-Host "  `$env:PIP_CACHE_DIR = `"$CACHE_DIR\pip`""
Write-Host ""
Write-Host "  # HuggingFace" -ForegroundColor Cyan
Write-Host "  `$env:HF_HOME = `"$CACHE_DIR\huggingface`""
Write-Host ""
Write-Host "  # Ollama" -ForegroundColor Cyan
Write-Host "  `$env:OLLAMA_MODELS = `"$CACHE_DIR\ollama`""
Write-Host ""

# Create a cache configs file for reference
$cacheconfigsPath = Join-Path $CACHE_DIR "cache-configs.json"
$cacheconfigs = @{
    noa_root = $NoaRoot
    cache_root = $CACHE_DIR
    created_at = (Get-Date -Format "o")
    directories = @{
        rust = Join-Path $CACHE_DIR "rust"
        go = Join-Path $CACHE_DIR "go"
        npm = Join-Path $CACHE_DIR "npm"
        pip = Join-Path $CACHE_DIR "pip"
        models = Join-Path $CACHE_DIR "models"
        ollama = Join-Path $CACHE_DIR "ollama"
        huggingface = Join-Path $CACHE_DIR "huggingface"
        downloads = Join-Path $CACHE_DIR "downloads"
    }
    env_vars = @{
        CARGO_HOME = "$NoaRoot\opt\rust\cargo"
        GOCACHE = "$CACHE_DIR\go"
        GOMODCACHE = "$NoaRoot\opt\go\pkg\mod"
        npm_configs_cache = "$CACHE_DIR\npm"
        PIP_CACHE_DIR = "$CACHE_DIR\pip"
        HF_HOME = "$CACHE_DIR\huggingface"
        OLLAMA_MODELS = "$CACHE_DIR\ollama"
    }
}
$cacheconfigs | ConvertTo-Json -Depth 3 | Set-Content -Path $cacheconfigsPath -Encoding UTF8
Write-Host "Cache configsuration saved to: $cacheconfigsPath" -ForegroundColor Green

