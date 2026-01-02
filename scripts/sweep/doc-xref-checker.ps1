<#
.SYNOPSIS
    Doc Cross-Reference Checker - Validates symbols against documentation

.DESCRIPTION
    Compares extracted symbols against wiki, runbooks, and pages documentation
    to identify undocumented symbols and stale references.

.PARAMETER SymbolsFile
    Path to JSON file containing extracted symbols

.PARAMETER OutputPath
    Path for gap report output

.EXAMPLE
    .\doc-xref-checker.ps1 -SymbolsFile "data/state/sweep/symbols.json"
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$SymbolsFile,

    [Parameter(Mandatory = $false)]
    [string]$OutputPath,

    [Parameter(Mandatory = $false)]
    [switch]$GenerateReport,

    [Parameter(Mandatory = $false)]
    [int]$MaxParallel = 8
)

$ErrorActionPreference = "Stop"
$script:NoaRoot = (Resolve-Path "$PSScriptRoot\..\..").Path

#region Configuration
$script:DocPaths = @{
    Wiki     = "$script:NoaRoot\docs\wiki"
    Runbooks = "$script:NoaRoot\docs\runbooks"
    Pages    = "$script:NoaRoot\docs\pages"
    Reference = "$script:NoaRoot\docs\reference"
    API      = "$script:NoaRoot\docs\api"
}

$script:OutputDir = $OutputPath ?? "$script:NoaRoot\data\state\sweep"
#endregion

