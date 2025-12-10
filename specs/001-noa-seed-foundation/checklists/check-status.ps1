$checklists = Get-ChildItem *.md | Where-Object { $_.Name -ne 'check-status.ps1' }
$results = @()

foreach ($file in $checklists) {
    $content = Get-Content $file.FullName -Raw
    $total = ([regex]::Matches($content, '- \[[ xX]\]')).Count
    $completed = ([regex]::Matches($content, '- \[[xX]\]')).Count
    $incomplete = ([regex]::Matches($content, '- \[ \]')).Count
    $status = if ($incomplete -eq 0) { '✓ PASS' } else { '✗ FAIL' }
    
    $results += [PSCustomObject]@{
        Checklist = $file.Name
        Total = $total
        Completed = $completed
        Incomplete = $incomplete
        Status = $status
    }
}

$results | Format-Table -AutoSize

$allComplete = ($results | Where-Object { $_.Incomplete -gt 0 }).Count -eq 0
Write-Host "`nOverall Status: $(if ($allComplete) { '✓ ALL PASS' } else { '✗ SOME INCOMPLETE' })"
