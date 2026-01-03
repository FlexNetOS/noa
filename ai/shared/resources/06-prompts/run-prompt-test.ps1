# Prompt Quality Test Runner (PowerShell)
# Usage: .\run-prompt-test.ps1 [prompt-file.md] [-Verbose] [-Threshold 70]

param(
    [Parameter(Position=0)]
    [string]$PromptFile = "project-mgmt-prompt.md",

    [switch]$ShowDetails,

    [int]$Threshold = 70
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PromptPath = Join-Path $ScriptDir $PromptFile

if (-not (Test-Path $PromptPath)) {
    Write-Error "File not found: $PromptPath"
    exit 1
}

Write-Host "`n" -NoNewline
Write-Host ("=" * 70) -ForegroundColor Cyan
Write-Host "PROMPT QUALITY TEST" -ForegroundColor White
Write-Host ("=" * 70) -ForegroundColor Cyan
Write-Host "Testing: $PromptFile"
Write-Host "Threshold: $Threshold%"
Write-Host ("-" * 70) -ForegroundColor DarkGray

$content = Get-Content $PromptPath -Raw
$words = ($content -split '\s+').Count
$lines = ($content -split '\n').Count
$sections = ([regex]::Matches($content, '^#{1,3}\s', 'Multiline')).Count
$codeBlocks = ([regex]::Matches($content, '```[\s\S]*?```')).Count
$tables = ([regex]::Matches($content, '\|[^|]+\|[^|]+\|')).Count

# Test Results
$tests = @()

# CLR-001: Readability
$sentences = ($content -split '[.!?]+' | Where-Object { $_.Trim() }).Count
$avgSentenceLen = $words / [Math]::Max(1, $sentences)
$readabilityScore = [Math]::Min(100, [Math]::Max(0, 100 - ($avgSentenceLen - 15) * 3))
$tests += @{
    ID = "CLR-001"
    Name = "Readability"
    Score = [Math]::Round($readabilityScore)
    Details = "Avg sentence: $([Math]::Round($avgSentenceLen, 1)) words"
    Passed = $readabilityScore -ge 60
}

# CLR-006: Structured Formatting
$hasHeadings = $content -match '^#{1,4}\s'
$hasTables = $content -match '\|[^|]+\|[^|]+\|'
$hasCode = $content -match '```'
$hasLists = $content -match '^[\s]*[-*]\s'
$structScore = ($hasHeadings -as [int]) * 25 + ($hasTables -as [int]) * 25 + ($hasCode -as [int]) * 25 + ($hasLists -as [int]) * 25
$tests += @{
    ID = "CLR-006"
    Name = "Formatting"
    Score = $structScore
    Details = "H:$(if($hasHeadings){'Y'}else{'N'}) T:$(if($hasTables){'Y'}else{'N'}) C:$(if($hasCode){'Y'}else{'N'}) L:$(if($hasLists){'Y'}else{'N'})"
    Passed = $structScore -ge 75
}

# CTX-001: Mission Statement
$first200 = ($content -split '\s+' | Select-Object -First 200) -join ' '
$missionWords = @('mission', 'objective', 'purpose', 'goal', 'consolidate', 'unified')
$missionFound = ($missionWords | Where-Object { $first200 -match $_ }).Count -ge 2
$tests += @{
    ID = "CTX-001"
    Name = "Mission"
    Score = if($missionFound){100}else{0}
    Details = if($missionFound){"Found in opening"}else{"MISSING"}
    Passed = $missionFound
}

# CTX-006: Success Criteria
$metricsCount = ([regex]::Matches($content, '[0-9]+%|≥|≤|target')).Count
$tests += @{
    ID = "CTX-006"
    Name = "Metrics"
    Score = [Math]::Min(100, ($metricsCount / 5) * 100)
    Details = "$metricsCount metrics found"
    Passed = $metricsCount -ge 5
}

# CTX-008: Version Control
$hasVersion = $content -match 'version[:\s]+[0-9]+\.[0-9]|v[0-9]+\.[0-9]+|Prompt Version'
$hasChangelog = $content -match 'changelog|### v[0-9]|## v[0-9]'
$versionScore = ($hasVersion -as [int]) * 50 + ($hasChangelog -as [int]) * 50
$tests += @{
    ID = "CTX-008"
    Name = "Versioning"
    Score = $versionScore
    Details = "Ver:$(if($hasVersion){'Y'}else{'N'}) Log:$(if($hasChangelog){'Y'}else{'N'})"
    Passed = $versionScore -ge 80
}

# FLW-007: Hierarchy
$maxDepth = 0
$content -split '\n' | ForEach-Object {
    if ($_ -match '^(#+)\s') {
        $depth = $Matches[1].Length
        if ($depth -gt $maxDepth) { $maxDepth = $depth }
    }
}
$tests += @{
    ID = "FLW-007"
    Name = "Hierarchy"
    Score = if($maxDepth -le 4){100}else{[Math]::Round((4/$maxDepth)*100)}
    Details = "Max depth: $maxDepth"
    Passed = $maxDepth -le 4
}

# FLW-008: Pipeline Coverage
$concepts = @('goal', 'policy', 'rules', 'plan', 'spec', 'task', 'execute', 'memory', 'orchestrat', 'test')
$foundConcepts = ($concepts | Where-Object { $content -match $_ }).Count
$pipelineScore = [Math]::Round(($foundConcepts / $concepts.Count) * 100)
$tests += @{
    ID = "FLW-008"
    Name = "Pipeline"
    Score = $pipelineScore
    Details = "$foundConcepts/$($concepts.Count) concepts"
    Passed = $foundConcepts -ge 7
}

# RES-001: Code Examples
$tests += @{
    ID = "RES-001"
    Name = "Examples"
    Score = [Math]::Min(100, ($codeBlocks / 3) * 100)
    Details = "$codeBlocks code blocks"
    Passed = $codeBlocks -ge 3
}

# RES-006: Metrics Dashboard
$allTableRows = [regex]::Matches($content, '\|[^|]+\|[^|]+\|')
$metricRows = ($allTableRows | Where-Object { $_.Value -match '[0-9]+%|target|score' }).Count
$tests += @{
    ID = "RES-006"
    Name = "Dashboard"
    Score = [Math]::Min(100, ($metricRows / 8) * 100)
    Details = "$metricRows metric rows"
    Passed = $metricRows -ge 8
}

# Calculate scores
$totalScore = 0
$passedCount = 0

Write-Host "`nTEST RESULTS:" -ForegroundColor Yellow
Write-Host ("-" * 70) -ForegroundColor DarkGray

foreach ($test in $tests) {
    $totalScore += $test.Score
    if ($test.Passed) { $passedCount++ }

    $statusColor = if($test.Passed){"Green"}else{"Red"}
    $statusIcon = if($test.Passed){"[PASS]"}else{"[FAIL]"}

    if ($ShowDetails) {
        Write-Host "  " -NoNewline
        Write-Host $statusIcon -ForegroundColor $statusColor -NoNewline
        Write-Host " [$($test.ID)] $($test.Name): " -NoNewline
        Write-Host "$($test.Score)/100" -ForegroundColor $statusColor -NoNewline
        Write-Host " - $($test.Details)" -ForegroundColor DarkGray
    }
}

$overallScore = [Math]::Round($totalScore / $tests.Count)

Write-Host "`n" -NoNewline
Write-Host ("=" * 70) -ForegroundColor Cyan
Write-Host "SUMMARY" -ForegroundColor White
Write-Host ("=" * 70) -ForegroundColor Cyan

Write-Host "Tests Passed: " -NoNewline
Write-Host "$passedCount/$($tests.Count)" -ForegroundColor $(if($passedCount -eq $tests.Count){"Green"}else{"Yellow"})

Write-Host "Overall Score: " -NoNewline
$scoreColor = if($overallScore -ge $Threshold){"Green"}else{"Red"}
Write-Host "$overallScore%" -ForegroundColor $scoreColor -NoNewline
Write-Host " (threshold: $Threshold%)"

$passed = $overallScore -ge $Threshold
Write-Host "`nStatus: " -NoNewline
if ($passed) {
    Write-Host "PASSED" -ForegroundColor Green
} else {
    Write-Host "FAILED" -ForegroundColor Red
}

Write-Host ("-" * 70) -ForegroundColor DarkGray
Write-Host "Stats: $words words | $lines lines | $sections sections | $codeBlocks code blocks | $tables tables"
Write-Host ("=" * 70) -ForegroundColor Cyan
Write-Host ""

# Failed tests recommendations
$failedTests = $tests | Where-Object { -not $_.Passed }
if ($failedTests.Count -gt 0) {
    Write-Host "RECOMMENDATIONS:" -ForegroundColor Yellow
    foreach ($test in $failedTests) {
        Write-Host "  • [$($test.ID)] $($test.Name): $($test.Details)" -ForegroundColor DarkYellow
    }
    Write-Host ""
}

if (-not $passed) {
    exit 1
}
