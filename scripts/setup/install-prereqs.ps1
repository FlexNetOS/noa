<#
.SYNOPSIS
    Install NOA prerequisites on Windows

.DESCRIPTION
    Checks for and installs required software:
    - PowerShell 7.4+
    - Git
    - 7-Zip (optional)
    
    Uses winget when available. Handles non-admin scenarios gracefully.

.PARAMETER SkipOptional
    Skip installation of optional components (7-Zip)

.EXAMPLE
    .\install-prereqs.ps1
    Check and install all prerequisites

.EXAMPLE
    .\install-prereqs.ps1 -SkipOptional
    Install only required prerequisites
#>

param(
    [Parameter(Mandatory=$false)]
    [switch]$SkipOptional = $false
)

$ErrorActionPreference = "Stop"

function Write-Status {
    param(
        [string]$Message,
        [ValidateSet('Info', 'Success', 'Warning', 'Error')]
        [string]$Level = 'Info'
    )
    
    $color = switch ($Level) {
        'Success' { 'Green' }
        'Warning' { 'Yellow' }
        'Error'   { 'Red' }
        default   { 'Cyan' }
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

try {
    Write-Host ""
    Write-Host "=== NOA Prerequisites Check ===" -ForegroundColor Cyan
    Write-Host ""
    
    $isAdmin = Test-Administrator
    if ($isAdmin) {
        Write-Status "Running with administrator privileges" -Level Success
    } else {
        Write-Status "Running without administrator privileges" -Level Warning
        Write-Status "Some installations may require elevation" -Level Warning
    }
    
    Write-Host ""
    
    # Check PowerShell version
    Write-Status "Checking PowerShell version..." -Level Info
    $psVersion = $PSVersionTable.PSVersion
    
    if ($psVersion.Major -lt 7 -or ($psVersion.Major -eq 7 -and $psVersion.Minor -lt 4)) {
        Write-Status "PowerShell 7.4+ required. Current: $psVersion" -Level Error
        Write-Status "Download from: https://aka.ms/powershell" -Level Info
        Write-Status "Or install via winget: winget install Microsoft.PowerShell" -Level Info
        $allOk = $false
    } else {
        Write-Status "PowerShell $psVersion" -Level Success
    }
    
    Write-Host ""
    
    # Check for winget
    Write-Status "Checking for winget..." -Level Info
    $winget = Get-Command winget -ErrorAction SilentlyContinue
    
    if (-not $winget) {
        Write-Status "winget not found" -Level Error
        Write-Status "Install App Installer from Microsoft Store to enable automated installations" -Level Info
        Write-Host ""
        Write-Host "Manual installation required for remaining prerequisites:" -ForegroundColor Yellow
        Write-Host "  Git: https://git-scm.com/download/win" -ForegroundColor White
        exit 1
    }
    
    Write-Status "winget detected: $($winget.Version)" -Level Success
    Write-Host ""
    
    # Check and install Git
    Write-Status "Checking for Git..." -Level Info
    $git = Get-Command git -ErrorAction SilentlyContinue
    
    if (-not $git) {
        Write-Status "Git not found. Installing..." -Level Warning
        
        if (-not $isAdmin) {
            Write-Status "Administrator rights required for installation" -Level Warning
            Write-Host ""
            Write-Host "Please run one of the following as Administrator:" -ForegroundColor Yellow
            Write-Host "  winget install Git.Git" -ForegroundColor Cyan
            Write-Host "  Or download from: https://git-scm.com/download/win" -ForegroundColor Cyan
            exit 1
        }
        
        try {
            Write-Status "Installing Git via winget..." -Level Info
            & winget install Git.Git --silent --accept-source-agreements --accept-package-agreements
            
            # Refresh PATH to detect newly installed Git
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
            
            # Verify installation
            $git = Get-Command git -ErrorAction SilentlyContinue
            if ($git) {
                Write-Status "Git installed successfully" -Level Success
            } else {
                Write-Status "Git installation completed but not detected in PATH" -Level Warning
                Write-Status "You may need to restart your terminal" -Level Info
            }
        } catch {
            Write-Status "Failed to install Git: $_" -Level Error
            Write-Status "Manual installation: https://git-scm.com/download/win" -Level Info
            exit 1
        }
    } else {
        $gitVersion = & git --version
        Write-Status "Git detected: $gitVersion" -Level Success
    }
    
    Write-Host ""
    
    # Check and install 7-Zip (optional)
    if (-not $SkipOptional) {
        Write-Status "Checking for 7-Zip (optional)..." -Level Info
        $sevenZip = Get-Command 7z -ErrorAction SilentlyContinue
        
        if (-not $sevenZip) {
            Write-Status "7-Zip not found" -Level Warning
            
            if ($isAdmin) {
                try {
                    Write-Status "Installing 7-Zip via winget..." -Level Info
                    & winget install 7zip.7zip --silent --accept-source-agreements --accept-package-agreements
                    
                    # Refresh PATH
                    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
                    
                    $sevenZip = Get-Command 7z -ErrorAction SilentlyContinue
                    if ($sevenZip) {
                        Write-Status "7-Zip installed successfully" -Level Success
                    } else {
                        Write-Status "7-Zip installation completed but not detected in PATH" -Level Warning
                    }
                } catch {
                    Write-Status "Failed to install 7-Zip (optional): $_" -Level Warning
                }
            } else {
                Write-Status "Skipping 7-Zip installation (requires admin)" -Level Info
                Write-Status "Install manually: winget install 7zip.7zip" -Level Info
            }
        } else {
            Write-Status "7-Zip detected" -Level Success
        }
        
        Write-Host ""
    }
    
    # Summary
    Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                                                            ║" -ForegroundColor Green
    Write-Host "║        Prerequisites Check Complete                        ║" -ForegroundColor Green
    Write-Host "║                                                            ║" -ForegroundColor Green
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    
    Write-Status "All required prerequisites are installed" -Level Success
    Write-Status "You can now run the NOA setup script" -Level Info
    
    exit 0
    
} catch {
    Write-Status "Prerequisites check failed: $_" -Level Error
    exit 1
}
