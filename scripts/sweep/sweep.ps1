<#
.SYNOPSIS
    NOA Sweep - Main entry point for codebase sweep operations

.DESCRIPTION
    Orchestrates the complete 10-sweep loop through the noa codebase:
    1. Symbol extraction from all source files
    2. Documentation cross-reference checking
    3. Ollama embedding generation
    4. SQLite storage of results
    5. Graph generation for visualization
    6. E2E test validation

.PARAMETER Sweep
    Which sweep to run (1-10) or 'all' for complete loop

.PARAMETER Operations
    Specific operations to run: extract, docs, embed, graph, test, all

.EXAMPLE
    .\sweep.ps1 -Sweep 1 -Operations all
    .\sweep.ps1 -Sweep 1 -Operations extract,docs
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [ValidateRange(1, 10)]
    [int]$Sweep = 1,

    [Parameter(Mandatory = $false)]
    [string[]]$Operations = @('all'),

    [Parameter(Mandatory = $false)]
    [int]$MaxParallel = 8,

    [Parameter(Mandatory = $false)]
    [string]$OllamaModel = 'nomic-embed-text',

    [Parameter(Mandatory = $false)]
    [switch]$DryRun,

    [Parameter(Mandatory = $false)]
    [switch]$SkipTests,

    [Parameter(Mandatory = $false)]
    [switch]$GenerateReport
)

$ErrorActionPreference = "Stop"
$script:NoaRoot = (Resolve-Path "$PSScriptRoot\..\..").Path
$script:SweepDir = $PSScriptRoot
$script:DataDir = "$script:NoaRoot\data\state\sweep"
$script:LogDir = "$script:NoaRoot\logs\sweep"

#region Initialization
function Initialize-SweepEnvironment {
    Write-Host "=" * 70 -ForegroundColor Cyan
    Write-Host "  NOA CODEBASE SWEEP - LOOP $Sweep OF 10" -ForegroundColor Cyan
    Write-Host "=" * 70 -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Operations: $($Operations -join ', ')"
    Write-Host "  Max Parallel: $MaxParallel"
    Write-Host "  Ollama Model: $OllamaModel"
    Write-Host "  Dry Run: $DryRun"
    Write-Host ""
    
    # Create directories
    @($script:DataDir, $script:LogDir) | ForEach-Object {
        if (!(Test-Path $_)) {
            New-Item -ItemType Directory -Path $_ -Force | Out-Null
        }
    }
    
    # Start logging
    $script:LogFile = "$script:LogDir\sweep-$Sweep-$(Get-Date -Format 'yyyyMMdd-HHmmss').log"
    Start-Transcript -Path $script:LogFile -Force
    
    Write-Host "Logging to: $script:LogFile" -ForegroundColor Gray
    Write-Host ""
}

function Complete-SweepEnvironment {
    Stop-Transcript -ErrorAction SilentlyContinue
}
#endregion

#region Operation Runners
function Invoke-SymbolExtraction {
    Write-Host "`n" + "-" * 50
    Write-Host "PHASE 1: SYMBOL EXTRACTION" -ForegroundColor Yellow
    Write-Host "-" * 50
    
    $orchestrator = "$script:SweepDir\sweep-orchestrator.ps1"
    
    if (Test-Path $orchestrator) {
        & $orchestrator -SweepNumber $Sweep -MaxParallel $MaxParallel -OllamaModel $OllamaModel -DryRun:$DryRun
    }
    else {
        Write-Error "sweep-orchestrator.ps1 not found"
    }
}

function Invoke-DocCrossReference {
    Write-Host "`n" + "-" * 50
    Write-Host "PHASE 2: DOCUMENTATION CROSS-REFERENCE" -ForegroundColor Yellow
    Write-Host "-" * 50
    
    $xrefChecker = "$script:SweepDir\doc-xref-checker.ps1"
    
    if (Test-Path $xrefChecker) {
        & $xrefChecker -GenerateReport:$GenerateReport
    }
    else {
        Write-Error "doc-xref-checker.ps1 not found"
    }
}

function Invoke-EmbeddingGeneration {
    Write-Host "`n" + "-" * 50
    Write-Host "PHASE 3: EMBEDDING GENERATION" -ForegroundColor Yellow
    Write-Host "-" * 50
    
    $embeddings = "$script:SweepDir\ollama-embeddings.ps1"
    $symbolsFile = "$script:DataDir\symbols.json"
    
    if (!(Test-Path $symbolsFile)) {
        Write-Host "No symbols file found - skipping embeddings" -ForegroundColor Yellow
        return
    }
    
    if (Test-Path $embeddings) {
        & $embeddings -InputFile $symbolsFile -Model $OllamaModel -OutputFile "$script:DataDir\symbols-with-embeddings.json"
    }
    else {
        Write-Error "ollama-embeddings.ps1 not found"
    }
}

