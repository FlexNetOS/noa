[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$CsvPath = "N:\noa\docs\plans\configs-audit-table.csv",

    [Parameter(Mandatory = $false)]
    [string]$RepoRoot = "N:\noa"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName 'Microsoft.VisualBasic'

function Get-NextAssetId {
    param([object[]]$Rows)

    $max = 0
    foreach ($r in $Rows) {
        $id = [string]$r.asset_id
        if ($id -match '^A(\d+)$') {
            $n = [int]$Matches[1]
            if ($n -gt $max) { $max = $n }
        }
    }
    return ('A{0:0000}' -f ($max + 1))
}

function New-Row {
    param(
        [string[]]$Header,
        [hashtable]$Values
    )

    $h = @{}
    foreach ($k in $Header) { $h[$k] = '' }
    foreach ($k in $Values.Keys) { $h[$k] = [string]$Values[$k] }
    return [pscustomobject]$h
}

function Add-FileRows {
    param(
        [string[]]$Header,
        [System.Collections.Generic.List[object]]$Out,
        [hashtable]$Existing,
        [string]$StartRel,
        [hashtable]$Defaults,
        [scriptblock]$PerFile
    )

    $startPath = Join-Path $RepoRoot $StartRel
    if (-not (Test-Path -LiteralPath $startPath)) {
        return
    }

    Get-ChildItem -Path $startPath -File -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
        $full = $_.FullName
        $rel = $full.Substring($RepoRoot.Length) -replace '^[\\/]+', ''
        $rel = $rel -replace '\\','/'

        if ($Existing.ContainsKey($rel)) { return }

        $vals = @{}
        foreach ($k in $Defaults.Keys) { $vals[$k] = $Defaults[$k] }

        $vals['location'] = $rel
        $vals['name'] = $_.Name
        $vals['truth_link'] = $rel

        if ($PerFile) {
            & $PerFile $_ $vals
        }

        if (-not $vals['location']) { return }

        $Out.Add((New-Row -Header $Header -Values $vals))
        $Existing[$rel] = $true
    }
}

if (-not (Test-Path -LiteralPath $CsvPath)) {
    throw "Missing CSV: $CsvPath"
}

# Read CSV with TextFieldParser
$rowsRaw = New-Object System.Collections.Generic.List[string[]]
$parser = New-Object Microsoft.VisualBasic.FileIO.TextFieldParser($CsvPath)
try {
    $parser.TextFieldType = [Microsoft.VisualBasic.FileIO.FieldType]::Delimited
    $parser.SetDelimiters(',')
    $parser.HasFieldsEnclosedInQuotes = $true

    while (-not $parser.EndOfData) {
        $rowsRaw.Add($parser.ReadFields())
    }
}
finally {
    $parser.Close()
}

if ($rowsRaw.Count -lt 2) {
    throw "CSV has no data rows: $CsvPath"
}

$header = $rowsRaw[0]
$width = $header.Count

# Convert to objects
$rows = New-Object System.Collections.Generic.List[object]
$existing = @{}
for ($i=1; $i -lt $rowsRaw.Count; $i++) {
    $r = $rowsRaw[$i]
    if ($r.Count -lt $width) {
        $r = $r + (('') * ($width - $r.Count))
    }
    if ($r.Count -gt $width) {
        $r = $r[0..($width-2)] + @((($r[($width-1)..($r.Count-1)]) -join ', '))
    }

    $h = @{
    }
    for ($c=0; $c -lt $width; $c++) { $h[$header[$c]] = $r[$c] }
    $obj = [pscustomobject]$h
    $rows.Add($obj)

    $loc = [string]$obj.location
    if ($loc) { $existing[$loc] = $true }
}

$added = New-Object System.Collections.Generic.List[object]

# Defaults
$aiDefaults = @{
    asset_type='configs'
    layer='mutable'
    authority='authoritative'
    owner='ai'
    consumer_status='none'
    planned_consumer=''
    validation_mode='warn'
    reloadable='no'
    contained='yes'
    containment_mechanism='env redirect'
    kernel_mode_relevance='NA'
    platform_scope='all'
    confidentiality='internal'
    has_secrets='unknown'
    tracked_in_git='yes'
    env_example_covered='yes'
    reproducible='yes'
    versioning='semver'
    lineage_required='yes'
    doc_tier='Draft'
    indexed='no'
    traced='no'
}

