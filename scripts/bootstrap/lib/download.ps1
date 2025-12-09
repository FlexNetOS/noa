<#
.SYNOPSIS
    Portable download and extraction utilities for NOA bootstrap

.DESCRIPTION
    Provides functions to download files, extract archives, and download
    GitHub releases - all targeting noa_root with no system pollution.
#>

# Ensure NOA_ROOT is set
if (-not $env:NOA_ROOT) {
    $script:NOA_ROOT = Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
} else {
    $script:NOA_ROOT = $env:NOA_ROOT
}

$script:TempDir = Join-Path $NOA_ROOT "tmp"
$script:CacheDir = Join-Path $NOA_ROOT "opt/cache"

function Initialize-DownloadDirectories {
    foreach ($dir in @($script:TempDir, $script:CacheDir)) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
        }
    }
}

function Get-NoaDownload {
    <#
    .SYNOPSIS
        Download a file to noa_root/tmp/ or noa_root/opt/cache/

    .PARAMETER Url
        URL to download from

    .PARAMETER DestinationName
        Filename for the downloaded file

    .PARAMETER UseCache
        If true, download to cache dir and skip if already exists

    .PARAMETER Checksum
        Optional SHA256 checksum to verify
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Url,

        [Parameter(Mandatory)]
        [string]$DestinationName,

        [switch]$UseCache,

        [string]$Checksum
    )

    Initialize-DownloadDirectories

    $destDir = if ($UseCache) { $script:CacheDir } else { $script:TempDir }
    $destPath = Join-Path $destDir $DestinationName

    # Check cache
    if ($UseCache -and (Test-Path $destPath)) {
        if ($Checksum) {
            $fileHash = (Get-FileHash -Path $destPath -Algorithm SHA256).Hash
            if ($fileHash -eq $Checksum) {
                Write-Verbose "Using cached file: $destPath"
                return $destPath
            } else {
                Write-Verbose "Cache checksum mismatch, re-downloading"
                Remove-Item $destPath -Force
            }
        } else {
            Write-Verbose "Using cached file (no checksum verification): $destPath"
            return $destPath
        }
    }

    # Download
    Write-Verbose "Downloading $Url to $destPath"
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $Url -OutFile $destPath -UseBasicParsing
    } catch {
        throw "Failed to download $Url : $_"
    }

    # Verify checksum
    if ($Checksum) {
        $fileHash = (Get-FileHash -Path $destPath -Algorithm SHA256).Hash
        if ($fileHash -ne $Checksum) {
            Remove-Item $destPath -Force
            throw "Checksum verification failed for $DestinationName"
        }
        Write-Verbose "Checksum verified: $Checksum"
    }

    return $destPath
}

function Expand-NoaArchive {
    <#
    .SYNOPSIS
        Extract an archive to a destination directory

    .PARAMETER ArchivePath
        Path to the archive file

    .PARAMETER DestinationPath
        Directory to extract to

    .PARAMETER StripComponents
        Number of leading path components to strip (like tar --strip-components)
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ArchivePath,

        [Parameter(Mandatory)]
        [string]$DestinationPath,

        [int]$StripComponents = 0
    )

    if (-not (Test-Path $ArchivePath)) {
        throw "Archive not found: $ArchivePath"
    }

    # Create destination
    if (-not (Test-Path $DestinationPath)) {
        New-Item -ItemType Directory -Path $DestinationPath -Force | Out-Null
    }

    $extension = [System.IO.Path]::GetExtension($ArchivePath).ToLower()

    switch ($extension) {
        ".zip" {
            if ($StripComponents -eq 0) {
                Expand-Archive -Path $ArchivePath -DestinationPath $DestinationPath -Force
            } else {
                # Extract to temp, then move with strip
                $tempExtract = Join-Path $script:TempDir "extract-$(Get-Random)"
                Expand-Archive -Path $ArchivePath -DestinationPath $tempExtract -Force

                # Find and move contents
                $contents = Get-ChildItem -Path $tempExtract -Recurse -Depth $StripComponents |
                            Where-Object { $_.PSIsContainer -eq $false -or $_.GetFileSystemInfos().Count -gt 0 }

                foreach ($item in $contents) {
                    $relativePath = $item.FullName.Substring($tempExtract.Length + 1)
                    $parts = $relativePath -split [regex]::Escape([System.IO.Path]::DirectorySeparatorChar)
                    if ($parts.Count -gt $StripComponents) {
                        $newRelativePath = ($parts | Select-Object -Skip $StripComponents) -join [System.IO.Path]::DirectorySeparatorChar
                        $newPath = Join-Path $DestinationPath $newRelativePath
                        $newDir = Split-Path $newPath -Parent
                        if (-not (Test-Path $newDir)) {
                            New-Item -ItemType Directory -Path $newDir -Force | Out-Null
                        }
                        Move-Item -Path $item.FullName -Destination $newPath -Force
                    }
                }

                Remove-Item $tempExtract -Recurse -Force
            }
        }
        ".gz" {
            if ($ArchivePath -like "*.tar.gz") {
                # Need 7-Zip or tar for .tar.gz
                $tarPath = $ArchivePath -replace '\.gz$', ''

                # Try using tar if available (Windows 10+)
                $tar = Get-Command tar -ErrorAction SilentlyContinue
                if ($tar) {
                    $tarArgs = @('-xzf', $ArchivePath, '-C', $DestinationPath)
                    if ($StripComponents -gt 0) {
                        $tarArgs += "--strip-components=$StripComponents"
                    }
                    & tar @tarArgs
                } else {
                    throw "tar command not available. Install Windows tar or 7-Zip."
                }
            }
        }
        ".xz" {
            if ($ArchivePath -like "*.tar.xz") {
                $tar = Get-Command tar -ErrorAction SilentlyContinue
                if ($tar) {
                    $tarArgs = @('-xJf', $ArchivePath, '-C', $DestinationPath)
                    if ($StripComponents -gt 0) {
                        $tarArgs += "--strip-components=$StripComponents"
                    }
                    & tar @tarArgs
                } else {
                    throw "tar command not available for .tar.xz extraction"
                }
            }
        }
        default {
            throw "Unsupported archive format: $extension"
        }
    }
}

