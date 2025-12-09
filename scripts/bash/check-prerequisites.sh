#!/bin/bash
#
# Shim for legacy path used by /tasks and /analyze commands.
# Delegates to the authoritative Unix checker at init/check-prereqs.sh.
#
# Supported arguments (passed through to init/check-prereqs.sh):
#   --json          Output results in JSON format
#   --paths-only    Return feature directory paths for spec-kit commands
#   --require-tasks Fail if tasks.md doesn't exist (for /analyze)
#   --include-tasks Include tasks.md in output (implied by --paths-only)
#   --allow-global  Permit detection of system-wide tools

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET="$REPO_ROOT/init/check-prereqs.sh"

if [[ ! -f "$TARGET" ]]; then
  echo "Missing target script: $TARGET" >&2
  exit 1
fi

exec "$TARGET" "$@"

