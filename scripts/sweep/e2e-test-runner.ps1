<#
.SYNOPSIS
    E2E Test Runner - Unified end-to-end testing with notebook support

.DESCRIPTION
    Runs end-to-end tests across the noa codebase including:
    - Jupyter notebook tests
    - Integration tests
    - Cross-crate tests
    - API endpoint tests

.PARAMETER TestType
    Type of tests to run: all, notebooks, integration, api

.PARAMETER Filter
    Filter pattern for test names

.EXAMPLE
    .\e2e-test-runner.ps1 -TestType all
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [ValidateSet('all', 'notebooks', 'integration', 'api', 'rust', 'typescript', 'python')]
    [string]$TestType = 'all',

    [Parameter(Mandatory = $false)]
    [string]$Filter,

    [Parameter(Mandatory = $false)]
    [switch]$Verbose,

    [Parameter(Mandatory = $false)]
    [switch]$Coverage,

    [Parameter(Mandatory = $false)]
    [string]$OutputPath,

    [Parameter(Mandatory = $false)]
    [int]$Timeout = 300  # 5 minutes default
)

$ErrorActionPreference = "Stop"
$script:NoaRoot = (Resolve-Path "$PSScriptRoot\..\..").Path
$script:TestsDir = "$script:NoaRoot\tests"
$script:OutputDir = $OutputPath ?? "$script:NoaRoot\data\state\sweep\test-results"

#region Logging
function Write-TestLog {
    param(
        [string]$Message,
        [ValidateSet('INFO', 'PASS', 'FAIL', 'SKIP', 'WARN')]
        [string]$Level = 'INFO'
    )
    
    $timestamp = Get-Date -Format 'HH:mm:ss'
    
    switch ($Level) {
        'PASS' { Write-Host "[$timestamp] ✓ $Message" -ForegroundColor Green }
        'FAIL' { Write-Host "[$timestamp] ✗ $Message" -ForegroundColor Red }
        'SKIP' { Write-Host "[$timestamp] ○ $Message" -ForegroundColor Yellow }
        'WARN' { Write-Host "[$timestamp] ! $Message" -ForegroundColor Yellow }
        default { Write-Host "[$timestamp] $Message" }
    }
}
#endregion

#region Notebook Testing
function Get-NotebookFiles {
    $notebooks = @()
    
    # Search for notebooks in common locations
    $searchPaths = @(
        "$script:NoaRoot\docs",
        "$script:NoaRoot\tests\notebooks",
        "$script:NoaRoot\examples"
    )
    
    foreach ($path in $searchPaths) {
        if (Test-Path $path) {
            $found = Get-ChildItem -Path $path -Filter "*.ipynb" -Recurse -ErrorAction SilentlyContinue
            $notebooks += $found
        }
    }
    
    return $notebooks
}

