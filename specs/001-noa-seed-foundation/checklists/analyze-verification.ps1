# Analyze Verification Checklist Status
# Counts completed vs incomplete items

$checklistFile = Join-Path $PSScriptRoot "verification.md"
$content = Get-Content $checklistFile -Raw

# Count items
$total = ([regex]::Matches($content, '- \[[ xX]\]')).Count
$completed = ([regex]::Matches($content, '- \[[xX]\]')).Count
$incomplete = ([regex]::Matches($content, '- \[ \]')).Count

Write-Host "Verification Checklist Status:"
Write-Host "Total Items: $total"
Write-Host "Completed: $completed"
Write-Host "Incomplete: $incomplete"
Write-Host "Completion Rate: $([math]::Round(($completed / $total) * 100, 2))%"

$status = if ($incomplete -eq 0) { "PASS" } else { "INCOMPLETE" }
Write-Host "Status: $status"

@{
    Total = $total
    Completed = $completed
    Incomplete = $incomplete
    Status = $status
} | ConvertTo-Json

