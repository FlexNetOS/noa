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

.EXAMPLE
    .\setup-noa.ps1 -InstallAllTools -InstallAiProviders
    Full setup with all toolchains and AI provider CLIs (FR-039)
#>

param(
    [Parameter(Mandatory=$false)]
    [string]$NoaRoot = "N:\noa",

    [Parameter(Mandatory=$false)]
    [switch]$InstallPrereqs = $false,

    [Parameter(Mandatory=$false)]
    [switch]$IntegrateProfile = $false,

    [Parameter(Mandatory=$false)]
    [switch]$InstallAllTools = $false,

    [Parameter(Mandatory=$false)]
    [switch]$InstallAiProviders = $false
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

    $jsonOutput = & $checker -Json 2>$null
    $exitCode = $LASTEXITCODE

    if ($exitCode -eq 0) {
        Write-Log "All prerequisites met." -Level Success
        Update-NoaEnvWithToolchains
        return
    }

    $installer = Join-Path $NoaRoot "scripts/setup/install-all-tools.ps1"
    if (-not (Test-Path $installer)) {
        Write-Log "Installer not found: $installer" -Level Error
        & $checker
        return
    }

    Write-Log "Attempting contained installation via install-all-tools.ps1..." -Level Info
    & $installer

    # Update environment file with any newly installed toolchains before re-checking
    Update-NoaEnvWithToolchains

    Write-Log "Re-running prerequisite checker after installation..." -Level Info
    & $checker
    if ($LASTEXITCODE -eq 0) {
        Write-Log "All prerequisites now satisfied!" -Level Success
    } elseif ($LASTEXITCODE -eq 2) {
        Write-Log "Critical tools installed. Some high-priority tools still missing." -Level Warning
    } else {
        Write-Log "Critical prerequisites still missing. Manual installation may be required." -Level Warning
    }
}