function Test-Notebook {
    param([string]$NotebookPath)
    
    $result = @{
        Path    = $NotebookPath
        Status  = 'unknown'
        Output  = ''
        Errors  = @()
        Runtime = 0
    }
    
    try {
        $startTime = Get-Date
        
        # Check if jupyter/nbconvert is available
        $jupyter = Get-Command jupyter -ErrorAction SilentlyContinue
        if (!$jupyter) {
            $result.Status = 'skipped'
            $result.Output = 'jupyter not found'
            return $result
        }
        
        # Run notebook with nbconvert
        $tempOutput = [System.IO.Path]::GetTempFileName()
        $process = Start-Process -FilePath "jupyter" `
            -ArgumentList "nbconvert", "--to", "notebook", "--execute", "--inplace", $NotebookPath `
            -NoNewWindow -Wait -PassThru -RedirectStandardOutput $tempOutput `
            -ErrorAction SilentlyContinue
        
        $result.Runtime = ((Get-Date) - $startTime).TotalSeconds
        
        if ($process.ExitCode -eq 0) {
            $result.Status = 'passed'
            $result.Output = Get-Content $tempOutput -Raw -ErrorAction SilentlyContinue
        }
        else {
            $result.Status = 'failed'
            $result.Errors += "Exit code: $($process.ExitCode)"
        }
        
        Remove-Item $tempOutput -ErrorAction SilentlyContinue
    }
    catch {
        $result.Status = 'error'
        $result.Errors += $_.Exception.Message
    }
    
    return $result
}

function Invoke-NotebookTests {
    Write-TestLog "Running notebook tests..."
    
    $notebooks = Get-NotebookFiles
    Write-TestLog "Found $($notebooks.Count) notebooks"
    
    if ($notebooks.Count -eq 0) {
        Write-TestLog "No notebooks found to test" -Level SKIP
        return @{
            total  = 0
            passed = 0
            failed = 0
            skipped = 0
            results = @()
        }
    }
    
    $results = @()
    $passed = 0
    $failed = 0
    $skipped = 0
    
    foreach ($notebook in $notebooks) {
        $relativePath = $notebook.FullName.Replace($script:NoaRoot, "").TrimStart("\", "/")
        Write-TestLog "Testing: $relativePath"
        
        $result = Test-Notebook -NotebookPath $notebook.FullName
        $results += $result
        
        switch ($result.Status) {
            'passed' { 
                Write-TestLog "$relativePath ($([math]::Round($result.Runtime, 2))s)" -Level PASS
                $passed++
            }
            'failed' { 
                Write-TestLog "$relativePath - $($result.Errors -join ', ')" -Level FAIL
                $failed++
            }
            'skipped' { 
                Write-TestLog "$relativePath - $($result.Output)" -Level SKIP
                $skipped++
            }
        }
    }
    
    return @{
        total   = $notebooks.Count
        passed  = $passed
        failed  = $failed
        skipped = $skipped
        results = $results
    }
}
#endregion

#region Rust Testing
function Invoke-RustTests {
    Write-TestLog "Running Rust tests..."
    
    $cargoManifests = Get-ChildItem -Path $script:NoaRoot -Filter "Cargo.toml" -Recurse |
        Where-Object { $_.FullName -notlike "*\target\*" }
    
    $results = @()
    $passed = 0
    $failed = 0
    
    foreach ($manifest in $cargoManifests) {
        $dir = $manifest.DirectoryName
        $relativePath = $dir.Replace($script:NoaRoot, "").TrimStart("\", "/")
        
        # Skip non-package directories
        if ($relativePath -eq "" -or $relativePath -match "^(target|node_modules|cache)") {
            continue
        }
        
        Write-TestLog "Testing: $relativePath"
        
        try {
            $startTime = Get-Date
            $output = & cargo test --manifest-path $manifest.FullName --no-fail-fast 2>&1
            $exitCode = $LASTEXITCODE
            $runtime = ((Get-Date) - $startTime).TotalSeconds
            
            $testCount = ($output | Select-String "test result:").Count
            
            if ($exitCode -eq 0) {
                Write-TestLog "$relativePath ($([math]::Round($runtime, 2))s)" -Level PASS
                $passed++
            }
            else {
                Write-TestLog "$relativePath - tests failed" -Level FAIL
                $failed++
            }
            
            $results += @{
                Path    = $relativePath
                Status  = if ($exitCode -eq 0) { 'passed' } else { 'failed' }
                Runtime = $runtime
                Output  = $output -join "`n"
            }
        }
        catch {
            Write-TestLog "$relativePath - error: $_" -Level FAIL
            $failed++
            $results += @{
                Path   = $relativePath
                Status = 'error'
                Error  = $_.Exception.Message
            }
        }
    }
    
    return @{
        total   = $results.Count
        passed  = $passed
        failed  = $failed
        results = $results
    }
}
#endregion

