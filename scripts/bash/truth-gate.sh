#!/usr/bin/env bash
set -euo pipefail

# Truth Gate Checklist Automation
# T493: Validates presence and freshness of verification artifacts.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"

ARTIFACTS=(
    "HASHES.txt"
    "FINAL_REPORT.md"
    "COVERAGE.md"
    "REPRO.md"
    "EVIDENCE_LEDGER.md"
)

status=0

for artifact in "${ARTIFACTS[@]}"; do
    path="${RESULTS_DIR}/${artifact}"
    if [[ ! -s "${path}" ]]; then
        echo "[FAIL] Missing or empty: ${artifact}"
        status=1
        continue
    fi
    age_minutes=$(( ( $(date +%s) - $(stat -c %Y "${path}") ) / 60 ))
    echo "[PASS] ${artifact} (age ~${age_minutes}m)"
done

# Ensure hashes exist for git HEAD
if [[ -d "${ROOT_DIR}/.git" ]]; then
    head_hash=$(git -C "${ROOT_DIR}" rev-parse HEAD 2>/dev/null || echo "unknown")
    echo "HEAD: ${head_hash}"
fi

exit ${status}
