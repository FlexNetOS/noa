# Rust Lovable - Status Script
$Process = Get-Process -Name "rust-lovable" -ErrorAction SilentlyContinue
if ($Process) {
    Write-Host "Rust Lovable is running (PID: $($Process.Id))" -ForegroundColor Green
    Write-Host "Memory: $([math]::Round($Process.WorkingSet64 / 1MB, 2)) MB" -ForegroundColor Cyan
    Write-Host "CPU Time: $($Process.TotalProcessorTime)" -ForegroundColor Cyan
} else {
    Write-Host "Rust Lovable is not running" -ForegroundColor Yellow
}
