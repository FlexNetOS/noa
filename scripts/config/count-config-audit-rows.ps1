[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$Path = "N:\noa\docs\plans\configs-audit-table.csv"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$data = @(Import-Csv -Path $Path)

$rows = $data.Length
$aiShared = ($data | Where-Object { $_.location -like 'ai/shared/*' } | Measure-Object).Count
$aiProviders = ($data | Where-Object { $_.location -like 'ai/providers/*' } | Measure-Object).Count
$kernel = ($data | Where-Object { $_.location -like 'sys/kernel/*' } | Measure-Object).Count
$crates = ($data | Where-Object { $_.location -like 'sys/core/crates/*' } | Measure-Object).Count

Write-Output "rows=$rows"
Write-Output "ai_shared=$aiShared"
Write-Output "ai_providers=$aiProviders"
Write-Output "sys_kernel=$kernel"
Write-Output "sys_core_crates=$crates"
