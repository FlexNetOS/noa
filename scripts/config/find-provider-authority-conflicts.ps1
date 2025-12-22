[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$CsvPath = "N:\noa\docs\plans\config-audit-table.csv"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$data = @(Import-Csv -Path $CsvPath)

$providers = @(
    'config/ai-providers.json',
    'config/providers/default.yaml'
)

$providerRows = @($data | Where-Object { $providers -contains $_.location -or $_.location -like 'ai/providers/*/config.json' })

Write-Output "provider_rows=$($providerRows.Length)"

$providerRows | Sort-Object location | Select-Object location,authority,owner,consumer_status,planned_consumer,conflicts_with,truth_link | Format-Table -AutoSize
