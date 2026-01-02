<#
.SYNOPSIS
    Graph Generator - Produces Mermaid diagrams for dependencies and coverage

.DESCRIPTION
    Generates visual graphs showing:
    - Symbol dependency relationships
    - Module/crate dependencies
    - Documentation coverage heatmaps
    - Test coverage visualization

.PARAMETER InputType
    Type of graph to generate: dependencies, coverage, modules, symbols

.PARAMETER OutputPath
    Path for generated Mermaid files

.EXAMPLE
    .\graph-generator.ps1 -InputType dependencies -OutputPath "docs/architecture"
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [ValidateSet('dependencies', 'coverage', 'modules', 'symbols', 'all')]
    [string]$InputType = 'all',

    [Parameter(Mandatory = $false)]
    [string]$OutputPath,

    [Parameter(Mandatory = $false)]
    [string]$SymbolsFile,

    [Parameter(Mandatory = $false)]
    [int]$MaxNodes = 100
)

$ErrorActionPreference = 'Stop'

# Resolve paths - handle when running from different locations
$scriptDir = $PSScriptRoot
if ([string]::IsNullOrEmpty($scriptDir)) {
    $scriptDir = (Get-Location).Path
}

# Navigate up from scripts/sweep to noa root
$script:NoaRoot = (Resolve-Path (Join-Path $scriptDir '..\..')).Path
if ([string]::IsNullOrEmpty($script:NoaRoot)) {
    $script:NoaRoot = 'N:\noa'
}

$script:OutputDir = if ($OutputPath) { $OutputPath } else { Join-Path $script:NoaRoot 'docs\architecture\graphs' }

#region Mermaid Generation
function New-MermaidHeader {
    param([string]$Title, [string]$Type = 'flowchart')
    
    return @"
---
title: $Title
---
$Type TD

"@
}

function ConvertTo-MermaidId {
    param([string]$Name)
    
    # Convert to valid Mermaid node ID
    $id = $Name -replace '[^a-zA-Z0-9_]', '_'
    $id = $id -replace '__+', '_'
    $id = $id.TrimStart('_').TrimEnd('_')
    
    if ($id -match '^\d') {
        $id = "n$id"
    }
    
    return $id.ToLower()
}
#endregion

#region Cargo Dependency Graph
function Get-CargoDependencies {
    Write-Host 'Analyzing Cargo dependencies...'
    
    $cargoFiles = Get-ChildItem -Path $script:NoaRoot -Filter 'Cargo.toml' -Recurse -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -notlike '*\target\*' }
    
    $crates = @{}
    $edges = @()
    
    foreach ($file in $cargoFiles) {
        $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
        if (!$content) { continue }
        
        # Extract package name
        if ($content -match '\[package\]\s*\r?\n\s*name\s*=\s*"([^"]+)"') {
            $crateName = $Matches[1]
            $relativePath = $file.DirectoryName.Replace($script:NoaRoot, '').TrimStart('\', '/')
            
            $crates[$crateName] = @{
                path = $relativePath
                file = $file.FullName
            }
            
            # Extract dependencies
            $depSection = $false
            foreach ($line in ($content -split "`n")) {
                if ($line -match '^\[(.*)?dependencies') {
                    $depSection = $true
                    continue
                }
                elseif ($line -match '^\[' -and $depSection) {
                    $depSection = $false
                }
                
                if ($depSection -and $line -match '^(\w[\w-]*)\s*=' ) {
                    $depName = $Matches[1]
                    
                    # Check if it's a workspace dependency
                    if ($line -match 'path\s*=\s*"([^"]+)"' -or $content -match "$depName\s*=\s*\{\s*path") {
                        $edges += @{
                            from = $crateName
                            to   = $depName
                            type = 'internal'
                        }
                    }
                }
            }
        }
    }
    
    return @{
        crates = $crates
        edges  = $edges
    }
}

