#!/usr/bin/env bash
set -euo pipefail

# SC verification report generator (SC-001..SC-012)

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/test-results"
SC_DIR="${RESULTS_DIR}/sc-benchmarks"
OUTPUT="${RESULTS_DIR}/SC_REPORT.md"

mkdir -p "${SC_DIR}"
mkdir -p "${RESULTS_DIR}"

if command -v python >/dev/null 2>&1; then
python - "$SC_DIR" "$OUTPUT" <<'PY'
import datetime
import json
import sys
from pathlib import Path

sc_dir = Path(sys.argv[1])
output = Path(sys.argv[2])

rows = []
for path in sorted(sc_dir.glob("*.json")):
    try:
        data = json.loads(path.read_text())
    except Exception as exc:  # noqa: BLE001
        rows.append({
            "id": path.stem,
            "status": "parse_error",
            "duration": "n/a",
            "target": "n/a",
            "notes": f"failed to parse: {exc}",
        })
        continue

    rows.append({
        "id": data.get("id", path.stem),
        "status": str(data.get("status", "unknown")).lower(),
        "duration": data.get("duration_ms", "n/a"),
        "target": data.get("target_ms", "n/a"),
        "notes": data.get("notes", ""),
    })

total = len(rows)
passed = sum(1 for row in rows if row["status"] in {"passed", "pass"})
failed = sum(1 for row in rows if row["status"] in {"failed", "fail"})
skipped = sum(1 for row in rows if row["status"] == "skipped")
timestamp = datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")

with output.open("w", encoding="utf-8") as fh:
    fh.write("# Success Criteria Report\n\n")
    fh.write(f"- Generated: {timestamp}\n")
    fh.write(f"- Source: {sc_dir}\n")
    fh.write(f"- Summary: {passed} passed / {failed} failed / {skipped} skipped / {total} total\n\n")
    fh.write("| ID | Status | Duration (ms) | Target (ms) | Notes |\n")
    fh.write("| --- | --- | --- | --- | --- |\n")

    if not rows:
        fh.write("| (none) | pending | - | - | No benchmark results found |\n")
    else:
        for row in rows:
            notes = str(row["notes"]).replace("|", "/")
            fh.write(
                f"| {row['id']} | {row['status']} | {row['duration']} | {row['target']} | {notes} |\n"
            )

print(f"SC report created at {output}")
PY
else
  echo "Python is required to render SC_REPORT.md" >&2
  exit 1
fi

echo "SC report written to ${OUTPUT}"
