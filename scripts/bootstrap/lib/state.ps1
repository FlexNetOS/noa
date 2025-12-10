<#
.SYNOPSIS
    Bootstrap state management for NOA

.DESCRIPTION
    Manages bootstrap-state.json which tracks installed tools, versions,
    and installation metadata. Per NOA Constitution §3.1.
#>

$script:StateFile = $null
$script:State = $null

function Initialize-BootstrapState {
    <#
    .SYNOPSIS
        Initialize or load the bootstrap state file
    #>
    param(
        [Parameter(Mandatory)]
        [string]$NoaRoot
    )

    $script:StateFile = Join-Path $NoaRoot "config/bootstrap-state.json"

    $configDir = Split-Path $script:StateFile -Parent
    if (-not (Test-Path $configDir)) {
        New-Item -ItemType Directory -Path $configDir -Force | Out-Null
    }

    if (Test-Path $script:StateFile) {
        try {
            $script:State = Get-Content $script:StateFile -Raw | ConvertFrom-Json -AsHashtable
        } catch {
            Write-Warning "Failed to parse bootstrap-state.json, creating new state"
            $script:State = New-BootstrapState
        }
    } else {
        $script:State = New-BootstrapState
    }

    return $script:State
}

function New-BootstrapState {
    <#
    .SYNOPSIS
        Create a new bootstrap state object
    #>
    return @{
        version = "1.0.0"
        created_at = (Get-Date -Format "o")
        updated_at = (Get-Date -Format "o")
        bootstrap_version = "1.0.0"
        tools = @{}
        toolchains = @{}
        providers = @{}
        platform = @{}
    }
}

function Get-ToolState {
    <#
    .SYNOPSIS
        Get the installation state of a specific tool
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ToolName
    )

    if (-not $script:State) {
        throw "Bootstrap state not initialized. Call Initialize-BootstrapState first."
    }

    if ($script:State.tools.ContainsKey($ToolName)) {
        return $script:State.tools[$ToolName]
    }

    return $null
}

function Set-ToolState {
    <#
    .SYNOPSIS
        Set the installation state of a tool
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ToolName,

        [Parameter(Mandatory)]
        [string]$Version,

        [Parameter(Mandatory)]
        [string]$Path,

        [ValidateSet("installed", "updated", "failed", "skipped", "pending")]
        [string]$Status = "installed",

        [hashtable]$Metadata = @{}
    )

    if (-not $script:State) {
        throw "Bootstrap state not initialized. Call Initialize-BootstrapState first."
    }

    $script:State.tools[$ToolName] = @{
        version = $Version
        path = $Path
        status = $Status
        installed_at = (Get-Date -Format "o")
        metadata = $Metadata
    }

    $script:State.updated_at = (Get-Date -Format "o")
    Save-BootstrapState
}

function Get-ToolchainState {
    <#
    .SYNOPSIS
        Get the installation state of a toolchain (Rust, Go, Node, Python)
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ToolchainName
    )

    if (-not $script:State) {
        throw "Bootstrap state not initialized. Call Initialize-BootstrapState first."
    }

    if ($script:State.toolchains.ContainsKey($ToolchainName)) {
        return $script:State.toolchains[$ToolchainName]
    }

    return $null
}

function Set-ToolchainState {
    <#
    .SYNOPSIS
        Set the installation state of a toolchain
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ToolchainName,

        [Parameter(Mandatory)]
        [string]$Version,

        [Parameter(Mandatory)]
        [string]$RootPath,

        [ValidateSet("installed", "updated", "failed", "skipped", "pending")]
        [string]$Status = "installed",

        [hashtable]$EnvVars = @{},

        [string[]]$Components = @()
    )

    if (-not $script:State) {
        throw "Bootstrap state not initialized. Call Initialize-BootstrapState first."
    }

    $script:State.toolchains[$ToolchainName] = @{
        version = $Version
        root_path = $RootPath
        status = $Status
        installed_at = (Get-Date -Format "o")
        env_vars = $EnvVars
        components = $Components
    }

    $script:State.updated_at = (Get-Date -Format "o")
    Save-BootstrapState
}

function Get-ProviderState {
    <#
    .SYNOPSIS
        Get the installation state of an AI provider
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ProviderName
    )

    if (-not $script:State) {
        throw "Bootstrap state not initialized. Call Initialize-BootstrapState first."
    }

    if ($script:State.providers.ContainsKey($ProviderName)) {
        return $script:State.providers[$ProviderName]
    }

    return $null
}

function Set-ProviderState {
    <#
    .SYNOPSIS
        Set the installation state of an AI provider
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ProviderName,

        [Parameter(Mandatory)]
        [string]$Version,

        [Parameter(Mandatory)]
        [string]$ConfigPath,

        [ValidateSet("installed", "updated", "failed", "skipped", "pending", "configured")]
        [string]$Status = "installed",

        [string]$Priority = "medium"
    )

    if (-not $script:State) {
        throw "Bootstrap state not initialized. Call Initialize-BootstrapState first."
    }

    $script:State.providers[$ProviderName] = @{
        version = $Version
        config_path = $ConfigPath
        status = $Status
        installed_at = (Get-Date -Format "o")
        priority = $Priority
    }

    $script:State.updated_at = (Get-Date -Format "o")
    Save-BootstrapState
}

function Save-BootstrapState {
    <#
    .SYNOPSIS
        Save the current state to disk with schema validation
    #>

    if (-not $script:StateFile -or -not $script:State) {
        throw "Bootstrap state not initialized."
    }

    # Validate state against schema if schema library is available
    $schemaLib = Join-Path (Split-Path $PSScriptRoot -Parent) "lib\schema.ps1"
    if (Test-Path $schemaLib) {
        . $schemaLib
        $validation = Test-BootstrapStateSchema -StateFilePath $script:StateFile
        if (-not $validation.IsValid) {
            Write-Warning "State validation warnings: $($validation.Errors -join ', ')"
        }
    }

    $script:State | ConvertTo-Json -Depth 10 | Set-Content -Path $script:StateFile -Encoding UTF8
}

function Get-FullBootstrapState {
    <#
    .SYNOPSIS
        Get the full bootstrap state object
    #>
    return $script:State
}

function Test-ToolInstalled {
    <#
    .SYNOPSIS
        Check if a tool is installed and optionally verify version
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ToolName,

        [string]$MinVersion = $null
    )

    $state = Get-ToolState -ToolName $ToolName
    if (-not $state -or $state.status -ne "installed") {
        return $false
    }

    if ($MinVersion) {
        # Simple version comparison (assumes semver-like)
        $installed = [version]($state.version -replace '[^0-9.]', '')
        $required = [version]($MinVersion -replace '[^0-9.]', '')
        return $installed -ge $required
    }

    return $true
}

# Export functions
Export-ModuleMember -Function @(
    'Initialize-BootstrapState',
    'Get-ToolState',
    'Set-ToolState',
    'Get-ToolchainState',
    'Set-ToolchainState',
    'Get-ProviderState',
    'Set-ProviderState',
    'Save-BootstrapState',
    'Get-FullBootstrapState',
    'Test-ToolInstalled'
)

