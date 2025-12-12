<#
.SYNOPSIS
  Generate editor-specific MCP configuration files from a single shared source of truth.

.DESCRIPTION
  Reads `config/mcp/servers.json` (shared, no secrets) and writes:
    - Cursor (user scope):   $env:USERPROFILE\.cursor\mcp.json              (key: mcpServers)
    - VS Code (workspace):   .\.vscode\mcp.json                              (key: servers)
    - VS Code (user):        $env:APPDATA\Code\User\settings.json            (key: "mcp.servers")
    - Windsurf (user):       $env:USERPROFILE\.codeium\windsurf\mcp_config.json (key: mcpServers)
    - Claude Desktop (user): $env:APPDATA\Claude\claude_desktop_config.json  (key: mcpServers)

  Secrets must be provided via environment variables (e.g. GITHUB_PERSONAL_ACCESS_TOKEN).

.PARAMETER NoaRoot
  Path to NOA root. Defaults to current directory.

.PARAMETER CursorOnly
  Only generate Cursor config.

.PARAMETER VscodeOnly
  Only generate VS Code workspace config.

.PARAMETER VscodeUserOnly
  Only update VS Code user settings (mcp.servers).

.PARAMETER WindsurfOnly
  Only generate Windsurf config.

.PARAMETER ClaudeDesktopOnly
  Only generate Claude Desktop config.

.PARAMETER Check
  Validate generated/installed configs against the shared source without writing anything.

.PARAMETER DryRun
  Print planned writes without writing.
#>

