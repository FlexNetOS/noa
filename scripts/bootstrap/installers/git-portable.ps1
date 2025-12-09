<#
.SYNOPSIS
    Install portable Git, GitHub CLI, and Git LFS to NOA opt directory

.DESCRIPTION
    Downloads and installs Git tools to NOA's contained environment.
    Per NOA Constitution §3.1: All tools must be self-contained.

    After installation, optionally prompts to remove system installations.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall even if already installed

.PARAMETER CleanupSystem
    Prompt to remove system-wide installations after NOA install

.EXAMPLE
    .\git-portable.ps1
    .\git-portable.ps1 -CleanupSystem
#>

[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force,
    [switch]$CleanupSystem
)

$ErrorActionPreference = "Stop"

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT }
    else { Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot))) }
}

$OptDir = Join-Path $NoaRoot "opt"
$BinDir = Join-Path $NoaRoot "bin"
$GitDir = Join-Path $OptDir "git"
$CacheDir = Join-Path $NoaRoot "cache/downloads"

# Ensure directories exist
@($OptDir, $BinDir, $GitDir, $CacheDir) | ForEach-Object {
    if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ -Force | Out-Null }
}

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $color = switch ($Level) {
        "Success" { "Green" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
        "Header" { "Cyan" }
        default { "White" }
    }
    $prefix = switch ($Level) {
        "Success" { "[OK]" }
        "Warning" { "[!!]" }
        "Error" { "[XX]" }
        "Header" { "===" }
        default { "[..]" }
    }
    Write-Host "$prefix $Message" -ForegroundColor $color
}

function Get-LatestGitRelease {
    # Get latest Git for Windows release
    $apiUrl = "https://api.github.com/repos/git-for-windows/git/releases/latest"
    try {
        $release = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "NOA-Bootstrap" }
        $asset = $release.assets | Where-Object { $_.name -match "PortableGit.*64-bit.*\.exe$" } | Select-Object -First 1
        if (-not $asset) {
            Write-Log "No matching Git portable asset found in release" -Level Error
            return $null
        }
        return @{
            Version = $release.tag_name -replace '^v', ''
            Url = $asset.browser_download_url
            FileName = $asset.name
        }
    } catch {
        Write-Log "Failed to get latest Git release: $_" -Level Error
        return $null
    }
}

function Get-LatestGhRelease {
    # Get latest GitHub CLI release
    $apiUrl = "https://api.github.com/repos/cli/cli/releases/latest"
    try {
        $release = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "NOA-Bootstrap" }
        $asset = $release.assets | Where-Object { $_.name -match "gh_.*_windows_amd64\.zip$" } | Select-Object -First 1
        if (-not $asset) {
            Write-Log "No matching GitHub CLI asset found in release" -Level Error
            return $null
        }
        return @{
            Version = $release.tag_name -replace '^v', ''
            Url = $asset.browser_download_url
            FileName = $asset.name
        }
    } catch {
        Write-Log "Failed to get latest gh release: $_" -Level Error
        return $null
    }
}

function Get-LatestGitLfsRelease {
    # Get latest Git LFS release
    $apiUrl = "https://api.github.com/repos/git-lfs/git-lfs/releases/latest"
    try {
        $release = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "NOA-Bootstrap" }
        $asset = $release.assets | Where-Object { $_.name -match "git-lfs-windows-amd64.*\.zip$" } | Select-Object -First 1
        if (-not $asset) {
            Write-Log "No matching Git LFS asset found in release" -Level Error
            return $null
        }
        return @{
            Version = $release.tag_name -replace '^v', ''
            Url = $asset.browser_download_url
            FileName = $asset.name
        }
    } catch {
        Write-Log "Failed to get latest git-lfs release: $_" -Level Error
        return $null
    }
}

