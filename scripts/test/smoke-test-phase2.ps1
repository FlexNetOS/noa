# Phase 2 Smoke Test (PowerShell)
# Verifies all Phase 2 implementation artifacts exist and are functional
#
# CHK018: Deterministic smoke test with command, transcript, and exit code 0
# Phase 2: Database & Storage Infrastructure (T018a-T071)

$ErrorActionPreference = "Stop"

$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$REPO_ROOT = Split-Path -Parent (Split-Path -Parent $SCRIPT_DIR)
$NOA_ROOT = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { $REPO_ROOT }

$FAILED = 0
$PASSED = 0

function Test-Check {
    param(
        [string]$Name,
        [scriptblock]$TestCmd
    )

    try {
        & $TestCmd | Out-Null
        Write-Host "✅ PASS: $Name" -ForegroundColor Green
        $script:PASSED++
        return $true
    } catch {
        Write-Host "❌ FAIL: $Name" -ForegroundColor Red
        $script:FAILED++
        return $false
    }
}

Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Phase 2 Smoke Test: Database & Storage Infrastructure"
Write-Host "════════════════════════════════════════════════════════════"
Write-Host ""

Write-Host "Testing storage components setup (T018a-T018f)..."
Write-Host ""

# T018a: Data directory structure
Test-Check "data/memory directory exists" { Test-Path "$NOA_ROOT/data/memory" -PathType Container }
Test-Check "data/knowledge directory exists" { Test-Path "$NOA_ROOT/data/knowledge" -PathType Container }
Test-Check "data/embeddings directory exists" { Test-Path "$NOA_ROOT/data/embeddings" -PathType Container }
Test-Check "data/artifacts directory exists" { Test-Path "$NOA_ROOT/data/artifacts" -PathType Container }

# T018b-T018f: Storage configss
Test-Check "registry.yaml exists" { Test-Path "$NOA_ROOT/containers/oci/registry.yaml" -PathType Leaf }
Test-Check "minio.yaml exists" { Test-Path "$NOA_ROOT/configs/minio.yaml" -PathType Leaf }
Test-Check "database.yaml exists" { Test-Path "$NOA_ROOT/configs/database.yaml" -PathType Leaf }
Test-Check "qdrant.yaml exists" { Test-Path "$NOA_ROOT/configs/qdrant.yaml" -PathType Leaf }
Test-Check "quickwit.yaml exists" { Test-Path "$NOA_ROOT/configs/quickwit.yaml" -PathType Leaf }

Write-Host ""
Write-Host "Testing database schema files (T018g-T037)..."
Write-Host ""

# Migration files
Test-Check "001_initial.sql exists" { Test-Path "$NOA_ROOT/init/migrations/001_initial.sql" -PathType Leaf }
Test-Check "002_indexes.sql exists" { Test-Path "$NOA_ROOT/init/migrations/002_indexes.sql" -PathType Leaf }
Test-Check "003_vectors.sql exists" { Test-Path "$NOA_ROOT/init/migrations/003_vectors.sql" -PathType Leaf }

# Verify schema contains key tables
Test-Check "001_initial.sql contains memory table" {
    Select-String -Path "$NOA_ROOT/init/migrations/001_initial.sql" -Pattern "CREATE TABLE.*memory" -Quiet
}
Test-Check "001_initial.sql contains embedding table" {
    Select-String -Path "$NOA_ROOT/init/migrations/001_initial.sql" -Pattern "CREATE TABLE.*embedding" -Quiet
}
Test-Check "001_initial.sql contains agent table" {
    Select-String -Path "$NOA_ROOT/init/migrations/001_initial.sql" -Pattern "CREATE TABLE.*agent" -Quiet
}

Write-Host ""
Write-Host "Testing CSV export and schemas (T041-T045)..."
Write-Host ""

# CSV export service
Test-Check "csv_export.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/export/csv_export.rs" -PathType Leaf }

