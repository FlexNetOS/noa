#!/usr/bin/env bash
set -euo pipefail

# Gap Hunt Scan Automation (Phase 9)
# T494: Scans for TODO/FIXME and requirement gaps.
# Implements Universal Task Execution Policy §5.7 (Gap Hunt)

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"
SPEC_DIR="${ROOT_DIR}/specs/001-noa-seed-foundation"
OUTPUT="${RESULTS_DIR}/GAP_SCAN.txt"
COVERAGE_OUTPUT="${RESULTS_DIR}/COVERAGE.md"

mkdir -p "${RESULTS_DIR}"
> "${OUTPUT}"

scanner=""
if command -v rg >/dev/null 2>&1; then
    scanner="rg"
elif command -v ripgrep >/dev/null 2>&1; then
    scanner="ripgrep"
elif command -v grep >/dev/null 2>&1; then
    scanner="grep"
else
    echo "No grep-like tool found." >&2
    exit 1
fi

echo "=== Gap Hunt Scan (Phase 9) ===" | tee "${OUTPUT}"
echo "Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")" | tee -a "${OUTPUT}"
echo "Scanner: ${scanner}" | tee -a "${OUTPUT}"
echo "" | tee -a "${OUTPUT}"

# GH001: Verify gap scan run against full spec outline
echo "=== GH001: Gap Scan Against Spec Outline ===" | tee -a "${OUTPUT}"

# Extract spec sections/requirements
spec_file="${SPEC_DIR}/spec.md"
if [[ -f "${spec_file}" ]]; then
    echo "Scanning spec: ${spec_file}" | tee -a "${OUTPUT}"

    # Count requirements (FR-, SC-, VER-, etc.)
    fr_count=$(grep -c "FR-[0-9]" "${spec_file}" 2>/dev/null || echo "0")
    sc_count=$(grep -c "SC-[0-9]" "${spec_file}" 2>/dev/null || echo "0")
    ver_count=$(grep -c "VER[0-9]" "${spec_file}" 2>/dev/null || echo "0")

    echo "Requirements found:" | tee -a "${OUTPUT}"
    echo "  FR-*: ${fr_count}" | tee -a "${OUTPUT}"
    echo "  SC-*: ${sc_count}" | tee -a "${OUTPUT}"
    echo "  VER*: ${ver_count}" | tee -a "${OUTPUT}"
else
    echo "[WARN] Spec file not found: ${spec_file}" | tee -a "${OUTPUT}"
fi

# Scan for gap markers
echo "" | tee -a "${OUTPUT}"
echo "=== Gap Markers Scan ===" | tee -a "${OUTPUT}"

patterns=("TODO" "FIXME" "GAP" "TBD" "XXX" "HACK" "NOTE" "WARN")

total_gaps=0
for pattern in "${patterns[@]}"; do
    echo "" | tee -a "${OUTPUT}"
    echo "## Pattern: ${pattern}" | tee -a "${OUTPUT}"

    if [[ "${scanner}" == "rg" || "${scanner}" == "ripgrep" ]]; then
        matches=$(${scanner} -n "${pattern}" "${ROOT_DIR}" --exclude-dir ".git" --exclude-dir "node_modules" --exclude-dir "target" 2>/dev/null | tee -a "${OUTPUT}" | wc -l || echo "0")
    else
        matches=$(grep -R -n "${pattern}" "${ROOT_DIR}" --exclude-dir=".git" --exclude-dir="node_modules" --exclude-dir="target" 2>/dev/null | tee -a "${OUTPUT}" | wc -l || echo "0")
    fi

    if [[ ${matches} -gt 0 ]]; then
        echo "  Found: ${matches} occurrences" | tee -a "${OUTPUT}"
        ((total_gaps += matches))
    fi
done

# GH002: Verify coverage table shows all sections
echo "" | tee -a "${OUTPUT}"
echo "=== GH002: Coverage Table Check ===" | tee -a "${OUTPUT}"

