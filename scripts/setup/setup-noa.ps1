<#
.SYNOPSIS
    NOA Environment Setup Script for Windows

.DESCRIPTION
    Creates the NOA directory structure, generates configuration files,
    and optionally installs prerequisites and integrates with PowerShell profile.

.PARAMETER NoaRoot
    Root directory for NOA installation. Defaults to N:\noa
    Accepts any valid path including UNC paths.

.PARAMETER InstallPrereqs
    If specified, runs the unified prerequisite checker (shimmed) and attempts installs.
    Uses winget when available; skipped in CI environments.

.PARAMETER IntegrateProfile
    If specified, adds NOA profile source line to PowerShell profile.
    Default is $false for CI/scripting scenarios.

.EXAMPLE
    .\setup-noa.ps1
    Run setup with default settings (N:\noa, no prereqs, no profile integration)

.EXAMPLE
    .\setup-noa.ps1 -NoaRoot "C:\noa" -IntegrateProfile
    Install to C:\noa and integrate with PowerShell profile

.EXAMPLE
    .\setup-noa.ps1 -NoaRoot "$env:TEMP\noa" -InstallPrereqs:$false -IntegrateProfile:$false
    Install to temp directory for testing (CI scenario)
#>

param(
    [Parameter(Mandatory=$false)]
    [string]$NoaRoot = "N:\noa",

    [Parameter(Mandatory=$false)]
    [switch]$InstallPrereqs = $false,

    [Parameter(Mandatory=$false)]
    [switch]$IntegrateProfile = $false
)

$ErrorActionPreference = "Stop"

# Script metadata
$ScriptVersion = "2.0.0"
$ScriptName = "NOA Setup"

#region Helper Functions

function Write-Log {
    param(
        [string]$Message,
        [ValidateSet('Info', 'Success', 'Warning', 'Error')]
        [string]$Level = 'Info'
    )

    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "$timestamp [$Level] $Message"

    # Write to log file if log directory exists and log file is set
    if ($script:LogFile -and (Test-Path (Join-Path $NoaRoot "logs"))) {
        $logMessage | Add-Content -Path $script:LogFile -Encoding UTF8
    }

    # Write to console with color
    $color = switch ($Level) {
        'Success' { 'Green' }
        'Warning' { 'Yellow' }
        'Error'   { 'Red' }
        default   { 'White' }
    }

    $prefix = switch ($Level) {
        'Success' { '[✓]' }
        'Warning' { '[!]' }
        'Error'   { '[✗]' }
        default   { '[i]' }
    }

    Write-Host "$prefix $Message" -ForegroundColor $color
}

function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Invoke-PrereqInstallation {
    Write-Log "Checking prerequisites via unified checker..." -Level Info

    $shimPrereqs = Join-Path $NoaRoot "scripts/powershell/check-prerequisites.ps1"
    $directPrereqs = Join-Path $NoaRoot "scripts/setup/check-prereqs.ps1"
    $checker = if (Test-Path $shimPrereqs) { $shimPrereqs } else { $directPrereqs }

    if (-not (Test-Path $checker)) {
        Write-Log "Prereq checker not found: $checker" -Level Error
        return
    }

    & $checker
    $exitCode = $LASTEXITCODE

    if ($exitCode -eq 0) {
        Write-Log "All prerequisites met." -Level Success
        return
    }

    $installer = Join-Path $NoaRoot "scripts/setup/install-prereqs.ps1"
    if (-not (Test-Path $installer)) {
        Write-Log "Installer shim not found: $installer" -Level Error
        return
    }

    Write-Log "Attempting prerequisite installation (winget-based)..." -Level Info
    & $installer

    & $checker
    if ($LASTEXITCODE -eq 0) {
        Write-Log "Prerequisites satisfied after installation." -Level Success
    } else {
        Write-Log "Prerequisites still missing; see checker output above." -Level Warning
    }
}

#endregion

#region Main Setup