function New-CargoDependencyGraph {
    $data = Get-CargoDependencies
    
    $mermaid = New-MermaidHeader -Title 'Cargo Crate Dependencies' -Type 'flowchart'
    
    # Add subgraphs by directory
    $byDir = @{}
    foreach ($crate in $data.crates.Keys) {
        $path = $data.crates[$crate].path
        $topDir = ($path -split '[\\/]')[0]
        if (!$byDir[$topDir]) {
            $byDir[$topDir] = @()
        }
        $byDir[$topDir] += $crate
    }
    
    foreach ($dir in $byDir.Keys | Sort-Object) {
        $dirId = ConvertTo-MermaidId $dir
        $mermaid += "    subgraph $dirId[`"$dir`"]`n"
        
        foreach ($crate in $byDir[$dir] | Sort-Object) {
            $crateId = ConvertTo-MermaidId $crate
            $mermaid += "        $crateId[`"$crate`"]`n"
        }
        
        $mermaid += "    end`n`n"
    }
    
    # Add edges
    $mermaid += "`n    %% Dependencies`n"
    $edgeCount = 0
    foreach ($edge in $data.edges) {
        if ($data.crates[$edge.to]) {
            $fromId = ConvertTo-MermaidId $edge.from
            $toId = ConvertTo-MermaidId $edge.to
            $mermaid += "    $fromId --> $toId`n"
            $edgeCount++
            
            if ($edgeCount -ge $MaxNodes) {
                $mermaid += "    %% ... truncated ($($data.edges.Count - $edgeCount) more edges)`n"
                break
            }
        }
    }
    
    return $mermaid
}
#endregion

#region Module Structure Graph
function Get-ModuleStructure {
    Write-Host 'Analyzing module structure...'
    
    $modules = @{}
    
    # Analyze top-level directories
    $topDirs = Get-ChildItem -Path $script:NoaRoot -Directory -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -notmatch '^(node_modules|target|\.git|cache|tmp|logs)$' }
    
    foreach ($dir in $topDirs) {
        $subDirs = Get-ChildItem -Path $dir.FullName -Directory -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -notmatch '^(node_modules|target|\.git)$' } |
            Select-Object -First 20
        
        $files = Get-ChildItem -Path $dir.FullName -File -Recurse -ErrorAction SilentlyContinue |
            Where-Object { $_.Extension -match '\.(rs|ts|py|ps1|sh)$' }
        $fileCount = if ($files) { @($files).Count } else { 0 }
        
        $subdirNames = if ($subDirs) { @($subDirs | ForEach-Object { $_.Name }) } else { @() }
        
        $modules[$dir.Name] = @{
            subdirs   = $subdirNames
            fileCount = $fileCount
            path      = $dir.FullName
        }
    }
    
    return $modules
}

function New-ModuleStructureGraph {
    $modules = Get-ModuleStructure
    
    $mermaid = New-MermaidHeader -Title 'NOA Module Structure' -Type 'flowchart'
    
    # Root node
    $mermaid += "    noa_root{{NOA}}`n`n"
    
    # Color coding by file count
    foreach ($mod in ($modules.Keys | Sort-Object)) {
        $modId = ConvertTo-MermaidId $mod
        $fileCount = $modules[$mod].fileCount
        
        # Node shape based on importance
        if ($fileCount -gt 100) {
            $shape = "[[$mod<br/>$fileCount files]]"  # Stadium shape for major modules
        }
        elseif ($fileCount -gt 10) {
            $shape = "[$mod<br/>$fileCount files]"    # Rectangle
        }
        else {
            $shape = "($mod)"                          # Rounded
        }
        
        $mermaid += "    $modId$shape`n"
        $mermaid += "    noa_root --> $modId`n"
        
        # Add subdirectories
        foreach ($subdir in ($modules[$mod].subdirs | Select-Object -First 5)) {
            $subId = ConvertTo-MermaidId "$mod-$subdir"
            $mermaid += "    $subId($subdir)`n"
            $mermaid += "    $modId --> $subId`n"
        }
        
        $mermaid += "`n"
    }
    
    # Add styling
    $mermaid += @'

    %% Styling
    classDef major fill:#4CAF50,stroke:#2E7D32,color:white
    classDef medium fill:#2196F3,stroke:#1565C0,color:white
    classDef minor fill:#9E9E9E,stroke:#616161,color:white
    
'@
    
    return $mermaid
}
#endregion

