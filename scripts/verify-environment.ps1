<#
.SYNOPSIS
    Verify NOA environment setup across all platforms
    
.DESCRIPTION
    Checks that all required directories, tools, and configurations are properly set up.
    Run this after cloning the repo or when troubleshooting environment issues.
#>

param(
    [switch]$Fix  # Attempt to fix issues automatically
)

$ErrorActionPreference = "Continue"

# Detect NOA_ROOT
$NOA_ROOT = if ($env:NOA_ROOT) { 
    $env:NOA_ROOT 
} else { 
    Split-Path -Parent $PSScriptRoot 
}

Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Environment Verification" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Host "NOA_ROOT: $NOA_ROOT" -ForegroundColor White
Write-Host "Platform: $([System.Environment]::OSVersion.Platform)" -ForegroundColor White
Write-Host "PowerShell: $($PSVersionTable.PSVersion)" -ForegroundColor White
Write-Host ""

$errors = 0
$warnings = 0

# ============================================
# 1. Directory Structure
# ============================================
Write-Host "1. Checking Directory Structure..." -ForegroundColor Yellow

$requiredDirs = @(
    "ai", "ai/shared", "ai/providers", "ai/devices", "ai/orchestration",
    "bin", "config", "containers", "etc", "git", "init", "lib", 
    "logs", "opt", "p2p", "repos", "scripts", "sys", "tmp", "workspace"
)

foreach ($dir in $requiredDirs) {
    $fullPath = Join-Path $NOA_ROOT $dir
    if (Test-Path $fullPath) {
        Write-Host "  [OK] $dir" -ForegroundColor Green
    } else {
        if ($Fix) {
            New-Item -ItemType Directory -Path $fullPath -Force | Out-Null
            Write-Host "  [CREATED] $dir" -ForegroundColor Blue
        } else {
            Write-Host "  [MISSING] $dir" -ForegroundColor Red
            $errors++
        }
    }
}

# ============================================
# 2. Environment Files
# ============================================
Write-Host ""
Write-Host "2. Checking Environment Files..." -ForegroundColor Yellow

$envFiles = @(
    @{ Path = ".noa-env"; Desc = "Bash environment" },
    @{ Path = "noa-env.ps1"; Desc = "PowerShell environment" },
    @{ Path = ".gitignore"; Desc = "Git ignore" },
    @{ Path = ".gitattributes"; Desc = "Git LFS attributes" }
)

foreach ($file in $envFiles) {
    $fullPath = Join-Path $NOA_ROOT $file.Path
    if (Test-Path $fullPath) {
        Write-Host "  [OK] $($file.Path) - $($file.Desc)" -ForegroundColor Green
    } else {
        Write-Host "  [MISSING] $($file.Path) - $($file.Desc)" -ForegroundColor Red
        $errors++
    }
}

# ============================================
# 3. Git Configuration
# ============================================
Write-Host ""
Write-Host "3. Checking Git Configuration..." -ForegroundColor Yellow

$gitConfig = @(
    @{ Key = "user.name"; Required = $true },
    @{ Key = "user.email"; Required = $true },
    @{ Key = "core.editor"; Required = $false },
    @{ Key = "init.defaultBranch"; Required = $false }
)

foreach ($cfg in $gitConfig) {
    $value = git config --global $cfg.Key 2>$null
    if ($value) {
        Write-Host "  [OK] $($cfg.Key) = $value" -ForegroundColor Green
    } elseif ($cfg.Required) {
        Write-Host "  [MISSING] $($cfg.Key)" -ForegroundColor Red
        $errors++
    } else {
        Write-Host "  [NOT SET] $($cfg.Key)" -ForegroundColor Yellow
        $warnings++
    }
}

# Check Git LFS
$lfsInstalled = git lfs version 2>$null
if ($lfsInstalled) {
    Write-Host "  [OK] Git LFS: $lfsInstalled" -ForegroundColor Green
} else {
    Write-Host "  [MISSING] Git LFS not installed" -ForegroundColor Red
    $errors++
}

# ============================================
# 4. Required Tools
# ============================================
Write-Host ""
Write-Host "4. Checking Required Tools..." -ForegroundColor Yellow

$tools = @(
    @{ Name = "git"; Required = $true },
    @{ Name = "gh"; Required = $true; Install = "winget install GitHub.cli" },
    @{ Name = "node"; Required = $false; Install = "winget install OpenJS.NodeJS.LTS" },
    @{ Name = "python"; Required = $false; Install = "winget install Python.Python.3.12" },
    @{ Name = "jq"; Required = $false },
    @{ Name = "rg"; Required = $false }
)

foreach ($tool in $tools) {
    $cmd = Get-Command $tool.Name -ErrorAction SilentlyContinue
    if ($cmd) {
        Write-Host "  [OK] $($tool.Name): $($cmd.Source)" -ForegroundColor Green
    } elseif ($tool.Required) {
        Write-Host "  [MISSING] $($tool.Name)" -ForegroundColor Red
        if ($tool.Install) {
            Write-Host "           Install with: $($tool.Install)" -ForegroundColor Gray
        }
        $errors++
    } else {
        Write-Host "  [OPTIONAL] $($tool.Name) not found" -ForegroundColor Yellow
        $warnings++
    }
}

# ============================================
# 5. GitHub CLI Auth
# ============================================
Write-Host ""
Write-Host "5. Checking GitHub CLI Authentication..." -ForegroundColor Yellow

$ghAuth = gh auth status 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  [OK] GitHub CLI authenticated" -ForegroundColor Green
} else {
    Write-Host "  [NOT AUTHENTICATED] Run: gh auth login" -ForegroundColor Yellow
    $warnings++
}

# ============================================
# 6. Scripts Executable
# ============================================
Write-Host ""
Write-Host "6. Checking Scripts..." -ForegroundColor Yellow

$scripts = @(
    "scripts/noa.ps1",
    "scripts/git-push.ps1",
    "scripts/sync.ps1",
    "scripts/download-static-binaries.ps1"
)

foreach ($script in $scripts) {
    $fullPath = Join-Path $NOA_ROOT $script
    if (Test-Path $fullPath) {
        Write-Host "  [OK] $script" -ForegroundColor Green
    } else {
        Write-Host "  [MISSING] $script" -ForegroundColor Red
        $errors++
    }
}

# ============================================
# Summary
# ============================================
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "Summary" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan

if ($errors -eq 0 -and $warnings -eq 0) {
    Write-Host "All checks passed!" -ForegroundColor Green
} else {
    if ($errors -gt 0) {
        Write-Host "Errors: $errors" -ForegroundColor Red
    }
    if ($warnings -gt 0) {
        Write-Host "Warnings: $warnings" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Quick Commands:" -ForegroundColor Cyan
Write-Host "  Load environment:  . .\noa-env.ps1" -ForegroundColor White
Write-Host "  Validate:          .\scripts\noa.ps1 validate" -ForegroundColor White
Write-Host "  Sync changes:      .\scripts\sync.ps1 'message'" -ForegroundColor White
Write-Host ""

exit $errors