[CmdletBinding()]
param(
  [string]$NoaRoot = (Get-Location).Path,
  [switch]$CursorOnly,
  [switch]$VscodeOnly,
  [switch]$VscodeUserOnly,
  [switch]$WindsurfOnly,
  [switch]$ClaudeDesktopOnly,
  [switch]$Check,
  [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Read-JsonFile([string]$Path) {
  if (-not (Test-Path $Path)) {
    throw "Missing file: $Path"
  }
  return Get-Content $Path -Raw -Encoding UTF8 | ConvertFrom-Json
}

function Ensure-Dir([string]$Path) {
  if (-not (Test-Path $Path)) {
    New-Item -ItemType Directory -Force -Path $Path | Out-Null
  }
}

function Write-JsonFile([string]$Path, $Obj) {
  $json = $Obj | ConvertTo-Json -Depth 20
  if ($DryRun) {
    Write-Host "DRY RUN: would write $Path"
    return
  }
  $dir = Split-Path -Parent $Path
  Ensure-Dir $dir
  $json | Out-File -FilePath $Path -Encoding UTF8 -NoNewline
  Write-Host "Wrote $Path"
}

function Convert-SharedToMcpServers($sharedServers) {
  $out = @{}
  foreach ($p in $sharedServers.PSObject.Properties) {
    $name = $p.Name
    $s = $p.Value
    $envMap = @{}
    if ($s.requiredEnv) {
      foreach ($k in $s.requiredEnv) {
        $envMap[$k] = '${' + $k + '}'
      }
    }
    $out[$name] = @{
      type    = 'stdio'
      command = $s.command
      args    = @($s.args)
    }
    if ($envMap.Keys.Count -gt 0) {
      $out[$name].env = $envMap
    }
  }
  return @{ mcpServers = $out }
}

function Convert-SharedToVscode($sharedServers) {
  # VS Code project format in this repo uses `.vscode/mcp.json` with a `servers` root.
  $out = @{}
  foreach ($p in $sharedServers.PSObject.Properties) {
    $name = $p.Name
    $s = $p.Value
    $envMap = @{}
    if ($s.requiredEnv) {
      foreach ($k in $s.requiredEnv) {
        $envMap[$k] = '${' + $k + '}'
      }
    }
    $out[$name] = @{
      type    = 'stdio'
      command = $s.command
      args    = @($s.args)
    }
    if ($envMap.Keys.Count -gt 0) {
      $out[$name].env = $envMap
    }
  }
  return @{ servers = $out }
}

function Get-VsCodeUserSettingsPath() {
  # Support common Windows installs
  $candidates = @()
  if ($env:APPDATA) {
    $candidates += (Join-Path $env:APPDATA 'Code\User\settings.json')
    $candidates += (Join-Path $env:APPDATA 'Code - Insiders\User\settings.json')
  }
  foreach ($p in $candidates) {
    if (Test-Path $p) { return $p }
  }
  # Default to stable path even if not present (caller may create)
  if ($env:APPDATA) {
    return (Join-Path $env:APPDATA 'Code\User\settings.json')
  }
  throw "APPDATA is not set; cannot locate VS Code user settings."
}

function Update-JsonObjectFile([string]$Path, [scriptblock]$Mutate) {
  $obj = $null
  if (Test-Path $Path) {
    try {
      $obj = Get-Content $Path -Raw -Encoding UTF8 | ConvertFrom-Json
    } catch {
      throw "Failed to parse JSON in $Path. If this is JSONC (comments), prefer workspace .vscode/mcp.json generation. Error: $($_.Exception.Message)"
    }
  } else {
    $obj = [pscustomobject]@{}
  }

  & $Mutate $obj
  Write-JsonFile $Path $obj
}

function Get-EnvPresence([string]$Name) {
  $v = [Environment]::GetEnvironmentVariable($Name, 'Process')
  if (-not $v) { $v = [Environment]::GetEnvironmentVariable($Name, 'User') }
  if (-not $v) { $v = [Environment]::GetEnvironmentVariable($Name, 'Machine') }
  return [bool]$v
}

function Get-ServerNamesFromObject($obj, [string]$keyPath) {
  switch ($keyPath) {
    'cursor' { return @($obj.mcpServers.PSObject.Properties.Name) }
    'windsurf' { return @($obj.mcpServers.PSObject.Properties.Name) }
    'claudeDesktop' { return @($obj.mcpServers.PSObject.Properties.Name) }
    'vscodeWorkspace' { return @($obj.servers.PSObject.Properties.Name) }
    'vscodeUser' {
      # property name includes a dot
      if ($null -eq $obj.'mcp.servers') { return @() }
      return @($obj.'mcp.servers'.PSObject.Properties.Name)
    }
    default { return @() }
  }
}

$sharedPath = Join-Path $NoaRoot 'config/mcp/servers.json'
$shared = Read-JsonFile $sharedPath
if (-not $shared.servers) {
  throw "Invalid shared MCP config: missing `.servers` in $sharedPath"
}

if ($Check) {
  if ($DryRun) { throw "Use either -Check or -DryRun, not both." }
}

$explicitTarget =
  $CursorOnly -or $VscodeOnly -or $VscodeUserOnly -or $WindsurfOnly -or $ClaudeDesktopOnly

$doCursor = ($CursorOnly -or (-not $explicitTarget))
$doVscodeWorkspace = ($VscodeOnly -or (-not $explicitTarget))
$doVscodeUser = $VscodeUserOnly
$doWindsurf = ($WindsurfOnly -or (-not $explicitTarget))
$doClaudeDesktop = ($ClaudeDesktopOnly -or (-not $explicitTarget))

function Write-OrCheck([string]$name, [string]$path, [string]$mode, [scriptblock]$writer) {
  if ($Check) {
    if (-not (Test-Path $path)) {
      Write-Host "CHECK: missing $name config at $path"
      return
    }
    $obj = Get-Content $path -Raw -Encoding UTF8 | ConvertFrom-Json
    $expected = @($shared.servers.PSObject.Properties.Name) | Sort-Object
    $actual = (Get-ServerNamesFromObject $obj $mode) | Sort-Object
    $missing = @($expected | Where-Object { $_ -notin $actual })
    $extra = @($actual | Where-Object { $_ -notin $expected })
    if ($missing.Count -gt 0) {
      Write-Host "CHECK: $name missing shared servers: [$($missing -join ', ')]"
    } else {
      if ($extra.Count -gt 0) {
        Write-Host "CHECK: $name OK (has shared servers). extra=[$($extra -join ', ')]"
      } else {
        Write-Host "CHECK: $name OK ($($actual -join ', '))"
      }
    }

    foreach ($p in $shared.servers.PSObject.Properties) {
      $s = $p.Value
      if ($s.requiredEnv) {
        foreach ($k in $s.requiredEnv) {
          if (-not (Get-EnvPresence $k)) {
            Write-Host "CHECK: missing required env var: $k"
          }
        }
      }
    }
    return
  }

  & $writer
}

if ($doCursor) {
  $cursorTarget = Join-Path $env:USERPROFILE '.cursor\mcp.json'
  Write-OrCheck 'Cursor(user)' $cursorTarget 'cursor' {
    $cursorCfg = Convert-SharedToMcpServers $shared.servers
    Write-JsonFile $cursorTarget $cursorCfg
  }
}

if ($doVscodeWorkspace) {
  $vscodeTarget = Join-Path $NoaRoot '.vscode\mcp.json'
  Write-OrCheck 'VSCode(workspace)' $vscodeTarget 'vscodeWorkspace' {
    $vscodeCfg = Convert-SharedToVscode $shared.servers
    Write-JsonFile $vscodeTarget $vscodeCfg
  }
}

if ($doVscodeUser) {
  $vscodeUserPath = Get-VsCodeUserSettingsPath
  Write-OrCheck 'VSCode(user)' $vscodeUserPath 'vscodeUser' {
    $serversObj = (Convert-SharedToMcpServers $shared.servers).mcpServers
    try {
      Update-JsonObjectFile $vscodeUserPath {
        param($o)
        # Keep other settings; only overwrite mcp.servers
        $o | Add-Member -NotePropertyName 'mcp.servers' -NotePropertyValue $serversObj -Force
      }
    } catch {
      Write-Host "WARN: $($_.Exception.Message)"
      Write-Host "WARN: Falling back to workspace generation (.vscode/mcp.json)."
      $vscodeTarget = Join-Path $NoaRoot '.vscode\mcp.json'
      $vscodeCfg = Convert-SharedToVscode $shared.servers
      Write-JsonFile $vscodeTarget $vscodeCfg
    }
  }
}

if ($doWindsurf) {
  $windsurfTarget = Join-Path $env:USERPROFILE '.codeium\windsurf\mcp_config.json'
  Write-OrCheck 'Windsurf(user)' $windsurfTarget 'windsurf' {
    $windsurfCfg = Convert-SharedToMcpServers $shared.servers
    Write-JsonFile $windsurfTarget $windsurfCfg
  }
}

if ($doClaudeDesktop) {
  if (-not $env:APPDATA) { Write-Host "WARN: APPDATA not set; skipping Claude Desktop config."; return }
  $claudeTarget = Join-Path $env:APPDATA 'Claude\claude_desktop_config.json'
  Write-OrCheck 'ClaudeDesktop(user)' $claudeTarget 'claudeDesktop' {
    $mcp = Convert-SharedToMcpServers $shared.servers
    # Preserve other Claude Desktop settings if present; only set/overwrite mcpServers
    Update-JsonObjectFile $claudeTarget {
      param($o)
      $o | Add-Member -NotePropertyName 'mcpServers' -NotePropertyValue $mcp.mcpServers -Force
    }
  }
}


