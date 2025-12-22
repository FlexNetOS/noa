[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$CsvPath = "N:\noa\docs\plans\config-audit-table.csv"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Always normalize first
& "$PSScriptRoot\normalize-config-audit-csv.ps1" -Path $CsvPath | Out-Null

$data = @(Import-Csv -Path $CsvPath)

foreach ($r in $data) {
    # Ensure core fields exist
    if (-not $r.asset_type) { $r.asset_type = 'config' }
    if (-not $r.layer) { $r.layer = 'mutable' }
    if (-not $r.authority) { $r.authority = 'authoritative' }
    if (-not $r.owner) { $r.owner = 'unknown' }

    # Normalize booleans-like fields
    foreach ($k in 'indexed','traced') {
        if (-not $r.$k) { $r.$k = 'no' }
    }
    foreach ($k in 'reloadable') {
        if (-not $r.$k) { $r.$k = 'no' }
    }

    # Containment defaults
    if (-not $r.contained) { $r.contained = 'NA' }
    if (-not $r.containment_mechanism) { $r.containment_mechanism = 'NA' }

    # Git defaults
    if (-not $r.tracked_in_git) { $r.tracked_in_git = 'yes' }

    # Security defaults
    if (-not $r.confidentiality) { $r.confidentiality = 'internal' }
    if (-not $r.has_secrets) { $r.has_secrets = 'unknown' }
}

# Persist
$header = $data[0].PSObject.Properties.Name
$tmp = "$CsvPath.tmp"
$data | Select-Object $header | Export-Csv -Path $tmp -NoTypeInformation -Encoding utf8
Move-Item -LiteralPath $tmp -Destination $CsvPath -Force

& "$PSScriptRoot\normalize-config-audit-csv.ps1" -Path $CsvPath | Out-Null

$missing = @($data | Where-Object { $_.asset_type -eq '' -or $_.layer -eq '' -or $_.authority -eq '' -or $_.owner -eq '' }).Length
Write-Output "Rebased rows=$($data.Length) missing_core_fields=$missing"
