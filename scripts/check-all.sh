#!/usr/bin/env bash
# Single-command repo verification (config + security + quality).
#
# Strict mode:
#   STRICT=true bash scripts/check-all.sh

set -euo pipefail

NOA_ROOT="${NOA_ROOT:-}"
STRICT="${STRICT:-false}"

if [[ -z "$NOA_ROOT" ]]; then
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  NOA_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
fi

export NOA_ROOT
export STRICT

echo "check-all for: $NOA_ROOT"

bash "$NOA_ROOT/scripts/validate/validate-configs.sh"
bash "$NOA_ROOT/scripts/validate/security-gates.sh"
bash "$NOA_ROOT/scripts/validate/quality-gates.sh"

echo "Done."


