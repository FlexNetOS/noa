#!/usr/bin/env bash
set -euo pipefail

PLATFORM="${1:-linux}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
DIST_ROOT="${REPO_ROOT}/dist/${PLATFORM}"

mkdir -p "${DIST_ROOT}"

BINARY_NAME="noa"
if [[ "${PLATFORM}" == "windows" ]]; then
  BINARY_NAME="noa.exe"
fi

SOURCE="${REPO_ROOT}/sys/core/target/release/${BINARY_NAME}"
if [[ ! -f "${SOURCE}" ]]; then
  echo "Release binary not found at ${SOURCE}. Run 'cargo build --release' in sys/core first." >&2
  exit 1
fi

cp "${SOURCE}" "${DIST_ROOT}/${BINARY_NAME}"
cat > "${DIST_ROOT}/README.txt" <<EOF
NOA cross-platform build artifact
Platform: ${PLATFORM}
Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
Source: sys/core/target/release/${BINARY_NAME}
EOF

tar -czf "${DIST_ROOT}/noa-${PLATFORM}.tar.gz" -C "${DIST_ROOT}" "${BINARY_NAME}" "README.txt"
echo "Packaged artifact at ${DIST_ROOT}/noa-${PLATFORM}.tar.gz"