function Invoke-GraphGeneration {
    Write-Host "`n" + "-" * 50
    Write-Host "PHASE 4: GRAPH GENERATION" -ForegroundColor Yellow
    Write-Host "-" * 50
    
    $graphGen = "$script:SweepDir\graph-generator.ps1"
    
    if (Test-Path $graphGen) {
        & $graphGen -InputType all
    }
    else {
        Write-Error "graph-generator.ps1 not found"
    }
}

function Invoke-E2ETesting {
    Write-Host "`n" + "-" * 50
    Write-Host "PHASE 5: E2E TESTING" -ForegroundColor Yellow
    Write-Host "-" * 50
    
    if ($SkipTests) {
        Write-Host "Tests skipped by user request" -ForegroundColor Yellow
        return
    }
    
    $testRunner = "$script:SweepDir\e2e-test-runner.ps1"
    
    if (Test-Path $testRunner) {
        & $testRunner -TestType all
    }
    else {
        Write-Error "e2e-test-runner.ps1 not found"
    }
}
#endregion

#region Summary Report
function New-SweepSummary {
    $timestamp = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
    
    $summary = @"
# NOA Sweep $Sweep Summary

**Completed**: $timestamp

## Operations Performed

| Phase | Operation | Status |
|-------|-----------|--------|
"@
    
    $phases = @(
        @{ name = 'Symbol Extraction'; op = 'extract' },
        @{ name = 'Doc Cross-Reference'; op = 'docs' },
        @{ name = 'Embedding Generation'; op = 'embed' },
        @{ name = 'Graph Generation'; op = 'graph' },
        @{ name = 'E2E Testing'; op = 'test' }
    )
    
    foreach ($phase in $phases) {
        $status = if ($Operations -contains 'all' -or $Operations -contains $phase.op) { '✓ Completed' } else { '○ Skipped' }
        $summary += "| $($phase.name) | $($phase.op) | $status |`n"
    }
    
    $summary += @"

## Output Files

| File | Description |
|------|-------------|
| ``data/state/sweep/sweep.db`` | SQLite database with all sweep data |
| ``data/state/sweep/symbols.json`` | Extracted symbols |
| ``data/state/sweep/doc-gap-report.md`` | Documentation gaps |
| ``docs/architecture/graphs/*.mmd`` | Mermaid diagrams |
| ``data/state/sweep/test-results/`` | E2E test results |

## Next Steps

1. Review documentation gaps in ``doc-gap-report.md``
2. View generated graphs at ``docs/architecture/graphs/``
3. Address any failing tests
4. Run sweep $($Sweep + 1) to continue improvement

## Metrics

"@
    
    # Add metrics if available
    $metricsFile = "$script:DataDir\sweep-state.json"
    if (Test-Path $metricsFile) {
        $metrics = Get-Content $metricsFile -Raw | ConvertFrom-Json
        $summary += "- Files Processed: $($metrics.filesProcessed ?? 'N/A')`n"
        $summary += "- Symbols Found: $($metrics.symbolsFound ?? 'N/A')`n"
        $summary += "- Errors: $($metrics.errors ?? 'N/A')`n"
    }
    
    # Save summary
    $summaryPath = "$script:DataDir\sweep-$Sweep-summary.md"
    Set-Content -Path $summaryPath -Value $summary -Encoding UTF8
    Write-Host "`nSummary saved to: $summaryPath" -ForegroundColor Green
    
    return $summaryPath
}
#endregion

#region Main Execution
function Start-NoaSweep {
    $startTime = Get-Date
    
    try {
        Initialize-SweepEnvironment
        
        $runAll = $Operations -contains 'all'
        
        if ($runAll -or $Operations -contains 'extract') {
            Invoke-SymbolExtraction
        }
        
        if ($runAll -or $Operations -contains 'docs') {
            Invoke-DocCrossReference
        }
        
        if ($runAll -or $Operations -contains 'embed') {
            Invoke-EmbeddingGeneration
        }
        
        if ($runAll -or $Operations -contains 'graph') {
            Invoke-GraphGeneration
        }
        
        if ($runAll -or $Operations -contains 'test') {
            Invoke-E2ETesting
        }
        
        # Generate summary
        $summaryPath = New-SweepSummary
        
        $totalTime = ((Get-Date) - $startTime).TotalMinutes
        
        Write-Host "`n" + "=" * 70 -ForegroundColor Green
        Write-Host "  SWEEP $Sweep COMPLETE" -ForegroundColor Green
        Write-Host "=" * 70 -ForegroundColor Green
        Write-Host ""
        Write-Host "  Total Time: $([math]::Round($totalTime, 2)) minutes"
        Write-Host "  Log: $script:LogFile"
        Write-Host "  Summary: $summaryPath"
        Write-Host ""
        
        if ($Sweep -lt 10) {
            Write-Host "  Next: Run sweep $($Sweep + 1) to continue improvement" -ForegroundColor Cyan
        }
        else {
            Write-Host "  All 10 sweeps complete! Review final reports." -ForegroundColor Green
        }
        Write-Host ""
    }
    finally {
        Complete-SweepEnvironment
    }
}

# Execute
Start-NoaSweep
