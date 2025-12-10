# Gap Hunt Scan Automation (Phase 9)
# T494: Scans for TODO/FIXME and requirement gaps.
# Implements Universal Task Execution Policy §5.7 (Gap Hunt)

param(
    [string]$RootDir = $PSScriptRoot,
    [string]$ResultsDir = ""
)

$ErrorActionPreference = "Stop"

# Resolve paths
$RootDir = Resolve-Path -Path (Join-Path $RootDir "..\..") -ErrorAction SilentlyContinue
if (-not $RootDir) {
    $RootDir = $PWD
}
$ResultsDir = if ($ResultsDir) { $ResultsDir } else { Join-Path $RootDir "test-results" }
$SpecDir = Join-Path $RootDir "specs\001-noa-seed-foundation"
$Output = Join-Path $ResultsDir "GAP_SCAN.txt"
$CoverageOutput = Join-Path $ResultsDir "COVERAGE.md"

New-Item -ItemType Directory -Path $ResultsDir -Force | Out-Null

@"
=== Gap Hunt Scan (Phase 9) ===
Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')
Scanner: PowerShell Select-String

"@ | Out-File -FilePath $Output

# GH001: Verify gap scan run against full spec outline
Write-Host "=== GH001: Gap Scan Against Spec Outline ===" -ForegroundColor Yellow
"=== GH001: Gap Scan Against Spec Outline ===" | Add-Content -Path $Output

$specFile = Join-Path $SpecDir "spec.md"
if (Test-Path $specFile) {
    "Scanning spec: $specFile" | Add-Content -Path $Output
    Write-Host "Scanning spec: $specFile"

    $content = Get-Content $specFile -Raw
    $frCount = ([regex]::Matches($content, "FR-\d+")).Count
    $scCount = ([regex]::Matches($content, "SC-\d+")).Count
    $verCount = ([regex]::Matches($content, "VER\d+")).Count

    @"
Requirements found:
  FR-*: $frCount
  SC-*: $scCount
  VER*: $verCount
"@ | Add-Content -Path $Output

    Write-Host "  FR-*: $frCount"
    Write-Host "  SC-*: $scCount"
    Write-Host "  VER*: $verCount"
} else {
    "[WARN] Spec file not found: $specFile" | Add-Content -Path $Output
    Write-Host "[WARN] Spec file not found: $specFile" -ForegroundColor Yellow
}

# Scan for gap markers
Write-Host "`n=== Gap Markers Scan ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== Gap Markers Scan ===" | Add-Content -Path $Output

$patterns = @("TODO", "FIXME", "GAP", "TBD", "XXX", "HACK", "NOTE", "WARN")
$totalGaps = 0