# AI shared
Add-FileRows -Header $header -Out $added -Existing $existing -StartRel 'ai/shared' -Defaults $aiDefaults -PerFile {
    param($file, $vals)
    $ext = $file.Extension.ToLowerInvariant()

    if ($ext -eq '.md') { $vals['asset_type']='doc'; $vals['validation_mode']='none'; $vals['has_secrets']='no' }
    elseif ($ext -eq '.ps1' -or $ext -eq '.js' -or $ext -eq '.ts') { $vals['asset_type']='tooling'; $vals['validation_mode']='none' }
    elseif ($ext -eq '.sql') { $vals['asset_type']='schema'; $vals['layer']='immutable'; $vals['validation_mode']='hard_fail'; $vals['has_secrets']='no' }

    if ($vals['location'] -like 'ai/shared/models/*') {
        if ($ext -eq '.json') { $vals['layer']='cache' }
    }
}

# AI providers
$provDefaults = $aiDefaults.Clone()
$provDefaults['consumer_status'] = 'planned'
$provDefaults['planned_consumer'] = 'sys/core/src/providers/*'
$provDefaults['schema_ref'] = 'configs/schemas/providers.yaml'
$provDefaults['schema_status'] = 'exists'
$provDefaults['reloadable'] = 'yes'
$provDefaults['env_vars_used'] = 'OPENAI_API_KEY;ANTHROPIC_API_KEY;ABACUS_API_KEY;GITHUB_TOKEN'

Add-FileRows -Header $header -Out $added -Existing $existing -StartRel 'ai/providers' -Defaults $provDefaults -PerFile {
    param($file, $vals)
    if ($file.Name -ne 'configs.json') {
        $vals['location'] = ''
        return
    }
}

# sys/core crates
$coreDefaults = @{
    asset_type='tooling'
    layer='mutable'
    authority='authoritative'
    owner='sys-core'
    consumer_status='none'
    validation_mode='hard_fail'
    reloadable='no'
    contained='NA'
    containment_mechanism='NA'
    kernel_mode_relevance='NA'
    platform_scope='all'
    confidentiality='internal'
    has_secrets='no'
    tracked_in_git='yes'
    reproducible='yes'
    versioning='semver'
    lineage_required='NA'
    doc_tier='NA'
    indexed='no'
    traced='no'
}

Add-FileRows -Header $header -Out $added -Existing $existing -StartRel 'sys/core/crates' -Defaults $coreDefaults -PerFile {
    param($file, $vals)
    if ($file.Name -ne 'Cargo.toml') {
        $vals['location'] = ''
        return
    }
    $vals['name'] = ($file.Directory.Name + '/Cargo.toml')
}

# sys/kernel
$kernelDefaults = @{
    asset_type='tooling'
    layer='immutable'
    authority='authoritative'
    owner='sys-kernel'
    consumer_status='planned'
    planned_consumer='sys/kernel/** scripts'
    validation_mode='warn'
    reloadable='no'
    contained='yes'
    containment_mechanism='env redirect'
    kernel_mode_relevance='native;vm;container;sandbox'
    platform_scope='all'
    confidentiality='internal'
    has_secrets='no'
    tracked_in_git='yes'
    reproducible='yes'
    versioning='none'
    lineage_required='NA'
    doc_tier='Draft'
    indexed='no'
    traced='no'
}

Add-FileRows -Header $header -Out $added -Existing $existing -StartRel 'sys/kernel' -Defaults $kernelDefaults -PerFile {
    param($file, $vals)
    $rel = $vals['location']
    if ($rel -like 'sys/kernel/windows/*') { $vals['platform_scope'] = 'windows' }
    elseif ($rel -like 'sys/kernel/linux/*') { $vals['platform_scope'] = 'linux' }
    elseif ($rel -like 'sys/kernel/macos/*') { $vals['platform_scope'] = 'macos' }

    if ($file.Extension.ToLowerInvariant() -eq '.md') { $vals['asset_type']='doc'; $vals['validation_mode']='none' }
    if ($file.Name -eq 'current.json') { $vals['asset_type']='runtime_state'; $vals['layer']='state' }
}

# Assign asset_id to newly added rows
foreach ($r in $added) {
    $r.asset_id = Get-NextAssetId -Rows $rows
    $rows.Add($r)
}

# Write back CSV (simple writer with quoting)
$tmp = "$CsvPath.tmp"
$sw = New-Object System.IO.StreamWriter($tmp, $false, [System.Text.Encoding]::UTF8)
try {
    $sw.WriteLine(($header -join ','))
    foreach ($r in $rows) {
        $fields = foreach ($h in $header) {
            $v = [string]$r.$h
            if ($null -eq $v) { $v = '' }
            $needsQuotes = $v.Contains(',') -or $v.Contains('"') -or $v.Contains("`n") -or $v.Contains("`r")
            $v = $v.Replace('"', '""')
            if ($needsQuotes) { '"' + $v + '"' } else { $v }
        }
        $sw.WriteLine(($fields -join ','))
    }
}
finally {
    $sw.Close()
}

Move-Item -LiteralPath $tmp -Destination $CsvPath -Force
Write-Output ("Added " + $added.Count + " rows")
