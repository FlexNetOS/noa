[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$CsvPath = "N:\noa\docs\plans\configs-audit-table.csv"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

& "$PSScriptRoot\normalize-configs-audit-csv.ps1" -Path $CsvPath | Out-Null
& "$PSScriptRoot\remove-bom.ps1" -Path $CsvPath | Out-Null
& "$PSScriptRoot\normalize-configs-audit-csv.ps1" -Path $CsvPath | Out-Null

$data = @(Import-Csv -Path $CsvPath)

$setValidation = @(
    '.env.example',
    'configs/schemas/providers.yaml',
    'configs/schemas/desktop-apps.json',
    'configs/schemas/mcp-servers.json'
)

foreach ($r in $data) {
    if ($setValidation -contains $r.location) {
        if (-not $r.validation_mode) { $r.validation_mode = 'hard_fail' }
    }

    if ($r.location -like 'data/appdata/*') {
        $r.contained = 'yes'
        if (-not $r.containment_mechanism) { $r.containment_mechanism = 'APPDATA redirect' }
    }
}

$header = $data[0].PSObject.Properties.Name
$tmp = "$CsvPath.tmp"
$data | Select-Object $header | Export-Csv -Path $tmp -NoTypeInformation -Encoding utf8
Move-Item -LiteralPath $tmp -Destination $CsvPath -Force

& "$PSScriptRoot\remove-bom.ps1" -Path $CsvPath | Out-Null
& "$PSScriptRoot\normalize-configs-audit-csv.ps1" -Path $CsvPath | Out-Null

Write-Output "fixed"
