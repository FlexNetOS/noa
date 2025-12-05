<#
.SYNOPSIS
    Complete NOA environment setup for Windows
    
.DESCRIPTION
    Sets up the complete NOA environment including:
    - Directory structure
    - Environment configuration
    - Tool downloads
    - PowerShell profile integration
    - Git configuration
    
.PARAMETER SkipTools
    Skip downloading optional tools (jq, rg, fd, bat)
    
.PARAMETER SkipProfile
    Skip PowerShell profile integration
    
.PARAMETER NoaRoot
    Override NOA_ROOT location (default: script parent directory)
    
.EXAMPLE
    .\setup.ps1
    .\setup.ps1 -SkipTools
    .\setup.ps1 -NoaRoot "D:\my-noa"
#>

param(
    [switch]$SkipTools,
    [switch]$SkipProfile,
    [string]$NoaRoot
)

$ErrorActionPreference = "Continue"

# Determine NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = Split-Path -Parent $PSScriptRoot
}

Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Environment Setup" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor White
Write-Host ""

# ============================================
# 1. Create Directory Structure
# ============================================
Write-Host "1. Creating directory structure..." -ForegroundColor Yellow

$dirs = @(
    "ai", "ai/shared", "ai/providers", "ai/devices", "ai/orchestration",
    "bin", "config", "containers", "etc", "git", "git/repos", "git/prs",
    "git/conflicts", "git/ci-cd", "git/mirrors", "init", "lib", "logs",
    "opt", "p2p", "p2p/storage", "repos", "scripts", "sys", "sys/namespace",
    "sys/cgroup", "sys/kernel", "sys/kernel/params", "tmp", "workspace"
)

foreach ($dir in $dirs) {
    $fullPath = Join-Path $NoaRoot $dir
    if (-not (Test-Path $fullPath)) {
        New-Item -ItemType Directory -Path $fullPath -Force | Out-Null
        Write-Host "  [CREATED] $dir" -ForegroundColor Green
    }
}
Write-Host "  Directory structure complete" -ForegroundColor Green

# ============================================
# 2. Set Environment Variables
# ============================================
Write-Host ""
Write-Host "2. Setting environment variables..." -ForegroundColor Yellow

$env:NOA_ROOT = $NoaRoot
$env:NOA_BIN = Join-Path $NoaRoot "bin"
$env:NOA_SCRIPTS = Join-Path $NoaRoot "scripts"
$env:PATH = "$env:NOA_BIN;$env:NOA_SCRIPTS;$env:PATH"

Write-Host "  NOA_ROOT = $env:NOA_ROOT" -ForegroundColor Green

# ============================================
# 3. Configure Git
# ============================================
Write-Host ""
Write-Host "3. Checking Git configuration..." -ForegroundColor Yellow

$gitUser = git config --global user.name 2>$null
$gitEmail = git config --global user.email 2>$null

if (-not $gitUser) {
    Write-Host "  [WARNING] Git user.name not set" -ForegroundColor Yellow
    Write-Host "  Run: git config --global user.name 'Your Name'" -ForegroundColor Gray
}
if (-not $gitEmail) {
    Write-Host "  [WARNING] Git user.email not set" -ForegroundColor Yellow
    Write-Host "  Run: git config --global user.email 'you@example.com'" -ForegroundColor Gray
}

# Set safe editor
$gitEditor = git config --global core.editor 2>$null
if (-not $gitEditor -or $gitEditor -match "code.*--wait") {
    git config --global core.editor "notepad"
    Write-Host "  [SET] core.editor = notepad" -ForegroundColor Green
}

# Initialize Git LFS
$lfsVersion = git lfs version 2>$null
if ($lfsVersion) {
    git lfs install 2>$null
    Write-Host "  [OK] Git LFS initialized" -ForegroundColor Green
} else {
    Write-Host "  [WARNING] Git LFS not installed" -ForegroundColor Yellow
    Write-Host "  Install: winget install GitHub.GitLFS" -ForegroundColor Gray
}

# ============================================
# 4. Download Tools
# ============================================
if (-not $SkipTools) {
    Write-Host ""
    Write-Host "4. Downloading optional tools..." -ForegroundColor Yellow
    
    $downloadScript = Join-Path $NoaRoot "scripts/download-static-binaries.ps1"
    if (Test-Path $downloadScript) {
        & $downloadScript
    } else {
        Write-Host "  [SKIP] Download script not found" -ForegroundColor Yellow
    }
} else {
    Write-Host ""
    Write-Host "4. Skipping tool download (use -SkipTools:$false to enable)" -ForegroundColor Yellow
}

# ============================================
# 5. Create .env file
# ============================================
Write-Host ""
Write-Host "5. Checking .env file..." -ForegroundColor Yellow

$envPath = Join-Path $NoaRoot ".env"
$envExamplePath = Join-Path $NoaRoot ".env.example"

if (-not (Test-Path $envPath)) {
    if (Test-Path $envExamplePath) {
        Copy-Item $envExamplePath $envPath
        Write-Host "  [CREATED] .env from .env.example" -ForegroundColor Green
        Write-Host "  [ACTION] Edit .env and add your API keys" -ForegroundColor Yellow
    } else {
        Write-Host "  [SKIP] No .env.example found" -ForegroundColor Yellow
    }
} else {
    Write-Host "  [OK] .env exists" -ForegroundColor Green
}

# ============================================
# 6. PowerShell Profile Integration
# ============================================
if (-not $SkipProfile) {
    Write-Host ""
    Write-Host "6. PowerShell profile integration..." -ForegroundColor Yellow
    
    $installScript = Join-Path $NoaRoot "scripts/install-profile.ps1"
    if (Test-Path $installScript) {
        & $installScript
    } else {
        Write-Host "  [SKIP] Profile install script not found" -ForegroundColor Yellow
    }
} else {
    Write-Host ""
    Write-Host "6. Skipping profile integration (use -SkipProfile:$false to enable)" -ForegroundColor Yellow
}

# ============================================
# 7. Verify Setup
# ============================================
Write-Host ""
Write-Host "7. Verifying setup..." -ForegroundColor Yellow

$verifyScript = Join-Path $NoaRoot "scripts/verify-environment.ps1"
if (Test-Path $verifyScript) {
    & $verifyScript
}

# ============================================
# Summary
# ============================================
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "Setup Complete!" -ForegroundColor Green
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Edit .env file with your API keys" -ForegroundColor White
Write-Host "  2. Restart PowerShell or run: . .\noa-env.ps1" -ForegroundColor White
Write-Host "  3. Test with: noa status" -ForegroundColor White
Write-Host ""
