#!/usr/bin/env bash
set -euo pipefail

# Cross-platform release helper
# T412: Builds NOA core in release mode and packages artifacts.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TARGET_OS="${1:-$(uname | tr '[:upper:]' '[:lower:]')}"

echo "Building NOA (release)…"
(cd "${ROOT_DIR}/sys/core" && cargo build --release)

echo "Packaging for ${TARGET_OS}…"
"${ROOT_DIR}/scripts/package/package-noa.sh" "${TARGET_OS}"

echo "Release artifacts are under ${ROOT_DIR}/dist/${TARGET_OS}"
