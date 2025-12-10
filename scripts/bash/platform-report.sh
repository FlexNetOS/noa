#!/usr/bin/env bash
set -euo pipefail

# Platform comparison report generator (SC-009 helper)

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"
OUTPUT="${RESULTS_DIR}/PLATFORM_REPORT.md"
SC_DIR="${RESULTS_DIR}/sc-benchmarks"

mkdir -p "${RESULTS_DIR}"

platform_name() {
    if command -v uname >/dev/null 2>&1; then
        uname -a
    elif command -v ver >/dev/null 2>&1; then
        ver
    else
        echo "unknown"
    fi
}

tool_version() {
    local cmd="$1"
    if command -v "${cmd}" >/dev/null 2>&1; then
        "${cmd}" --version 2>/dev/null | head -n 1
    else
        echo "not installed"
    fi
}

ARCH="$(uname -m 2>/dev/null || echo unknown)"

{
    echo "# Platform Comparison Report"
    echo ""
    echo "- Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    echo "- Host: $(platform_name)"
    echo "- Arch: ${ARCH}"
    echo ""
    echo "## Toolchain Versions"
    echo "- Rust: $(tool_version rustc)"
    echo "- Go: $(tool_version go)"
    echo "- Node: $(tool_version node)"
    echo "- Python: $(tool_version python)"
    echo "- Protoc: $(tool_version protoc)"
    echo ""
    echo "## Success Criteria Benchmarks"
    if [ -d "${SC_DIR}" ]; then
        if command -v python >/dev/null 2>&1; then
            python - "$SC_DIR" <<'PY'
import json
import os
import sys
from pathlib import Path

sc_dir = Path(sys.argv[1])
files = sorted(sc_dir.glob("*.json"))
if not files:
    print("- No benchmark results found in", sc_dir)
    sys.exit(0)

print(f"- Found {len(files)} benchmark result(s) in {sc_dir}")
for path in files:
    try:
        data = json.loads(path.read_text())
    except Exception as exc:  # noqa: BLE001
        print(f"  - {path.name}: failed to parse ({exc})")
        continue
    status = data.get("status", "unknown")
    duration = data.get("duration_ms", "n/a")
    target = data.get("target_ms", "n/a")
    print(f"  - {data.get('id', path.stem)}: {status} ({duration}ms vs target {target}ms)")
PY
        else
            echo "- Python not available; cannot summarize SC benchmarks"
        fi
    else
        echo "- SC benchmark directory not found (${SC_DIR})"
    fi
} > "${OUTPUT}"

echo "Platform report written to ${OUTPUT}"