function Update-NoaEnvWithToolchains {
    <#
    .SYNOPSIS
        Updates noa-env.ps1 with portable toolchain environment variables
    #>

    $envPath = Join-Path $NoaRoot "noa-env.ps1"
    $NOA_OPT = Join-Path $NoaRoot "opt"

    # Check which portable toolchains are installed
    $toolchainEnv = @()

    # Rust
    $rustCargoHome = Join-Path $NOA_OPT "rust/cargo"
    $rustupHome = Join-Path $NOA_OPT "rust/rustup"
    if (Test-Path (Join-Path $rustCargoHome "bin/rustc.exe")) {
        $toolchainEnv += "`$env:RUSTUP_HOME = `"$rustupHome`""
        $toolchainEnv += "`$env:CARGO_HOME = `"$rustCargoHome`""
        $toolchainEnv += "`$env:PATH = `"$rustCargoHome\bin;`$env:PATH`""
    }

    # Go
    $goRoot = Join-Path $NOA_OPT "go"
    $goPath = Join-Path $NOA_OPT "go/workspace"
    $goBin = Join-Path $goPath "bin"
    if (Test-Path (Join-Path $goRoot "bin/go.exe")) {
        $toolchainEnv += "`$env:GOROOT = `"$goRoot`""
        $toolchainEnv += "`$env:GOPATH = `"$goPath`""
        $toolchainEnv += "`$env:GOBIN = `"$goBin`""
        $toolchainEnv += "`$env:GOCACHE = `"$(Join-Path $NOA_OPT "go/cache")`""
        $toolchainEnv += "`$env:GOMODCACHE = `"$(Join-Path $NOA_OPT "go/pkg/mod")`""
        $toolchainEnv += "`$env:PATH = `"$goRoot\bin;$goBin;`$env:PATH`""
    }

    # Node.js
    $nodeRoot = Join-Path $NOA_OPT "node"
    $npmCache = Join-Path $NOA_OPT "npm-cache"
    if (Test-Path (Join-Path $nodeRoot "node.exe")) {
        $toolchainEnv += "`$env:npm_config_prefix = `"$nodeRoot`""
        $toolchainEnv += "`$env:npm_config_cache = `"$npmCache`""
        $toolchainEnv += "`$env:PATH = `"$nodeRoot;`$env:PATH`""
    }

    # Python
    $pythonRoot = Join-Path $NOA_OPT "python"
    $venvPath = Join-Path $NOA_OPT "venv"
    if (Test-Path (Join-Path $pythonRoot "python.exe")) {
        $toolchainEnv += "`$env:PATH = `"$pythonRoot;$pythonRoot\Scripts;`$env:PATH`""
        if (Test-Path $venvPath) {
            $toolchainEnv += "# Activate venv: & `"$venvPath\Scripts\Activate.ps1`""
        }
    }

    # protoc
    $protobufBin = Join-Path $NOA_OPT "protobuf/bin"
    if (Test-Path (Join-Path $protobufBin "protoc.exe")) {
        $toolchainEnv += "`$env:PATH = `"$protobufBin;`$env:PATH`""
    }

    # NOA bin directory (self-contained utilities)
    $noaBin = Join-Path $NoaRoot "bin"
    $toolchainEnv += "`$env:PATH = `"$noaBin;`$env:PATH`""

    if ($toolchainEnv.Count -gt 0) {
        Write-Log "Updating noa-env.ps1 with portable toolchain paths..." -Level Info

        $envContent = @(
            "# NOA Environment Configuration"
            "# Auto-generated by setup-noa.ps1 - Portable Toolchains"
            "# Last Updated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
            ""
            "# NOA Root"
            "`$env:NOA_ROOT = `"$NoaRoot`""
            ""
            "# Portable Toolchain Configuration"
            "# These paths point to noa_root/opt/ per Constitution §3.1"
        )
        $envContent += $toolchainEnv
        $envContent += @(
            ""
            "# Verify NOA environment is loaded"
            "Write-Host `"NOA environment loaded (portable toolchains active)`" -ForegroundColor Green"
        )

        $envContent -join "`r`n" | Set-Content -Path $envPath -Encoding UTF8
        Write-Log "  Created: noa-env.ps1 with portable toolchain paths" -Level Success
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

    # Create AI provider directories
    $aiProviderDirs = @(
        "ai/providers/cloud/claude-code",
        "ai/providers/cloud/codex",
        "ai/providers/cloud/abacus",
        "ai/providers/local",
        "ai/providers/hybrid",
        "ai/shared/agents",
        "ai/shared/workflows",
        "ai/shared/prompts",
        "ai/shared/skills",
        "ai/shared/tools",
        "ai/shared/models"
    )
    foreach ($dir in $aiProviderDirs) {
        $dirPath = Join-Path $NoaRoot $dir
        if (-not (Test-Path $dirPath)) {
            New-Item -ItemType Directory -Path $dirPath -Force | Out-Null
        }
    }
    Write-Log "  Created AI provider directories" -Level Success

    # Install all tools if requested (calls install-all-tools.ps1)
    if ($InstallAllTools -or $InstallAiProviders) {
        Write-Log "Installing toolchains and utilities..." -Level Info

        $installAllScript = Join-Path $NoaRoot "scripts/setup/install-all-tools.ps1"
        if (Test-Path $installAllScript) {
            if ($InstallAllTools) {
                Write-Log "  Running full tool installation..." -Level Info
                & $installAllScript -NoaRoot $NoaRoot
            } elseif ($InstallAiProviders) {
                Write-Log "  Installing AI Provider CLIs only (FR-039)..." -Level Info
                & $installAllScript -NoaRoot $NoaRoot -Tool "node","ai-providers"
            }

            if ($LASTEXITCODE -eq 0) {
                Write-Log "  Tool installation complete" -Level Success
            } else {
                Write-Log "  Some tools may have failed to install" -Level Warning
            }
        } else {
            Write-Log "  install-all-tools.ps1 not found: $installAllScript" -Level Warning
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
        '# MCP / Provider Secrets (local-only)'
        '# Create (locally, not in repo): %USERPROFILE%\.noa\secrets.ps1'
        '# Example: $env:GITHUB_PERSONAL_ACCESS_TOKEN = "github_pat_..."'
        '$noaSecretsPath = Join-Path $env:USERPROFILE ".noa\secrets.ps1"'
        'if (Test-Path $noaSecretsPath) { . $noaSecretsPath }'
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
