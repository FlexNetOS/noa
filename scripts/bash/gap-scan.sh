#!/usr/bin/env bash
set -euo pipefail

# Gap Hunt Scan Automation
# T494: Scans for TODO/FIXME and requirement gaps.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"
OUTPUT="${RESULTS_DIR}/GAP_SCAN.txt"

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

echo "Running gap scan with ${scanner}" | tee -a "${OUTPUT}"

patterns=("TODO" "FIXME" "GAP" "TBD")

for pattern in "${patterns[@]}"; do
    echo "" >> "${OUTPUT}"
    echo "## Pattern: ${pattern}" >> "${OUTPUT}"
    if [[ "${scanner}" == "rg" || "${scanner}" == "ripgrep" ]]; then
        ${scanner} -n "${pattern}" "${ROOT_DIR}" >> "${OUTPUT}" || true
    else
        ${scanner} -R -n "${pattern}" "${ROOT_DIR}" >> "${OUTPUT}" || true
    fi
done

echo "Gap scan complete -> ${OUTPUT}"