#region Documentation Coverage Graph
function Get-DocCoverage {
    Write-Host 'Analyzing documentation coverage...'
    
    $coverage = @{}
    
    # Count docs per module
    $docDirs = @('docs\wiki', 'docs\runbooks', 'docs\pages', 'docs\reference')
    
    foreach ($docDir in $docDirs) {
        $fullPath = Join-Path $script:NoaRoot $docDir
        if (!(Test-Path $fullPath)) { continue }
        
        $files = Get-ChildItem -Path $fullPath -Filter '*.md' -Recurse -ErrorAction SilentlyContinue
        
        foreach ($file in $files) {
            $relativePath = $file.FullName.Replace($script:NoaRoot, '').TrimStart('\', '/')
            $docType = ($docDir -split '[\\/]')[-1]
            
            if (!$coverage[$docType]) {
                $coverage[$docType] = @{
                    files     = @()
                    totalSize = 0
                    subdirs   = @{}
                }
            }
            
            $coverage[$docType].files += $relativePath
            $coverage[$docType].totalSize += $file.Length
            
            # Track subdirectories
            $subdir = ($relativePath -split '[\\/]')[2]
            if ($subdir -and $subdir -ne $file.Name) {
                if (!$coverage[$docType].subdirs[$subdir]) {
                    $coverage[$docType].subdirs[$subdir] = 0
                }
                $coverage[$docType].subdirs[$subdir]++
            }
        }
    }
    
    return $coverage
}

function New-DocCoverageGraph {
    $coverage = Get-DocCoverage
    
    $mermaid = @'
---
title: Documentation Coverage
---
pie showData
    title Documentation Distribution
'@
    
    foreach ($docType in $coverage.Keys | Sort-Object) {
        $count = $coverage[$docType].files.Count
        $mermaid += "    `"$docType ($count)`" : $count`n"
    }
    
    return $mermaid
}

function New-DocCoverageHeatmap {
    $coverage = Get-DocCoverage
    
    $mermaid = New-MermaidHeader -Title 'Documentation Coverage Heatmap' -Type 'flowchart'
    
    $mermaid += "    docs_root{{Documentation}}`n`n"
    
    foreach ($docType in ($coverage.Keys | Sort-Object)) {
        $typeId = ConvertTo-MermaidId $docType
        $count = $coverage[$docType].files.Count
        $sizeKB = [math]::Round($coverage[$docType].totalSize / 1024, 1)
        
        $mermaid += "    $typeId[`"$docType<br/>$count files<br/>$sizeKB KB`"]`n"
        $mermaid += "    docs_root --> $typeId`n"
        
        # Add subdirectories
        $subdirCount = 0
        foreach ($subdir in ($coverage[$docType].subdirs.Keys | Sort-Object)) {
            $subdirCount++
            if ($subdirCount -gt 8) {
                $mermaid += "    ${typeId}_more[`"... +$($coverage[$docType].subdirs.Count - 8) more`"]`n"
                $mermaid += "    $typeId --> ${typeId}_more`n"
                break
            }
            
            $subId = ConvertTo-MermaidId "$docType-$subdir"
            $fileCount = $coverage[$docType].subdirs[$subdir]
            $mermaid += "    $subId($subdir<br/>$fileCount)`n"
            $mermaid += "    $typeId --> $subId`n"
        }
        
        $mermaid += "`n"
    }
    
    # Styling based on coverage
    $mermaid += @'

    %% Coverage styling
    classDef high fill:#4CAF50,stroke:#2E7D32,color:white
    classDef medium fill:#FFC107,stroke:#FFA000,color:black
    classDef low fill:#F44336,stroke:#C62828,color:white
'@
    
    return $mermaid
}
#endregion

#region Symbol Relationship Graph
function New-SymbolGraph {
    param([object[]]$Symbols)
    
    Write-Host 'Generating symbol relationship graph...'
    
    if (!$Symbols -or $Symbols.Count -eq 0) {
        return "flowchart TD`n    empty[No symbols to display]"
    }
    
    $mermaid = New-MermaidHeader -Title 'Symbol Relationships' -Type 'flowchart'
    
    # Group by file
    $byFile = @{}
    foreach ($sym in $Symbols) {
        $file = $sym.FilePath ?? $sym.File
        if (!$file) { continue }
        
        $shortFile = ($file -split '[\\/]')[-1]
        if (!$byFile[$shortFile]) {
            $byFile[$shortFile] = @()
        }
        $byFile[$shortFile] += $sym
    }
    
    $nodeCount = 0
    foreach ($file in ($byFile.Keys | Sort-Object | Select-Object -First 20)) {
        $fileId = ConvertTo-MermaidId $file
        $mermaid += "    subgraph $fileId[`"$file`"]`n"
        
        foreach ($sym in ($byFile[$file] | Select-Object -First 10)) {
            $symId = ConvertTo-MermaidId "$file-$($sym.Name)"
            $shape = switch ($sym.Type) {
                'function' { "($($sym.Name))" }
                'struct' { "[[$($sym.Name)]]" }
                'class' { "[[$($sym.Name)]]" }
                'trait' { "{$($sym.Name)}" }
                'interface' { "{$($sym.Name)}" }
                'enum' { "{{$($sym.Name)}}" }
                default { "[$($sym.Name)]" }
            }
            
            $mermaid += "        $symId$shape`n"
            $nodeCount++
            
            if ($nodeCount -ge $MaxNodes) {
                $mermaid += "        %% ... truncated`n"
                break
            }
        }
        
        $mermaid += "    end`n`n"
        
        if ($nodeCount -ge $MaxNodes) { break }
    }
    
    # Add legend
    $mermaid += @'

    %% Legend
    subgraph legend[Legend]
        leg_fn(function)
        leg_struct[[struct/class]]
        leg_trait{trait/interface}
        leg_enum{{enum}}
    end
    
    style legend fill:#f5f5f5,stroke:#ccc
'@
    
    return $mermaid
}
#endregion

#region Main Execution
function Start-GraphGeneration {
    Write-Host '=' * 60
    Write-Host 'GRAPH GENERATION'
    Write-Host '=' * 60
    
    # Ensure output directory exists
    if (!(Test-Path $script:OutputDir)) {
        New-Item -ItemType Directory -Path $script:OutputDir -Force | Out-Null
    }
    
    $generated = @()
    
    # Generate requested graphs
    if ($InputType -eq 'all' -or $InputType -eq 'dependencies') {
        Write-Host "`nGenerating Cargo dependency graph..."
        $graph = New-CargoDependencyGraph
        $path = "$script:OutputDir\cargo-dependencies.mmd"
        Set-Content -Path $path -Value $graph -Encoding UTF8
        $generated += $path
        Write-Host "  Saved: $path" -ForegroundColor Green
    }
    
    if ($InputType -eq 'all' -or $InputType -eq 'modules') {
        Write-Host "`nGenerating module structure graph..."
        $graph = New-ModuleStructureGraph
        $path = "$script:OutputDir\module-structure.mmd"
        Set-Content -Path $path -Value $graph -Encoding UTF8
        $generated += $path
        Write-Host "  Saved: $path" -ForegroundColor Green
    }
    
    if ($InputType -eq 'all' -or $InputType -eq 'coverage') {
        Write-Host "`nGenerating documentation coverage graphs..."
        
        $graph = New-DocCoverageGraph
        $path = "$script:OutputDir\doc-coverage-pie.mmd"
        Set-Content -Path $path -Value $graph -Encoding UTF8
        $generated += $path
        Write-Host "  Saved: $path" -ForegroundColor Green
        
        $graph = New-DocCoverageHeatmap
        $path = "$script:OutputDir\doc-coverage-heatmap.mmd"
        Set-Content -Path $path -Value $graph -Encoding UTF8
        $generated += $path
        Write-Host "  Saved: $path" -ForegroundColor Green
    }
    
    if ($InputType -eq 'all' -or $InputType -eq 'symbols') {
        Write-Host "`nGenerating symbol relationship graph..."
        
        # Load symbols
        $symbols = @()
        if ($SymbolsFile -and (Test-Path $SymbolsFile)) {
            $symbols = Get-Content $SymbolsFile -Raw | ConvertFrom-Json
        }
        
        $graph = New-SymbolGraph -Symbols $symbols
        $path = "$script:OutputDir\symbol-relationships.mmd"
        Set-Content -Path $path -Value $graph -Encoding UTF8
        $generated += $path
        Write-Host "  Saved: $path" -ForegroundColor Green
    }
    
    # Summary
    Write-Host "`n" + '=' * 60
    Write-Host "GENERATED $($generated.Count) GRAPHS"
    Write-Host '=' * 60
    
    foreach ($path in $generated) {
        $relativePath = $path.Replace($script:NoaRoot, '').TrimStart('\', '/')
        Write-Host "  - $relativePath"
    }
    
    Write-Host "`nTo view graphs, open in VS Code with Mermaid extension or paste into:"
    Write-Host '  https://mermaid.live/' -ForegroundColor Cyan
    
    return $generated
}

# Execute
Start-GraphGeneration
