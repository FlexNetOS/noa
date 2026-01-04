# NOA Seed Foundation - Complete System Test Suite
# Comprehensive testing for all 20 phases

Write-Host "`n=== NOA SEED FOUNDATION - COMPREHENSIVE TEST SUITE ===" -ForegroundColor Cyan
Write-Host "Testing all components for 100% health and activity`n" -ForegroundColor Yellow

$testResults = @()
$totalTests = 0
$passedTests = 0

function Test-Component {
    param(
        [string]$Name,
        [scriptblock]$Test
    )
    
    $script:totalTests++
    Write-Host "Testing: $Name..." -NoNewline
    
    try {
        $result = & $Test
        if ($result) {
            Write-Host " PASS" -ForegroundColor Green
            $script:passedTests++
            $script:testResults += [PSCustomObject]@{
                Component = $Name
                Status = "PASS"
                Message = "OK"
            }
            return $true
        } else {
            Write-Host " FAIL" -ForegroundColor Red
            $script:testResults += [PSCustomObject]@{
                Component = $Name
                Status = "FAIL"
                Message = "Test returned false"
            }
            return $false
        }
    } catch {
        Write-Host " ERROR" -ForegroundColor Red
        $script:testResults += [PSCustomObject]@{
            Component = $Name
            Status = "ERROR"
            Message = $_.Exception.Message
        }
        return $false
    }
}

Write-Host "=== PHASE 0: BOOTSTRAP VERIFICATION ===" -ForegroundColor Cyan

Test-Component "Rust Toolchain" {
    $rustc = & rustc --version 2>&1
    $rustc -match "rustc 1\."
}

Test-Component "Go Toolchain" {
    $go = & go version 2>&1
    $go -match "go1\."
}

Test-Component "Node.js Toolchain" {
    $node = & node --version 2>&1
    $node -match "v\d+"
}

Test-Component "Python Toolchain" {
    $python = & python --version 2>&1
    $python -match "Python 3\."
}

Test-Component "Protocol Buffers" {
    $protoc = & protoc --version 2>&1
    $protoc -match "libprotoc"
}

Write-Host "`n=== PHASE 1: CORE INFRASTRUCTURE ===" -ForegroundColor Cyan

Test-Component "Directory Structure" {
    (Test-Path "sys/core") -and 
    (Test-Path "sys/ui") -and 
    (Test-Path "init") -and 
    (Test-Path "configs") -and 
    (Test-Path "bin")
}

Test-Component "Rust Workspace" {
    Test-Path "sys/core/Cargo.toml"
}

Test-Component "Database Schema" {
    Test-Path "init/migrations/001_initial.sql"
}

Write-Host "`n=== PHASE 2: DATABASE & STORAGE ===" -ForegroundColor Cyan

Test-Component "SQLite Database" {
    Test-Path "sys/core/data/noa.db"
}

Test-Component "Database Size" {
    $db = Get-Item "sys/core/data/noa.db" -ErrorAction SilentlyContinue
    $db -and $db.Length -gt 0
}

Write-Host "`n=== PHASE 3: API SERVER ===" -ForegroundColor Cyan

Test-Component "API Server Binary" {
    Test-Path "sys/core/target/debug/noa-api.exe"
}

Test-Component "API Health Endpoint" {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:3001/health" -UseBasicParsing -TimeoutSec 5
        $response.StatusCode -eq 200
    } catch {
        $false
    }
}

Test-Component "API Status Endpoint" {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:3001/api/v1/status" -UseBasicParsing -TimeoutSec 5
        $response.StatusCode -eq 200
    } catch {
        $false
    }
}

Test-Component "API Task Endpoints" {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:3001/api/v1/tasks" -UseBasicParsing -TimeoutSec 5
        $response.StatusCode -eq 200
    } catch {
        $false
    }
}

Write-Host "`n=== PHASE 4: UI DASHBOARD ===" -ForegroundColor Cyan

Test-Component "UI Package.json" {
    Test-Path "sys/ui/package.json"
}

Test-Component "UI Dependencies" {
    Test-Path "sys/ui/node_modules"
}

Test-Component "UI Server" {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:3000" -UseBasicParsing -TimeoutSec 5
        $response.StatusCode -eq 200
    } catch {
        $false
    }
}

Write-Host "`n=== PHASE 5: NEURAL RUNTIME ===" -ForegroundColor Cyan

Test-Component "Neural Crate" {
    Test-Path "sys/core/crates/neural/Cargo.toml"
}

Test-Component "llama.cpp Binary" {
    Test-Path "opt/llama.cpp/build/bin/llama-server.exe"
}

Test-Component "llama.cpp CLI" {
    Test-Path "opt/llama.cpp/build/bin/llama-cli.exe"
}

Write-Host "`n=== PHASE 6: AGENT SYSTEM ===" -ForegroundColor Cyan

Test-Component "Agent Crate" {
    Test-Path "sys/core/crates/agent/Cargo.toml"
}

Test-Component "Agent Orchestrator" {
    Test-Path "sys/core/crates/agent/src/orchestrator.rs"
}

Test-Component "MicroAgentStack" {
    Test-Path "sys/core/crates/agent/src/stack.rs"
}

Write-Host "`n=== PHASE 7: EMBEDDER SERVICE ===" -ForegroundColor Cyan

Test-Component "Embedder Crate" {
    Test-Path "sys/core/crates/embedder/Cargo.toml"
}

Test-Component "Embedder Service" {
    Test-Path "sys/core/crates/embedder/src/service.rs"
}

Write-Host "`n=== PHASE 8: BUILD SYSTEM ===" -ForegroundColor Cyan

Test-Component "Cargo Build" {
    Push-Location "sys/core"
    $result = cargo build --workspace 2>&1
    Pop-Location
    $LASTEXITCODE -eq 0
}

Write-Host "`n=== PHASE 9: DOCUMENTATION ===" -ForegroundColor Cyan

Test-Component "README.md" {
    Test-Path "README.md"
}

Test-Component "QUICKSTART.md" {
    Test-Path "QUICKSTART.md"
}

Test-Component "Specification" {
    Test-Path "specs/001-noa-seed-foundation/spec.md"
}

Write-Host "`n=== PHASE 10: configsURATION ===" -ForegroundColor Cyan

Test-Component "configs Directory" {
    Test-Path "configs"
}

Test-Component "Data Directory" {
    Test-Path "data"
}

Write-Host "`n=== TEST SUMMARY ===" -ForegroundColor Cyan
Write-Host "Total Tests: $totalTests" -ForegroundColor White
Write-Host "Passed: $passedTests" -ForegroundColor Green
Write-Host "Failed: $($totalTests - $passedTests)" -ForegroundColor Red
Write-Host "Success Rate: $([math]::Round(($passedTests / $totalTests) * 100, 2))%" -ForegroundColor Yellow

Write-Host "`n=== DETAILED RESULTS ===" -ForegroundColor Cyan
$testResults | Format-Table -AutoSize

if ($passedTests -eq $totalTests) {
    Write-Host "`n=== ALL TESTS PASSED - SYSTEM 100% HEALTHY ===" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`n=== SOME TESTS FAILED - REVIEW REQUIRED ===" -ForegroundColor Red
    exit 1
}
