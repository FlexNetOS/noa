<#
.SYNOPSIS
    Tool verification utilities for NOA bootstrap

.DESCRIPTION
    Verifies installed tools and determines required actions:
    SKIP, UPDATE, INSTALL, or RELOCATE.
    Per NOA Constitution §3.1.
#>

# Verification result actions
enum VerifyAction {
    SKIP       # Already installed and up-to-date
    UPDATE     # Installed but needs update
    INSTALL    # Not installed
    RELOCATE   # Installed but in wrong location
}

function Test-ToolVerification {
    <#
    .SYNOPSIS
        Verify a tool installation and determine required action

    .PARAMETER ToolName
        Name of the tool to verify

    .PARAMETER ExpectedPath
        Expected installation path (within noa_root)

    .PARAMETER MinVersion
        Minimum required version

    .PARAMETER VersionCommand
        Command to get installed version (e.g., "rustc --version")

    .OUTPUTS
        Object with Action (VerifyAction) and Details
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ToolName,

        [Parameter(Mandatory)]
        [string]$ExpectedPath,

        [string]$MinVersion,

        [string]$VersionCommand
    )

    $result = @{
        Tool = $ToolName
        Action = [VerifyAction]::INSTALL
        InstalledPath = $null
        InstalledVersion = $null
        ExpectedPath = $ExpectedPath
        RequiredVersion = $MinVersion
        Reason = ""
    }

    # Check if tool exists at expected path
    $toolExe = $ToolName
    if ($env:OS -eq "Windows_NT" -and -not $ToolName.EndsWith(".exe")) {
        $toolExe = "$ToolName.exe"
    }

    $expectedExePath = Join-Path $ExpectedPath $toolExe
    $existsAtExpected = Test-Path $expectedExePath

    # Check if tool exists in system PATH
    $systemTool = Get-Command $ToolName -ErrorAction SilentlyContinue
    $existsInPath = $null -ne $systemTool

    if ($existsAtExpected) {
        # Tool exists at expected location
        $result.InstalledPath = $ExpectedPath

        if ($VersionCommand) {
            try {
                $versionOutput = Invoke-Expression $VersionCommand 2>&1
                $versionMatch = [regex]::Match($versionOutput, '\d+\.\d+(\.\d+)?')
                if ($versionMatch.Success) {
                    $result.InstalledVersion = $versionMatch.Value
                }
            } catch {
                $result.InstalledVersion = "unknown"
            }
        }

        if ($MinVersion -and $result.InstalledVersion) {
            try {
                $installed = [version]($result.InstalledVersion -replace '[^0-9.]', '')
                $required = [version]($MinVersion -replace '[^0-9.]', '')
                if ($installed -lt $required) {
                    $result.Action = [VerifyAction]::UPDATE
                    $result.Reason = "Version $($result.InstalledVersion) < required $MinVersion"
                } else {
                    $result.Action = [VerifyAction]::SKIP
                    $result.Reason = "Already installed (v$($result.InstalledVersion))"
                }
            } catch {
                $result.Action = [VerifyAction]::SKIP
                $result.Reason = "Already installed (version check failed)"
            }
        } else {
            $result.Action = [VerifyAction]::SKIP
            $result.Reason = "Already installed at expected location"
        }
    }
    elseif ($existsInPath) {
        # Tool exists in PATH but not at expected location
        $result.InstalledPath = $systemTool.Source
        $result.Action = [VerifyAction]::RELOCATE
        $result.Reason = "Installed at $($systemTool.Source), should be at $ExpectedPath"

        if ($VersionCommand) {
            try {
                $versionOutput = Invoke-Expression $VersionCommand 2>&1
                $versionMatch = [regex]::Match($versionOutput, '\d+\.\d+(\.\d+)?')
                if ($versionMatch.Success) {
                    $result.InstalledVersion = $versionMatch.Value
                }
            } catch {
                $result.InstalledVersion = "unknown"
            }
        }
    }
    else {
        # Tool not installed
        $result.Action = [VerifyAction]::INSTALL
        $result.Reason = "Not installed"
    }

    return $result
}

