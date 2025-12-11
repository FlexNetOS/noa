#!/usr/bin/env bash
# Run staged quality gates (report-only by default).
#
# Strict mode:
#   STRICT=true bash scripts/validate/quality-gates.sh

set -euo pipefail

NOA_ROOT="${NOA_ROOT:-}"
STRICT="${STRICT:-false}"

if [[ -z "$NOA_ROOT" ]]; then
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  NOA_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

warn() { echo "WARN: $*" >&2; }
ok() { echo "OK: $*"; }

try_run() {
  local name="$1"
  shift
  if "$@"; then
    ok "$name"
    return 0
  fi
  local rc=$?
  if [[ "$STRICT" == "true" ]]; then
    echo "ERROR: $name failed (exit $rc)" >&2
    exit "$rc"
  fi
  warn "$name failed (exit $rc)"
  return 0
}

echo "Quality gates (staged) for: $NOA_ROOT"

try_run "config-validate" bash "$NOA_ROOT/scripts/validate/validate-configs.sh"

# Prefer PowerShell provider validator (bash variant historically drifted)
if command -v pwsh >/dev/null 2>&1; then
  try_run "provider-config-validate" pwsh -NoLogo -NoProfile -File "$NOA_ROOT/scripts/bootstrap/verify/validate-provider-configs.ps1" -NoaRoot "$NOA_ROOT"
else
  if [[ "$STRICT" == "true" ]]; then
    echo "ERROR: pwsh not found; cannot run provider-config-validate" >&2
    exit 2
  fi
  warn "pwsh not found; skipping provider-config-validate"
fi

if command -v cargo >/dev/null 2>&1; then
  try_run "rustfmt(sys/core)" bash -lc "cd \"$NOA_ROOT/sys/core\" && cargo fmt --check"
  try_run "clippy(sys/core)" bash -lc "cd \"$NOA_ROOT/sys/core\" && cargo clippy -- -D warnings"
else
  if [[ "$STRICT" == "true" ]]; then
    echo "ERROR: cargo not found; cannot run rust quality gates" >&2
    exit 2
  fi
  warn "cargo not found; skipping rust quality gates"
fi

echo "Done."


