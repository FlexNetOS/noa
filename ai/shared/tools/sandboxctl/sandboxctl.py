#!/usr/bin/env python3
"""Initial scaffolding for sandbox lifecycle management."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional

WORKSPACE_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_REGISTRY = WORKSPACE_ROOT / "workspace.yaml"
BACKUP_LEDGER = WORKSPACE_ROOT / "logs" / "backups"
REMOTE_INDEX_PATH = BACKUP_LEDGER / "remote-index.json"
TOKEN_TTL_MINUTES = 30


@dataclass
class Sandbox:
    name: str
    path: Path
    metadata: Dict[str, Any]


def load_registry(registry_path: Path = DEFAULT_REGISTRY) -> List[Sandbox]:
    """Load sandbox metadata; keeps implementation minimal until registry schema stabilises."""
    if not registry_path.exists():
        return []
    try:
        import yaml  # type: ignore
    except Exception as exc:  # pragma: no cover
        raise SystemExit(
            "workspace.yaml found but PyYAML is not installed; install it or adjust registry loader"
        ) from exc
    with registry_path.open("r", encoding="utf-8") as handle:
        data = yaml.safe_load(handle) or {}
    sandboxes = []
    for entry in data.get("sandboxes", []):
        name = entry.get("name")
        location = entry.get("path")
        if not name or not location:
            continue
        sandboxes.append(
            Sandbox(
                name=name,
                path=(WORKSPACE_ROOT / location).resolve(),
                metadata=entry,
            )
        )
    return sandboxes


def read_backup_token(token_path: Path) -> Dict[str, Any]:
    if not token_path.exists():
        raise SystemExit(f"Backup token not found: {token_path}")
    with token_path.open("r", encoding="utf-8") as handle:
        try:
            payload = json.load(handle)
        except json.JSONDecodeError as exc:  # pragma: no cover
            raise SystemExit(f"Invalid backup token JSON: {token_path}") from exc
    return payload


def ensure_backup_token(operation: str, target: str, token: Optional[str]) -> None:
    if not token:
        raise SystemExit("Destructive operations require --backup-token pointing to an approval token")
    payload = read_backup_token(Path(token))
    token_operation = payload.get("operation")
    token_target = payload.get("target")
    expires_at = payload.get("expires_at")
    manifest_sha = payload.get("manifest_sha512")
    archive_sha = payload.get("archive_sha512")
    operation_id = payload.get("operation_id")
    if token_operation != operation:
        raise SystemExit(f"Backup token operation mismatch: expected {operation}, got {token_operation}")
    if token_target != target:
        raise SystemExit(f"Backup token target mismatch: expected {target}, got {token_target}")
    if not manifest_sha:
        raise SystemExit("Backup token missing manifest hash")
    if not archive_sha:
        raise SystemExit("Backup token missing archive hash")
    if not operation_id:
        raise SystemExit("Backup token missing operation identifier")
    if not expires_at:
        raise SystemExit("Backup token missing expiry timestamp")
    try:
        expiry = datetime.fromisoformat(expires_at.replace("Z", "+00:00"))
    except ValueError as exc:  # pragma: no cover
        raise SystemExit("Backup token expiry timestamp invalid") from exc
    now = datetime.now(timezone.utc)
    if now > expiry:
        raise SystemExit("Backup token expired; run backupctl again")
    # Placeholder for manifest lookup / Sigstore verification.
    # TODO: validate manifest_sha exists in V:\\ index and verify signature.
    ensure_remote_replication(operation_id, manifest_sha, archive_sha)


def ensure_remote_replication(operation_id: str, manifest_sha: str, archive_sha: str) -> None:
    if not REMOTE_INDEX_PATH.exists():
        raise SystemExit(
            "Remote replication index missing; run `backupctl push --mark` before destructive operations"
        )
    with REMOTE_INDEX_PATH.open("r", encoding="utf-8") as handle:
        try:
            index = json.load(handle)
        except json.JSONDecodeError as exc:  # pragma: no cover
            raise SystemExit("Remote replication index is corrupted; regenerate via backupctl push --mark") from exc
    entry = index.get(operation_id)
    if not entry:
        raise SystemExit(
            f"Operation {operation_id} not found in remote index; run `backupctl push --mark {operation_id}`"
        )
    if entry.get("manifest_sha512") != manifest_sha:
        raise SystemExit("Manifest hash mismatch between token and remote index; investigate before proceeding")
    if entry.get("archive_sha512") != archive_sha:
        raise SystemExit("Archive hash mismatch between token and remote index; investigate before proceeding")


def cmd_init(args: argparse.Namespace) -> None:
    sandbox_name = args.name
    target_dir = (WORKSPACE_ROOT / "envs" / sandbox_name).resolve()
    if target_dir.exists():
        print(f"Sandbox already exists at {target_dir}")
        return
    target_dir.mkdir(parents=True, exist_ok=True)
    print(f"Created sandbox directory: {target_dir}")
    # TODO: scaffold from templates and register sandbox entry.


def cmd_list(_: argparse.Namespace) -> None:
    sandboxes = load_registry()
    if not sandboxes:
        print("No sandboxes registered yet")
        return
    for item in sandboxes:
        print(f"{item.name}\t{item.path}")


def cmd_destroy(args: argparse.Namespace) -> None:
    sandbox_name = args.name
    ensure_backup_token("destroy", sandbox_name, args.backup_token)
    target_dir = (WORKSPACE_ROOT / "envs" / sandbox_name).resolve()
    if not target_dir.exists():
        raise SystemExit(f"Sandbox not found: {sandbox_name}")
    if args.dry_run:
        print(f"[dry-run] Would destroy sandbox at {target_dir}")
        return
    # TODO: teardown containers/VMs before filesystem removal.
    # For now we only emit the intended action; actual deletion logic is deferred until backup pipeline is live.
    print(f"Destroying sandbox at {target_dir} (deferred)")


def cmd_prune(args: argparse.Namespace) -> None:
    ensure_backup_token("prune", "workspace", args.backup_token)
    if args.dry_run:
        print("[dry-run] Would prune stale sandboxes")
        return
    # TODO: implement pruning logic once retention policy defined.
    print("Pruning stale sandboxes (deferred)")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Manage sandbox environments")
    subparsers = parser.add_subparsers(dest="command", required=True)

    init_parser = subparsers.add_parser("init", help="Create a new sandbox")
    init_parser.add_argument("name", help="Sandbox name")
    init_parser.set_defaults(func=cmd_init)

    list_parser = subparsers.add_parser("list", help="List registered sandboxes")
    list_parser.set_defaults(func=cmd_list)

    destroy_parser = subparsers.add_parser("destroy", help="Destroy a sandbox (requires backup token)")
    destroy_parser.add_argument("name", help="Sandbox name")
    destroy_parser.add_argument("--backup-token", dest="backup_token", help="Path to approved backup token")
    destroy_parser.add_argument("--dry-run", action="store_true", help="Print actions without changing state")
    destroy_parser.set_defaults(func=cmd_destroy)

    prune_parser = subparsers.add_parser("prune", help="Prune stale sandboxes (requires backup token)")
    prune_parser.add_argument("--backup-token", dest="backup_token", help="Path to approved backup token")
    prune_parser.add_argument("--dry-run", action="store_true", help="Print actions without changing state")
    prune_parser.set_defaults(func=cmd_prune)

    return parser


def main(argv: Optional[List[str]] = None) -> None:
    parser = build_parser()
    args = parser.parse_args(argv)
    args.func(args)


if __name__ == "__main__":  # pragma: no cover
    try:
        main()
    except KeyboardInterrupt:
        sys.exit(130)
