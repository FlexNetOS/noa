<#
.SYNOPSIS
    NOA Build Script (Windows)

.DESCRIPTION
    Builds all NOA components: Rust, Go, TypeScript, Python

.PARAMETER Component
    Component to build: all, rust, go, ui, digest

.EXAMPLE
    .\build.ps1
    Build all components

.EXAMPLE
    .\build.ps1 -Component rust
    Build only Rust components
#>

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet("all", "rust", "go", "ui", "digest")]
    [string]$Component = "all"
)

$ErrorActionPreference = "Stop"

$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent (Split-Path -Parent $PSScriptRoot) }

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        default { "Cyan" }
    }
    Write-Host "[$Level] $Message" -ForegroundColor $color
}

function Build-Rust {
    Write-Log "Building Rust components..."
    Push-Location "$NoaRoot\sys\core"
    try {
        if (Get-Command cargo -ErrorAction SilentlyContinue) {
            & cargo build --release
            Write-Log "Rust build complete" -Level Success
        } else {
            Write-Log "cargo not found, skipping Rust build" -Level Warning
        }
    } finally {
        Pop-Location
    }
}

function Build-Go {
    Write-Log "Building Go components..."
    Push-Location "$NoaRoot\p2p"
    try {
        if (Get-Command go -ErrorAction SilentlyContinue) {
            & go build -o "$NoaRoot\bin\noa-p2p.exe" ./cmd/p2p-node
            Write-Log "Go build complete" -Level Success
        } else {
            Write-Log "go not found, skipping Go build" -Level Warning
        }
    } finally {
        Pop-Location
    }
}

function Build-UI {
    Write-Log "Building UI components..."
    Push-Location "$NoaRoot\sys\ui"
    try {
        if (Get-Command npm -ErrorAction SilentlyContinue) {
            & npm install
            & npm run build
            Write-Log "UI build complete" -Level Success
        } else {
            Write-Log "npm not found, skipping UI build" -Level Warning
        }
    } finally {
        Pop-Location
    }
}

function Build-Digest {
    Write-Log "Building Digest pipeline..."
    Push-Location "$NoaRoot\sys\digest"
    try {
        if (Get-Command pip -ErrorAction SilentlyContinue) {
            & pip install -e .
            Write-Log "Digest build complete" -Level Success
        } else {
            Write-Log "pip not found, skipping Digest build" -Level Warning
        }
    } finally {
        Pop-Location
    }
}

function Build-All {
    Write-Log "Building all NOA components..."
    Build-Rust
    Build-Go
    Build-UI
    Build-Digest
    Write-Log "All builds complete!" -Level Success
}

# Main
switch ($Component) {
    "rust"   { Build-Rust }
    "go"     { Build-Go }
    "ui"     { Build-UI }
    "digest" { Build-Digest }
    "all"    { Build-All }
}

