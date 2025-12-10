param(
    [ValidateSet("windows", "linux", "macos")]
    [string]$Platform = "windows"
)

$ErrorActionPreference = "Stop"

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptRoot "..\\..")
$distRoot = Join-Path $repoRoot "dist\\$Platform"

if (-not (Test-Path $distRoot)) {
    New-Item -ItemType Directory -Force -Path $distRoot | Out-Null
}

$binaryName = if ($Platform -eq "windows") { "noa.exe" } else { "noa" }
$source = Join-Path $repoRoot "sys\\core\\target\\release\\$binaryName"

if (-not (Test-Path $source)) {
    throw "Release binary not found at $source. Run 'cargo build --release' in sys/core first."
}

$destinationBinary = Join-Path $distRoot $binaryName
Copy-Item -Path $source -Destination $destinationBinary -Force

$readmePath = Join-Path $distRoot "README.txt"
$timestamp = Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ"
@(
    "NOA cross-platform build artifact",
    "Platform: $Platform",
    "Generated: $timestamp",
    "Source: sys/core/target/release/$binaryName"
) | Set-Content -Path $readmePath

$archive = Join-Path $distRoot "noa-$Platform.zip"
if (Test-Path $archive) {
    Remove-Item -Path $archive -Force
}

Compress-Archive -Path @($destinationBinary, $readmePath) -DestinationPath $archive -Force
Write-Output "Packaged artifact at $archive"