function Test-ToolchainVerification {
    <#
    .SYNOPSIS
        Verify a toolchain installation (Rust, Go, Node, Python)

    .PARAMETER Toolchain
        Name of the toolchain

    .PARAMETER ExpectedRoot
        Expected root directory (e.g., noa_root/opt/rust)

    .PARAMETER MinVersion
        Minimum required version

    .PARAMETER EnvVars
        Hashtable of environment variables that should be set
    #>
    param(
        [Parameter(Mandatory)]
        [ValidateSet("rust", "go", "node", "python")]
        [string]$Toolchain,

        [Parameter(Mandatory)]
        [string]$ExpectedRoot,

        [string]$MinVersion,

        [hashtable]$EnvVars = @{}
    )

    $result = @{
        Toolchain = $Toolchain
        Action = [VerifyAction]::INSTALL
        InstalledPath = $null
        InstalledVersion = $null
        ExpectedRoot = $ExpectedRoot
        RequiredVersion = $MinVersion
        EnvVarsSet = @{}
        Reason = ""
    }

    # Toolchain-specific checks
    switch ($Toolchain) {
        "rust" {
            $binaryPath = Join-Path $ExpectedRoot "cargo/bin/rustc.exe"
            if (-not $binaryPath.EndsWith(".exe") -and $env:OS -ne "Windows_NT") {
                $binaryPath = Join-Path $ExpectedRoot "cargo/bin/rustc"
            }

            if (Test-Path $binaryPath) {
                $result.InstalledPath = $ExpectedRoot
                $env:RUSTUP_HOME = Join-Path $ExpectedRoot "rustup"
                $env:CARGO_HOME = Join-Path $ExpectedRoot "cargo"

                try {
                    $version = & $binaryPath --version 2>&1
                    $versionMatch = [regex]::Match($version, '\d+\.\d+\.\d+')
                    if ($versionMatch.Success) {
                        $result.InstalledVersion = $versionMatch.Value
                    }
                } catch { }

                $result = Test-VersionAndSetAction $result $MinVersion
            }
        }

        "go" {
            $binaryPath = Join-Path $ExpectedRoot "bin/go.exe"
            if (-not $binaryPath.EndsWith(".exe") -and $env:OS -ne "Windows_NT") {
                $binaryPath = Join-Path $ExpectedRoot "bin/go"
            }

            if (Test-Path $binaryPath) {
                $result.InstalledPath = $ExpectedRoot

                try {
                    $version = & $binaryPath version 2>&1
                    $versionMatch = [regex]::Match($version, 'go(\d+\.\d+(\.\d+)?)')
                    if ($versionMatch.Success) {
                        $result.InstalledVersion = $versionMatch.Groups[1].Value
                    }
                } catch { }

                $result = Test-VersionAndSetAction $result $MinVersion
            }
        }

        "node" {
            $binaryPath = Join-Path $ExpectedRoot "node.exe"
            if (-not $binaryPath.EndsWith(".exe") -and $env:OS -ne "Windows_NT") {
                $binaryPath = Join-Path $ExpectedRoot "bin/node"
            }

            if (Test-Path $binaryPath) {
                $result.InstalledPath = $ExpectedRoot

                try {
                    $version = & $binaryPath --version 2>&1
                    $versionMatch = [regex]::Match($version, '(\d+\.\d+\.\d+)')
                    if ($versionMatch.Success) {
                        $result.InstalledVersion = $versionMatch.Value
                    }
                } catch { }

                $result = Test-VersionAndSetAction $result $MinVersion
            }
        }

        "python" {
            $binaryPath = Join-Path $ExpectedRoot "python.exe"
            if (-not $binaryPath.EndsWith(".exe") -and $env:OS -ne "Windows_NT") {
                $binaryPath = Join-Path $ExpectedRoot "bin/python3"
            }

            if (Test-Path $binaryPath) {
                $result.InstalledPath = $ExpectedRoot

                try {
                    $version = & $binaryPath --version 2>&1
                    $versionMatch = [regex]::Match($version, '(\d+\.\d+\.\d+)')
                    if ($versionMatch.Success) {
                        $result.InstalledVersion = $versionMatch.Value
                    }
                } catch { }

                $result = Test-VersionAndSetAction $result $MinVersion
            }
        }
    }

    # Check environment variables
    foreach ($envKey in $EnvVars.Keys) {
        $result.EnvVarsSet[$envKey] = ($env:$envKey -eq $EnvVars[$envKey])
    }

    return $result
}

function Test-VersionAndSetAction {
    param(
        [hashtable]$Result,
        [string]$MinVersion
    )

    if ($MinVersion -and $Result.InstalledVersion) {
        try {
            $installed = [version]($Result.InstalledVersion -replace '[^0-9.]', '')
            $required = [version]($MinVersion -replace '[^0-9.]', '')
            if ($installed -lt $required) {
                $Result.Action = [VerifyAction]::UPDATE
                $Result.Reason = "Version $($Result.InstalledVersion) < required $MinVersion"
            } else {
                $Result.Action = [VerifyAction]::SKIP
                $Result.Reason = "Already installed (v$($Result.InstalledVersion))"
            }
        } catch {
            $Result.Action = [VerifyAction]::SKIP
            $Result.Reason = "Already installed (version check failed)"
        }
    } else {
        $Result.Action = [VerifyAction]::SKIP
        $Result.Reason = "Already installed"
    }

    return $Result
}

function Get-VerificationSummary {
    <#
    .SYNOPSIS
        Generate a summary table of verification results
    #>
    param(
        [array]$Results
    )

    $summary = @{
        Skip = @()
        Update = @()
        Install = @()
        Relocate = @()
    }

    foreach ($r in $Results) {
        $key = $r.Action.ToString()
        if ($r.Tool) {
            $summary[$key] += $r.Tool
        } elseif ($r.Toolchain) {
            $summary[$key] += $r.Toolchain
        }
    }

    return $summary
}

# Export functions
Export-ModuleMember -Function @(
    'Test-ToolVerification',
    'Test-ToolchainVerification',
    'Get-VerificationSummary'
)

