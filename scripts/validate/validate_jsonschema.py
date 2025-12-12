#!/usr/bin/env python3
"""
Validate a JSON or YAML file against a JSON Schema (schema may be JSON or YAML).

Usage:
  python scripts/validate/validate_jsonschema.py <data_path> <schema_path>
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


def _load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def _load_yaml(path: Path):
    import yaml  # type: ignore

    return yaml.safe_load(path.read_text(encoding="utf-8"))


def load_data(path: Path):
    ext = path.suffix.lower()
    if ext in (".json",):
        return _load_json(path)
    if ext in (".yaml", ".yml"):
        return _load_yaml(path)
    raise SystemExit(f"Unsupported data file extension: {path}")


def load_schema(path: Path):
    ext = path.suffix.lower()
    if ext in (".json",):
        return _load_json(path)
    if ext in (".yaml", ".yml"):
        return _load_yaml(path)
    raise SystemExit(f"Unsupported schema file extension: {path}")


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(__doc__.strip(), file=sys.stderr)
        return 2

    data_path = Path(argv[1]).resolve()
    schema_path = Path(argv[2]).resolve()

    if not data_path.exists():
        print(f"Data file not found: {data_path}", file=sys.stderr)
        return 2
    if not schema_path.exists():
        print(f"Schema file not found: {schema_path}", file=sys.stderr)
        return 2

    try:
        data = load_data(data_path)
        schema = load_schema(schema_path)
    except Exception as e:
        print(f"Parse error: {e}", file=sys.stderr)
        return 1

    try:
        from jsonschema import Draft202012Validator, Draft7Validator  # type: ignore

        schema_id = (schema.get("$schema") if isinstance(schema, dict) else None) or ""
        if "2020-12" in schema_id:
            validator = Draft202012Validator(schema)
        else:
            validator = Draft7Validator(schema)

        errors = sorted(validator.iter_errors(data), key=lambda e: list(e.path))
        if errors:
            print(f"Schema validation failed: {data_path}", file=sys.stderr)
            for err in errors[:50]:
                where = "." + ".".join([str(p) for p in err.path]) if err.path else "."
                print(f"- {where}: {err.message}", file=sys.stderr)
            if len(errors) > 50:
                print(f"... {len(errors) - 50} more errors", file=sys.stderr)
            return 1

        return 0
    except ImportError as e:
        print(
            "Missing python deps for schema validation. Install:\n"
            "  pip install pyyaml jsonschema\n"
            "or (repo-wide):\n"
            "  pip install -r config/requirements.txt\n",
            file=sys.stderr,
        )
        print(f"ImportError: {e}", file=sys.stderr)
        return 2
    except Exception as e:
        print(f"Validation error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))


