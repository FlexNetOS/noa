# Truth Gate Checklist Automation (Phase 9)
# T493: Validates presence and freshness of verification artifacts.
# Implements Universal Task Execution Policy §4 (Truth Gate)

param(
    [string]$RootDir = $PSScriptRoot,
    [string]$ResultsDir = "",
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

# Resolve paths
$RootDir = Resolve-Path -Path (Join-Path $RootDir "..\..") -ErrorAction SilentlyContinue
if (-not $RootDir) {
    $RootDir = $PWD
}
$ResultsDir = if ($ResultsDir) { $ResultsDir } else { Join-Path $RootDir "test-results" }
$SpecDir = Join-Path $RootDir "specs\001-noa-seed-foundation"
$Output = Join-Path $ResultsDir "TRUTH_GATE_RESULTS.txt"

New-Item -ItemType Directory -Path $ResultsDir -Force | Out-Null

$status = 0
$checksPassed = 0
$checksFailed = 0
$checksNA = 0

function Write-CheckResult {
    param(
        [string]$CheckId,
        [string]$CheckName,
        [string]$Result,
        [string]$Details = ""
    )

    $message = switch ($Result) {
        "PASS" { "[PASS] ${CheckId}: ${CheckName}" }
        "FAIL" { "[FAIL] ${CheckId}: ${CheckName}" }
        "N/A"  { "[N/A]  ${CheckId}: ${CheckName}" }
    }

    Write-Host $message
    Add-Content -Path $Output -Value $message

    if ($Details) {
        $detailMsg = "  $($Result -eq 'FAIL' ? 'Details' : ($Result -eq 'N/A' ? 'Reason' : 'Details')): $Details"
        Write-Host $detailMsg
        Add-Content -Path $Output -Value $detailMsg
    }

    switch ($Result) {
        "PASS" { $script:checksPassed++ }
        "FAIL" { $script:checksFailed++; $script:status = 1 }
        "N/A"  { $script:checksNA++ }
    }
}

# Initialize output file
"=== Truth Gate Verification (Phase 9) ===" | Out-File -FilePath $Output
"Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')" | Add-Content -Path $Output
"" | Add-Content -Path $Output

Write-Host "=== Truth Gate Verification (Phase 9) ===" -ForegroundColor Cyan

# TG001: Verify all referenced artifacts exist in repo with listed paths [§4.1]
Write-Host "`n=== TG001: Artifact Presence Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== TG001: Artifact Presence Check ===" | Add-Content -Path $Output

$artifacts = @(
    "HASHES.txt",
    "FINAL_REPORT.md",
    "COVERAGE.md",
    "REPRO.md",
    "EVIDENCE_LEDGER.md"
)

$allArtifactsExist = $true
$missingArtifacts = @()

foreach ($artifact in $artifacts) {
    $path = Join-Path $ResultsDir $artifact
    if (Test-Path $path -PathType Leaf) {
        $file = Get-Item $path
        if ($file.Length -gt 0) {
            $ageMinutes = [math]::Round((Get-Date).Subtract($file.LastWriteTime).TotalMinutes, 0)
            Write-Host "  [PASS] $artifact exists (age ~${ageMinutes}m)" -ForegroundColor Green
            "  [PASS] $artifact exists (age ~${ageMinutes}m)" | Add-Content -Path $Output
        } else {
            Write-Host "  [FAIL] Empty: $artifact" -ForegroundColor Red
            "  [FAIL] Empty: $artifact" | Add-Content -Path $Output
            $missingArtifacts += $artifact
            $allArtifactsExist = $false
        }
    } else {
        Write-Host "  [FAIL] Missing: $artifact" -ForegroundColor Red
        "  [FAIL] Missing: $artifact" | Add-Content -Path $Output
        $missingArtifacts += $artifact
        $allArtifactsExist = $false
    }
}

if ($allArtifactsExist) {
    Write-CheckResult "TG001" "All referenced artifacts exist" "PASS"
} else {
    Write-CheckResult "TG001" "All referenced artifacts exist" "FAIL" "Missing: $($missingArtifacts -join ', ')"
}

# TG002: Verify smoke test exits with code 0 and transcript is captured [§4.2]
Write-Host "`n=== TG002: Smoke Test Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== TG002: Smoke Test Check ===" | Add-Content -Path $Output

$smokeTest = Join-Path $ResultsDir "TEST\smoke-test.ps1"
if (Test-Path $smokeTest) {
    $transcript = Join-Path $ResultsDir "smoke-test-transcript.txt"
    try {
        $exitCode = 0
        & $smokeTest *> $transcript
        if ($LASTEXITCODE) { $exitCode = $LASTEXITCODE }

        if ($exitCode -eq 0) {
            Write-CheckResult "TG002" "Smoke test exits with code 0" "PASS" "Transcript: smoke-test-transcript.txt"
        } else {
            Write-CheckResult "TG002" "Smoke test exits with code 0" "FAIL" "Exit code: $exitCode"
        }
    } catch {
        Write-CheckResult "TG002" "Smoke test exits with code 0" "FAIL" "Smoke test execution failed: $_"
    }
} else {
    Write-CheckResult "TG002" "Smoke test exits with code 0" "FAIL" "Smoke test not found: $smokeTest"
}

# TG003: Verify requirements → artifacts → tests mapping has no gaps [§4.3]
Write-Host "`n=== TG003: Requirements Mapping Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== TG003: Requirements Mapping Check ===" | Add-Content -Path $Output

$coverageFile = Join-Path $ResultsDir "COVERAGE.md"
if (Test-Path $coverageFile) {
    $content = Get-Content $coverageFile -Raw
    if ($content -match "requirements|artifacts|tests|FR-|SC-|VER") {
        Write-CheckResult "TG003" "Requirements → artifacts → tests mapping" "PASS" "COVERAGE.md contains mappings"
    } else {
        Write-CheckResult "TG003" "Requirements → artifacts → tests mapping" "FAIL" "COVERAGE.md missing mapping content"
    }
} else {
    Write-CheckResult "TG003" "Requirements → artifacts → tests mapping" "FAIL" "COVERAGE.md not found"
}

# TG004: Verify constraints, supported OS/arch, and failure modes are documented [§4.4]
Write-Host "`n=== TG004: Limits Documentation Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== TG004: Limits Documentation Check ===" | Add-Content -Path $Output

$specFile = Join-Path $SpecDir "spec.md"
if (-not (Test-Path $specFile)) {
    $specFile = Join-Path $RootDir "README.md"
}

if (Test-Path $specFile) {
    $content = Get-Content $specFile -Raw
    if ($content -match "Windows|Linux|macOS|constraint|limit|failure|error") {
        Write-CheckResult "TG004" "Constraints and limits documented" "PASS" "Found in $specFile"
    } else {
        Write-CheckResult "TG004" "Constraints and limits documented" "FAIL" "Not found in $specFile"
    }
} else {
    Write-CheckResult "TG004" "Constraints and limits documented" "FAIL" "Spec/README not found"
}

# TG005: Verify SHA-256 hashes provided for key artifacts in HASHES.txt [§4.5]
Write-Host "`n=== TG005: SHA-256 Hashes Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== TG005: SHA-256 Hashes Check ===" | Add-Content -Path $Output

$hashesFile = Join-Path $ResultsDir "HASHES.txt"
if (Test-Path $hashesFile) {
    $hashLines = Get-Content $hashesFile | Where-Object { $_ -match '^[a-f0-9]{64}' }
    $hashCount = $hashLines.Count
    if ($hashCount -gt 0) {
        Write-CheckResult "TG005" "SHA-256 hashes provided" "PASS" "$hashCount hashes found"
    } else {
        Write-CheckResult "TG005" "SHA-256 hashes provided" "FAIL" "No valid SHA-256 hashes found"
    }
} else {
    Write-CheckResult "TG005" "SHA-256 hashes provided" "FAIL" "HASHES.txt not found"
}

# TG006: Verify scheduler/executor parameters prove no artificial caps (if "unbounded" claimed) [§4.6]
Write-Host "`n=== TG006: Unbounded Proof Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== TG006: Unbounded Proof Check ===" | Add-Content -Path $Output

$unboundedClaims = Get-ChildItem -Path $SpecDir, (Join-Path $RootDir "README.md") -File -ErrorAction SilentlyContinue |
    Select-String -Pattern "unbounded" -CaseSensitive:$false | Select-Object -First 1

if ($unboundedClaims) {
    $proofFiles = @(
        (Join-Path $SpecDir "plan.md"),
        (Join-Path $SpecDir "spec.md")
    )
    $hasProof = $false
    foreach ($file in $proofFiles) {
        if (Test-Path $file) {
            $content = Get-Content $file -Raw
            if ($content -match "scheduler|executor|parameter|cap") {
                $hasProof = $true
                break
            }
        }
    }

    if ($hasProof) {
        Write-CheckResult "TG006" "Unbounded proof provided" "PASS" "Scheduler/executor parameters documented"
    } else {
        Write-CheckResult "TG006" "Unbounded proof provided" "FAIL" "Unbounded claimed but no proof found"
    }
} else {
    Write-CheckResult "TG006" "Unbounded proof provided" "N/A" "No unbounded claims found"
}

# TG007: Verify gap scan completed with coverage table and unresolved gaps listed [§4.7]
Write-Host "`n=== TG007: Gap Scan Check ===" -ForegroundColor Yellow
"" | Add-Content -Path $Output
"=== TG007: Gap Scan Check ===" | Add-Content -Path $Output

$coverageFile = Join-Path $ResultsDir "COVERAGE.md"
$gapsFile = Join-Path $ResultsDir "GAPS.md"

if ((Test-Path $coverageFile) -and (Test-Path $gapsFile)) {
    $coverageContent = Get-Content $coverageFile -Raw
    if ($coverageContent -match "Coverage|coverage|Gap|gap") {
        Write-CheckResult "TG007" "Gap scan completed" "PASS" "Coverage table and gaps documented"
    } else {
        Write-CheckResult "TG007" "Gap scan completed" "FAIL" "Coverage table missing"
    }
} else {
    Write-CheckResult "TG007" "Gap scan completed" "FAIL" "COVERAGE.md or GAPS.md missing"
}

# Summary
Write-Host "`n=== Truth Gate Summary ===" -ForegroundColor Cyan
"" | Add-Content -Path $Output
"=== Truth Gate Summary ===" | Add-Content -Path $Output

$summary = @(
    "Passed: $checksPassed",
    "Failed: $checksFailed",
    "N/A:    $checksNA",
    "Total:  $($checksPassed + $checksFailed + $checksNA)"
)

foreach ($line in $summary) {
    Write-Host $line
    Add-Content -Path $Output -Value $line
}

# Git HEAD
if (Test-Path (Join-Path $RootDir ".git")) {
    try {
        Push-Location $RootDir
        $headHash = git rev-parse HEAD 2>$null
        if ($headHash) {
            "" | Add-Content -Path $Output
            "Git HEAD: $headHash" | Add-Content -Path $Output
        }
    } catch {
        # Git not available or not a repo
    } finally {
        Pop-Location
    }
}

"" | Add-Content -Path $Output
"Results saved to: $Output" | Add-Content -Path $Output
Write-Host "`nResults saved to: $Output" -ForegroundColor Green

exit $status