function Install-PortableGit {
    param([hashtable]$Release)

    $gitBinDir = Join-Path $GitDir "bin"
    $gitExe = Join-Path $gitBinDir "git.exe"

    if ((Test-Path $gitExe) -and -not $Force) {
        Write-Log "Git already installed at $gitExe" -Level Success
        return $true
    }

    Write-Log "Downloading Git $($Release.Version)..." -Level Info
    $downloadPath = Join-Path $CacheDir $Release.FileName

    try {
        Invoke-WebRequest -Uri $Release.Url -OutFile $downloadPath -UseBasicParsing
        Write-Log "Downloaded: $downloadPath" -Level Success
    } catch {
        Write-Log "Failed to download Git: $_" -Level Error
        return $false
    }

    # Extract portable Git (it's a self-extracting archive)
    Write-Log "Extracting Git to $GitDir..." -Level Info
    try {
        # PortableGit is a 7z self-extractor, run with -o to specify output
        Start-Process -FilePath $downloadPath -ArgumentList "-o`"$GitDir`"", "-y" -Wait -NoNewWindow
        Write-Log "Git extracted successfully" -Level Success
        return $true
    } catch {
        Write-Log "Failed to extract Git: $_" -Level Error
        return $false
    }
}

function Install-GitHubCli {
    param([hashtable]$Release)

    $ghDir = Join-Path $GitDir "gh"
    $ghExe = Join-Path $ghDir "bin/gh.exe"

    if ((Test-Path $ghExe) -and -not $Force) {
        Write-Log "GitHub CLI already installed at $ghExe" -Level Success
        return $true
    }

    Write-Log "Downloading GitHub CLI $($Release.Version)..." -Level Info
    $downloadPath = Join-Path $CacheDir $Release.FileName

    try {
        Invoke-WebRequest -Uri $Release.Url -OutFile $downloadPath -UseBasicParsing
        Write-Log "Downloaded: $downloadPath" -Level Success
    } catch {
        Write-Log "Failed to download gh: $_" -Level Error
        return $false
    }

    # Extract zip
    Write-Log "Extracting GitHub CLI to $ghDir..." -Level Info
    try {
        if (-not (Test-Path $ghDir)) { New-Item -ItemType Directory -Path $ghDir -Force | Out-Null }
        Expand-Archive -Path $downloadPath -DestinationPath $ghDir -Force

        # Move contents from nested folder
        $nestedDir = Get-ChildItem -Path $ghDir -Directory | Select-Object -First 1
        if ($nestedDir) {
            Get-ChildItem -Path $nestedDir.FullName | Move-Item -Destination $ghDir -Force
            Remove-Item -Path $nestedDir.FullName -Force -Recurse
        }

        Write-Log "GitHub CLI extracted successfully" -Level Success
        return $true
    } catch {
        Write-Log "Failed to extract gh: $_" -Level Error
        return $false
    }
}

function Install-GitLfs {
    param([hashtable]$Release)

    $lfsDir = Join-Path $GitDir "lfs"
    $lfsExe = Join-Path $lfsDir "git-lfs.exe"

    if ((Test-Path $lfsExe) -and -not $Force) {
        Write-Log "Git LFS already installed at $lfsExe" -Level Success
        return $true
    }

    Write-Log "Downloading Git LFS $($Release.Version)..." -Level Info
    $downloadPath = Join-Path $CacheDir $Release.FileName

    try {
        Invoke-WebRequest -Uri $Release.Url -OutFile $downloadPath -UseBasicParsing
        Write-Log "Downloaded: $downloadPath" -Level Success
    } catch {
        Write-Log "Failed to download git-lfs: $_" -Level Error
        return $false
    }

    # Extract zip
    Write-Log "Extracting Git LFS to $lfsDir..." -Level Info
    try {
        if (-not (Test-Path $lfsDir)) { New-Item -ItemType Directory -Path $lfsDir -Force | Out-Null }
        Expand-Archive -Path $downloadPath -DestinationPath $lfsDir -Force

        # Move exe from nested folder if present
        $nestedExe = Get-ChildItem -Path $lfsDir -Recurse -Filter "git-lfs.exe" | Select-Object -First 1
        if ($nestedExe -and $nestedExe.DirectoryName -ne $lfsDir) {
            Move-Item -Path $nestedExe.FullName -Destination $lfsDir -Force
        }

        Write-Log "Git LFS extracted successfully" -Level Success
        return $true
    } catch {
        Write-Log "Failed to extract git-lfs: $_" -Level Error
        return $false
    }
}

function Update-BinSymlinks {
    Write-Log "Updating bin/ symlinks to internal locations..." -Level Info

    $symlinks = @(
        @{ Name = "git.exe"; Target = Join-Path $GitDir "bin/git.exe" },
        @{ Name = "gh.exe"; Target = Join-Path $GitDir "gh/bin/gh.exe" },
        @{ Name = "git-lfs.exe"; Target = Join-Path $GitDir "lfs/git-lfs.exe" }
    )

    foreach ($link in $symlinks) {
        $linkPath = Join-Path $BinDir $link.Name
        $targetPath = $link.Target

        if (-not (Test-Path $targetPath)) {
            Write-Log "Target not found: $targetPath" -Level Warning
            continue
        }

        # Remove existing symlink/file
        if (Test-Path $linkPath) {
            Remove-Item -Path $linkPath -Force
        }

        # Create new symlink
        try {
            New-Item -ItemType SymbolicLink -Path $linkPath -Target $targetPath -Force | Out-Null
            Write-Log "Linked: $($link.Name) -> $targetPath" -Level Success
        } catch {
            Write-Log "Failed to create symlink for $($link.Name): $_" -Level Error
        }
    }
}

function Prompt-CleanupSystemInstalls {
    Write-Host ""
    Write-Log "System Installation Cleanup" -Level Header
    Write-Host ""

    $systemPaths = @(
        "C:\Program Files\Git",
        "C:\Program Files\GitHub CLI",
        "C:\Program Files\Git LFS"
    )

    $foundPaths = @()
    foreach ($path in $systemPaths) {
        if (Test-Path $path) {
            $foundPaths += $path
            Write-Log "Found: $path" -Level Warning
        }
    }

    if ($foundPaths.Count -eq 0) {
        Write-Log "No system installations found to clean up" -Level Success
        return
    }

    Write-Host ""
    Write-Host "The following system installations can be removed:" -ForegroundColor Yellow
    $foundPaths | ForEach-Object { Write-Host "  - $_" -ForegroundColor Gray }
    Write-Host ""
    Write-Host "NOA now has its own contained copies of these tools." -ForegroundColor Cyan
    Write-Host ""

    $response = Read-Host "Do you want to remove these system installations? (y/N)"

    if ($response -eq 'y' -or $response -eq 'Y') {
        foreach ($path in $foundPaths) {
            Write-Log "Removing: $path" -Level Info
            try {
                # Try to remove - may need admin rights
                Remove-Item -Path $path -Recurse -Force -ErrorAction Stop
                Write-Log "Removed: $path" -Level Success
            } catch {
                Write-Log "Failed to remove $path (may need admin rights): $_" -Level Warning
                Write-Host "  Run as Administrator to remove, or manually delete: $path" -ForegroundColor Gray
            }
        }
    } else {
        Write-Log "Skipped system cleanup (user declined)" -Level Info
        Write-Host ""
        Write-Host "You can manually uninstall via:" -ForegroundColor Gray
        Write-Host "  - Settings > Apps > Installed apps > Git / GitHub CLI / Git LFS" -ForegroundColor Gray
    }
}

# Main execution
Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "NOA Portable Git Tools Installer" -ForegroundColor Cyan
Write-Host "Constitution §3.1: Self-Contained Installation" -ForegroundColor Gray
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""
Write-Log "NOA_ROOT: $NoaRoot" -Level Info
Write-Log "Git Dir: $GitDir" -Level Info
Write-Host ""

# Get latest releases
$gitRelease = Get-LatestGitRelease
$ghRelease = Get-LatestGhRelease
$lfsRelease = Get-LatestGitLfsRelease

# Install tools
$success = $true

if ($gitRelease) {
    if (-not (Install-PortableGit -Release $gitRelease)) { $success = $false }
} else {
    Write-Log "Could not determine Git release" -Level Error
    $success = $false
}

if ($ghRelease) {
    if (-not (Install-GitHubCli -Release $ghRelease)) { $success = $false }
} else {
    Write-Log "Could not determine gh release" -Level Error
    $success = $false
}

if ($lfsRelease) {
    if (-not (Install-GitLfs -Release $lfsRelease)) { $success = $false }
} else {
    Write-Log "Could not determine git-lfs release" -Level Error
    $success = $false
}

# Update symlinks
if ($success) {
    Update-BinSymlinks
}

# Cleanup prompt
if ($CleanupSystem -or $success) {
    Prompt-CleanupSystemInstalls
}

Write-Host ""
if ($success) {
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host "Git tools installation complete!" -ForegroundColor Green
    Write-Host "=" * 60 -ForegroundColor Green
    Write-Host ""
    Write-Host "Tools are now contained in: $GitDir" -ForegroundColor Cyan
    Write-Host "Symlinks updated in: $BinDir" -ForegroundColor Cyan
    exit 0
} else {
    Write-Host "=" * 60 -ForegroundColor Red
    Write-Host "Some installations failed. Check errors above." -ForegroundColor Red
    Write-Host "=" * 60 -ForegroundColor Red
    exit 1
}

