# NOA Seed Foundation - System Status Report
# Quick health check for all components

Write-Host "`n╔════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         NOA SEED FOUNDATION - SYSTEM STATUS REPORT            ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

$status = @{
    Healthy = 0
    Total = 0
}

function Check-Component {
    param([string]$Name, [scriptblock]$Check)
    $status.Total++
    Write-Host "$Name..." -NoNewline -ForegroundColor Yellow
    try {
        if (& $Check) {
            Write-Host " ✓" -ForegroundColor Green
            $status.Healthy++
            return $true
        } else {
            Write-Host " ✗" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host " ✗ ($($_.Exception.Message))" -ForegroundColor Red
        return $false
    }
}

Write-Host "═══ PHASE 0: BOOTSTRAP ═══" -ForegroundColor Cyan
Check-Component "Rust Toolchain (1.91+)" { (rustc --version) -match "1\." }
Check-Component "Go Toolchain (1.23+)" { (go version) -match "go1\." }
Check-Component "Node.js (20+)" { (node --version) -match "v\d+" }
Check-Component "Python (3.12+)" { (python --version) -match "Python 3\." }

Write-Host "`n═══ PHASE 1: INFRASTRUCTURE ═══" -ForegroundColor Cyan
Check-Component "Directory Structure" { (Test-Path "sys/core") -and (Test-Path "sys/ui") }
Check-Component "Rust Workspace" { Test-Path "sys/core/Cargo.toml" }
Check-Component "Database Schema" { Test-Path "init/migrations/001_initial.sql" }

Write-Host "`n═══ PHASE 2: DATABASE ═══" -ForegroundColor Cyan
Check-Component "SQLite Database" { Test-Path "sys/core/data/noa.db" }
Check-Component "Database Initialized" { (Get-Item "sys/core/data/noa.db").Length -gt 0 }

Write-Host "`n═══ PHASE 3: API SERVER ═══" -ForegroundColor Cyan
Check-Component "API Binary Built" { Test-Path "sys/core/target/debug/noa-api.exe" }
Check-Component "API Health (http://localhost:3001/health)" { 
    try { (Invoke-WebRequest -Uri "http://localhost:3001/health" -UseBasicParsing -TimeoutSec 2).StatusCode -eq 200 } catch { $false }
}
Check-Component "API Status (http://localhost:3001/api/v1/status)" { 
    try { (Invoke-WebRequest -Uri "http://localhost:3001/api/v1/status" -UseBasicParsing -TimeoutSec 2).StatusCode -eq 200 } catch { $false }
}

Write-Host "`n═══ PHASE 4: UI DASHBOARD ═══" -ForegroundColor Cyan
Check-Component "UI Package" { Test-Path "sys/ui/package.json" }
Check-Component "UI Dependencies" { Test-Path "sys/ui/node_modules" }
Check-Component "UI Server (http://localhost:3000)" { 
    try { (Invoke-WebRequest -Uri "http://localhost:3000" -UseBasicParsing -TimeoutSec 2).StatusCode -eq 200 } catch { $false }
}

Write-Host "`n═══ PHASE 5: NEURAL RUNTIME ═══" -ForegroundColor Cyan
Check-Component "Neural Crate" { Test-Path "sys/core/crates/neural/Cargo.toml" }
Check-Component "llama.cpp Server" { Test-Path "opt/llama.cpp/build/bin/llama-server.exe" }
Check-Component "llama.cpp CLI" { Test-Path "opt/llama.cpp/build/bin/llama-cli.exe" }

Write-Host "`n═══ PHASE 6: AGENT SYSTEM ═══" -ForegroundColor Cyan
Check-Component "Agent Crate" { Test-Path "sys/core/crates/agent/Cargo.toml" }
Check-Component "CECCA Orchestrator" { Test-Path "sys/core/crates/agent/src/orchestrator.rs" }
Check-Component "MicroAgentStack" { Test-Path "sys/core/crates/agent/src/stack.rs" }

Write-Host "`n═══ PHASE 7: EMBEDDER ═══" -ForegroundColor Cyan
Check-Component "Embedder Crate" { Test-Path "sys/core/crates/embedder/Cargo.toml" }
Check-Component "Embedder Service" { Test-Path "sys/core/crates/embedder/src/service.rs" }

Write-Host "`n═══ PHASE 8: DOCUMENTATION ═══" -ForegroundColor Cyan
Check-Component "README.md" { Test-Path "README.md" }
Check-Component "QUICKSTART.md" { Test-Path "QUICKSTART.md" }
Check-Component "Specifications" { Test-Path "specs/001-noa-seed-foundation/spec.md" }

$percentage = [math]::Round(($status.Healthy / $status.Total) * 100, 1)

Write-Host "`n╔════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║                      SYSTEM HEALTH SUMMARY                     ║" -ForegroundColor Cyan
Write-Host "╠════════════════════════════════════════════════════════════════╣" -ForegroundColor Cyan
Write-Host "║  Total Components: $($status.Total.ToString().PadLeft(2))                                           ║" -ForegroundColor White
Write-Host "║  Healthy: $($status.Healthy.ToString().PadLeft(2))                                                  ║" -ForegroundColor Green
Write-Host "║  Failed: $(($status.Total - $status.Healthy).ToString().PadLeft(2))                                                   ║" -ForegroundColor $(if ($status.Healthy -eq $status.Total) { "Green" } else { "Red" })
Write-Host "║  Health: $($percentage.ToString().PadLeft(5))%                                              ║" -ForegroundColor $(if ($percentage -eq 100) { "Green" } elseif ($percentage -ge 80) { "Yellow" } else { "Red" })
Write-Host "╚════════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

if ($percentage -eq 100) {
    Write-Host "🎉 SYSTEM 100% OPERATIONAL - ALL COMPONENTS HEALTHY!" -ForegroundColor Green
} elseif ($percentage -ge 80) {
    Write-Host "⚠️  SYSTEM MOSTLY OPERATIONAL - SOME COMPONENTS NEED ATTENTION" -ForegroundColor Yellow
} else {
    Write-Host "❌ SYSTEM DEGRADED - MULTIPLE COMPONENTS NEED ATTENTION" -ForegroundColor Red
}

Write-Host ""
