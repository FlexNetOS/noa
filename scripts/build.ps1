[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [ValidateSet('debug','release')]
    [string]$Configuration = 'debug'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot

$cmd = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cmd) {
    throw "cargo not found on PATH"
}

$profile = if ($Configuration -eq 'release') { '--release' } else { '' }

Push-Location $root
try {
    if (Test-Path -LiteralPath (Join-Path $root 'sys/core/Cargo.toml')) {
        Push-Location (Join-Path $root 'sys/core')
        try {
            Write-Host "Building sys/core ($Configuration)..."
            if ($profile) { cargo build $profile } else { cargo build }
        }
        finally {
            Pop-Location
        }
    }

    if (Test-Path -LiteralPath (Join-Path $root 'sys/desktop/Cargo.toml')) {
        Push-Location (Join-Path $root 'sys/desktop')
        try {
            Write-Host "Building sys/desktop ($Configuration)..."
            if ($profile) { cargo build $profile } else { cargo build }
        }
        finally {
            Pop-Location
        }
    }

    if (Test-Path -LiteralPath (Join-Path $root 'coordinator-plane/Cargo.toml')) {
        Push-Location (Join-Path $root 'coordinator-plane')
        try {
            Write-Host "Building coordinator-plane ($Configuration)..."
            if ($profile) { cargo build $profile } else { cargo build }
        }
        finally {
            Pop-Location
        }
    }
}
finally {
    Pop-Location
}
