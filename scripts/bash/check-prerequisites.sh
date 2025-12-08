#!/bin/bash
#
# Shim for legacy path used by /tasks and /analyze commands.
# Delegates to the authoritative Unix checker at init/check-prereqs.sh.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TARGET="$REPO_ROOT/init/check-prereqs.sh"

if [[ ! -f "$TARGET" ]]; then
  echo "Missing target script: $TARGET" >&2
  exit 1
fi

exec "$TARGET" "$@"

