<#
.SYNOPSIS
  Generate editor-specific MCP configuration files from a single shared source of truth.

.DESCRIPTION
  Reads `config/mcp/servers.json` (shared, no secrets) and writes:
    - Cursor (user scope):   $env:USERPROFILE\.cursor\mcp.json      (key: mcpServers)
    - VS Code (workspace):   .\.vscode\mcp.json                     (key: servers)

  Secrets must be provided via environment variables (e.g. GITHUB_PERSONAL_ACCESS_TOKEN).

.PARAMETER NoaRoot
  Path to NOA root. Defaults to current directory.

.PARAMETER CursorOnly
  Only generate Cursor config.

.PARAMETER VscodeOnly
  Only generate VS Code workspace config.

.PARAMETER DryRun
  Print planned writes without writing.
#>

[CmdletBinding()]
param(
  [string]$NoaRoot = (Get-Location).Path,
  [switch]$CursorOnly,
  [switch]$VscodeOnly,
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

function Convert-SharedToCursor($sharedServers) {
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

$sharedPath = Join-Path $NoaRoot 'config/mcp/servers.json'
$shared = Read-JsonFile $sharedPath
if (-not $shared.servers) {
  throw "Invalid shared MCP config: missing `.servers` in $sharedPath"
}

$doCursor = -not $VscodeOnly
$doVscode = -not $CursorOnly

if ($doCursor) {
  $cursorTarget = Join-Path $env:USERPROFILE '.cursor\mcp.json'
  $cursorCfg = Convert-SharedToCursor $shared.servers
  Write-JsonFile $cursorTarget $cursorCfg
}

if ($doVscode) {
  $vscodeTarget = Join-Path $NoaRoot '.vscode\mcp.json'
  $vscodeCfg = Convert-SharedToVscode $shared.servers
  Write-JsonFile $vscodeTarget $vscodeCfg
}


