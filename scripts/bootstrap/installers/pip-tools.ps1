<#
.SYNOPSIS
    Install Python quality tools (ruff, semgrep) via pip

.DESCRIPTION
    Installs Python tools to the portable Python venv.
    Per NOA Constitution 3.1: Self-contained installation.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall even if already installed
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT }
    else { Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))) }
}

$PythonRoot = Join-Path $NoaRoot "opt/python"
$VenvRoot = Join-Path $NoaRoot "opt/venv"
$BinDir = Join-Path $NoaRoot "bin"

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        default { "White" }
    }
    $prefix = switch ($Level) {
        "Success" { "[OK]" }
        "Warning" { "[!!]" }
        "Error" { "[XX]" }
        default { "[..]" }
    }
    Write-Host "$prefix $Message" -ForegroundColor $color
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA pip Tools Installer" -ForegroundColor Cyan
Write-Host "Installing: ruff, semgrep" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

# Check if pip is available - try venv first, then portable python, then system
$pipBin = $null
$pipCandidates = @(
    (Join-Path $VenvRoot "Scripts/pip.exe"),
    (Join-Path $PythonRoot "Scripts/pip.exe"),
    (Join-Path $PythonRoot "pip.exe")
)

foreach ($candidate in $pipCandidates) {
    if (Test-Path $candidate) {
        $pipBin = $candidate
        break
    }
}

if (-not $pipBin) {
    # Try system pip
    $pipBin = Get-Command pip -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
    if (-not $pipBin) {
        $pipBin = Get-Command pip3 -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
    }
}

if (-not $pipBin) {
    Write-Log "pip not found. Please install Python first." -Level Error
    Write-Log "Run: scripts/bootstrap/installers/python-portable.ps1" -Level Info
    exit 1
}

Write-Log "Using pip: $pipBin" -Level Info

# Set pip cache to noa_root
$env:PIP_CACHE_DIR = Join-Path $NoaRoot "cache/pip"
if (-not (Test-Path $env:PIP_CACHE_DIR)) {
    New-Item -ItemType Directory -Path $env:PIP_CACHE_DIR -Force | Out-Null
}

# Tools to install
$tools = @(
    "ruff",
    "semgrep"
)

foreach ($tool in $tools) {
    Write-Log "Installing $tool..." -Level Info
    try {
        & $pipBin install --upgrade $tool 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Log "Installed: $tool" -Level Success
        } else {
            Write-Log "Failed to install $tool (pip exit code: $LASTEXITCODE)" -Level Warning
        }
    } catch {
        Write-Log "Error installing $tool : $_" -Level Warning
    }
}

# Verify installations
Write-Host ""
Write-Log "Verifying installations..." -Level Info

# Check for ruff
$ruffBin = Get-Command ruff -ErrorAction SilentlyContinue
if ($ruffBin) {
    $version = & ruff --version 2>&1
    Write-Log "ruff: $version" -Level Success
} else {
    # Check venv/Scripts
    $venvRuff = Join-Path $VenvRoot "Scripts/ruff.exe"
    if (Test-Path $venvRuff) {
        $version = & $venvRuff --version 2>&1
        Write-Log "ruff: $version (venv)" -Level Success
        # Copy to bin
        Copy-Item -Path $venvRuff -Destination (Join-Path $BinDir "ruff.exe") -Force
        Write-Log "Copied to bin/ruff.exe" -Level Success
    } else {
        Write-Log "ruff not found" -Level Warning
    }
}

# Check for semgrep
$semgrepBin = Get-Command semgrep -ErrorAction SilentlyContinue
if ($semgrepBin) {
    $version = & semgrep --version 2>&1 | Select-Object -First 1
    Write-Log "semgrep: $version" -Level Success
} else {
    $venvSemgrep = Join-Path $VenvRoot "Scripts/semgrep.exe"
    if (Test-Path $venvSemgrep) {
        Write-Log "semgrep: installed (venv)" -Level Success
        Copy-Item -Path $venvSemgrep -Destination (Join-Path $BinDir "semgrep.exe") -Force
        Write-Log "Copied to bin/semgrep.exe" -Level Success
    } else {
        Write-Log "semgrep not found" -Level Warning
    }
}

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Green
Write-Host "pip tools installation complete!" -ForegroundColor Green
Write-Host "=" * 60 -ForegroundColor Green

exit 0


