<#
.SYNOPSIS
    Install all required NOA toolchains in a self-contained way under NOA_ROOT.

.DESCRIPTION
    - Installs portable/contained builds into NOA_ROOT/opt and symlinks/wraps into NOA_ROOT/bin.
    - Avoids system-wide package managers by default.
    - Supports selective install via -Tool; default installs all known tools.
    - Uses existing bootstrap installers when available (e.g., scripts/bootstrap/installers/*-portable.ps1).

.PARAMETER NoaRoot
    Override NOA root (defaults to env:NOA_ROOT or repo root).

.PARAMETER Tool
    One or more tool names to install (default: all).
    Supported: rust, go, protoc, golangci-lint, eslint, ruff, gitleaks, trivy, grype, semgrep, gh,
               cmake, ninja, llvm, mingw

.PARAMETER AllowGlobal
    If specified, allows falling back to system-wide installers (winget/choco/apt/brew).
    Default is containment-only.

.EXAMPLE
    pwsh -File scripts/setup/install-all-tools.ps1

.EXAMPLE
    pwsh -File scripts/setup/install-all-tools.ps1 -Tool rust -Tool go
#>

[CmdletBinding()]
param(
    [string] $NoaRoot,
    [string[]] $Tool,
    [switch] $AllowGlobal,
    [switch] $UpdateExisting,
    [switch] $Rollback,
    [switch] $ListArchived
)

$ErrorActionPreference = "Stop"

# Resolve NOA_ROOT
if (-not $NoaRoot) {
    if ($env:NOA_ROOT) {
        $NoaRoot = $env:NOA_ROOT
    } else {
        $NoaRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    }
}
$NoaRoot = [IO.Path]::GetFullPath($NoaRoot)
$BinDir = Join-Path $NoaRoot "bin"
$OptDir = Join-Path $NoaRoot "opt"
$DevToolsDir = Join-Path $OptDir "dev-tools"

$ArchiveDir = Join-Path $OptDir "archive"

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $DevToolsDir | Out-Null
New-Item -ItemType Directory -Force -Path $ArchiveDir | Out-Null

function Write-Info($msg) { Write-Host "[INFO] $msg" -ForegroundColor Cyan }
function Write-Ok($msg)   { Write-Host "[OK]   $msg" -ForegroundColor Green }
function Write-Warn($msg) { Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Write-Err($msg)  { Write-Host "[ERR]  $msg" -ForegroundColor Red }

#region Archive & Rollback Functions (FR-163, B159, B160)

<#
.SYNOPSIS
    Archive a tool before upgrade (B159)
.DESCRIPTION
    Moves the current tool installation to opt/archive/{tool}-{version}-{timestamp}/
    This enables rollback if the upgrade fails or has issues.
#>
function Archive-Tool {
    param(
        [Parameter(Mandatory)]
        [string]$ToolName,
        [Parameter(Mandatory)]
        [string]$SourcePath,
        [string]$Version = "unknown"
    )

    if (-not (Test-Path $SourcePath)) {
        Write-Info "  No existing installation to archive for $ToolName"
        return $null
    }

    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $archiveName = "$ToolName-$Version-$timestamp"
    $archivePath = Join-Path $ArchiveDir $archiveName

    Write-Info "  Archiving $ToolName to $archivePath"

    try {
        # Create archive directory
        New-Item -ItemType Directory -Force -Path $archivePath | Out-Null

        # Move the tool directory/files
        if (Test-Path $SourcePath -PathType Container) {
            Copy-Item -Path "$SourcePath\*" -Destination $archivePath -Recurse -Force
        } else {
            Copy-Item -Path $SourcePath -Destination $archivePath -Force
        }

        # Create metadata file
        $metadata = @{
            tool = $ToolName
            version = $Version
            archivedAt = (Get-Date).ToString("o")
            sourcePath = $SourcePath
            archivePath = $archivePath
        }
        $metadata | ConvertTo-Json | Set-Content (Join-Path $archivePath "archive-metadata.json")

        Write-Ok "  Archived $ToolName (version: $Version)"
        return $archivePath
    } catch {
        Write-Err "  Failed to archive $ToolName`: $($_.Exception.Message)"
        return $null
    }
}

<#
.SYNOPSIS
    Get the version of an installed tool
#>
function Get-ToolVersion {
    param(
        [Parameter(Mandatory)]
        [string]$ToolName,
        [string]$ToolPath
    )

    switch ($ToolName.ToLower()) {
        "rust" {
            $rustc = Join-Path $OptDir "rust/cargo/bin/rustc.exe"
            if (Test-Path $rustc) { return (& $rustc --version 2>$null | Select-String -Pattern '\d+\.\d+\.\d+' -AllMatches).Matches.Value }
        }
        "go" {
            $go = Join-Path $DevToolsDir "go/bin/go.exe"
            if (Test-Path $go) { return (& $go version 2>$null | Select-String -Pattern '\d+\.\d+\.\d+' -AllMatches).Matches.Value }
        }
        "node" {
            $node = Join-Path $DevToolsDir "node/node.exe"
            if (Test-Path $node) { return (& $node --version 2>$null).TrimStart('v') }
        }
        "python" {
            $python = Join-Path $DevToolsDir "python/python.exe"
            if (Test-Path $python) { return (& $python --version 2>$null | Select-String -Pattern '\d+\.\d+\.\d+' -AllMatches).Matches.Value }
        }
        default { return "unknown" }
    }
    return "unknown"
}

<#
.SYNOPSIS
    List all archived tool versions (B160 helper)
#>
function Get-ArchivedTools {
    if (-not (Test-Path $ArchiveDir)) {
        return @()
    }

    $archives = Get-ChildItem -Path $ArchiveDir -Directory | ForEach-Object {
        $metadataPath = Join-Path $_.FullName "archive-metadata.json"
        if (Test-Path $metadataPath) {
            $metadata = Get-Content $metadataPath -Raw | ConvertFrom-Json
            [PSCustomObject]@{
                Name = $_.Name
                Tool = $metadata.tool
                Version = $metadata.version
                ArchivedAt = $metadata.archivedAt
                SourcePath = $metadata.sourcePath
                ArchivePath = $_.FullName
            }
        } else {
            # Parse from directory name
            $parts = $_.Name -split '-'
            [PSCustomObject]@{
                Name = $_.Name
                Tool = $parts[0]
                Version = if ($parts.Count -gt 1) { $parts[1] } else { "unknown" }
                ArchivedAt = $_.CreationTime.ToString("o")
                SourcePath = $null
                ArchivePath = $_.FullName
            }
        }
    }
    return $archives | Sort-Object ArchivedAt -Descending
}

<#
.SYNOPSIS
    Rollback a tool to a previously archived version (B160)
.DESCRIPTION
    Restores the most recent archived version of a tool, removing the current installation.
#>
function Restore-ArchivedTool {
    param(
        [Parameter(Mandatory)]
        [string]$ToolName
    )

    $archives = Get-ArchivedTools | Where-Object { $_.Tool -eq $ToolName }

    if (-not $archives -or $archives.Count -eq 0) {
        Write-Err "No archived versions found for $ToolName"
        return $false
    }

    $mostRecent = $archives | Select-Object -First 1
    Write-Info "Rolling back $ToolName to version $($mostRecent.Version) from $($mostRecent.ArchivedAt)"

    try {
        $sourcePath = $mostRecent.SourcePath
        if (-not $sourcePath) {
            # Determine source path based on tool name
            switch ($ToolName.ToLower()) {
                "rust" { $sourcePath = Join-Path $OptDir "rust" }
                "go" { $sourcePath = Join-Path $DevToolsDir "go" }
                "node" { $sourcePath = Join-Path $DevToolsDir "node" }
                "python" { $sourcePath = Join-Path $DevToolsDir "python" }
                default { $sourcePath = Join-Path $DevToolsDir $ToolName }
            }
        }

        # Remove current installation if exists
        if (Test-Path $sourcePath) {
            Write-Info "  Removing current installation at $sourcePath"
            Remove-Item -Path $sourcePath -Recurse -Force
        }

        # Restore from archive
        Write-Info "  Restoring from $($mostRecent.ArchivePath)"
        $parentDir = Split-Path $sourcePath -Parent
        if (-not (Test-Path $parentDir)) {
            New-Item -ItemType Directory -Force -Path $parentDir | Out-Null
        }

        Copy-Item -Path "$($mostRecent.ArchivePath)\*" -Destination $sourcePath -Recurse -Force -Exclude "archive-metadata.json"

        Write-Ok "Successfully rolled back $ToolName to version $($mostRecent.Version)"

        # Optionally remove the archive after successful rollback
        # Remove-Item -Path $mostRecent.ArchivePath -Recurse -Force

        return $true
    } catch {
        Write-Err "Failed to rollback $ToolName`: $($_.Exception.Message)"
        return $false
    }
}

<#
.SYNOPSIS
    Clean old archives beyond retention period
#>
function Remove-OldArchives {
    param([int]$RetentionDays = 7)

    $cutoffDate = (Get-Date).AddDays(-$RetentionDays)
    $oldArchives = Get-ArchivedTools | Where-Object {
        try { [DateTime]::Parse($_.ArchivedAt) -lt $cutoffDate } catch { $false }
    }

    foreach ($archive in $oldArchives) {
        Write-Info "Removing old archive: $($archive.Name)"
        Remove-Item -Path $archive.ArchivePath -Recurse -Force -ErrorAction SilentlyContinue
    }

    if ($oldArchives.Count -gt 0) {
        Write-Ok "Removed $($oldArchives.Count) old archives"
    }
}

#endregion

function Should-Install {
    param(
        [string[]] $Names,
        [string] $ToolName = ""
    )
    foreach ($n in $Names) {
        $p = Join-Path $BinDir $n
        if (Test-Path $p) {
            if ($UpdateExisting.IsPresent) {
                # Archive before update if tool exists
                if ($ToolName) {
                    $version = Get-ToolVersion -ToolName $ToolName -ToolPath $p
                    Archive-Tool -ToolName $ToolName -SourcePath (Split-Path $p -Parent) -Version $version
                }
                return $true
            }
            return $false
        }
    }
    return $true
}

function Download-And-ExtractZip {
    param(
        [Parameter(Mandatory)] [string] $Url,
        [Parameter(Mandatory)] [string] $DestDir,
        [string] $StripRoot = $null
    )
    $tmp = Join-Path ([IO.Path]::GetTempPath()) ([IO.Path]::GetRandomFileName() + ".zip")
    Write-Info "Downloading $Url"
    Invoke-WebRequest -Uri $Url -OutFile $tmp -UseBasicParsing
    Write-Info "Extracting to $DestDir"
    if (-not (Test-Path $DestDir)) { New-Item -ItemType Directory -Force -Path $DestDir | Out-Null }
    # Extract with overwrite - use Expand-Archive for better compatibility
    Expand-Archive -Path $tmp -DestinationPath $DestDir -Force
    Remove-Item $tmp -Force

    if ($StripRoot) {
        $inner = Join-Path $DestDir $StripRoot
        if (Test-Path $inner) {
            Get-ChildItem -Path $inner | Move-Item -Destination $DestDir -Force
            Remove-Item $inner -Recurse -Force
        }
    }
}

function Add-Link {
    param(
        [string] $Source,
        [string] $LinkName
    )
    $target = Join-Path $BinDir $LinkName
    if (Test-Path $target) { Remove-Item $target -Force }
    New-Item -ItemType SymbolicLink -Path $target -Target $Source | Out-Null
    Write-Ok "Linked $LinkName -> $Source"
}

function Install-Rust {
    $rustHome = Join-Path $OptDir "rust"
    $cargoBin = Join-Path $rustHome "cargo/bin"

    # Check if already installed
    if (Test-Path (Join-Path $cargoBin "rustc.exe")) {
        if (-not $UpdateExisting.IsPresent) {
            Write-Info "rust/cargo already present; skipping (use -UpdateExisting to force)"
            return
        }
        # Archive before update (B159)
        $version = Get-ToolVersion -ToolName "rust" -ToolPath $cargoBin
        Write-Info "Archiving existing Rust installation (v$version) before update..."
        Archive-Tool -ToolName "rust" -SourcePath $rustHome -Version $version
    }
    $env:RUSTUP_HOME = Join-Path $rustHome "rustup"
    $env:CARGO_HOME  = Join-Path $rustHome "cargo"
    New-Item -ItemType Directory -Force -Path $env:RUSTUP_HOME | Out-Null
    New-Item -ItemType Directory -Force -Path $env:CARGO_HOME | Out-Null

    $rustup = Join-Path ([IO.Path]::GetTempPath()) "rustup-init.exe"
    Invoke-WebRequest -Uri "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" -OutFile $rustup -UseBasicParsing
    & $rustup -y --no-modify-path --profile default --default-toolchain stable --default-host x86_64-pc-windows-msvc
    Remove-Item $rustup -Force

    $cargoBin = Join-Path $env:CARGO_HOME "bin"
    Add-Link (Join-Path $cargoBin "rustc.exe") "rustc.exe"
    Add-Link (Join-Path $cargoBin "cargo.exe") "cargo.exe"
    Add-Link (Join-Path $cargoBin "rustfmt.exe") "rustfmt.exe"
    Add-Link (Join-Path $cargoBin "cargo-clippy.exe") "cargo-clippy.exe"
}

function Install-Go {
    $goDir = Join-Path $DevToolsDir "go"
    $goBin = Join-Path $goDir "bin/go.exe"

    # Check if already installed
    if (Test-Path $goBin) {
        if (-not $UpdateExisting.IsPresent) {
            Write-Info "go already present; skipping (use -UpdateExisting to force)"
            return
        }
        # Archive before update (B159)
        $version = Get-ToolVersion -ToolName "go" -ToolPath $goBin
        Write-Info "Archiving existing Go installation (v$version) before update..."
        Archive-Tool -ToolName "go" -SourcePath $goDir -Version $version
    }

    Download-And-ExtractZip -Url "https://go.dev/dl/go1.23.0.windows-amd64.zip" -DestDir $DevToolsDir
    $goBin = Join-Path $goDir "bin"
    Add-Link (Join-Path $goBin "go.exe") "go.exe"
}

function Install-Protoc {
    if (-not (Should-Install @("protoc.exe","protoc"))) {
        Write-Info "protoc already present; skipping (use -UpdateExisting to force)"
        return
    }
    $protoDir = Join-Path $DevToolsDir "protobuf"
    Download-And-ExtractZip -Url "https://github.com/protocolbuffers/protobuf/releases/download/v28.0/protoc-28.0-win64.zip" -DestDir $protoDir
    $bin = Join-Path $protoDir "bin"
    Add-Link (Join-Path $bin "protoc.exe") "protoc.exe"
}

function Install-GolangciLint {
    if (-not (Should-Install @("golangci-lint.exe","golangci-lint"))) {
        Write-Info "golangci-lint already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "golangci-lint"
    Download-And-ExtractZip -Url "https://github.com/golangci/golangci-lint/releases/download/v1.62.0/golangci-lint-1.62.0-windows-amd64.zip" -DestDir $toolDir -StripRoot "golangci-lint-1.62.0-windows-amd64"
    Add-Link (Join-Path $toolDir "golangci-lint.exe") "golangci-lint.exe"
}

function Install-Ruff {
    if (-not (Should-Install @("ruff.exe","ruff"))) {
        Write-Info "ruff already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "ruff"
    Download-And-ExtractZip -Url "https://github.com/astral-sh/ruff/releases/download/v0.8.1/ruff-0.8.1-x86_64-pc-windows-msvc.zip" -DestDir $toolDir
    Add-Link (Join-Path $toolDir "ruff.exe") "ruff.exe"
}

function Install-Gitleaks {
    if (-not (Should-Install @("gitleaks.exe","gitleaks"))) {
        Write-Info "gitleaks already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "gitleaks"
    Download-And-ExtractZip -Url "https://github.com/gitleaks/gitleaks/releases/download/v8.21.2/gitleaks_8.21.2_windows_x64.zip" -DestDir $toolDir
    Add-Link (Join-Path $toolDir "gitleaks.exe") "gitleaks.exe"
}

function Install-Trivy {
    if (-not (Should-Install @("trivy.exe","trivy"))) {
        Write-Info "trivy already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "trivy"
    Download-And-ExtractZip -Url "https://github.com/aquasecurity/trivy/releases/download/v0.57.1/trivy_0.57.1_Windows-64bit.zip" -DestDir $toolDir
    Add-Link (Join-Path $toolDir "trivy.exe") "trivy.exe"
}

function Install-Grype {
    if (-not (Should-Install @("grype.exe","grype"))) {
        Write-Info "grype already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "grype"
    Download-And-ExtractZip -Url "https://github.com/anchore/grype/releases/download/v0.84.0/grype_0.84.0_windows_amd64.zip" -DestDir $toolDir
    Add-Link (Join-Path $toolDir "grype.exe") "grype.exe"
}

function Install-Semgrep {
    if (-not (Should-Install @("semgrep.exe","semgrep"))) {
        Write-Info "semgrep already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "semgrep"
    Invoke-WebRequest -Uri "https://semgrep.dev/api/cli_v1/releases/latest/download?os=windows&arch=amd64" -OutFile (Join-Path $toolDir "semgrep.exe") -UseBasicParsing
    Add-Link (Join-Path $toolDir "semgrep.exe") "semgrep.exe"
}

function Install-GitHubCli {
    if (-not (Should-Install @("gh.exe","gh"))) {
        Write-Info "gh already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "gh"
    Download-And-ExtractZip -Url "https://github.com/cli/cli/releases/download/v2.53.0/gh_2.53.0_windows_amd64.zip" -DestDir $toolDir -StripRoot "gh_2.53.0_windows_amd64"
    Add-Link (Join-Path $toolDir "bin" "gh.exe") "gh.exe"
}

function Install-Git {
    if (-not (Should-Install @("git.exe"))) {
        Write-Info "git already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "git"
    Download-And-ExtractZip -Url "https://github.com/git-for-windows/git/releases/download/v2.47.0.windows.1/PortableGit-2.47.0-64-bit.zip" -DestDir $toolDir
    Add-Link (Join-Path $toolDir "cmd" "git.exe") "git.exe"
    Add-Link (Join-Path $toolDir "mingw64" "bin" "git.exe") "git-mingw.exe"
}

function Install-GitLfs {
    if (-not (Should-Install @("git-lfs.exe","git-lfs"))) {
        Write-Info "git-lfs already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "git-lfs"
    Download-And-ExtractZip -Url "https://github.com/git-lfs/git-lfs/releases/download/v3.5.1/git-lfs-windows-amd64-v3.5.1.zip" -DestDir $toolDir
    Add-Link (Join-Path $toolDir "git-lfs.exe") "git-lfs.exe"
}

function Install-Eslint {
    if (-not (Should-Install @("eslint.cmd","eslint"))) {
        Write-Info "eslint already present; skipping (use -UpdateExisting to force)"
        return
    }
    # Requires Node portable. If node.exe not present in NOA_ROOT/bin, emit warning.
    $nodeExe = Join-Path $BinDir "node.exe"
    if (-not (Test-Path $nodeExe)) {
        Write-Warn "Node not found in $BinDir. Install Node portable before ESLint."
        return
    }
    $npmCache = Join-Path $OptDir "npm-cache"
    New-Item -ItemType Directory -Force -Path $npmCache | Out-Null
    $npmPrefix = Join-Path $DevToolsDir "npm-global"
    New-Item -ItemType Directory -Force -Path $npmPrefix | Out-Null
    $env:NPM_CONFIG_CACHE = $npmCache
    $env:NPM_CONFIG_PREFIX = $npmPrefix
    & $nodeExe (Join-Path (Split-Path $nodeExe -Parent) "npm") install -g eslint@9.13.0
    $eslintPath = Join-Path $npmPrefix "node_modules" ".bin" "eslint.cmd"
    if (Test-Path $eslintPath) { Add-Link $eslintPath "eslint.cmd" }
}

function Install-NodePortable {
    if (-not (Should-Install @("node.exe","npm.cmd","npx.cmd"))) {
        Write-Info "node/npm already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "node"
    Download-And-ExtractZip -Url "https://nodejs.org/dist/v20.18.0/node-v20.18.0-win-x64.zip" -DestDir $toolDir -StripRoot "node-v20.18.0-win-x64"
    Add-Link (Join-Path $toolDir "node.exe") "node.exe"
    Add-Link (Join-Path $toolDir "npm.cmd") "npm.cmd"
    Add-Link (Join-Path $toolDir "npx.cmd") "npx.cmd"
}

function Install-PythonPortable {
    if (-not (Should-Install @("python.exe","pythonw.exe"))) {
        Write-Info "python already present; skipping (use -UpdateExisting to force)"
        return
    }
    $toolDir = Join-Path $DevToolsDir "python"
    Download-And-ExtractZip -Url "https://www.python.org/ftp/python/3.12.7/python-3.12.7-embed-amd64.zip" -DestDir $toolDir
    Add-Link (Join-Path $toolDir "python.exe") "python.exe"
    Add-Link (Join-Path $toolDir "pythonw.exe") "pythonw.exe"
}

#region Build Tools (B161-B170)

function Install-CMake {
    <#
    .SYNOPSIS
        Install portable CMake via bootstrap installer
    #>
    $cmakeExe = Join-Path $BinDir "cmake.exe"
    if ((Test-Path $cmakeExe) -and -not $UpdateExisting) {
        Write-Info "CMake already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing CMake..."

    $installer = Join-Path $NoaRoot "scripts/bootstrap/installers/cmake-portable.ps1"
    if (Test-Path $installer) {
        $force = if ($UpdateExisting) { "-Force" } else { "" }
        & $installer -NoaRoot $NoaRoot $force
    } else {
        Write-Err "CMake installer not found: $installer"
    }
}

function Install-Ninja {
    <#
    .SYNOPSIS
        Install portable Ninja via bootstrap installer
    #>
    $ninjaExe = Join-Path $BinDir "ninja.exe"
    if ((Test-Path $ninjaExe) -and -not $UpdateExisting) {
        Write-Info "Ninja already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing Ninja..."

    $installer = Join-Path $NoaRoot "scripts/bootstrap/installers/ninja-portable.ps1"
    if (Test-Path $installer) {
        $force = if ($UpdateExisting) { "-Force" } else { "" }
        & $installer -NoaRoot $NoaRoot $force
    } else {
        Write-Err "Ninja installer not found: $installer"
    }
}

function Install-LLVM {
    <#
    .SYNOPSIS
        Install portable LLVM/Clang via bootstrap installer
    #>
    $clangExe = Join-Path $BinDir "clang.exe"
    if ((Test-Path $clangExe) -and -not $UpdateExisting) {
        Write-Info "LLVM/Clang already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing LLVM/Clang..."

    $installer = Join-Path $NoaRoot "scripts/bootstrap/installers/llvm-portable.ps1"
    if (Test-Path $installer) {
        $force = if ($UpdateExisting) { "-Force" } else { "" }
        & $installer -NoaRoot $NoaRoot $force
    } else {
        Write-Err "LLVM installer not found: $installer"
    }
}

function Install-MinGW {
    <#
    .SYNOPSIS
        Install portable MinGW-w64 via bootstrap installer
    #>
    $gccExe = Join-Path $BinDir "gcc.exe"
    if ((Test-Path $gccExe) -and -not $UpdateExisting) {
        Write-Info "MinGW already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing MinGW-w64..."

    $installer = Join-Path $NoaRoot "scripts/bootstrap/installers/mingw-portable.ps1"
    if (Test-Path $installer) {
        $force = if ($UpdateExisting) { "-Force" } else { "" }
        & $installer -NoaRoot $NoaRoot $force
    } else {
        Write-Warn "MinGW installer not found: $installer"
        Write-Warn "Create scripts/bootstrap/installers/mingw-portable.ps1 to enable MinGW installation"
    }
}

#endregion

#region AI Provider CLIs (FR-039)

function Install-ClaudeCode {
    <#
    .SYNOPSIS
        Install Claude Code CLI via npm or delegate to bootstrap installer
    #>
    if (-not (Should-Install @("claude.cmd","claude"))) {
        Write-Info "Claude Code CLI already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing Claude Code CLI..."

    # First, try to use the bootstrap installer if available
    $bootstrapInstaller = Join-Path $NoaRoot "scripts/bootstrap/installers/ai-providers/claude-code.ps1"
    if (Test-Path $bootstrapInstaller) {
        Write-Info "  Using bootstrap installer..."
        & $bootstrapInstaller -NoaRoot $NoaRoot -Method npm
        return
    }

    # Fallback: install via npm
    $nodeExe = Join-Path $BinDir "node.exe"
    if (-not (Test-Path $nodeExe)) {
        Write-Warn "Node not found in $BinDir. Install Node portable before Claude Code CLI."
        return
    }

    $npmCache = Join-Path $OptDir "npm-cache"
    $npmPrefix = Join-Path $DevToolsDir "npm-global"
    New-Item -ItemType Directory -Force -Path $npmCache, $npmPrefix | Out-Null
    $env:NPM_CONFIG_CACHE = $npmCache
    $env:NPM_CONFIG_PREFIX = $npmPrefix

    $npmCmd = Join-Path (Split-Path $nodeExe -Parent) "npm.cmd"
    & $npmCmd install -g @anthropic-ai/claude-code

    $claudePath = Join-Path $npmPrefix "node_modules\.bin\claude.cmd"
    if (Test-Path $claudePath) {
        Add-Link $claudePath "claude.cmd"
        Write-Ok "Claude Code CLI installed"
    }

    # Ensure provider config directory exists
    $providerConfigDir = Join-Path $NoaRoot "ai/providers/cloud/claude-code"
    if (-not (Test-Path $providerConfigDir)) {
        New-Item -ItemType Directory -Force -Path $providerConfigDir | Out-Null
        Write-Info "  Created provider config directory: $providerConfigDir"
    }
}

function Install-CursorCli {
    <#
    .SYNOPSIS
        Install Cursor CLI (download from cursor.com)
    #>
    Write-Info "Installing Cursor CLI..."

    $toolDir = Join-Path $DevToolsDir "cursor-cli"
    New-Item -ItemType Directory -Force -Path $toolDir | Out-Null

    # Note: Cursor CLI requires manual download or installation from cursor.com
    # The CLI is bundled with Cursor IDE or available separately
    Write-Warn "Cursor CLI requires manual installation from https://cursor.com"
    Write-Warn "Once installed, add cursor to PATH or link to $BinDir"
}

function Install-CodexCli {
    <#
    .SYNOPSIS
        Install Codex CLI via npm or clone FlexNetOS fork
    #>
    if (-not (Should-Install @("codex.cmd","codex"))) {
        Write-Info "codex already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing Codex CLI..."

    # First, try to use the bootstrap installer if available
    $bootstrapInstaller = Join-Path $NoaRoot "scripts/bootstrap/installers/ai-providers/codex-cli.ps1"
    if (Test-Path $bootstrapInstaller) {
        Write-Info "  Using bootstrap installer..."
        & $bootstrapInstaller -NoaRoot $NoaRoot -Method npm
        return
    }

    # Fallback: install via npm
    $nodeExe = Join-Path $BinDir "node.exe"
    if (-not (Test-Path $nodeExe)) {
        Write-Warn "Node not found in $BinDir. Install Node portable before Codex CLI."
        return
    }

    $npmCache = Join-Path $OptDir "npm-cache"
    $npmPrefix = Join-Path $DevToolsDir "npm-global"
    New-Item -ItemType Directory -Force -Path $npmCache, $npmPrefix | Out-Null
    $env:NPM_CONFIG_CACHE = $npmCache
    $env:NPM_CONFIG_PREFIX = $npmPrefix

    $npmCmd = Join-Path (Split-Path $nodeExe -Parent) "npm.cmd"

    # Try @openai/codex first, fallback to codex-cli
    try {
        & $npmCmd install -g @openai/codex 2>$null
    } catch {
        Write-Info "  @openai/codex not found, trying codex-cli..."
        & $npmCmd install -g codex-cli
    }

    $codexPath = Join-Path $npmPrefix "node_modules\.bin\codex.cmd"
    if (Test-Path $codexPath) {
        Add-Link $codexPath "codex.cmd"
        Write-Ok "Codex CLI installed"
    } else {
        Write-Warn "Codex CLI install pending (binary not found after npm). Will retry on next run."
    }

    # Ensure provider config directory exists
    $providerConfigDir = Join-Path $NoaRoot "ai/providers/cloud/codex"
    if (-not (Test-Path $providerConfigDir)) {
        New-Item -ItemType Directory -Force -Path $providerConfigDir | Out-Null
        Write-Info "  Created provider config directory: $providerConfigDir"
    }
}

function Install-AbacusCli {
    <#
    .SYNOPSIS
        Install Abacus AI CLI via npm
    #>
    if (-not (Should-Install @("abacusai.cmd","abacusai"))) {
        Write-Info "Abacus AI CLI already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing Abacus AI CLI..."

    $nodeExe = Join-Path $BinDir "node.exe"
    if (-not (Test-Path $nodeExe)) {
        Write-Warn "Node not found in $BinDir. Install Node portable before Abacus CLI."
        return
    }

    $npmCache = Join-Path $OptDir "npm-cache"
    $npmPrefix = Join-Path $DevToolsDir "npm-global"
    New-Item -ItemType Directory -Force -Path $npmCache, $npmPrefix | Out-Null
    $env:NPM_CONFIG_CACHE = $npmCache
    $env:NPM_CONFIG_PREFIX = $npmPrefix

    $npmCmd = Join-Path (Split-Path $nodeExe -Parent) "npm.cmd"
    & $npmCmd install -g @abacus-ai/cli

    $abacusPath = Join-Path $npmPrefix "node_modules\.bin\abacusai.cmd"
    if (Test-Path $abacusPath) {
        Add-Link $abacusPath "abacusai.cmd"
        Write-Ok "Abacus AI CLI installed"
    }
}

function Install-VSCodeWithCopilot {
    <#
    .SYNOPSIS
        Install VS Code portable and GitHub Copilot extension (Priority 5)
    #>
    if (-not (Should-Install @("code.cmd","code"))) {
        Write-Info "VS Code already present; skipping (use -UpdateExisting to force)"
        return
    }
    Write-Info "Installing VS Code (portable) with GitHub Copilot..."

    $toolDir = Join-Path $DevToolsDir "vscode"
    New-Item -ItemType Directory -Force -Path $toolDir | Out-Null

    # Download VS Code portable (Windows x64)
    $vscodeUrl = "https://code.visualstudio.com/sha/download?build=stable&os=win32-x64-archive"
    $tmpZip = Join-Path ([IO.Path]::GetTempPath()) "vscode-portable.zip"

    try {
        Write-Info "  Downloading VS Code portable..."
        Invoke-WebRequest -Uri $vscodeUrl -OutFile $tmpZip -UseBasicParsing

        Write-Info "  Extracting to $toolDir..."
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        [System.IO.Compression.ZipFile]::ExtractToDirectory($tmpZip, $toolDir, $true)
        Remove-Item $tmpZip -Force

        # Create portable mode marker
        $dataDir = Join-Path $toolDir "data"
        New-Item -ItemType Directory -Force -Path $dataDir | Out-Null

        $codePath = Join-Path $toolDir "bin/code.cmd"
        if (Test-Path $codePath) {
            Add-Link $codePath "code.cmd"
            Write-Ok "VS Code portable installed"

            # Install GitHub Copilot extension
            Write-Info "  Installing GitHub Copilot extension..."
            $codeExe = Join-Path $toolDir "Code.exe"
            if (Test-Path $codeExe) {
                & $codeExe --install-extension GitHub.copilot --force 2>$null
                & $codeExe --install-extension GitHub.copilot-chat --force 2>$null
                Write-Ok "GitHub Copilot extensions installed"
            }
        }
    } catch {
        Write-Warn "VS Code download failed: $_"
        Write-Warn "Manual install: https://code.visualstudio.com/download"
    }

    # Ensure provider config directory exists
    $providerConfigDir = Join-Path $NoaRoot "ai/providers/ide/vscode-copilot"
    if (-not (Test-Path $providerConfigDir)) {
        New-Item -ItemType Directory -Force -Path $providerConfigDir | Out-Null
        Write-Info "  Created provider config directory: $providerConfigDir"
    }
}

function Install-GitCliProvider {
    <#
    .SYNOPSIS
        Install Git CLI as AI provider (Priority 6) with shared resources access
    #>
    # First ensure Git is installed
    if (-not (Should-Install @("git.exe"))) {
        Write-Info "Git CLI already present as provider"
    } else {
        Install-Git
    }

    Write-Info "Configuring Git CLI as AI provider (Priority 6)..."

    # Ensure provider config directory exists
    $providerConfigDir = Join-Path $NoaRoot "ai/providers/local/git-cli"
    if (-not (Test-Path $providerConfigDir)) {
        New-Item -ItemType Directory -Force -Path $providerConfigDir | Out-Null
        Write-Info "  Created provider config directory: $providerConfigDir"
    }

    Write-Ok "Git CLI configured as AI provider"
}

function Install-AllAiProviders {
    <#
    .SYNOPSIS
        Install all AI provider CLIs in priority order
    #>
    Write-Info "Installing AI Provider CLIs (FR-039)..."

    # Ensure Node is installed first (required for npm-based CLIs)
    $nodeExe = Join-Path $BinDir "node.exe"
    if (-not (Test-Path $nodeExe)) {
        Write-Info "  Installing Node.js first (required for AI provider CLIs)..."
        Install-NodePortable
    }

    # Install in priority order (from plan.md Provider Priority table)
    # Priority 1: llama.cpp (handled separately as submodule)
    Install-CursorCli          # Priority 2 (manual - requires Cursor IDE)
    Install-ClaudeCode         # Priority 3
    Install-CodexCli           # Priority 4
    Install-VSCodeWithCopilot  # Priority 5 (IDE with extension)
    Install-GitCliProvider     # Priority 6 (local)
    Install-AbacusCli          # Priority 7

    # Install shared resources after all providers
    Install-SharedResources

    Write-Ok "AI Provider CLI installation complete"
}

#endregion

#region Shared Provider Resources (FR-037 to FR-042)

function Install-SharedResources {
    <#
    .SYNOPSIS
        Create shared resource directories and initialize execution memory bus
        This enables all AI providers to share context, reasoning state, and resources
    #>
    Write-Info "Installing Shared Provider Resources (FR-037 to FR-042)..."

    $sharedDir = Join-Path $NoaRoot "ai/shared"

    # Create all shared resource directories
    $sharedDirs = @(
        "agents",
        "workflows",
        "prompts",
        "skills",
        "tools",
        "models",
        "commands",
        "resources",
        "resources/context",
        "resources/state"
    )

    foreach ($dir in $sharedDirs) {
        $fullPath = Join-Path $sharedDir $dir
        if (-not (Test-Path $fullPath)) {
            New-Item -ItemType Directory -Force -Path $fullPath | Out-Null
            Write-Info "  Created: ai/shared/$dir"
        }
    }

    # Create shared execution memory database (SQLite)
    $dbPath = Join-Path $sharedDir "resources/execution-memory.db"
    if (-not (Test-Path $dbPath)) {
        # Create empty SQLite database file
        New-Item -ItemType File -Force -Path $dbPath | Out-Null
        Write-Info "  Created execution memory database: $dbPath"
    }

    # Create shared resources config
    $configPath = Join-Path $NoaRoot "config/shared-resources.json"
    if (-not (Test-Path $configPath)) {
        $config = @{
            version = "1.0.0"
            basePath = "`${NOA_ROOT}/ai/shared"
            executionMemory = @{
                enabled = $true
                path = "`${NOA_ROOT}/ai/shared/resources/execution-memory.db"
            }
            directories = @{
                agents = "`${NOA_ROOT}/ai/shared/agents"
                workflows = "`${NOA_ROOT}/ai/shared/workflows"
                prompts = "`${NOA_ROOT}/ai/shared/prompts"
                skills = "`${NOA_ROOT}/ai/shared/skills"
                tools = "`${NOA_ROOT}/ai/shared/tools"
                models = "`${NOA_ROOT}/ai/shared/models"
                commands = "`${NOA_ROOT}/ai/shared/commands"
                resources = "`${NOA_ROOT}/ai/shared/resources"
            }
        }
        $config | ConvertTo-Json -Depth 10 | Set-Content -Path $configPath -Encoding UTF8
        Write-Info "  Created shared resources config: $configPath"
    }

    # Update provider configs to reference shared resources
    $providers = @(
        "ai/providers/cloud/claude-code",
        "ai/providers/cloud/codex",
        "ai/providers/cloud/abacus",
        "ai/providers/hybrid/cursor",
        "ai/providers/ide/vscode-copilot",
        "ai/providers/local/git-cli"
    )

    foreach ($providerPath in $providers) {
        $fullProviderPath = Join-Path $NoaRoot $providerPath
        if (-not (Test-Path $fullProviderPath)) {
            New-Item -ItemType Directory -Force -Path $fullProviderPath | Out-Null
            Write-Info "  Created provider directory: $providerPath"
        }
    }

    Write-Ok "Shared Provider Resources installed"
}

#endregion

#region Main Entry Point

# Handle -ListArchived switch (B160)
if ($ListArchived) {
    Write-Info "Archived tool versions in ${ArchiveDir}:"
    $archives = Get-ArchivedTools
    if ($archives.Count -eq 0) {
        Write-Host "  No archived tools found" -ForegroundColor Gray
    } else {
        $archives | ForEach-Object {
            Write-Host "  $($_.Tool) v$($_.Version) - archived $($_.ArchivedAt)" -ForegroundColor Cyan
            Write-Host "    Path: $($_.ArchivePath)" -ForegroundColor Gray
        }
    }
    exit 0
}

# Handle -Rollback switch (B160)
if ($Rollback) {
    if (-not $Tool -or $Tool.Count -eq 0) {
        Write-Err "Usage: install-all-tools.ps1 -Rollback -Tool <toolname>"
        Write-Info "Use -ListArchived to see available archived versions"
        exit 1
    }

    $success = $true
    foreach ($t in $Tool) {
        if (-not (Restore-ArchivedTool -ToolName $t)) {
            $success = $false
        }
    }

    if ($success) {
        Write-Ok "Rollback complete"
        exit 0
    } else {
        Write-Err "Some rollbacks failed"
        exit 1
    }
}

# Clean old archives on regular runs
Remove-OldArchives -RetentionDays 7

$allTools = @(
    "rust","go","protoc","golangci-lint","eslint","ruff",
    "gitleaks","trivy","grype","semgrep","gh","git","gitlfs","node","python",
    "cmake","ninja","llvm","mingw",
    "claude-code","codex-cli","cursor-cli","abacus-cli","vscode-copilot","git-cli","ai-providers","shared-resources"
)
$targets = if ($Tool -and $Tool.Count -gt 0) { $Tool } else { $allTools }

#endregion

foreach ($t in $targets) {
    switch ($t.ToLower()) {
        "rust"         { Install-Rust }
        "go"           { Install-Go }
        "protoc"       { Install-Protoc }
        "golangci-lint"{ Install-GolangciLint }
        "eslint"       { Install-Eslint }
        "ruff"         { Install-Ruff }
        "gitleaks"     { Install-Gitleaks }
        "trivy"        { Install-Trivy }
        "grype"        { Install-Grype }
        "semgrep"      { Install-Semgrep }
        "gh"           { Install-GitHubCli }
        "git"          { Install-Git }
        "gitlfs"       { Install-GitLfs }
        "node"         { Install-NodePortable }
        "python"       { Install-PythonPortable }
        # Build tools (B161-B170)
        "cmake"        { Install-CMake }
        "ninja"        { Install-Ninja }
        "llvm"         { Install-LLVM }
        "mingw"        { Install-MinGW }
        # AI Provider CLIs (FR-039)
        "claude-code"  { Install-ClaudeCode }
        "codex-cli"    { Install-CodexCli }
        "cursor-cli"   { Install-CursorCli }
        "abacus-cli"   { Install-AbacusCli }
        "vscode-copilot" { Install-VSCodeWithCopilot }
        "git-cli"      { Install-GitCliProvider }
        "ai-providers" { Install-AllAiProviders }
        # Shared Provider Resources (FR-037 to FR-042)
        "shared-resources" { Install-SharedResources }
        default        { Write-Warn "Unknown tool: $t" }
    }
}

Write-Ok "Install finished. Ensure $BinDir is first in PATH for this session."
Write-Info "AI Provider configs located at: $NoaRoot\ai\providers\"
Write-Info "Shared resources at: $NoaRoot\ai\shared\"

