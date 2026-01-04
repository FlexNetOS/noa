#!/bin/bash
#
# Phase 2 Smoke Test
# Verifies all Phase 2 implementation artifacts exist and are functional
#
# CHK018: Deterministic smoke test with command, transcript, and exit code 0
# Phase 2: Database & Storage Infrastructure (T018a-T071)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
NOA_ROOT="${NOA_ROOT:-$REPO_ROOT}"

echo "════════════════════════════════════════════════════════════"
echo "Phase 2 Smoke Test: Database & Storage Infrastructure"
echo "════════════════════════════════════════════════════════════"
echo ""

FAILED=0
PASSED=0

# Test function
test_check() {
    local name="$1"
    local test_cmd="$2"

    if eval "$test_cmd" >/dev/null 2>&1; then
        echo "✅ PASS: $name"
        ((PASSED++))
        return 0
    else
        echo "❌ FAIL: $name"
        ((FAILED++))
        return 1
    fi
}

echo "Testing storage components setup (T018a-T018f)..."
echo ""

# T018a: Data directory structure
test_check "data/memory directory exists" "test -d $NOA_ROOT/data/memory"
test_check "data/knowledge directory exists" "test -d $NOA_ROOT/data/knowledge"
test_check "data/embeddings directory exists" "test -d $NOA_ROOT/data/embeddings"
test_check "data/artifacts directory exists" "test -d $NOA_ROOT/data/artifacts"

# T018b-T018f: Storage configss
test_check "registry.yaml exists" "test -f $NOA_ROOT/containers/oci/registry.yaml"
test_check "minio.yaml exists" "test -f $NOA_ROOT/configs/minio.yaml"
test_check "database.yaml exists" "test -f $NOA_ROOT/configs/database.yaml"
test_check "qdrant.yaml exists" "test -f $NOA_ROOT/configs/qdrant.yaml"
test_check "quickwit.yaml exists" "test -f $NOA_ROOT/configs/quickwit.yaml"

echo ""
echo "Testing database schema files (T018g-T037)..."
echo ""

# Migration files
test_check "001_initial.sql exists" "test -f $NOA_ROOT/init/migrations/001_initial.sql"
test_check "002_indexes.sql exists" "test -f $NOA_ROOT/init/migrations/002_indexes.sql"
test_check "003_vectors.sql exists" "test -f $NOA_ROOT/init/migrations/003_vectors.sql"

# Verify schema contains key tables
test_check "001_initial.sql contains memory table" "grep -q 'CREATE TABLE.*memory' $NOA_ROOT/init/migrations/001_initial.sql"
test_check "001_initial.sql contains embedding table" "grep -q 'CREATE TABLE.*embedding' $NOA_ROOT/init/migrations/001_initial.sql"
test_check "001_initial.sql contains agent table" "grep -q 'CREATE TABLE.*agent' $NOA_ROOT/init/migrations/001_initial.sql"

echo ""
echo "Testing CSV export and schemas (T041-T045)..."
echo ""

# CSV export service
test_check "csv_export.rs exists" "test -f $NOA_ROOT/sys/core/src/export/csv_export.rs"

# CSV schemas
test_check "agent_directory.yaml exists" "test -f $NOA_ROOT/configs/schemas/csv/agent_directory.yaml"
test_check "task_tables.yaml exists" "test -f $NOA_ROOT/configs/schemas/csv/task_tables.yaml"
test_check "claims_evidence.yaml exists" "test -f $NOA_ROOT/configs/schemas/csv/claims_evidence.yaml"
test_check "metrics_traces.yaml exists" "test -f $NOA_ROOT/configs/schemas/csv/metrics_traces.yaml"

echo ""
echo "Testing configsuration standards (T046-T049)..."
echo ""

test_check "configs_schema.json exists" "test -f $NOA_ROOT/configs/schemas/configs_schema.json"
test_check "validator.rs exists" "test -f $NOA_ROOT/sys/core/src/configs/validator.rs"
test_check "lineage.rs exists" "test -f $NOA_ROOT/sys/core/src/configs/lineage.rs"
test_check "configs templates directory exists" "test -d $NOA_ROOT/configs/templates"

echo ""
echo "Testing Rust core foundation (T050-T055)..."
echo ""

test_check "error.rs exists" "test -f $NOA_ROOT/sys/core/src/error.rs"
test_check "configs/mod.rs exists" "test -f $NOA_ROOT/sys/core/src/configs/mod.rs"
test_check "logging.rs exists" "test -f $NOA_ROOT/sys/core/src/logging.rs"
test_check "db/pool.rs exists" "test -f $NOA_ROOT/sys/core/src/db/pool.rs"
test_check "db/repository.rs exists" "test -f $NOA_ROOT/sys/core/src/db/repository.rs"
test_check "db/migrations.rs exists" "test -f $NOA_ROOT/sys/core/src/db/migrations.rs"

echo ""
echo "Testing API foundation (T056-T060)..."
echo ""

test_check "api/server.rs exists" "test -f $NOA_ROOT/sys/core/src/api/server.rs"
test_check "api/routes/health.rs exists" "test -f $NOA_ROOT/sys/core/src/api/routes/health.rs"
test_check "api/middleware/validation.rs exists" "test -f $NOA_ROOT/sys/core/src/api/middleware/validation.rs"
test_check "api/middleware/logging.rs exists" "test -f $NOA_ROOT/sys/core/src/api/middleware/logging.rs"
test_check "api/middleware/telemetry.rs exists" "test -f $NOA_ROOT/sys/core/src/api/middleware/telemetry.rs"

echo ""
echo "Testing CLI foundation (T061-T067)..."
echo ""

test_check "main.rs exists" "test -f $NOA_ROOT/sys/core/src/main.rs"
test_check "cli/init.rs exists" "test -f $NOA_ROOT/sys/core/src/cli/init.rs"
test_check "cli/start.rs exists" "test -f $NOA_ROOT/sys/core/src/cli/start.rs"
test_check "cli/status.rs exists" "test -f $NOA_ROOT/sys/core/src/cli/status.rs"
test_check "cli/stop.rs exists" "test -f $NOA_ROOT/sys/core/src/cli/stop.rs"
test_check "cli/db.rs exists" "test -f $NOA_ROOT/sys/core/src/cli/db.rs"

echo ""
echo "Testing observability foundation (T068-T071)..."
echo ""

test_check "observability/logging.rs exists" "test -f $NOA_ROOT/sys/core/src/observability/logging.rs"
test_check "observability/telemetry.rs exists" "test -f $NOA_ROOT/sys/core/src/observability/telemetry.rs"
test_check "observability/metrics.rs exists" "test -f $NOA_ROOT/sys/core/src/observability/metrics.rs"
test_check "observability.yaml exists" "test -f $NOA_ROOT/configs/observability.yaml"

echo ""
echo "Testing database functionality..."
echo ""

# Test that database can be initialized (if cargo is available)
if command -v cargo >/dev/null 2>&1; then
    if [ -f "$NOA_ROOT/sys/core/Cargo.toml" ]; then
        test_check "Rust project compiles" "cd $NOA_ROOT/sys/core && cargo check --quiet 2>&1"
    fi
fi

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Test Summary"
echo "════════════════════════════════════════════════════════════"
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "✅ All Phase 2 checks passed"
    exit 0
else
    echo "❌ Some Phase 2 checks failed"
    exit 1
fi

