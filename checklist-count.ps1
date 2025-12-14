$checklists = @(
    "comprehensive.md",
    "quality.md",
    "kernel-independence.md",
    "verification.md",
    "requirements.md"
)

$results = @()

foreach ($file in $checklists) {
    $path = "n:\noa\specs\001-noa-seed-foundation\checklists\$file"
    if (Test-Path $path) {
        $content = Get-Content $path -Raw
        $total = ([regex]::Matches($content, '- \[[ Xx]\]')).Count
        $completed = ([regex]::Matches($content, '- \[[Xx]\]')).Count
        $incomplete = $total - $completed
        $status = if ($incomplete -eq 0) { "✓ PASS" } else { "✗ FAIL" }
        
        $results += [PSCustomObject]@{
            Checklist = $file
            Total = $total
            Completed = $completed
            Incomplete = $incomplete
            Status = $status
        }
    }
}

Write-Output "| Checklist | Total | Completed | Incomplete | Status |"
Write-Output "|-----------|-------|-----------|------------|--------|"
foreach ($r in $results) {
    Write-Output "| $($r.Checklist) | $($r.Total) | $($r.Completed) | $($r.Incomplete) | $($r.Status) |"
}

$allPass = ($results | Where-Object { $_.Incomplete -gt 0 }).Count -eq 0
Write-Output ""
Write-Output "Overall Status: $(if ($allPass) { '✓ PASS - All checklists complete' } else { '✗ FAIL - Some checklists incomplete' })"


