<# 
NOA installer (Windows)
T413: Installs a packaged NOA binary from dist/ into a target prefix.
#>

param(
    [string]$Archive = "",
    [string]$Prefix = "$HOME\\.noa"
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Join-Path $ScriptDir "..\\.." | Resolve-Path

function Detect-Archive {
    param([string]$ArchiveParam)
    if ($ArchiveParam -ne "") { return $ArchiveParam }
    $candidate = Join-Path $RootDir "dist\\windows\\noa-windows.tar.gz"
    return $candidate
}

$ArchivePath = Detect-Archive -ArchiveParam $Archive
if (-not (Test-Path $ArchivePath)) {
    Write-Error "Archive not found: $ArchivePath. Run scripts\\bash\\release.sh or provide -Archive."
    exit 1
}

Write-Host "Installing NOA to $Prefix"
New-Item -ItemType Directory -Force -Path (Join-Path $Prefix "bin") | Out-Null

tar -xzf $ArchivePath -C (Join-Path $Prefix "bin")

Write-Host "NOA installed. Add to PATH (PowerShell):"
Write-Host "  `$env:PATH = \"$(Join-Path $Prefix 'bin');$($env:PATH)\""
