<#
.SYNOPSIS
    NOA Build Script (Windows)

.DESCRIPTION
    Builds all NOA components: Rust, Go, TypeScript, Python

.PARAMETER Component
    Component to build: all, rust, p2p, ui, digest

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
    [ValidateSet('all', 'rust', 'p2p', 'go', 'ui', 'digest')]
    [string]$Component = 'all'
)

$ErrorActionPreference = 'Stop'

$NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent (Split-Path -Parent $PSScriptRoot) }

function Write-Log {
    param([string]$Message, [string]$Level = 'Info')
    $color = switch ($Level) {
        'Success' { 'Green' }
        'Warning' { 'Yellow' }
        'Error' { 'Red' }
        default { 'Cyan' }
    }
    Write-Host "[$Level] $Message" -ForegroundColor $color
}

function Build-Rust {
    Write-Log 'Building Rust components (sys/core)...'
    Push-Location "$NoaRoot\sys\core"
    try {
        if (Get-Command cargo -ErrorAction SilentlyContinue) {
            & cargo build --release
            Write-Log 'Rust build complete' -Level Success
        }
        else {
            Write-Log 'cargo not found, skipping Rust build' -Level Warning
        }
    }
    finally {
        Pop-Location
    }
}

function Build-P2P {
    Write-Log 'Building P2P components (rust-libp2p)...'
    Push-Location "$NoaRoot\p2p"
    try {
        if (Get-Command cargo -ErrorAction SilentlyContinue) {
            # Build the workspace (library and examples)
            & cargo build --release
            Write-Log 'P2P build complete' -Level Success
        }
        else {
            Write-Log 'cargo not found, skipping P2P build' -Level Warning
        }
    }
    finally {
        Pop-Location
    }
}

function Build-Go {
    Write-Log 'Go p2p-node has been replaced by rust-libp2p. Use -Component p2p to build.' -Level Warning
}

function Build-UI {
    Write-Log 'Building UI components...'
    Push-Location "$NoaRoot\sys\ui"
    try {
        if (Get-Command pnpm -ErrorAction SilentlyContinue) {
            & pnpm install --frozen-lockfile
            & pnpm run build
            Write-Log 'UI build complete' -Level Success
        }
        elseif (Get-Command corepack -ErrorAction SilentlyContinue) {
            & corepack pnpm install --frozen-lockfile
            & corepack pnpm run build
            Write-Log 'UI build complete' -Level Success
        }
        else {
            Write-Log 'pnpm/corepack not found, skipping UI build' -Level Warning
        }
    }
    finally {
        Pop-Location
    }
}

function Build-Digest {
    Write-Log 'Building Digest pipeline...'
    Push-Location "$NoaRoot\sys\digest"
    try {
        if (Get-Command pip -ErrorAction SilentlyContinue) {
            & pip install -e .
            Write-Log 'Digest build complete' -Level Success
        }
        else {
            Write-Log 'pip not found, skipping Digest build' -Level Warning
        }
    }
    finally {
        Pop-Location
    }
}

function Build-All {
    Write-Log 'Building all NOA components...'
    Build-Rust
    Build-P2P
    Build-UI
    Build-Digest
    Write-Log 'All builds complete!' -Level Success
}

# Main
switch ($Component) {
    'rust' { Build-Rust }
    'p2p' { Build-P2P }
    'go' { Build-Go }
    'ui' { Build-UI }
    'digest' { Build-Digest }
    'all' { Build-All }
}