try {
    # Banner
    Write-Host ""
    Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║                                                            ║" -ForegroundColor Cyan
    Write-Host "║           $ScriptName v$ScriptVersion                     ║" -ForegroundColor Cyan
    Write-Host "║                                                            ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""

    # Normalize path
    $NoaRoot = [System.IO.Path]::GetFullPath($NoaRoot)
    Write-Log "NOA Root: $NoaRoot" -Level Info

    # Create base directory
    if (-not (Test-Path $NoaRoot)) {
        Write-Log "Creating root directory..." -Level Info
        New-Item -ItemType Directory -Path $NoaRoot -Force | Out-Null
        Write-Log "Root directory created" -Level Success
    } else {
        Write-Log "Root directory exists" -Level Info
    }

    # Initialize log file
    $logsDir = Join-Path $NoaRoot "logs"
    if (-not (Test-Path $logsDir)) {
        New-Item -ItemType Directory -Path $logsDir -Force | Out-Null
    }

    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $script:LogFile = Join-Path $logsDir "setup-$timestamp.log"
    Write-Log "=== NOA Setup Started ===" -Level Info
    Write-Log "Log file: $script:LogFile" -Level Info

    # Install prerequisites if requested
    if ($InstallPrereqs -and -not $env:CI) {
        Invoke-PrereqInstallation
    } elseif ($InstallPrereqs -and $env:CI) {
        Write-Log "Skipping prerequisite installation in CI environment" -Level Info
    }

    # Create directory structure
    Write-Log "Creating NOA directory structure..." -Level Info

    $directories = @(
        "repos",
        "containers",
        "workspace",
        "config",
        "scripts",
        "logs",
        "tmp",
        "p2p",
        "ai",
        "git",
        "bin",
        "etc",
        "lib",
        "opt",
        "sys",
        "init"
    )

    foreach ($dir in $directories) {
        $dirPath = Join-Path $NoaRoot $dir
        if (-not (Test-Path $dirPath)) {
            New-Item -ItemType Directory -Path $dirPath -Force | Out-Null
            Write-Log "  Created: $dir" -Level Success
        } else {
            Write-Log "  Exists: $dir" -Level Info
        }
    }

    # Generate noa-profile.ps1
    Write-Log "Generating noa-profile.ps1..." -Level Info

    $profilePath = Join-Path $NoaRoot "noa-profile.ps1"

    # Build profile content using array (quote-safe approach)
    $profileLines = @(
        '# NOA Environment Profile'
        '# Auto-generated by setup-noa.ps1'
        ''
        "# Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
        ''
        '# Environment Variables'
        "`$env:NOA_ROOT = `"$NoaRoot`""
        '$env:NOA_REPOS = "$env:NOA_ROOT\repos"'
        '$env:NOA_CONTAINERS = "$env:NOA_ROOT\containers"'
        '$env:NOA_WORKSPACE = "$env:NOA_ROOT\workspace"'
        '$env:NOA_CONFIG = "$env:NOA_ROOT\config"'
        '$env:NOA_SCRIPTS = "$env:NOA_ROOT\scripts"'
        '$env:NOA_LOGS = "$env:NOA_ROOT\logs"'
        '$env:NOA_TMP = "$env:NOA_ROOT\tmp"'
        '$env:NOA_P2P = "$env:NOA_ROOT\p2p"'
        '$env:NOA_AI = "$env:NOA_ROOT\ai"'
        '$env:NOA_GIT = "$env:NOA_ROOT\git"'
        '$env:NOA_BIN = "$env:NOA_ROOT\bin"'
        ''
        '# Navigation Helper Functions'
        'function cda { Set-Location $env:NOA_ROOT }'
        'function cdr { Set-Location $env:NOA_REPOS }'
        'function cdc { Set-Location $env:NOA_CONTAINERS }'
        'function cdw { Set-Location $env:NOA_WORKSPACE }'
        'function cds { Set-Location $env:NOA_SCRIPTS }'
        'function cdl { Set-Location $env:NOA_LOGS }'
        'function cdp { Set-Location $env:NOA_P2P }'
        'function cdai { Set-Location $env:NOA_AI }'
        'function cdgit { Set-Location $env:NOA_GIT }'
        ''
        '# Status indicator'
        'Write-Host "NOA environment loaded from: $env:NOA_ROOT" -ForegroundColor Green'
    )

    $profileLines -join "`r`n" | Set-Content -Path $profilePath -Encoding UTF8
    Write-Log "  Created: noa-profile.ps1" -Level Success

    # Create .noa marker file
    Write-Log "Creating .noa marker file..." -Level Info
    $markerPath = Join-Path $NoaRoot ".noa"
    $markerContent = @(
        "# NOA Root Directory Marker"
        "# Created: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
        "version=$ScriptVersion"
        "root=$NoaRoot"
    )
    $markerContent -join "`r`n" | Set-Content -Path $markerPath -Encoding UTF8
    Write-Log "  Created: .noa" -Level Success

    # Create config/noa.json
    Write-Log "Creating config/noa.json..." -Level Info
    $configPath = Join-Path $NoaRoot "config\noa.json"
    $configContent = @'
{
  "version": "2.0.0",
  "name": "NOA",
  "description": "Network Orchestration and Automation",
  "created": "TIMESTAMP_PLACEHOLDER",
  "root": "ROOT_PLACEHOLDER",
  "directories": {
    "repos": "Git repositories",
    "containers": "Container images and configs",
    "workspace": "Active project workspace",
    "config": "Configuration files",
    "scripts": "Automation scripts",
    "logs": "Log files",
    "tmp": "Temporary files",
    "p2p": "Peer-to-peer networking",
    "ai": "AI models and configs",
    "git": "Git workflows and hooks",
    "bin": "Executables",
    "etc": "Additional configuration",
    "lib": "Libraries",
    "opt": "Optional packages",
    "sys": "System files",
    "init": "Initialization scripts"
  }
}
'@

    $configContent = $configContent.Replace('TIMESTAMP_PLACEHOLDER', (Get-Date -Format 'yyyy-MM-dd HH:mm:ss'))
    $configContent = $configContent.Replace('ROOT_PLACEHOLDER', $NoaRoot.Replace('\', '\\'))

    $configContent | Set-Content -Path $configPath -Encoding UTF8
    Write-Log "  Created: config\noa.json" -Level Success

    # Integrate with PowerShell profile if requested
    if ($IntegrateProfile) {
        Write-Log "Integrating with PowerShell profile..." -Level Info

        $profileScriptPath = $PROFILE.CurrentUserAllHosts
        $sourceLine = ". `"$profilePath`""

        # Create profile directory if it doesn't exist
        $profileDir = Split-Path $profileScriptPath -Parent
        if (-not (Test-Path $profileDir)) {
            New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
            Write-Log "  Created profile directory: $profileDir" -Level Success
        }

        # Check if source line already exists
        $profileExists = Test-Path $profileScriptPath
        $alreadyIntegrated = $false

        if ($profileExists) {
            $profileContent = Get-Content $profileScriptPath -Raw -ErrorAction SilentlyContinue
            $alreadyIntegrated = $profileContent -match [regex]::Escape($sourceLine)
        }

        if ($alreadyIntegrated) {
            Write-Log "  Profile already integrated" -Level Info
        } else {
            # Append source line
            $sourceLine | Add-Content -Path $profileScriptPath -Encoding UTF8
            Write-Log "  Profile integration complete" -Level Success
            Write-Log "  Added to: $profileScriptPath" -Level Info
        }
    }

    # Summary
    Write-Host ""
    Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                                                            ║" -ForegroundColor Green
    Write-Host "║              Setup Completed Successfully!                 ║" -ForegroundColor Green
    Write-Host "║                                                            ║" -ForegroundColor Green
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""

    Write-Log "=== Setup Summary ===" -Level Info
    Write-Log "Root Directory: $NoaRoot" -Level Info
    Write-Log "Profile Script: $profilePath" -Level Info
    Write-Log "Config File: $configPath" -Level Info
    Write-Log "Log File: $script:LogFile" -Level Info

    Write-Host ""
    Write-Host "Next Steps:" -ForegroundColor Yellow
    Write-Host "  1. Load the environment:" -ForegroundColor White
    Write-Host "     . `"$profilePath`"" -ForegroundColor Cyan
    Write-Host ""

    if (-not $IntegrateProfile) {
        Write-Host "  2. (Optional) To auto-load NOA in all PowerShell sessions:" -ForegroundColor White
        Write-Host "     Run setup again with -IntegrateProfile" -ForegroundColor Cyan
        Write-Host ""
    }

    Write-Host "  Navigation commands available after loading profile:" -ForegroundColor White
    Write-Host "     cda   - Navigate to NOA root" -ForegroundColor Cyan
    Write-Host "     cdr   - Navigate to repos" -ForegroundColor Cyan
    Write-Host "     cdw   - Navigate to workspace" -ForegroundColor Cyan
    Write-Host ""

    Write-Log "=== NOA Setup Completed Successfully ===" -Level Success

    exit 0

} catch {
    Write-Log "Setup failed: $_" -Level Error
    Write-Log "Stack trace: $($_.ScriptStackTrace)" -Level Error

    Write-Host ""
    Write-Host "Setup failed. See log file for details: $script:LogFile" -ForegroundColor Red
    Write-Host ""

    exit 1
}

#endregion
