#!/usr/bin/env bash
set -euo pipefail

# Truth Gate Checklist Automation (Phase 9)
# T493: Validates presence and freshness of verification artifacts.
# Implements Universal Task Execution Policy §4 (Truth Gate)

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"
SPEC_DIR="${ROOT_DIR}/specs/001-noa-seed-foundation"

# Output file for Truth Gate results
OUTPUT="${RESULTS_DIR}/TRUTH_GATE_RESULTS.txt"

mkdir -p "${RESULTS_DIR}"
> "${OUTPUT}"

status=0
checks_passed=0
checks_failed=0
checks_na=0

check_result() {
    local check_id="$1"
    local check_name="$2"
    local result="$3"
    local details="${4:-}"

    if [[ "${result}" == "PASS" ]]; then
        echo "[PASS] ${check_id}: ${check_name}" | tee -a "${OUTPUT}"
        ((checks_passed++))
    elif [[ "${result}" == "FAIL" ]]; then
        echo "[FAIL] ${check_id}: ${check_name}" | tee -a "${OUTPUT}"
        if [[ -n "${details}" ]]; then
            echo "  Details: ${details}" | tee -a "${OUTPUT}"
        fi
        ((checks_failed++))
        status=1
    elif [[ "${result}" == "N/A" ]]; then
        echo "[N/A]  ${check_id}: ${check_name}" | tee -a "${OUTPUT}"
        if [[ -n "${details}" ]]; then
            echo "  Reason: ${details}" | tee -a "${OUTPUT}"
        fi
        ((checks_na++))
    fi
}

echo "=== Truth Gate Verification (Phase 9) ===" | tee "${OUTPUT}"
echo "Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")" | tee -a "${OUTPUT}"
echo "" | tee -a "${OUTPUT}"

# TG001: Verify all referenced artifacts exist in repo with listed paths [§4.1]
echo "=== TG001: Artifact Presence Check ===" | tee -a "${OUTPUT}"
ARTIFACTS=(
    "HASHES.txt"
    "FINAL_REPORT.md"
    "COVERAGE.md"
    "REPRO.md"
    "EVIDENCE_LEDGER.md"
)

all_artifacts_exist=true
missing_artifacts=()

for artifact in "${ARTIFACTS[@]}"; do
    path="${RESULTS_DIR}/${artifact}"
    if [[ ! -s "${path}" ]]; then
        echo "  [FAIL] Missing or empty: ${artifact}" | tee -a "${OUTPUT}"
        missing_artifacts+=("${artifact}")
        all_artifacts_exist=false
    else
        age_minutes=$(( ( $(date +%s) - $(stat -c %Y "${path}" 2>/dev/null || echo 0) ) / 60 ))
        echo "  [PASS] ${artifact} exists (age ~${age_minutes}m)" | tee -a "${OUTPUT}"
    fi
done

if [[ "${all_artifacts_exist}" == "true" ]]; then
    check_result "TG001" "All referenced artifacts exist" "PASS"
else
    check_result "TG001" "All referenced artifacts exist" "FAIL" "Missing: ${missing_artifacts[*]}"
fi

# TG002: Verify smoke test exits with code 0 and transcript is captured [§4.2]
echo "" | tee -a "${OUTPUT}"
echo "=== TG002: Smoke Test Check ===" | tee -a "${OUTPUT}"
SMOKE_TEST="${RESULTS_DIR}/TEST/smoke-test.sh"
if [[ -f "${SMOKE_TEST}" ]]; then
    if bash "${SMOKE_TEST}" > "${RESULTS_DIR}/smoke-test-transcript.txt" 2>&1; then
        exit_code=$?
        if [[ ${exit_code} -eq 0 ]]; then
            check_result "TG002" "Smoke test exits with code 0" "PASS" "Transcript: smoke-test-transcript.txt"
        else
            check_result "TG002" "Smoke test exits with code 0" "FAIL" "Exit code: ${exit_code}"
        fi
    else
        check_result "TG002" "Smoke test exits with code 0" "FAIL" "Smoke test execution failed"
    fi
else
    check_result "TG002" "Smoke test exits with code 0" "FAIL" "Smoke test not found: ${SMOKE_TEST}"
fi

# TG003: Verify requirements → artifacts → tests mapping has no gaps [§4.3]
echo "" | tee -a "${OUTPUT}"
echo "=== TG003: Requirements Mapping Check ===" | tee -a "${OUTPUT}"
if [[ -f "${RESULTS_DIR}/COVERAGE.md" ]]; then
    # Check if COVERAGE.md contains mapping information
    if grep -q "requirements\|artifacts\|tests\|FR-\|SC-\|VER" "${RESULTS_DIR}/COVERAGE.md" 2>/dev/null; then
        check_result "TG003" "Requirements → artifacts → tests mapping" "PASS" "COVERAGE.md contains mappings"
    else
        check_result "TG003" "Requirements → artifacts → tests mapping" "FAIL" "COVERAGE.md missing mapping content"
    fi