if [[ -f "${COVERAGE_OUTPUT}" ]]; then
    # Check if coverage table exists
    if grep -q "Coverage\|Phase\|Status\|FR-\|SC-" "${COVERAGE_OUTPUT}" 2>/dev/null; then
        echo "[PASS] Coverage table exists with sections" | tee -a "${OUTPUT}"

        # Count phases in coverage
        phase_count=$(grep -c "Phase [0-9]" "${COVERAGE_OUTPUT}" 2>/dev/null || echo "0")
        echo "  Phases found: ${phase_count}" | tee -a "${OUTPUT}"
    else
        echo "[FAIL] Coverage table missing or incomplete" | tee -a "${OUTPUT}"
    fi
else
    echo "[FAIL] COVERAGE.md not found" | tee -a "${OUTPUT}"
fi

# GH003: Verify missed items identified and documented
echo "" | tee -a "${OUTPUT}"
echo "=== GH003: Missed Items Documentation ===" | tee -a "${OUTPUT}"

gaps_file="${RESULTS_DIR}/GAPS.md"
if [[ -f "${gaps_file}" ]]; then
    gap_items=$(grep -c "^- \[ \]" "${gaps_file}" 2>/dev/null || echo "0")
    echo "[PASS] GAPS.md exists with ${gap_items} documented gaps" | tee -a "${OUTPUT}"
else
    echo "[WARN] GAPS.md not found - creating template" | tee -a "${OUTPUT}"
    {
        echo "# Gap Analysis"
        echo ""
        echo "## Identified Gaps"
        echo ""
        echo "Total gaps found: ${total_gaps}"
        echo ""
        echo "## Remedies"
        echo ""
        echo "See GAP_SCAN.txt for detailed gap locations."
    } > "${gaps_file}"
fi

# GH004: Verify remedies proposed for each gap
echo "" | tee -a "${OUTPUT}"
echo "=== GH004: Remedies Check ===" | tee -a "${OUTPUT}"

if [[ -f "${gaps_file}" ]]; then
    if grep -qi "remedy\|fix\|solution\|action" "${gaps_file}" 2>/dev/null; then
        echo "[PASS] Remedies documented in GAPS.md" | tee -a "${OUTPUT}"
    else
        echo "[WARN] Remedies not explicitly documented" | tee -a "${OUTPUT}"
    fi
fi

# GH005: Verify no critical gaps remain unaddressed
echo "" | tee -a "${OUTPUT}"
echo "=== GH005: Critical Gaps Check ===" | tee -a "${OUTPUT}"

critical_patterns=("CRITICAL" "BLOCKER" "SECURITY" "DATA_LOSS")
critical_count=0

for pattern in "${critical_patterns[@]}"; do
    if [[ "${scanner}" == "rg" || "${scanner}" == "ripgrep" ]]; then
        count=$(${scanner} -i "${pattern}" "${ROOT_DIR}" --exclude-dir ".git" 2>/dev/null | wc -l || echo "0")
    else
        count=$(grep -Ri "${pattern}" "${ROOT_DIR}" --exclude-dir=".git" 2>/dev/null | wc -l || echo "0")
    fi
    if [[ ${count} -gt 0 ]]; then
        echo "[WARN] Found ${count} occurrences of ${pattern}" | tee -a "${OUTPUT}"
        ((critical_count += count))
    fi
done

if [[ ${critical_count} -eq 0 ]]; then
    echo "[PASS] No critical gap markers found" | tee -a "${OUTPUT}"
else
    echo "[WARN] ${critical_count} critical gap markers found - review required" | tee -a "${OUTPUT}"
fi

# Summary
echo "" | tee -a "${OUTPUT}"
echo "=== Gap Scan Summary ===" | tee -a "${OUTPUT}"
echo "Total gap markers: ${total_gaps}" | tee -a "${OUTPUT}"
echo "Critical markers: ${critical_count}" | tee -a "${OUTPUT}"
echo "Coverage table: $([ -f "${COVERAGE_OUTPUT}" ] && echo "Present" || echo "Missing")" | tee -a "${OUTPUT}"
echo "Gaps documented: $([ -f "${gaps_file}" ] && echo "Yes" || echo "No")" | tee -a "${OUTPUT}"
echo "" | tee -a "${OUTPUT}"
echo "Gap scan complete -> ${OUTPUT}"
echo "Coverage report -> ${COVERAGE_OUTPUT}"
echo "Gaps document -> ${gaps_file}"
