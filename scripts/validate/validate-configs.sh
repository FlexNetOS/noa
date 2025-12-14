#!/usr/bin/env bash
# Validate committed NOA config files (parse + basic policy + schema when available).

set -euo pipefail

NOA_ROOT="${NOA_ROOT:-}"
STRICT="${STRICT:-false}"

if [[ -z "$NOA_ROOT" ]]; then
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  NOA_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

CONFIG_DIR="$NOA_ROOT/config"
SCHEMAS_DIR="$CONFIG_DIR/schemas"
PY_VALIDATOR="$NOA_ROOT/scripts/validate/validate_jsonschema.py"

if [[ ! -d "$CONFIG_DIR" ]]; then
  echo "config/ not found at: $CONFIG_DIR" >&2
  exit 2
fi

if [[ ! -d "$SCHEMAS_DIR" ]]; then
  echo "config/schemas/ not found at: $SCHEMAS_DIR" >&2
  exit 2
fi

if [[ ! -f "$PY_VALIDATOR" ]]; then
  echo "Missing validator: $PY_VALIDATOR" >&2
  exit 2
fi

errors=()
warnings=()

resolve_schema_path() {
  local schema_uri="$1"
  local file
  file="$(basename "$schema_uri" 2>/dev/null || true)"
  if [[ -z "$file" ]]; then
    return 1
  fi
  local candidate="$SCHEMAS_DIR/$file"
  if [[ -f "$candidate" ]]; then
    echo "$candidate"
    return 0
  fi
  return 1
}

python - <<'PY' >/dev/null 2>&1 || {
import sys
sys.exit(0)
PY

# Find configs (excluding schemas/)
while IFS= read -r -d '' f; do
  rel="${f#"$NOA_ROOT"/}"
  ext="${f##*.}"

  if [[ "$ext" == "json" ]]; then
    # Parse + policy checks via python (works without jq dependency)
    schema_uri="$(python - "$f" <<'PY'
import json, sys
from pathlib import Path
p=Path(sys.argv[1])
o=json.loads(p.read_text(encoding="utf-8"))
print(o.get("$schema","") or "")
PY
)"

    version="$(python - "$f" <<'PY'
import json, sys
from pathlib import Path
p=Path(sys.argv[1])
o=json.loads(p.read_text(encoding="utf-8"))
print(o.get("version","") or "")
PY
)"

    if [[ -z "$version" ]]; then
      errors+=("CFG-001 missing version: $rel")
    fi

    if [[ -n "$schema_uri" ]]; then
      if schema_path="$(resolve_schema_path "$schema_uri")"; then
        python "$PY_VALIDATOR" "$f" "$schema_path" >/dev/null || errors+=("Schema validation failed: $rel vs ${schema_path#"$NOA_ROOT"/}")
      else
        msg="CFG-002 ${rel##*/} declares \$schema ($schema_uri) but no local schema found under config/schemas/"
        if [[ "$STRICT" == "true" ]]; then
          errors+=("$msg")
        else
          warnings+=("$msg")
        fi
      fi
    fi
  else
    # YAML parseability check
    python - "$f" <<'PY' >/dev/null || exit_code=$?
import sys
from pathlib import Path
import yaml
p=Path(sys.argv[1])
yaml.safe_load(p.read_text(encoding="utf-8"))
PY
    if [[ "${exit_code:-0}" -ne 0 ]]; then
      errors+=("Parse error: $rel")
      unset exit_code
    fi
  fi
done < <(find "$CONFIG_DIR" -type f \( -name "*.json" -o -name "*.yaml" -o -name "*.yml" \) ! -path "$SCHEMAS_DIR/*" ! -name "README.md" ! -name "requirements.txt" -print0)

echo "Config validation completed for: $NOA_ROOT"
if (( ${#warnings[@]} > 0 )); then
  echo "Warnings:"
  for w in "${warnings[@]}"; do
    echo "  - $w"
  done
fi
if (( ${#errors[@]} > 0 )); then
  echo "Errors:" >&2
  for e in "${errors[@]}"; do
    echo "  - $e" >&2
  done
  exit 1
fi

echo "OK: configs validated"


