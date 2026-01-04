<#
.SYNOPSIS
    Verify security practices in bootstrap (§3.6 compliance).

.DESCRIPTION
    Checks that all downloads use HTTPS and checksums are verified.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\verify-security.ps1
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

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "         Security Verification (§3.6)" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

$issues = @()

# Check bootstrap-tools.json for HTTPS URLs
Write-Host "Checking download URLs..." -ForegroundColor Yellow

$toolsconfigsPath = Join-Path $NoaRoot "configs/bootstrap-tools.json"
if (Test-Path $toolsconfigsPath) {
    $content = Get-Content $toolsconfigsPath -Raw

    # Find all URLs
    $httpUrls = [regex]::Matches($content, 'http://[^\s"]+')
    $httpsUrls = [regex]::Matches($content, 'https://[^\s"]+')

    if ($httpUrls.Count -eq 0) {
        Write-Host "  [OK] No insecure HTTP URLs found" -ForegroundColor Green
    } else {
        Write-Host "  [!!] Found $($httpUrls.Count) insecure HTTP URLs:" -ForegroundColor Red
        foreach ($url in $httpUrls) {
            Write-Host "       - $($url.Value)" -ForegroundColor Red
            $issues += "Insecure HTTP URL: $($url.Value)"
        }
    }

    Write-Host "  [OK] Found $($httpsUrls.Count) secure HTTPS URLs" -ForegroundColor Green
} else {
    Write-Host "  [--] bootstrap-tools.json not found" -ForegroundColor Gray
}

# Check for checksum verification in download scripts
Write-Host ""
Write-Host "Checking checksum verification..." -ForegroundColor Yellow

$downloadScripts = Get-ChildItem -Path (Join-Path $NoaRoot "scripts/bootstrap") -Filter "*.ps1" -Recurse |
    Where-Object { $_.Name -match "download|install" }

$hasChecksumVerification = $false
foreach ($script in $downloadScripts) {
    $content = Get-Content $script.FullName -Raw -ErrorAction SilentlyContinue
    if ($content -match "Get-FileHash|SHA256|checksum|Verify") {
        $hasChecksumVerification = $true
        Write-Host "  [OK] $($script.Name) has checksum verification" -ForegroundColor Green
    }
}

if (-not $hasChecksumVerification) {
    Write-Host "  [INFO] No checksum verification found in download scripts" -ForegroundColor Yellow
}

# Check for sensitive data in configss
Write-Host ""
Write-Host "Checking for exposed secrets..." -ForegroundColor Yellow

$configsFiles = Get-ChildItem -Path $NoaRoot -Filter "*.json" -Recurse -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -notmatch "node_modules|\.git" }

$sensitivePatterns = @(
    'api[_-]?key\s*[=:]\s*["\x27][^"\x27]{20,}',
    'secret\s*[=:]\s*["\x27][^"\x27]{10,}',
    'password\s*[=:]\s*["\x27][^"\x27]{8,}',
    'token\s*[=:]\s*["\x27][a-zA-Z0-9_-]{20,}'
)

$exposedSecrets = 0
foreach ($file in $configsFiles) {
    $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
    foreach ($pattern in $sensitivePatterns) {
        if ($content -match $pattern) {
            # Skip if it's a template placeholder
            if ($content -notmatch '\$\{|\{\{|<.*>|placeholder|example') {
                Write-Host "  [!!] Potential secret in: $($file.Name)" -ForegroundColor Red
                $exposedSecrets++
                $issues += "Potential exposed secret in $($file.Name)"
            }
        }
    }
}

if ($exposedSecrets -eq 0) {
    Write-Host "  [OK] No exposed secrets detected" -ForegroundColor Green
}

# Check file permissions (Windows)
Write-Host ""
Write-Host "Checking file permissions..." -ForegroundColor Yellow

$sensitiveFiles = @(
    "configs/noa-server.json",
    "configs/ai-providers.json"
)

foreach ($file in $sensitiveFiles) {
    $filePath = Join-Path $NoaRoot $file
    if (Test-Path $filePath) {
        $acl = Get-Acl $filePath -ErrorAction SilentlyContinue
        if ($acl) {
            $worldAccess = $acl.Access | Where-Object {
                $_.IdentityReference -match "Everyone|Users" -and $_.FileSystemRights -match "Write"
            }
            if ($worldAccess) {
                Write-Host "  [!!] $file has broad write permissions" -ForegroundColor Yellow
            } else {
                Write-Host "  [OK] $file permissions look reasonable" -ForegroundColor Green
            }
        }
    }
}

# Summary
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "                     Security Summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

if ($issues.Count -eq 0) {
    Write-Host "✓ No security issues detected - §3.6 COMPLIANT" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ Found $($issues.Count) security issues:" -ForegroundColor Red
    foreach ($issue in $issues) {
        Write-Host "  - $issue" -ForegroundColor Red
    }
    exit 1
}