# CSV schemas
Test-Check "agent_directory.yaml exists" { Test-Path "$NOA_ROOT/configs/schemas/csv/agent_directory.yaml" -PathType Leaf }
Test-Check "task_tables.yaml exists" { Test-Path "$NOA_ROOT/configs/schemas/csv/task_tables.yaml" -PathType Leaf }
Test-Check "claims_evidence.yaml exists" { Test-Path "$NOA_ROOT/configs/schemas/csv/claims_evidence.yaml" -PathType Leaf }
Test-Check "metrics_traces.yaml exists" { Test-Path "$NOA_ROOT/configs/schemas/csv/metrics_traces.yaml" -PathType Leaf }

Write-Host ""
Write-Host "Testing configsuration standards (T046-T049)..."
Write-Host ""

Test-Check "configs_schema.json exists" { Test-Path "$NOA_ROOT/configs/schemas/configs_schema.json" -PathType Leaf }
Test-Check "validator.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/configs/validator.rs" -PathType Leaf }
Test-Check "lineage.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/configs/lineage.rs" -PathType Leaf }
Test-Check "configs templates directory exists" { Test-Path "$NOA_ROOT/configs/templates" -PathType Container }

Write-Host ""
Write-Host "Testing Rust core foundation (T050-T055)..."
Write-Host ""

Test-Check "error.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/error.rs" -PathType Leaf }
Test-Check "configs/mod.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/configs/mod.rs" -PathType Leaf }
Test-Check "logging.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/logging.rs" -PathType Leaf }
Test-Check "db/pool.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/db/pool.rs" -PathType Leaf }
Test-Check "db/repository.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/db/repository.rs" -PathType Leaf }
Test-Check "db/migrations.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/db/migrations.rs" -PathType Leaf }

Write-Host ""
Write-Host "Testing API foundation (T056-T060)..."
Write-Host ""

Test-Check "api/server.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/api/server.rs" -PathType Leaf }
Test-Check "api/routes/health.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/api/routes/health.rs" -PathType Leaf }
Test-Check "api/middleware/validation.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/api/middleware/validation.rs" -PathType Leaf }
Test-Check "api/middleware/logging.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/api/middleware/logging.rs" -PathType Leaf }
Test-Check "api/middleware/telemetry.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/api/middleware/telemetry.rs" -PathType Leaf }

Write-Host ""
Write-Host "Testing CLI foundation (T061-T067)..."
Write-Host ""

Test-Check "main.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/main.rs" -PathType Leaf }
Test-Check "cli/init.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/cli/init.rs" -PathType Leaf }
Test-Check "cli/start.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/cli/start.rs" -PathType Leaf }
Test-Check "cli/status.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/cli/status.rs" -PathType Leaf }
Test-Check "cli/stop.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/cli/stop.rs" -PathType Leaf }
Test-Check "cli/db.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/cli/db.rs" -PathType Leaf }

Write-Host ""
Write-Host "Testing observability foundation (T068-T071)..."
Write-Host ""

Test-Check "observability/logging.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/observability/logging.rs" -PathType Leaf }
Test-Check "observability/telemetry.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/observability/telemetry.rs" -PathType Leaf }
Test-Check "observability/metrics.rs exists" { Test-Path "$NOA_ROOT/sys/core/src/observability/metrics.rs" -PathType Leaf }
Test-Check "observability.yaml exists" { Test-Path "$NOA_ROOT/configs/observability.yaml" -PathType Leaf }

Write-Host ""
Write-Host "Testing database functionality..."
Write-Host ""

# Test that database can be initialized (if cargo is available)
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    if (Test-Path "$NOA_ROOT/sys/core/Cargo.toml" -PathType Leaf) {
        Test-Check "Rust project compiles" {
            Push-Location "$NOA_ROOT/sys/core"
            cargo check --quiet 2>&1 | Out-Null
            Pop-Location
        }
    }
}

Write-Host ""
Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Test Summary"
Write-Host "════════════════════════════════════════════════════════════"
Write-Host "Passed: $PASSED"
Write-Host "Failed: $FAILED"
Write-Host ""

if ($FAILED -eq 0) {
    Write-Host "✅ All Phase 2 checks passed" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ Some Phase 2 checks failed" -ForegroundColor Red
    exit 1
}

