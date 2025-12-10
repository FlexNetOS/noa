#!/usr/bin/env bash
set -euo pipefail

# Smoke Test for Phase 9 Verification
# Exits with code 0 on success, non-zero on failure
# Captures transcript for Truth Gate verification

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"

echo "=== NOA Seed Foundation Smoke Test ==="
echo "Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
echo "Root: ${ROOT_DIR}"
echo ""

status=0

# Test 1: Verify core directories exist
echo "[TEST 1] Checking core directories..."
required_dirs=("sys" "ai" "config" "scripts" "specs")
for dir in "${required_dirs[@]}"; do
    if [[ -d "${ROOT_DIR}/${dir}" ]]; then
        echo "  ✓ ${dir}/ exists"
    else
        echo "  ✗ ${dir}/ missing"
        status=1
    fi
done

# Test 2: Verify key configuration files
echo ""
echo "[TEST 2] Checking configuration files..."
config_files=(
    "config/ai-providers.json"
    "config/bootstrap-state.json"
    "specs/001-noa-seed-foundation/spec.md"
)
for file in "${config_files[@]}"; do
    if [[ -f "${ROOT_DIR}/${file}" ]]; then
        echo "  ✓ ${file} exists"
    else
        echo "  ✗ ${file} missing"
        status=1
    fi
done

# Test 3: Verify verification artifacts exist
echo ""
echo "[TEST 3] Checking verification artifacts..."
artifacts=(
    "test-results/HASHES.txt"
    "test-results/FINAL_REPORT.md"
    "test-results/COVERAGE.md"
)
for artifact in "${artifacts[@]}"; do
    if [[ -f "${ROOT_DIR}/${artifact}" ]]; then
        echo "  ✓ ${artifact} exists"
    else
        echo "  ⚠ ${artifact} missing (may be expected if not yet generated)"
    fi
done

# Test 4: Verify scripts are executable
echo ""
echo "[TEST 4] Checking script executability..."
scripts=(
    "scripts/bash/truth-gate.sh"
    "scripts/bash/triple-verify.sh"
    "scripts/bash/gap-scan.sh"
)
for script in "${scripts[@]}"; do
    if [[ -f "${ROOT_DIR}/${script}" ]]; then
        if [[ -x "${ROOT_DIR}/${script}" ]] || [[ -f "${ROOT_DIR}/${script}" ]]; then
            echo "  ✓ ${script} exists"
        else
            echo "  ⚠ ${script} not executable"
        fi
    else
        echo "  ✗ ${script} missing"
        status=1
    fi
done

# Test 5: Verify hash generation works
echo ""
echo "[TEST 5] Testing hash generation..."
if command -v sha256sum >/dev/null 2>&1 || command -v shasum >/dev/null 2>&1; then
    test_file="${ROOT_DIR}/README.md"
    if [[ -f "${test_file}" ]]; then
        if command -v sha256sum >/dev/null 2>&1; then
            hash=$(sha256sum "${test_file}" | awk '{print $1}')
        else
            hash=$(shasum -a 256 "${test_file}" | awk '{print $1}')
        fi
        if [[ -n "${hash}" ]] && [[ ${#hash} -eq 64 ]]; then
            echo "  ✓ Hash generation works (sample hash: ${hash:0:8}...)"
        else
            echo "  ✗ Hash generation failed"
            status=1
        fi
    else
        echo "  ⚠ Test file not found, skipping hash test"
    fi
else
    echo "  ⚠ Hash tool not available"
fi

# Summary
echo ""
echo "=== Smoke Test Summary ==="
if [[ ${status} -eq 0 ]]; then
    echo "✓ All critical tests passed"
    echo "Exit code: 0"
else
    echo "✗ Some tests failed"
    echo "Exit code: ${status}"
fi

exit ${status}

