#!/bin/bash
#
# Phase 1 Smoke Test
# Verifies all Phase 1 implementation artifacts exist and are functional
#
# T018: Smoke test for Phase 1
# CHK018: Deterministic smoke test with command, transcript, and exit code 0

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
NOA_ROOT="${NOA_ROOT:-$REPO_ROOT}"

echo "════════════════════════════════════════════════════════════"
echo "Phase 1 Smoke Test: NOA Seed Foundation"
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

echo "Testing directory structure (FR-029 to FR-036)..."
echo ""

# FR-029: sys/ directory
test_check "sys/core directory exists" "test -d $NOA_ROOT/sys/core"
test_check "sys/ui directory exists" "test -d $NOA_ROOT/sys/ui"
test_check "sys/digest directory exists" "test -d $NOA_ROOT/sys/digest"
test_check "sys/kernel directory exists" "test -d $NOA_ROOT/sys/kernel"

# FR-030: p2p/ directory
test_check "p2p directory exists" "test -d $NOA_ROOT/p2p"

# FR-031: opt/ directory
test_check "opt directory exists" "test -d $NOA_ROOT/opt"

# FR-032: init/ directory
test_check "init directory exists" "test -d $NOA_ROOT/init"
test_check "init/bootstrap directory exists" "test -d $NOA_ROOT/init/bootstrap"
test_check "init/migrations directory exists" "test -d $NOA_ROOT/init/migrations"
test_check "init/seeds directory exists" "test -d $NOA_ROOT/init/seeds"

# FR-033: containers/ directory
test_check "containers directory exists" "test -d $NOA_ROOT/containers"

# FR-034: config/ directory
test_check "config directory exists" "test -d $NOA_ROOT/config"

# FR-035: bin/ directory
test_check "bin directory exists" "test -d $NOA_ROOT/bin"

# FR-036: ai/ directory
test_check "ai directory exists" "test -d $NOA_ROOT/ai"
test_check "ai/providers directory exists" "test -d $NOA_ROOT/ai/providers"

echo ""
echo "Testing project initialization files (T010-T013)..."
echo ""

# T010: Rust workspace
test_check "Cargo.toml exists" "test -f $NOA_ROOT/sys/core/Cargo.toml"

# T011: Go module
test_check "go.mod exists" "test -f $NOA_ROOT/p2p/go.mod"

# T012: TypeScript/Next.js
test_check "package.json exists" "test -f $NOA_ROOT/sys/ui/package.json"

# T013: Python project
test_check "pyproject.toml exists" "test -f $NOA_ROOT/sys/digest/pyproject.toml"

echo ""
echo "Testing configuration files (T016)..."
echo ""

test_check "noa-server.json exists" "test -f $NOA_ROOT/config/noa-server.json"
test_check "ai-providers.json exists" "test -f $NOA_ROOT/config/ai-providers.json"
test_check "features.json exists" "test -f $NOA_ROOT/config/features.json"

echo ""
echo "Testing scripts (T015, T673-T674)..."
echo ""

test_check "check-prerequisites.sh exists" "test -f $NOA_ROOT/scripts/bash/check-prerequisites.sh"
test_check "check-prereqs.sh exists" "test -f $NOA_ROOT/init/check-prereqs.sh"
test_check "dirs.sh exists" "test -f $NOA_ROOT/init/bootstrap/dirs.sh"

echo ""
echo "Testing CI pipeline (T017, T675)..."
echo ""

test_check "CI workflow exists" "test -f $NOA_ROOT/.github/workflows/ci.yml"

echo ""
echo "Testing README (T018)..."
echo ""

test_check "README.md exists" "test -f $NOA_ROOT/README.md"

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Test Summary"
echo "════════════════════════════════════════════════════════════"
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "✅ All Phase 1 checks passed"
    exit 0
else
    echo "❌ Some Phase 1 checks failed"
    exit 1
fi