#region Documentation Indexing
function Build-DocIndex {
    param([string]$DocPath, [string]$DocType)
    
    $index = @{
        files    = @{}
        symbols  = @{}
        headings = @{}
        links    = @{}
    }
    
    if (!(Test-Path $DocPath)) {
        return $index
    }
    
    $mdFiles = Get-ChildItem -Path $DocPath -Filter "*.md" -Recurse -ErrorAction SilentlyContinue
    
    foreach ($file in $mdFiles) {
        $relativePath = $file.FullName.Replace($script:NoaRoot, "").TrimStart("\", "/")
        $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
        
        if (!$content) { continue }
        
        $index.files[$relativePath] = @{
            path       = $file.FullName
            modified   = $file.LastWriteTime
            size       = $file.Length
            type       = $DocType
        }
        
        # Extract headings
        $lineNum = 0
        foreach ($line in ($content -split "`n")) {
            $lineNum++
            if ($line -match '^(#{1,6})\s+(.+)') {
                $heading = $Matches[2].Trim()
                if (!$index.headings[$heading]) {
                    $index.headings[$heading] = @()
                }
                $index.headings[$heading] += @{
                    file = $relativePath
                    line = $lineNum
                    level = $Matches[1].Length
                }
            }
        }
        
        # Extract code references (backticked symbols)
        $codeRefs = [regex]::Matches($content, '`([^`]+)`')
        foreach ($match in $codeRefs) {
            $symbol = $match.Groups[1].Value
            if ($symbol.Length -gt 2 -and $symbol.Length -lt 100 -and $symbol -match '^\w') {
                if (!$index.symbols[$symbol]) {
                    $index.symbols[$symbol] = @()
                }
                $index.symbols[$symbol] += $relativePath
            }
        }
        
        # Extract links
        $linkRefs = [regex]::Matches($content, '\[([^\]]+)\]\(([^)]+)\)')
        foreach ($match in $linkRefs) {
            $linkText = $match.Groups[1].Value
            $linkTarget = $match.Groups[2].Value
            
            $index.links[$linkTarget] = @{
                text = $linkText
                from = $relativePath
            }
        }
    }
    
    return $index
}

function Build-AllDocIndexes {
    Write-Host "Building documentation indexes..."
    
    $indexes = @{}
    
    foreach ($docType in $script:DocPaths.Keys) {
        $path = $script:DocPaths[$docType]
        Write-Host "  Indexing $docType at $path"
        $indexes[$docType] = Build-DocIndex -DocPath $path -DocType $docType
    }
    
    return $indexes
}
#endregion

#region Cross-Reference Analysis
function Find-SymbolInDocs {
    param(
        [string]$SymbolName,
        [hashtable]$DocIndexes
    )
    
    $refs = @{
        Found     = $false
        Wiki      = @()
        Runbooks  = @()
        Pages     = @()
        Reference = @()
        API       = @()
    }
    
    foreach ($docType in $DocIndexes.Keys) {
        $index = $DocIndexes[$docType]
        
        # Check direct symbol references
        if ($index.symbols[$SymbolName]) {
            $refs[$docType] += $index.symbols[$SymbolName]
            $refs.Found = $true
        }
        
        # Check headings
        if ($index.headings[$SymbolName]) {
            $refs[$docType] += ($index.headings[$SymbolName] | ForEach-Object { $_.file })
            $refs.Found = $true
        }
        
        # Fuzzy match (symbol appears in heading)
        foreach ($heading in $index.headings.Keys) {
            if ($heading -like "*$SymbolName*") {
                $refs[$docType] += ($index.headings[$heading] | ForEach-Object { $_.file })
                $refs.Found = $true
            }
        }
    }
    
    # Deduplicate
    foreach ($docType in $refs.Keys) {
        if ($refs[$docType] -is [array]) {
            $refs[$docType] = $refs[$docType] | Select-Object -Unique
        }
    }
    
    return $refs
}

function Get-DocumentedSymbols {
    param([hashtable]$DocIndexes)
    
    $allSymbols = @{}
    
    foreach ($docType in $DocIndexes.Keys) {
        foreach ($symbol in $DocIndexes[$docType].symbols.Keys) {
            if (!$allSymbols[$symbol]) {
                $allSymbols[$symbol] = @()
            }
            $allSymbols[$symbol] += $docType
        }
    }
    
    return $allSymbols
}
#endregion

#region Gap Analysis
function Get-DocumentationGaps {
    param(
        [object[]]$CodeSymbols,
        [hashtable]$DocIndexes
    )
    
    $gaps = @{
        Undocumented       = @()  # Symbols in code but not in docs
        WellDocumented     = @()  # Symbols with full coverage
        PartiallyDocumented = @()  # Symbols in some docs
        Orphaned           = @()  # Docs referencing non-existent symbols
    }
    
    $docSymbols = Get-DocumentedSymbols -DocIndexes $DocIndexes
    $codeSymbolNames = @{}
    
    # Analyze each code symbol
    foreach ($symbol in $CodeSymbols) {
        $name = $symbol.Name
        $codeSymbolNames[$name] = $true
        
        $refs = Find-SymbolInDocs -SymbolName $name -DocIndexes $DocIndexes
        
        if (!$refs.Found) {
            $gaps.Undocumented += [PSCustomObject]@{
                Symbol     = $name
                Type       = $symbol.Type
                File       = $symbol.FilePath
                Line       = $symbol.Line
                Visibility = $symbol.Visibility
            }
        }
        else {
            $docCount = ($refs.Wiki.Count -gt 0) + ($refs.Runbooks.Count -gt 0) + ($refs.Pages.Count -gt 0)
            
            if ($docCount -ge 2) {
                $gaps.WellDocumented += [PSCustomObject]@{
                    Symbol    = $name
                    Type      = $symbol.Type
                    File      = $symbol.FilePath
                    Wiki      = $refs.Wiki -join ", "
                    Runbooks  = $refs.Runbooks -join ", "
                    Pages     = $refs.Pages -join ", "
                }
            }
            else {
                $gaps.PartiallyDocumented += [PSCustomObject]@{
                    Symbol   = $name
                    Type     = $symbol.Type
                    File     = $symbol.FilePath
                    Coverage = @($refs.Wiki, $refs.Runbooks, $refs.Pages, $refs.Reference, $refs.API) | 
                        Where-Object { $_.Count -gt 0 } | 
                        ForEach-Object { $_ -join ", " }
                }
            }
        }
    }
    
    # Find orphaned documentation (docs referencing non-existent code)
    foreach ($symbol in $docSymbols.Keys) {
        # Skip common words and short symbols
        if ($symbol.Length -lt 3 -or $symbol -match '^(the|and|for|with|this|that|from|have|will|can|are|was|been|being|has|had|get|set|new|old|all|any|not|but|use|see|also|note|todo|fixme)$') {
            continue
        }
        
        # Check if it looks like a code symbol
        if ($symbol -match '^[A-Z][a-z]' -or $symbol -match '^[a-z]+_[a-z]' -or $symbol -match '^\$\w+') {
            if (!$codeSymbolNames[$symbol]) {
                $gaps.Orphaned += [PSCustomObject]@{
                    Symbol    = $symbol
                    DocTypes  = $docSymbols[$symbol] -join ", "
                }
            }
        }
    }
    
    return $gaps
}
#endregion

#region Report Generation
function New-GapReport {
    param(
        [hashtable]$Gaps,
        [string]$OutputPath
    )
    
    $timestamp = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
    
    $report = @"
# Documentation Gap Report

**Generated**: $timestamp
**Noa Root**: $script:NoaRoot

## Summary

| Category | Count |
|----------|-------|
| Undocumented Symbols | $($Gaps.Undocumented.Count) |
| Partially Documented | $($Gaps.PartiallyDocumented.Count) |
| Well Documented | $($Gaps.WellDocumented.Count) |
| Orphaned Doc References | $($Gaps.Orphaned.Count) |

## Undocumented Symbols (Top Priority)

These public symbols have no documentation in wiki, runbooks, or pages:

| Symbol | Type | File | Line | Visibility |
|--------|------|------|------|------------|
"@

    foreach ($item in ($Gaps.Undocumented | Where-Object { $_.Visibility -eq 'public' -or $_.Visibility -eq 'export' } | Select-Object -First 100)) {
        $file = $item.File.Replace($script:NoaRoot, "").TrimStart("\", "/")
        $report += "| ``$($item.Symbol)`` | $($item.Type) | $file | $($item.Line) | $($item.Visibility) |`n"
    }

    $report += @"

## Partially Documented Symbols

These symbols have some documentation but lack full coverage:

| Symbol | Type | Coverage |
|--------|------|----------|
"@

    foreach ($item in ($Gaps.PartiallyDocumented | Select-Object -First 50)) {
        $coverage = ($item.Coverage -join "; ").Substring(0, [Math]::Min(80, ($item.Coverage -join "; ").Length))
        $report += "| ``$($item.Symbol)`` | $($item.Type) | $coverage |`n"
    }

    $report += @"

## Well Documented Symbols

These symbols have comprehensive documentation:

| Symbol | Type | Wiki | Pages |
|--------|------|------|-------|
"@

    foreach ($item in ($Gaps.WellDocumented | Select-Object -First 50)) {
        $report += "| ``$($item.Symbol)`` | $($item.Type) | $($item.Wiki) | $($item.Pages) |`n"
    }

    $report += @"

## Orphaned Documentation References

These documentation references may point to renamed or removed code:

| Symbol | Found In |
|--------|----------|
"@

    foreach ($item in ($Gaps.Orphaned | Select-Object -First 50)) {
        $report += "| ``$($item.Symbol)`` | $($item.DocTypes) |`n"
    }

    # Save report
    $reportPath = "$OutputPath\doc-gap-report.md"
    Set-Content -Path $reportPath -Value $report -Encoding UTF8
    Write-Host "Gap report saved to: $reportPath" -ForegroundColor Green
    
    # Also save as JSON for programmatic access
    $jsonPath = "$OutputPath\doc-gaps.json"
    $Gaps | ConvertTo-Json -Depth 10 | Set-Content -Path $jsonPath -Encoding UTF8
    Write-Host "Gap data saved to: $jsonPath" -ForegroundColor Green
    
    return $reportPath
}
#endregion

#region Ripgrep Integration
function Search-WithRipgrep {
    param(
        [string]$Pattern,
        [string]$Path,
        [string]$FileType = "md"
    )
    
    $rg = Get-Command rg -ErrorAction SilentlyContinue
    if (!$rg) {
        $rg = "$script:NoaRoot\bin\rg"
        if (!(Test-Path $rg)) {
            return $null
        }
    }
    
    try {
        $results = & $rg --json -t $FileType -e $Pattern $Path 2>$null | 
            Where-Object { $_ } |
            ForEach-Object { $_ | ConvertFrom-Json -ErrorAction SilentlyContinue } |
            Where-Object { $_.type -eq 'match' }
        
        return $results
    }
    catch {
        return $null
    }
}
#endregion

#region Main Execution
function Start-DocXrefCheck {
    Write-Host "=" * 60
    Write-Host "DOCUMENTATION CROSS-REFERENCE CHECK"
    Write-Host "=" * 60
    
    # Ensure output directory exists
    if (!(Test-Path $script:OutputDir)) {
        New-Item -ItemType Directory -Path $script:OutputDir -Force | Out-Null
    }
    
    # Build documentation indexes
    $docIndexes = Build-AllDocIndexes
    
    $totalDocs = 0
    foreach ($docType in $docIndexes.Keys) {
        $count = $docIndexes[$docType].files.Count
        Write-Host "  $docType`: $count files"
        $totalDocs += $count
    }
    Write-Host "  Total: $totalDocs documentation files"
    
    # Load symbols if provided, otherwise extract fresh
    $codeSymbols = @()
    
    if ($SymbolsFile -and (Test-Path $SymbolsFile)) {
        Write-Host "`nLoading symbols from $SymbolsFile"
        $codeSymbols = Get-Content $SymbolsFile -Raw | ConvertFrom-Json
    }
    else {
        Write-Host "`nExtracting symbols from codebase..."
        
        $extractor = "$PSScriptRoot\symbol-extractor.ps1"
        if (Test-Path $extractor) {
            # Find all source files
            $sourceFiles = Get-ChildItem -Path $script:NoaRoot -Include "*.rs", "*.ts", "*.py" -Recurse -ErrorAction SilentlyContinue |
                Where-Object { 
                    $_.FullName -notlike "*\node_modules\*" -and 
                    $_.FullName -notlike "*\target\*" -and
                    $_.FullName -notlike "*\.git\*"
                } |
                Select-Object -First 1000  # Limit for initial run
            
            Write-Host "  Found $($sourceFiles.Count) source files to analyze"
            
            $count = 0
            foreach ($file in $sourceFiles) {
                $count++
                if ($count % 100 -eq 0) {
                    Write-Host "  Processed $count files..."
                }
                
                try {
                    $symbols = & $extractor -FilePath $file.FullName -OutputFormat object 2>$null
                    if ($symbols) {
                        $codeSymbols += $symbols
                    }
                }
                catch {
                    # Skip files that fail extraction
                }
            }
        }
    }
    
    Write-Host "`nFound $($codeSymbols.Count) symbols in codebase"
    
    # Perform gap analysis
    Write-Host "`nAnalyzing documentation coverage..."
    $gaps = Get-DocumentationGaps -CodeSymbols $codeSymbols -DocIndexes $docIndexes
    
    # Generate report
    if ($GenerateReport) {
        $reportPath = New-GapReport -Gaps $gaps -OutputPath $script:OutputDir
    }
    
    # Summary
    Write-Host "`n" + "=" * 60
    Write-Host "SUMMARY"
    Write-Host "=" * 60
    Write-Host "  Undocumented symbols: $($gaps.Undocumented.Count)" -ForegroundColor $(if ($gaps.Undocumented.Count -gt 100) { 'Red' } else { 'Yellow' })
    Write-Host "  Partially documented: $($gaps.PartiallyDocumented.Count)" -ForegroundColor Yellow
    Write-Host "  Well documented: $($gaps.WellDocumented.Count)" -ForegroundColor Green
    Write-Host "  Orphaned references: $($gaps.Orphaned.Count)" -ForegroundColor $(if ($gaps.Orphaned.Count -gt 50) { 'Red' } else { 'Yellow' })
    
    return $gaps
}

# Execute
Start-DocXrefCheck
