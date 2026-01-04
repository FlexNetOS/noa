[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$CsvPath = "N:\noa\docs\plans\configs-audit-table.csv",

    [Parameter(Mandatory = $false)]
    [string]$OutPath = "N:\noa\configs\README.generated.md"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$data = @(Import-Csv -Path $CsvPath)

# Core: configss under configs/
$configsRows = $data |
    Where-Object { $_.location -like 'configs/*' -and $_.asset_type -in @('configs','schema','template','policy') } |
    Sort-Object location

$sb = New-Object System.Text.StringBuilder

[void]$sb.AppendLine('# NOA configs Registry Quick Reference')
[void]$sb.AppendLine('')
[void]$sb.AppendLine('This file is auto-generated from `docs/plans/configs-audit-table.csv`.')
[void]$sb.AppendLine('')
[void]$sb.AppendLine('## Files')
[void]$sb.AppendLine('')

foreach ($r in $configsRows) {
    $loc = $r.location
    $typ = $r.asset_type
    $auth = $r.authority
    $schema = $r.schema_ref
    $reload = $r.reloadable
    $owner = $r.owner

    [void]$sb.AppendLine("- `$loc`  ")
    [void]$sb.AppendLine("  - type: $typ; authority: $auth; owner: $owner; reloadable: $reload")
    if ($schema) {
        [void]$sb.AppendLine("  - schema: $schema")
    }
    if ($r.merge_key) {
        [void]$sb.AppendLine("  - merge_key: $($r.merge_key)")
    }
}

$dir = Split-Path -Parent $OutPath
if (-not (Test-Path -LiteralPath $dir)) {
    New-Item -ItemType Directory -Path $dir -Force | Out-Null
}

[System.IO.File]::WriteAllText($OutPath, $sb.ToString(), [System.Text.Encoding]::UTF8)
Write-Output "Wrote $OutPath"
