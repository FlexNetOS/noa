[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$CsvPath = "N:\noa\docs\plans\config-audit-table.csv"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# normalize first
& "$PSScriptRoot\normalize-config-audit-csv.ps1" -Path $CsvPath | Out-Null

$data = @(Import-Csv -Path $CsvPath)

$errors = New-Object System.Collections.Generic.List[string]

function Assert-True {
    param([bool]$Cond, [string]$Message)
    if (-not $Cond) { $errors.Add($Message) }
}

# 1) Required fields for all rows
foreach ($r in $data) {
    Assert-True ([string]::IsNullOrWhiteSpace($r.asset_id) -eq $false) "Missing asset_id for location=$($r.location)"
    Assert-True ([string]::IsNullOrWhiteSpace($r.location) -eq $false) "Missing location for asset_id=$($r.asset_id)"
    Assert-True ([string]::IsNullOrWhiteSpace($r.asset_type) -eq $false) "Missing asset_type for $($r.location)"
    Assert-True ([string]::IsNullOrWhiteSpace($r.layer) -eq $false) "Missing layer for $($r.location)"
    Assert-True ([string]::IsNullOrWhiteSpace($r.authority) -eq $false) "Missing authority for $($r.location)"
    Assert-True ([string]::IsNullOrWhiteSpace($r.owner) -eq $false) "Missing owner for $($r.location)"
}

# 2) Authoritative assets must declare validation_mode
$auth = $data | Where-Object { $_.authority -eq 'authoritative' }
foreach ($r in $auth) {
    Assert-True (-not [string]::IsNullOrWhiteSpace($r.validation_mode)) "Authoritative asset missing validation_mode: $($r.location)"
}

# 3) Secrets policy: secret files must not be tracked in git
$secretInGit = $data | Where-Object { $_.confidentiality -eq 'secret' -and $_.tracked_in_git -eq 'yes' }
foreach ($r in $secretInGit) {
    $errors.Add("Secret tracked in git: $($r.location)")
}

# 4) Containment: any under data/appdata must be contained=yes
$appdata = $data | Where-Object { $_.location -like 'data/appdata/*' }
foreach ($r in $appdata) {
    Assert-True ($r.contained -eq 'yes') "AppData row not marked contained=yes: $($r.location)"
}

if ($errors.Count -gt 0) {
    Write-Error ("Config audit validation failed (" + $errors.Count + " errors):`n" + ($errors -join "`n"))
    exit 1
}

Write-Output "OK: validated $($data.Length) rows"
