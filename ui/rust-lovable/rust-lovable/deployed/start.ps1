# Rust Lovable - Start Script
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$env:RUST_LOVABLE_CONFIG_DIR = "$ScriptDir\config"
$env:RUST_LOG = if ($env:RUST_LOG) { $env:RUST_LOG } else { "info" }

$RuntimeConfig = "$ScriptDir\config\runtime.json"
if (Test-Path $RuntimeConfig) {
    $Config = Get-Content $RuntimeConfig | ConvertFrom-Json
    $env:RUST_LOVABLE_ADDRESS = "$($Config.server.host):$($Config.server.port)"
}

Write-Host "Starting Rust Lovable..." -ForegroundColor Cyan
Write-Host "Server: $env:RUST_LOVABLE_ADDRESS" -ForegroundColor Green
Write-Host "Config: $env:RUST_LOVABLE_CONFIG_DIR" -ForegroundColor Green

& "$ScriptDir\rust-lovable.exe" @args
