#!/usr/bin/env bash
set -euo pipefail

# NOA installer (Unix)
# T413: Installs a packaged NOA binary from dist/ into a target prefix.

ARCHIVE="${1:-}"
PREFIX="${2:-${HOME}/.local/noa}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

detect_archive() {
    if [[ -n "${ARCHIVE}" ]]; then
        echo "${ARCHIVE}"
        return
    fi
    platform="$(uname | tr '[:upper:]' '[:lower:]')"
    candidate="${ROOT_DIR}/dist/${platform}/noa-${platform}.tar.gz"
    echo "${candidate}"
}

archive_path="$(detect_archive)"

if [[ ! -f "${archive_path}" ]]; then
    echo "Archive not found: ${archive_path}" >&2
    echo "Run scripts/bash/release.sh first or provide an explicit archive path." >&2
    exit 1
fi

echo "Installing NOA to ${PREFIX}"
mkdir -p "${PREFIX}/bin"
tar -xzf "${archive_path}" -C "${PREFIX}/bin" --strip-components=0

echo "NOA installed. Add to PATH:"
echo "  export PATH=\"${PREFIX}/bin:\${PATH}\""
