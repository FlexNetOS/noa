[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$Path = "N:\noa\docs\plans\config-audit-table.csv"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$bytes = [System.IO.File]::ReadAllBytes($Path)
if ($bytes.Length -ge 3 -and $bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB -and $bytes[2] -eq 0xBF) {
    $trimmed = New-Object byte[] ($bytes.Length - 3)
    [Array]::Copy($bytes, 3, $trimmed, 0, $trimmed.Length)
    [System.IO.File]::WriteAllBytes($Path, $trimmed)
    Write-Output "removed_bom=yes"
}
else {
    Write-Output "removed_bom=no"
}