function Get-GitHubRelease {
    <#
    .SYNOPSIS
        Get latest release info from a GitHub repository

    .PARAMETER Repository
        Repository in format "owner/repo"

    .PARAMETER AssetPattern
        Wildcard pattern to match release asset name
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Repository,

        [string]$AssetPattern = "*"
    )

    $apiUrl = "https://api.github.com/repos/$Repository/releases/latest"

    try {
        $release = Invoke-RestMethod -Uri $apiUrl -UseBasicParsing

        $asset = $release.assets | Where-Object { $_.name -like $AssetPattern } | Select-Object -First 1

        if (-not $asset) {
            throw "No asset matching '$AssetPattern' found in $Repository"
        }

        return @{
            Version = $release.tag_name
            AssetName = $asset.name
            DownloadUrl = $asset.browser_download_url
            Size = $asset.size
        }
    } catch {
        throw "Failed to get GitHub release info for $Repository : $_"
    }
}

function Install-GitHubReleaseBinary {
    <#
    .SYNOPSIS
        Download and install a binary from GitHub releases

    .PARAMETER Repository
        GitHub repository (owner/repo)

    .PARAMETER AssetPattern
        Pattern to match the release asset

    .PARAMETER BinaryPath
        Path to binary within the archive (supports wildcards like */binary.exe)

    .PARAMETER DestinationPath
        Where to install the binary

    .PARAMETER BinaryName
        Final name for the binary (optional, defaults to extracted name)
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Repository,

        [Parameter(Mandatory)]
        [string]$AssetPattern,

        [string]$BinaryPath,

        [Parameter(Mandatory)]
        [string]$DestinationPath,

        [string]$BinaryName
    )

    Initialize-DownloadDirectories

    # Get release info
    $release = Get-GitHubRelease -Repository $Repository -AssetPattern $AssetPattern
    Write-Verbose "Found release: $($release.Version) - $($release.AssetName)"

    # Download
    $archivePath = Get-NoaDownload -Url $release.DownloadUrl -DestinationName $release.AssetName -UseCache

    # Determine if it's an archive or direct binary
    $extension = [System.IO.Path]::GetExtension($release.AssetName).ToLower()

    if ($extension -in @(".exe", "")) {
        # Direct binary download
        $finalName = if ($BinaryName) { $BinaryName } else { $release.AssetName }
        $destFile = Join-Path $DestinationPath $finalName
        Copy-Item -Path $archivePath -Destination $destFile -Force
    } else {
        # Archive - extract
        $extractDir = Join-Path $script:TempDir "extract-$(Get-Random)"
        Expand-NoaArchive -ArchivePath $archivePath -DestinationPath $extractDir

        # Find binary
        if ($BinaryPath) {
            $binaryFile = Get-ChildItem -Path $extractDir -Filter (Split-Path $BinaryPath -Leaf) -Recurse | Select-Object -First 1
        } else {
            # Look for common binary patterns
            $binaryFile = Get-ChildItem -Path $extractDir -Include "*.exe" -Recurse | Select-Object -First 1
            if (-not $binaryFile) {
                $binaryFile = Get-ChildItem -Path $extractDir -Exclude "*.md","*.txt","*.json" -Recurse |
                              Where-Object { -not $_.PSIsContainer } | Select-Object -First 1
            }
        }

        if (-not $binaryFile) {
            Remove-Item $extractDir -Recurse -Force
            throw "Could not find binary in archive"
        }

        # Copy to destination
        $finalName = if ($BinaryName) { $BinaryName } else { $binaryFile.Name }
        $destFile = Join-Path $DestinationPath $finalName

        if (-not (Test-Path $DestinationPath)) {
            New-Item -ItemType Directory -Path $DestinationPath -Force | Out-Null
        }

        Copy-Item -Path $binaryFile.FullName -Destination $destFile -Force

        # Cleanup
        Remove-Item $extractDir -Recurse -Force
    }

    return @{
        Version = $release.Version
        InstalledTo = $destFile
    }
}

# Export functions
Export-ModuleMember -Function @(
    'Get-NoaDownload',
    'Expand-NoaArchive',
    'Get-GitHubRelease',
    'Install-GitHubReleaseBinary',
    'Initialize-DownloadDirectories'
)

