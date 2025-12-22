[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$Path = "N:\noa\docs\plans\config-audit-table.csv"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName 'Microsoft.VisualBasic'

function Normalize-Row {
    param(
        [string[]]$Row,
        [int]$Width
    )

    if ($Row.Count -gt $Width) {
        $head = $Row[0..($Width - 2)]
        $tail = ($Row[($Width - 1)..($Row.Count - 1)] -join ', ')
        return @($head + @($tail))
    }

    if ($Row.Count -lt $Width) {
        $pad = @('') * ($Width - $Row.Count)
        return @($Row + $pad)
    }

    return $Row
}

if (-not (Test-Path -LiteralPath $Path)) {
    throw "CSV not found: $Path"
}

$rows = New-Object System.Collections.Generic.List[string[]]

$parser = New-Object Microsoft.VisualBasic.FileIO.TextFieldParser($Path)
try {
    $parser.TextFieldType = [Microsoft.VisualBasic.FileIO.FieldType]::Delimited
    $parser.SetDelimiters(',')
    $parser.HasFieldsEnclosedInQuotes = $true

    while (-not $parser.EndOfData) {
        $fields = $parser.ReadFields()
        $rows.Add($fields)
    }
}
finally {
    $parser.Close()
}

if ($rows.Count -lt 1) {
    throw "CSV is empty: $Path"
}

$header = $rows[0]
$width = $header.Count
$fixed = New-Object System.Collections.Generic.List[string[]]
$fixed.Add($header)

$fixedCount = 0
for ($i = 1; $i -lt $rows.Count; $i++) {
    $r = $rows[$i]
    if ($r.Count -ne $width) {
        $fixedCount++
    }
    $fixed.Add((Normalize-Row -Row $r -Width $width))
}

$tmp = "$Path.tmp"

$sw = New-Object System.IO.StreamWriter($tmp, $false, [System.Text.Encoding]::UTF8)
try {
    foreach ($r in $fixed) {
        $escaped = foreach ($f in $r) {
            if ($null -eq $f) { $f = '' }
            $s = [string]$f
            $needsQuotes = $s.Contains(',') -or $s.Contains('"') -or $s.Contains("`n") -or $s.Contains("`r")
            $s = $s.Replace('"', '""')
            if ($needsQuotes) { '"' + $s + '"' } else { $s }
        }
        $sw.WriteLine(($escaped -join ','))
    }
}
finally {
    $sw.Close()
}

Move-Item -LiteralPath $tmp -Destination $Path -Force
Write-Output "Normalized $Path (fixed $fixedCount rows)"
