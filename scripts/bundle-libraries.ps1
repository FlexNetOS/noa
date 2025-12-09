<#
.SYNOPSIS
    NOA Library Bundling System for Windows

.DESCRIPTION
    Copies all required DLLs for a binary to $NOA_ROOT/lib
    Windows equivalent of scripts/bundle-libraries (bash)

.PARAMETER TargetBinary
    Path to the binary to analyze

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.EXAMPLE
    .\bundle-libraries.ps1 -TargetBinary "bin\myapp.exe"
#>

param(
    [Parameter(Mandatory=$true)]
    [string]$TargetBinary,

    [string]$NoaRoot
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

$NOA_LIB = Join-Path $NoaRoot "lib"

if (-not (Test-Path $TargetBinary)) {
    Write-Error "Binary not found: $TargetBinary"
    exit 1
}

Write-Host "Bundling libraries for: $TargetBinary" -ForegroundColor Cyan

# Create lib directory
if (-not (Test-Path $NOA_LIB)) {
    New-Item -ItemType Directory -Path $NOA_LIB -Force | Out-Null
}

# Get DLL dependencies using dumpbin (requires Visual Studio) or Dependencies tool
function Get-DllDependencies {
    param([string]$Binary)

    $deps = @()

    # Try dumpbin first (Visual Studio)
    $dumpbin = Get-Command "dumpbin.exe" -ErrorAction SilentlyContinue
    if ($dumpbin) {
        $output = & dumpbin /dependents $Binary 2>$null
        $deps = $output | Where-Object { $_ -match '^\s+(\S+\.dll)$' } | ForEach-Object { $Matches[1] }
        return $deps
    }

    # Fallback: use Dependencies CLI if available
    $depsCli = Join-Path $NoaRoot "bin/Dependencies.exe"
    if (Test-Path $depsCli) {
        $output = & $depsCli -json $Binary 2>$null | ConvertFrom-Json
        return $output.Dependencies | Where-Object { $_.Path } | ForEach-Object { $_.Path }
    }

    # Manual: look for common DLLs in same directory
    $binaryDir = Split-Path $Binary -Parent
    return Get-ChildItem -Path $binaryDir -Filter "*.dll" -ErrorAction SilentlyContinue | Select-Object -ExpandProperty FullName
}

# Get dependencies
$dependencies = Get-DllDependencies -Binary $TargetBinary

# Copy each dependency
foreach ($dll in $dependencies) {
    $dllName = Split-Path $dll -Leaf

    # Skip system DLLs (in Windows directory)
    if ($dll -like "$env:SystemRoot*" -or $dll -like "$env:windir*") {
        continue
    }

    # Find the DLL
    $sourcePath = $null
    if (Test-Path $dll) {
        $sourcePath = $dll
    } else {
        # Search common paths
        $searchPaths = @(
            (Split-Path $TargetBinary -Parent),
            (Join-Path $NoaRoot "bin"),
            (Join-Path $NoaRoot "opt"),
            $env:PATH -split ';'
        ) | Where-Object { $_ }

        foreach ($searchPath in $searchPaths) {
            $candidate = Join-Path $searchPath $dllName
            if (Test-Path $candidate) {
                $sourcePath = $candidate
                break
            }
        }
    }

    if ($sourcePath) {
        $destPath = Join-Path $NOA_LIB $dllName
        if (-not (Test-Path $destPath)) {
            Copy-Item -Path $sourcePath -Destination $destPath -Force
            Write-Host "  Bundled: $dllName" -ForegroundColor Green
        }
    }
}

Write-Host "Library bundling complete" -ForegroundColor Green
Write-Host "Libraries in: $NOA_LIB" -ForegroundColor Gray

