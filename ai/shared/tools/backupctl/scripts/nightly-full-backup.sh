#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
BACKUPCTL="${ROOT_DIR}/tools/backupctl/backupctl.py"
TIMESTAMP="$(date -u +"%Y%m%dT%H%M%SZ")"
OPERATION_ID="scheduled-full-${TIMESTAMP}"

note="Nightly full backup"

"${BACKUPCTL}" run --operation scheduled-full --target workspace --note "${note}" --id "${OPERATION_ID}"
"${BACKUPCTL}" push "${OPERATION_ID}" --exec --mark
