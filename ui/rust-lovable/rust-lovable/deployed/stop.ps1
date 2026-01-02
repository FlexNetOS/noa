# Rust Lovable - Stop Script
Write-Host "Stopping Rust Lovable..." -ForegroundColor Yellow
Get-Process -Name "rust-lovable" -ErrorAction SilentlyContinue | Stop-Process -Force
Write-Host "Stopped." -ForegroundColor Green
