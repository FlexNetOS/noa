#!/usr/bin/env bash
set -euo pipefail

# Generate SHA-256 hashes for key artifacts.
# T486/T487: Writes hashes to test-results/HASHES.txt

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"
OUTPUT_FILE="${RESULTS_DIR}/HASHES.txt"

DEFAULT_TARGETS=(
    "sys/core/src"
    "ai/agents"
    "ai/shared"
    "scripts"
    "config"
    "docs"
    "ruler"
    "README.md"
)

hash_tool() {
    if command -v sha256sum >/dev/null 2>&1; then
        echo "sha256sum"
    elif command -v shasum >/dev/null 2>&1; then
        echo "shasum -a 256"
    else
        echo ""
    fi
}

HASH_BIN="$(hash_tool)"
if [[ -z "${HASH_BIN}" ]]; then
    echo "No SHA-256 tool found (sha256sum or shasum). Install coreutils." >&2
    exit 1
fi

mkdir -p "${RESULTS_DIR}"
> "${OUTPUT_FILE}"

echo "Generating hashes into ${OUTPUT_FILE}"

for target in "${@:-${DEFAULT_TARGETS[@]}}"; do
    path="${ROOT_DIR}/${target}"
    if [[ -d "${path}" ]]; then
        find "${path}" -type f -print0 | LC_ALL=C sort -z | while IFS= read -r -d '' file; do
            rel_path="${file#"${ROOT_DIR}/"}"
            hash_val=$( ${HASH_BIN} "${file}" | awk '{print $1}' )
            printf "%s  %s\n" "${hash_val}" "${rel_path}" >> "${OUTPUT_FILE}"
        done
    elif [[ -f "${path}" ]]; then
        hash_val=$( ${HASH_BIN} "${path}" | awk '{print $1}' )
        printf "%s  %s\n" "${hash_val}" "${target}" >> "${OUTPUT_FILE}"
    else
        echo "Skipping missing target: ${target}" >&2
    fi
done

echo "Hash generation complete."