else
    check_result "TG003" "Requirements → artifacts → tests mapping" "FAIL" "COVERAGE.md not found"
fi

# TG004: Verify constraints, supported OS/arch, and failure modes are documented [§4.4]
echo "" | tee -a "${OUTPUT}"
echo "=== TG004: Limits Documentation Check ===" | tee -a "${OUTPUT}"
if [[ -f "${SPEC_DIR}/spec.md" ]] || [[ -f "${ROOT_DIR}/README.md" ]]; then
    spec_file="${SPEC_DIR}/spec.md"
    if [[ ! -f "${spec_file}" ]]; then
        spec_file="${ROOT_DIR}/README.md"
    fi
    if grep -qiE "Windows|Linux|macOS|constraint|limit|failure|error" "${spec_file}" 2>/dev/null; then
        check_result "TG004" "Constraints and limits documented" "PASS" "Found in ${spec_file}"
    else
        check_result "TG004" "Constraints and limits documented" "FAIL" "Not found in ${spec_file}"
    fi
else
    check_result "TG004" "Constraints and limits documented" "FAIL" "Spec/README not found"
fi

# TG005: Verify SHA-256 hashes provided for key artifacts in HASHES.txt [§4.5]
echo "" | tee -a "${OUTPUT}"
echo "=== TG005: SHA-256 Hashes Check ===" | tee -a "${OUTPUT}"
if [[ -f "${RESULTS_DIR}/HASHES.txt" ]]; then
    hash_count=$(grep -c "^[a-f0-9]\{64\}" "${RESULTS_DIR}/HASHES.txt" 2>/dev/null || echo "0")
    if [[ ${hash_count} -gt 0 ]]; then
        check_result "TG005" "SHA-256 hashes provided" "PASS" "${hash_count} hashes found"
    else
        check_result "TG005" "SHA-256 hashes provided" "FAIL" "No valid SHA-256 hashes found"
    fi
else
    check_result "TG005" "SHA-256 hashes provided" "FAIL" "HASHES.txt not found"
fi

# TG006: Verify scheduler/executor parameters prove no artificial caps (if "unbounded" claimed) [§4.6]
echo "" | tee -a "${OUTPUT}"
echo "=== TG006: Unbounded Proof Check ===" | tee -a "${OUTPUT}"
# Check if any "unbounded" claims exist in documentation
if grep -ri "unbounded" "${SPEC_DIR}" "${ROOT_DIR}/README.md" 2>/dev/null | grep -v "\.git" | head -1 >/dev/null; then
    # If unbounded claims exist, check for proof
    if grep -ri "scheduler\|executor\|parameter\|cap" "${SPEC_DIR}/plan.md" "${SPEC_DIR}/spec.md" 2>/dev/null | head -1 >/dev/null; then
        check_result "TG006" "Unbounded proof provided" "PASS" "Scheduler/executor parameters documented"
    else
        check_result "TG006" "Unbounded proof provided" "FAIL" "Unbounded claimed but no proof found"
    fi
else
    check_result "TG006" "Unbounded proof provided" "N/A" "No unbounded claims found"
fi

# TG007: Verify gap scan completed with coverage table and unresolved gaps listed [§4.7]
echo "" | tee -a "${OUTPUT}"
echo "=== TG007: Gap Scan Check ===" | tee -a "${OUTPUT}"
if [[ -f "${RESULTS_DIR}/COVERAGE.md" ]] && [[ -f "${RESULTS_DIR}/GAPS.md" ]]; then
    # Check if coverage table exists
    if grep -q "Coverage\|coverage\|Gap\|gap" "${RESULTS_DIR}/COVERAGE.md" 2>/dev/null; then
        check_result "TG007" "Gap scan completed" "PASS" "Coverage table and gaps documented"
    else
        check_result "TG007" "Gap scan completed" "FAIL" "Coverage table missing"
    fi
else
    check_result "TG007" "Gap scan completed" "FAIL" "COVERAGE.md or GAPS.md missing"
fi

# Summary
echo "" | tee -a "${OUTPUT}"
echo "=== Truth Gate Summary ===" | tee -a "${OUTPUT}"
echo "Passed: ${checks_passed}" | tee -a "${OUTPUT}"
echo "Failed: ${checks_failed}" | tee -a "${OUTPUT}"
echo "N/A:    ${checks_na}" | tee -a "${OUTPUT}"
echo "Total:  $((checks_passed + checks_failed + checks_na))" | tee -a "${OUTPUT}"

# Ensure hashes exist for git HEAD
if [[ -d "${ROOT_DIR}/.git" ]]; then
    head_hash=$(git -C "${ROOT_DIR}" rev-parse HEAD 2>/dev/null || echo "unknown")
    echo "" | tee -a "${OUTPUT}"
    echo "Git HEAD: ${head_hash}" | tee -a "${OUTPUT}"
fi

echo "" | tee -a "${OUTPUT}"
echo "Results saved to: ${OUTPUT}" | tee -a "${OUTPUT}"

exit ${status}