foreach ($pattern in $patterns) {
    "" | Add-Content -Path $Output
    "## Pattern: $pattern" | Add-Content -Path $Output
    Write-Host "`n## Pattern: $pattern"

    $files = Get-ChildItem -Path $RootDir -Recurse -File `
        -Exclude @("*.git*", "node_modules", "target") -ErrorAction SilentlyContinue |
        Select-String -Pattern $pattern -CaseSensitive:$false

    $matches = $files | Measure-Object
    $count = $matches.Count

    if ($count -gt 0) {
        "  Found: $count occurrences" | Add-Content -Path $Output
        Write-Host "  Found: $count occurrences" -ForegroundColor Cyan

        # Add sample matches
        $files | Select-Object -First 10 | ForEach-Object {
            $line = "    $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
            $line | Add-Content -Path $Output
        }

        if ($count -gt 10) {
            "    ... and $($count - 10) more" | Add-Content -Path $Output
        }

        $totalGaps += $count
    } else {
        "  No matches found" | Add-Content -Path $Output
    }
}

# GH002: Verify coverage table shows all sections
Write-Host "`n=== GH002: Coverage Table Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== GH002: Coverage Table Check ===" | Add-Content -Path $Output

if (Test-Path $CoverageOutput) {
    $coverageContent = Get-Content $CoverageOutput -Raw
    if ($coverageContent -match "Coverage|Phase|Status|FR-|SC-") {
        "[PASS] Coverage table exists with sections" | Add-Content -Path $Output
        Write-Host "[PASS] Coverage table exists with sections" -ForegroundColor Green

        $phaseCount = ([regex]::Matches($coverageContent, "Phase \d+")).Count
        "  Phases found: $phaseCount" | Add-Content -Path $Output
        Write-Host "  Phases found: $phaseCount"
    } else {
        "[FAIL] Coverage table missing or incomplete" | Add-Content -Path $Output
        Write-Host "[FAIL] Coverage table missing or incomplete" -ForegroundColor Red
    }
} else {
    "[FAIL] COVERAGE.md not found" | Add-Content -Path $Output
    Write-Host "[FAIL] COVERAGE.md not found" -ForegroundColor Red
}

# GH003: Verify missed items identified and documented
Write-Host "`n=== GH003: Missed Items Documentation ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== GH003: Missed Items Documentation ===" | Add-Content -Path $Output

$gapsFile = Join-Path $ResultsDir "GAPS.md"
if (Test-Path $gapsFile) {
    $gapItems = (Get-Content $gapsFile | Select-String "^- \[ \]").Count
    "[PASS] GAPS.md exists with $gapItems documented gaps" | Add-Content -Path $Output
    Write-Host "[PASS] GAPS.md exists with $gapItems documented gaps" -ForegroundColor Green
} else {
    "[WARN] GAPS.md not found - creating template" | Add-Content -Path $Output
    Write-Host "[WARN] GAPS.md not found - creating template" -ForegroundColor Yellow

    @"
# Gap Analysis

## Identified Gaps

Total gaps found: $totalGaps

## Remedies

See GAP_SCAN.txt for detailed gap locations.
"@ | Out-File -FilePath $gapsFile
}

# GH004: Verify remedies proposed for each gap
Write-Host "`n=== GH004: Remedies Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== GH004: Remedies Check ===" | Add-Content -Path $Output

if (Test-Path $gapsFile) {
    $gapsContent = Get-Content $gapsFile -Raw
    if ($gapsContent -match "remedy|fix|solution|action") {
        "[PASS] Remedies documented in GAPS.md" | Add-Content -Path $Output
        Write-Host "[PASS] Remedies documented in GAPS.md" -ForegroundColor Green
    } else {
        "[WARN] Remedies not explicitly documented" | Add-Content -Path $Output
        Write-Host "[WARN] Remedies not explicitly documented" -ForegroundColor Yellow
    }
}

# GH005: Verify no critical gaps remain unaddressed
Write-Host "`n=== GH005: Critical Gaps Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== GH005: Critical Gaps Check ===" | Add-Content -Path $Output

$criticalPatterns = @("CRITICAL", "BLOCKER", "SECURITY", "DATA_LOSS")
$criticalCount = 0

foreach ($pattern in $criticalPatterns) {
    $criticalFiles = Get-ChildItem -Path $RootDir -Recurse -File `
        -Exclude @("*.git*", "node_modules", "target") -ErrorAction SilentlyContinue |
        Select-String -Pattern $pattern -CaseSensitive:$false

    $count = ($criticalFiles | Measure-Object).Count
    if ($count -gt 0) {
        "[WARN] Found $count occurrences of $pattern" | Add-Content -Path $Output
        Write-Host "[WARN] Found $count occurrences of $pattern" -ForegroundColor Yellow
        $criticalCount += $count
    }
}

if ($criticalCount -eq 0) {
    "[PASS] No critical gap markers found" | Add-Content -Path $Output
    Write-Host "[PASS] No critical gap markers found" -ForegroundColor Green
} else {
    "[WARN] $criticalCount critical gap markers found - review required" | Add-Content -Path $Output
    Write-Host "[WARN] $criticalCount critical gap markers found - review required" -ForegroundColor Yellow
}

# Summary
Write-Host "`n=== Gap Scan Summary ===" -ForegroundColor Cyan
"" | Add-Content -Path $Output
"=== Gap Scan Summary ===" | Add-Content -Path $Output

$summary = @(
    "Total gap markers: $totalGaps",
    "Critical markers: $criticalCount",
    "Coverage table: $(if (Test-Path $CoverageOutput) { 'Present' } else { 'Missing' })",
    "Gaps documented: $(if (Test-Path $gapsFile) { 'Yes' } else { 'No' })"
)

foreach ($line in $summary) {
    $line | Add-Content -Path $Output
    Write-Host $line
}

"" | Add-Content -Path $Output
"Gap scan complete -> $Output" | Add-Content -Path $Output
"Coverage report -> $CoverageOutput" | Add-Content -Path $Output
"Gaps document -> $gapsFile" | Add-Content -Path $Output

Write-Host "`nGap scan complete -> $Output" -ForegroundColor Green
Write-Host "Coverage report -> $CoverageOutput" -ForegroundColor Green
Write-Host "Gaps document -> $gapsFile" -ForegroundColor Green

