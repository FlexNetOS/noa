<#
.SYNOPSIS
    Download static binaries for common tools (Windows version)
    
.DESCRIPTION
    Downloads portable executables for Windows that don't require installation.
#>

$NOA_ROOT = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
$NOA_BIN = Join-Path $NOA_ROOT "bin"

# Create bin directory if it doesn't exist
if (-not (Test-Path $NOA_BIN)) {
    New-Item -ItemType Directory -Path $NOA_BIN -Force | Out-Null
}

Write-Host "Downloading static binaries to $NOA_BIN..." -ForegroundColor Cyan

# Helper function
function Download-Binary {
    param(
        [string]$Name,
        [string]$Url,
        [string]$Dest
    )
    
    Write-Host "  Downloading $Name..." -ForegroundColor White
    try {
        Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing -ErrorAction Stop
        Write-Host "    [OK] $Name installed" -ForegroundColor Green
        return $true
    } catch {
        Write-Host "    [FAILED] $Name - $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

# jq for Windows
$jqPath = Join-Path $NOA_BIN "jq.exe"
if (-not (Test-Path $jqPath)) {
    Download-Binary "jq" "https://github.com/jqlang/jq/releases/latest/download/jq-win64.exe" $jqPath
}

# ripgrep for Windows
$rgPath = Join-Path $NOA_BIN "rg.exe"
if (-not (Test-Path $rgPath)) {
    Write-Host "  Downloading ripgrep..." -ForegroundColor White
    try {
        $rgRelease = Invoke-RestMethod "https://api.github.com/repos/BurntSushi/ripgrep/releases/latest"
        $rgAsset = $rgRelease.assets | Where-Object { $_.name -match "x86_64-pc-windows-msvc.zip" } | Select-Object -First 1
        if ($rgAsset) {
            $rgZip = Join-Path $env:TEMP "rg.zip"
            Invoke-WebRequest -Uri $rgAsset.browser_download_url -OutFile $rgZip -UseBasicParsing
            Expand-Archive -Path $rgZip -DestinationPath $env:TEMP -Force
            $rgExe = Get-ChildItem -Path $env:TEMP -Filter "rg.exe" -Recurse | Select-Object -First 1
            if ($rgExe) {
                Copy-Item $rgExe.FullName -Destination $rgPath -Force
                Write-Host "    [OK] ripgrep installed" -ForegroundColor Green
            }
            Remove-Item $rgZip -Force -ErrorAction SilentlyContinue
        }
    } catch {
        Write-Host "    [FAILED] ripgrep - $($_.Exception.Message)" -ForegroundColor Red
    }
}

# fd for Windows
$fdPath = Join-Path $NOA_BIN "fd.exe"
if (-not (Test-Path $fdPath)) {
    Write-Host "  Downloading fd..." -ForegroundColor White
    try {
        $fdRelease = Invoke-RestMethod "https://api.github.com/repos/sharkdp/fd/releases/latest"
        $fdAsset = $fdRelease.assets | Where-Object { $_.name -match "x86_64-pc-windows-msvc.zip" } | Select-Object -First 1
        if ($fdAsset) {
            $fdZip = Join-Path $env:TEMP "fd.zip"
            Invoke-WebRequest -Uri $fdAsset.browser_download_url -OutFile $fdZip -UseBasicParsing
            Expand-Archive -Path $fdZip -DestinationPath $env:TEMP -Force
            $fdExe = Get-ChildItem -Path $env:TEMP -Filter "fd.exe" -Recurse | Select-Object -First 1
            if ($fdExe) {
                Copy-Item $fdExe.FullName -Destination $fdPath -Force
                Write-Host "    [OK] fd installed" -ForegroundColor Green
            }
            Remove-Item $fdZip -Force -ErrorAction SilentlyContinue
        }
    } catch {
        Write-Host "    [FAILED] fd - $($_.Exception.Message)" -ForegroundColor Red
    }
}

# bat for Windows
$batPath = Join-Path $NOA_BIN "bat.exe"
if (-not (Test-Path $batPath)) {
    Write-Host "  Downloading bat..." -ForegroundColor White
    try {
        $batRelease = Invoke-RestMethod "https://api.github.com/repos/sharkdp/bat/releases/latest"
        $batAsset = $batRelease.assets | Where-Object { $_.name -match "x86_64-pc-windows-msvc.zip" } | Select-Object -First 1
        if ($batAsset) {
            $batZip = Join-Path $env:TEMP "bat.zip"
            Invoke-WebRequest -Uri $batAsset.browser_download_url -OutFile $batZip -UseBasicParsing
            Expand-Archive -Path $batZip -DestinationPath $env:TEMP -Force
            $batExe = Get-ChildItem -Path $env:TEMP -Filter "bat.exe" -Recurse | Select-Object -First 1
            if ($batExe) {
                Copy-Item $batExe.FullName -Destination $batPath -Force
                Write-Host "    [OK] bat installed" -ForegroundColor Green
            }
            Remove-Item $batZip -Force -ErrorAction SilentlyContinue
        }
    } catch {
        Write-Host "    [FAILED] bat - $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "Static binary download complete" -ForegroundColor Green
Write-Host "Binaries installed to: $NOA_BIN" -ForegroundColor Cyan

# Verify
Write-Host ""
Write-Host "Installed binaries:" -ForegroundColor Yellow
Get-ChildItem $NOA_BIN -Filter "*.exe" | ForEach-Object { Write-Host "  - $($_.Name)" }
