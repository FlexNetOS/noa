<#
.SYNOPSIS
    Platform detection utilities for NOA bootstrap

.DESCRIPTION
    Detects OS, architecture, shell, and platform capabilities.
    Per NOA Constitution §3.1: Cross-platform support
#>

function Get-PlatformInfo {
    <#
    .SYNOPSIS
        Get comprehensive platform information
    #>

    $info = @{
        OS = "unknown"
        Architecture = "unknown"
        Shell = "unknown"
        IsWSL = $false
        WSLVersion = 0
        IsAdmin = $false
        IsElevated = $false
        HasDocker = $false
        HasGPU = $false
        HostOS = "unknown"
    }

    # Detect OS
    if ($IsWindows -or $env:OS -eq "Windows_NT") {
        $info.OS = "windows"
        $info.HostOS = "windows"

        # Check for WSL
        if (Test-Path "/proc/version") {
            $procVersion = Get-Content "/proc/version" -ErrorAction SilentlyContinue
            if ($procVersion -match "microsoft") {
                $info.IsWSL = $true
                $info.HostOS = "windows"
                if (Test-Path "/run/WSL") {
                    $info.OS = "wsl2"
                    $info.WSLVersion = 2
                } else {
                    $info.OS = "wsl1"
                    $info.WSLVersion = 1
                }
            }
        }
    } elseif ($IsMacOS) {
        $info.OS = "macos"
        $info.HostOS = "macos"
    } elseif ($IsLinux) {
        $info.OS = "linux"
        $info.HostOS = "linux"
    }

    # Detect architecture
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    $info.Architecture = switch ($arch) {
        "X64"  { "amd64" }
        "Arm64" { "arm64" }
        "X86"  { "x86" }
        "Arm"  { "arm" }
        default { "unknown" }
    }

    # Detect shell
    $info.Shell = if ($PSVersionTable.PSEdition -eq "Core") {
        "pwsh"
    } elseif ($env:SHELL) {
        Split-Path $env:SHELL -Leaf
    } else {
        "powershell"
    }

    # Check admin/elevated
    if ($info.OS -eq "windows" -or $info.OS -like "wsl*") {
        $info.IsAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
        $info.IsElevated = $info.IsAdmin
    } else {
        $info.IsAdmin = (id -u) -eq 0
        $info.IsElevated = $info.IsAdmin
    }

    # Check Docker
    $info.HasDocker = $null -ne (Get-Command docker -ErrorAction SilentlyContinue)

    # Check GPU (CUDA)
    if (Get-Command nvidia-smi -ErrorAction SilentlyContinue) {
        $info.HasGPU = $true
    }

    return $info
}

function Get-OSDownloadSuffix {
    <#
    .SYNOPSIS
        Get the appropriate download suffix for the current platform
    #>
    param(
        [string]$ToolName,
        [hashtable]$PlatformInfo = $null
    )

    if (-not $PlatformInfo) {
        $PlatformInfo = Get-PlatformInfo
    }

    $os = $PlatformInfo.OS
    $arch = $PlatformInfo.Architecture

    # Common patterns
    $suffix = switch ($os) {
        "windows" { "windows-$arch" }
        "macos"   { "darwin-$arch" }
        "linux"   { "linux-$arch" }
        "wsl1"    { "linux-$arch" }
        "wsl2"    { "linux-$arch" }
        default   { "linux-$arch" }
    }

    # Tool-specific overrides
    $suffix = switch ($ToolName) {
        "rust" {
            switch ($os) {
                "windows" { "x86_64-pc-windows-msvc" }
                "macos"   { if ($arch -eq "arm64") { "aarch64-apple-darwin" } else { "x86_64-apple-darwin" } }
                default   { "x86_64-unknown-linux-gnu" }
            }
        }
        "go" {
            switch ($os) {
                "windows" { "windows-amd64.zip" }
                "macos"   { if ($arch -eq "arm64") { "darwin-arm64.tar.gz" } else { "darwin-amd64.tar.gz" } }
                default   { "linux-amd64.tar.gz" }
            }
        }
        "node" {
            switch ($os) {
                "windows" { "win-x64.zip" }
                "macos"   { if ($arch -eq "arm64") { "darwin-arm64.tar.gz" } else { "darwin-x64.tar.gz" } }
                default   { "linux-x64.tar.xz" }
            }
        }
        "protoc" {
            switch ($os) {
                "windows" { "win64.zip" }
                "macos"   { if ($arch -eq "arm64") { "osx-aarch_64.zip" } else { "osx-x86_64.zip" } }
                default   { "linux-x86_64.zip" }
            }
        }
        default { $suffix }
    }

    return $suffix
}

function Get-ExecutableExtension {
    <#
    .SYNOPSIS
        Get the executable extension for the current platform
    #>
    param([hashtable]$PlatformInfo = $null)

    if (-not $PlatformInfo) {
        $PlatformInfo = Get-PlatformInfo
    }

    if ($PlatformInfo.OS -eq "windows") {
        return ".exe"
    }
    return ""
}

function Test-CommandExists {
    <#
    .SYNOPSIS
        Check if a command exists in PATH
    #>
    param([string]$Command)

    return $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

# Export functions
Export-ModuleMember -Function @(
    'Get-PlatformInfo',
    'Get-OSDownloadSuffix',
    'Get-ExecutableExtension',
    'Test-CommandExists'
)