#region TypeScript Testing
function Invoke-TypeScriptTests {
    Write-TestLog "Running TypeScript tests..."
    
    $packageJsons = Get-ChildItem -Path $script:NoaRoot -Filter "package.json" -Recurse |
        Where-Object { 
            $_.FullName -notlike "*\node_modules\*" -and
            $_.FullName -notlike "*\target\*"
        }
    
    $results = @()
    $passed = 0
    $failed = 0
    $skipped = 0
    
    foreach ($pkg in $packageJsons) {
        $dir = $pkg.DirectoryName
        $relativePath = $dir.Replace($script:NoaRoot, "").TrimStart("\", "/")
        
        # Read package.json to check for test script
        $pkgContent = Get-Content $pkg.FullName -Raw | ConvertFrom-Json
        
        if (!$pkgContent.scripts -or !$pkgContent.scripts.test) {
            continue
        }
        
        Write-TestLog "Testing: $relativePath"
        
        try {
            Push-Location $dir
            
            # Check if node_modules exists
            if (!(Test-Path "node_modules")) {
                Write-TestLog "$relativePath - node_modules not found" -Level SKIP
                $skipped++
                Pop-Location
                continue
            }
            
            $startTime = Get-Date
            $output = & npm test 2>&1
            $exitCode = $LASTEXITCODE
            $runtime = ((Get-Date) - $startTime).TotalSeconds
            
            if ($exitCode -eq 0) {
                Write-TestLog "$relativePath ($([math]::Round($runtime, 2))s)" -Level PASS
                $passed++
            }
            else {
                Write-TestLog "$relativePath - tests failed" -Level FAIL
                $failed++
            }
            
            $results += @{
                Path    = $relativePath
                Status  = if ($exitCode -eq 0) { 'passed' } else { 'failed' }
                Runtime = $runtime
                Output  = $output -join "`n"
            }
            
            Pop-Location
        }
        catch {
            Pop-Location
            Write-TestLog "$relativePath - error: $_" -Level FAIL
            $failed++
        }
    }
    
    return @{
        total   = $results.Count
        passed  = $passed
        failed  = $failed
        skipped = $skipped
        results = $results
    }
}
#endregion

#region Python Testing
function Invoke-PythonTests {
    Write-TestLog "Running Python tests..."
    
    $testDirs = @(
        "$script:NoaRoot\tests",
        "$script:NoaRoot\sys\digest\tests"
    )
    
    $results = @()
    $passed = 0
    $failed = 0
    
    foreach ($testDir in $testDirs) {
        if (!(Test-Path $testDir)) { continue }
        
        $pythonTests = Get-ChildItem -Path $testDir -Filter "test_*.py" -Recurse -ErrorAction SilentlyContinue
        
        foreach ($test in $pythonTests) {
            $relativePath = $test.FullName.Replace($script:NoaRoot, "").TrimStart("\", "/")
            Write-TestLog "Testing: $relativePath"
            
            try {
                $startTime = Get-Date
                $output = & python -m pytest $test.FullName -v 2>&1
                $exitCode = $LASTEXITCODE
                $runtime = ((Get-Date) - $startTime).TotalSeconds
                
                if ($exitCode -eq 0) {
                    Write-TestLog "$relativePath ($([math]::Round($runtime, 2))s)" -Level PASS
                    $passed++
                }
                else {
                    Write-TestLog "$relativePath - tests failed" -Level FAIL
                    $failed++
                }
                
                $results += @{
                    Path    = $relativePath
                    Status  = if ($exitCode -eq 0) { 'passed' } else { 'failed' }
                    Runtime = $runtime
                    Output  = $output -join "`n"
                }
            }
            catch {
                Write-TestLog "$relativePath - error: $_" -Level FAIL
                $failed++
            }
        }
    }
    
    return @{
        total   = $results.Count
        passed  = $passed
        failed  = $failed
        results = $results
    }
}
#endregion

#region Integration Tests
function Invoke-IntegrationTests {
    Write-TestLog "Running integration tests..."
    
    $integrationDir = "$script:TestsDir\integration"
    
    if (!(Test-Path $integrationDir)) {
        Write-TestLog "No integration test directory found" -Level SKIP
        return @{
            total   = 0
            passed  = 0
            failed  = 0
            skipped = 1
            results = @()
        }
    }
    
    # Run PowerShell integration tests
    $psTests = Get-ChildItem -Path $integrationDir -Filter "*.ps1" -Recurse
    $results = @()
    $passed = 0
    $failed = 0
    
    foreach ($test in $psTests) {
        $relativePath = $test.FullName.Replace($script:NoaRoot, "").TrimStart("\", "/")
        Write-TestLog "Testing: $relativePath"
        
        try {
            $startTime = Get-Date
            $output = & $test.FullName 2>&1
            $exitCode = $LASTEXITCODE
            $runtime = ((Get-Date) - $startTime).TotalSeconds
            
            if ($exitCode -eq 0) {
                Write-TestLog "$relativePath ($([math]::Round($runtime, 2))s)" -Level PASS
                $passed++
            }
            else {
                Write-TestLog "$relativePath - exit code $exitCode" -Level FAIL
                $failed++
            }
            
            $results += @{
                Path    = $relativePath
                Status  = if ($exitCode -eq 0) { 'passed' } else { 'failed' }
                Runtime = $runtime
            }
        }
        catch {
            Write-TestLog "$relativePath - error: $_" -Level FAIL
            $failed++
        }
    }
    
    return @{
        total   = $results.Count
        passed  = $passed
        failed  = $failed
        results = $results
    }
}
#endregion

#region Report Generation
function New-TestReport {
    param([hashtable]$AllResults)
    
    $timestamp = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
    
    $totalPassed = 0
    $totalFailed = 0
    $totalSkipped = 0
    $totalTests = 0
    
    foreach ($category in $AllResults.Keys) {
        $totalPassed += $AllResults[$category].passed
        $totalFailed += $AllResults[$category].failed
        $totalSkipped += $AllResults[$category].skipped ?? 0
        $totalTests += $AllResults[$category].total
    }
    
    $report = @"
# E2E Test Report

**Generated**: $timestamp
**Total Tests**: $totalTests
**Passed**: $totalPassed
**Failed**: $totalFailed
**Skipped**: $totalSkipped

## Summary

| Category | Total | Passed | Failed | Skipped |
|----------|-------|--------|--------|---------|
"@
    
    foreach ($category in $AllResults.Keys | Sort-Object) {
        $cat = $AllResults[$category]
        $report += "| $category | $($cat.total) | $($cat.passed) | $($cat.failed) | $($cat.skipped ?? 0) |`n"
    }
    
    # Details
    $report += "`n## Details`n"
    
    foreach ($category in $AllResults.Keys | Sort-Object) {
        $report += "`n### $category`n`n"
        
        foreach ($result in $AllResults[$category].results) {
            $status = switch ($result.Status) {
                'passed' { '✓' }
                'failed' { '✗' }
                'skipped' { '○' }
                default { '?' }
            }
            
            $report += "- $status ``$($result.Path)``"
            if ($result.Runtime) {
                $report += " ($([math]::Round($result.Runtime, 2))s)"
            }
            $report += "`n"
        }
    }
    
    return $report
}
#endregion

#region Main Execution
function Start-E2ETests {
    Write-Host "=" * 60
    Write-Host "E2E TEST RUNNER"
    Write-Host "=" * 60
    Write-Host "Test Type: $TestType"
    Write-Host ""
    
    $startTime = Get-Date
    $allResults = @{}
    
    # Ensure output directory
    if (!(Test-Path $script:OutputDir)) {
        New-Item -ItemType Directory -Path $script:OutputDir -Force | Out-Null
    }
    
    # Run tests based on type
    if ($TestType -eq 'all' -or $TestType -eq 'notebooks') {
        Write-Host "`n--- NOTEBOOK TESTS ---"
        $allResults['Notebooks'] = Invoke-NotebookTests
    }
    
    if ($TestType -eq 'all' -or $TestType -eq 'rust') {
        Write-Host "`n--- RUST TESTS ---"
        $allResults['Rust'] = Invoke-RustTests
    }
    
    if ($TestType -eq 'all' -or $TestType -eq 'typescript') {
        Write-Host "`n--- TYPESCRIPT TESTS ---"
        $allResults['TypeScript'] = Invoke-TypeScriptTests
    }
    
    if ($TestType -eq 'all' -or $TestType -eq 'python') {
        Write-Host "`n--- PYTHON TESTS ---"
        $allResults['Python'] = Invoke-PythonTests
    }
    
    if ($TestType -eq 'all' -or $TestType -eq 'integration') {
        Write-Host "`n--- INTEGRATION TESTS ---"
        $allResults['Integration'] = Invoke-IntegrationTests
    }
    
    $totalRuntime = ((Get-Date) - $startTime).TotalSeconds
    
    # Generate report
    $report = New-TestReport -AllResults $allResults
    $reportPath = "$script:OutputDir\e2e-test-report.md"
    Set-Content -Path $reportPath -Value $report -Encoding UTF8
    
    # Save JSON results
    $jsonPath = "$script:OutputDir\e2e-test-results.json"
    $allResults | ConvertTo-Json -Depth 10 | Set-Content -Path $jsonPath -Encoding UTF8
    
    # Summary
    Write-Host "`n" + "=" * 60
    Write-Host "E2E TEST SUMMARY"
    Write-Host "=" * 60
    
    $totalPassed = 0
    $totalFailed = 0
    
    foreach ($category in $allResults.Keys) {
        $cat = $allResults[$category]
        Write-Host "  $category`: $($cat.passed)/$($cat.total) passed"
        $totalPassed += $cat.passed
        $totalFailed += $cat.failed
    }
    
    Write-Host ""
    Write-Host "Total: $totalPassed passed, $totalFailed failed"
    Write-Host "Runtime: $([math]::Round($totalRuntime, 2))s"
    Write-Host ""
    Write-Host "Report: $reportPath" -ForegroundColor Cyan
    
    # Exit code based on failures
    if ($totalFailed -gt 0) {
        exit 1
    }
    
    return $allResults
}

# Execute
Start-E2ETests
