#!/usr/bin/env bash
# Run staged security gates (report-only by default).
#
# Strict mode:
#   STRICT=true bash scripts/validate/security-gates.sh

set -euo pipefail

NOA_ROOT="${NOA_ROOT:-}"
STRICT="${STRICT:-false}"

if [[ -z "$NOA_ROOT" ]]; then
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  NOA_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

warn() { echo "WARN: $*" >&2; }
fail_or_warn() {
  if [[ "$STRICT" == "true" ]]; then
    echo "ERROR: $*" >&2
    exit 2
  fi
  warn "$*"
}

try_run() {
  local name="$1"
  shift
  if "$@"; then
    echo "OK: $name"
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

echo "Security gates (staged) for: $NOA_ROOT"

if command -v gitleaks >/dev/null 2>&1; then
  try_run "gitleaks" bash -lc "cd \"$NOA_ROOT\" && gitleaks detect --source . --redact --no-banner"
else
  fail_or_warn "gitleaks not found"
fi

if command -v trivy >/dev/null 2>&1; then
  try_run "trivy(fs)" bash -lc "cd \"$NOA_ROOT\" && trivy fs --severity HIGH,CRITICAL --ignore-unfixed --exit-code 1 ."
else
  fail_or_warn "trivy not found"
fi

if command -v grype >/dev/null 2>&1; then
  try_run "grype(dir)" bash -lc "cd \"$NOA_ROOT\" && grype dir:. --fail-on high"
else
  fail_or_warn "grype not found"
fi

if command -v semgrep >/dev/null 2>&1; then
  try_run "semgrep(p/default)" bash -lc "cd \"$NOA_ROOT\" && semgrep --config p/default"
else
  fail_or_warn "semgrep not found"
fi

echo "Done."


