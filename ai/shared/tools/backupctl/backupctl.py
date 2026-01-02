#!/usr/bin/env python3
"""Scaffolding for backup orchestration and approval tokens."""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import tarfile
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional
from fnmatch import fnmatch
import subprocess

WORKSPACE_ROOT = Path(__file__).resolve().parents[2]
BACKUPS_ROOT = WORKSPACE_ROOT / ".backups"
MANIFEST_ROOT = BACKUPS_ROOT / "manifests"
ARCHIVE_ROOT = BACKUPS_ROOT / "archives"
TMP_ROOT = BACKUPS_ROOT / "tmp"
LEDGER_PATH = WORKSPACE_ROOT / "logs" / "backups" / "ledger.jsonl"
TOKENS_ROOT = WORKSPACE_ROOT / "logs" / "backups" / "tokens"
REMOTE_INDEX_PATH = WORKSPACE_ROOT / "logs" / "backups" / "remote-index.json"
CONFIG_PATH = WORKSPACE_ROOT / "backupctl.yml"
TOKEN_TTL_MINUTES = 30
ARCHIVE_SUFFIX = ".tar.gz"

DEFAULT_CONFIG: Dict[str, Any] = {
    "include_paths": ["."],
    "exclude_globs": [
        ".backups/**",
        "logs/backups/**",
        ".git/**",
    ],
    "replication": {
        "mode": "manual",
        "remote_path": "V:\\backups",
        "rclone_remote": "vdrive:backups",
    },
}


def load_config() -> Dict[str, Any]:
    if not CONFIG_PATH.exists():
        return DEFAULT_CONFIG
    try:
        import yaml  # type: ignore
    except Exception as exc:  # pragma: no cover
        raise SystemExit(
            "backupctl.yml detected but PyYAML is unavailable; install PyYAML or remove config"
        ) from exc
    with CONFIG_PATH.open("r", encoding="utf-8") as handle:
        data = yaml.safe_load(handle) or {}
    config = DEFAULT_CONFIG.copy()
    config.update(data)
    config.setdefault("include_paths", DEFAULT_CONFIG["include_paths"])
    config.setdefault("exclude_globs", DEFAULT_CONFIG["exclude_globs"])
    config.setdefault("replication", DEFAULT_CONFIG["replication"])
    return config


def ensure_paths() -> None:
    for path in (MANIFEST_ROOT, ARCHIVE_ROOT, TMP_ROOT, TOKENS_ROOT):
        path.mkdir(parents=True, exist_ok=True)
    LEDGER_PATH.parent.mkdir(parents=True, exist_ok=True)
    REMOTE_INDEX_PATH.parent.mkdir(parents=True, exist_ok=True)


def utcnow() -> datetime:
    return datetime.now(timezone.utc)


def isoformat(dt: datetime) -> str:
    return dt.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")


def sanitize_segment(segment: str) -> str:
    return "".join(ch if ch.isalnum() or ch in ("-", "_") else "-" for ch in segment)


def is_excluded(rel_path: Path, exclude_globs: Iterable[str]) -> bool:
    rel_posix = rel_path.as_posix()
    for pattern in exclude_globs:
        if fnmatch(rel_posix, pattern):
            return True
    return False


def iter_included_paths(include_paths: Iterable[str]) -> List[Path]:
    paths: List[Path] = []
    for entry in include_paths:
        candidate = (WORKSPACE_ROOT / entry).resolve()
        if candidate.exists():
            paths.append(candidate)
    return paths


def collect_file_metadata(include_paths: Iterable[str], exclude_globs: Iterable[str]) -> List[Dict[str, Any]]:
    metadata: List[Dict[str, Any]] = []
    include_roots = iter_included_paths(include_paths)
    for root in include_roots:
        if root.is_file():
            rel = root.relative_to(WORKSPACE_ROOT)
            if not is_excluded(rel, exclude_globs):
                metadata.append(build_file_record(root, rel))
            continue
        for filesystem_item in root.rglob("*"):
            if filesystem_item.is_dir():
                continue
            rel = filesystem_item.relative_to(WORKSPACE_ROOT)
            if is_excluded(rel, exclude_globs):
                continue
            metadata.append(build_file_record(filesystem_item, rel))
    metadata.sort(key=lambda item: item["path"])
    return metadata


def build_file_record(abs_path: Path, rel_path: Path) -> Dict[str, Any]:
    stat = abs_path.stat()
    checksum = compute_sha512(abs_path)
    return {
        "path": rel_path.as_posix(),
        "size": stat.st_size,
        "modified_at": isoformat(datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc)),
        "sha512": checksum,
        "_abs_path": str(abs_path),
    }


def compute_sha512(path: Path) -> str:
    hasher = hashlib.sha512()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def append_ledger(entry: Dict[str, Any]) -> None:
    LEDGER_PATH.parent.mkdir(parents=True, exist_ok=True)
    with LEDGER_PATH.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(entry) + "\n")


def load_remote_index() -> Dict[str, Any]:
    if not REMOTE_INDEX_PATH.exists():
        return {}
    with REMOTE_INDEX_PATH.open("r", encoding="utf-8") as handle:
        try:
            return json.load(handle)
        except json.JSONDecodeError:
            return {}


def write_remote_index(index: Dict[str, Any]) -> None:
    REMOTE_INDEX_PATH.parent.mkdir(parents=True, exist_ok=True)
    with REMOTE_INDEX_PATH.open("w", encoding="utf-8") as handle:
        json.dump(index, handle, indent=2)
        handle.write("\n")


def load_manifest(operation_id: str) -> Dict[str, Any]:
    manifest_path = MANIFEST_ROOT / f"{operation_id}.json"
    if not manifest_path.exists():
        raise SystemExit(f"Manifest not found for operation {operation_id}")
    with manifest_path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def write_manifest(operation_id: str, manifest: Dict[str, Any]) -> Path:
    manifest_path = MANIFEST_ROOT / f"{operation_id}.json"
    with manifest_path.open("w", encoding="utf-8") as handle:
        json.dump(manifest, handle, indent=2)
        handle.write("\n")
    return manifest_path


def create_archive(operation_id: str, file_records: List[Dict[str, Any]]) -> Path:
    archive_path = ARCHIVE_ROOT / f"{operation_id}{ARCHIVE_SUFFIX}"
    archive_path.parent.mkdir(parents=True, exist_ok=True)
    with tarfile.open(archive_path, "w:gz") as tar:
        for record in file_records:
            abs_path = Path(record["_abs_path"])
            if not abs_path.exists():
                continue
            rel_path = Path(record["path"])
            tar.add(abs_path, arcname=rel_path.as_posix())
    return archive_path


def cmd_run(args: argparse.Namespace) -> None:
    ensure_paths()
    config = load_config()
    now = utcnow()
    safe_target = sanitize_segment(args.target)
    safe_operation = sanitize_segment(args.operation)
    default_id = f"{safe_operation}-{safe_target}-{now.strftime('%Y%m%dT%H%M%SZ')}"
    operation_id = args.id or default_id
    manifest_path = MANIFEST_ROOT / f"{operation_id}.json"
    if manifest_path.exists() and not args.overwrite:
        raise SystemExit(
            f"Manifest already exists for operation {operation_id}; use --overwrite to replace"
        )

    file_records = collect_file_metadata(
        include_paths=config.get("include_paths", []),
        exclude_globs=config.get("exclude_globs", []),
    )
    archive_path = create_archive(operation_id, file_records)
    archive_sha = compute_sha512(archive_path)

    manifest_file_records: List[Dict[str, Any]] = []
    for record in file_records:
        record_copy = record.copy()
        record_copy.pop("_abs_path", None)
        manifest_file_records.append(record_copy)

    manifest = {
        "operation_id": operation_id,
        "operation": args.operation,
        "target": args.target,
        "generated_at": isoformat(now),
        "note": args.note,
        "status": "pending-approval",
        "archive_path": str(archive_path),
        "archive_sha512": archive_sha,
        "files": manifest_file_records,
        "config_snapshot": {
            "include_paths": config.get("include_paths"),
            "exclude_globs": config.get("exclude_globs"),
        },
    }
    manifest_path = write_manifest(operation_id, manifest)
    manifest_sha = compute_sha512(manifest_path)

    ledger_entry = {
        "operation_id": operation_id,
        "operation": args.operation,
        "target": args.target,
        "manifest_path": str(manifest_path),
        "manifest_sha512": manifest_sha,
        "archive_path": str(archive_path),
        "archive_sha512": archive_sha,
        "generated_at": manifest["generated_at"],
        "note": args.note,
        "status": "pending-approval",
    }
    append_ledger(ledger_entry)

    print("Backup manifest generated:")
    print(f"  id: {operation_id}")
    print(f"  manifest: {manifest_path}")
    print(f"  manifest sha512: {manifest_sha}")
    print(f"  archive: {archive_path}")
    print(f"  archive sha512: {archive_sha}")
    print(f"  replicate: backupctl push {operation_id}")
    print("Next: seek approver to run `backupctl approve --id` before executing destructive commands.")


def cmd_approve(args: argparse.Namespace) -> None:
    ensure_paths()
    operation_id = args.id
    manifest = load_manifest(operation_id)
    manifest_path = MANIFEST_ROOT / f"{operation_id}.json"
    manifest_sha = compute_sha512(manifest_path)
    archive_path = Path(manifest.get("archive_path", ""))
    if not archive_path.exists():
        raise SystemExit("Archive file referenced in manifest not found; rerun backupctl run")
    archive_sha = compute_sha512(archive_path)

    approved_at = utcnow()
    expires_at = approved_at + timedelta(minutes=TOKEN_TTL_MINUTES)
    token_payload = {
        "token_version": 1,
        "operation": manifest["operation"],
        "target": manifest["target"],
        "operation_id": operation_id,
        "manifest_path": str(manifest_path),
        "manifest_sha512": manifest_sha,
        "archive_path": str(archive_path),
        "archive_sha512": archive_sha,
        "approved_at": isoformat(approved_at),
        "expires_at": isoformat(expires_at),
        "approver": args.approver,
        "signature": None,  # TODO: integrate Sigstore/gitsign signature.
    }

    token_path = Path(args.token_out) if args.token_out else (TOKENS_ROOT / f"{operation_id}.token.json")
    token_path.parent.mkdir(parents=True, exist_ok=True)
    with token_path.open("w", encoding="utf-8") as handle:
        json.dump(token_payload, handle, indent=2)
        handle.write("\n")

    manifest["status"] = "approved"
    manifest["approved_at"] = token_payload["approved_at"]
    manifest["approver"] = args.approver
    write_manifest(operation_id, manifest)

    append_ledger(
        {
            "operation_id": operation_id,
            "event": "approved",
            "token_path": str(token_path),
            "approver": args.approver,
            "approved_at": token_payload["approved_at"],
            "expires_at": token_payload["expires_at"],
            "archive_sha512": archive_sha,
            "manifest_sha512": manifest_sha,
        }
    )

    print(f"Approval token written to {token_path}")
    print("Remember to deliver the token path to sandboxctl via --backup-token.")


def cmd_list(_: argparse.Namespace) -> None:
    if not LEDGER_PATH.exists():
        print("No backups recorded yet")
        return
    with LEDGER_PATH.open("r", encoding="utf-8") as handle:
        lines = handle.readlines()
    if not lines:
        print("No backups recorded yet")
        return
    print("Recent ledger entries:")
    for line in lines[-10:]:
        try:
            entry = json.loads(line)
        except json.JSONDecodeError:
            continue
        operation_id = entry.get("operation_id", "<unknown>")
        event = entry.get("event", "generated")
        status = entry.get("status")
        timestamp = entry.get("approved_at") or entry.get("generated_at")
        target = entry.get("target")
        print(f"- {operation_id} | {event or status} | target={target} | time={timestamp}")


def cmd_verify(args: argparse.Namespace) -> None:
    operation_id = args.id
    manifest_path = MANIFEST_ROOT / f"{operation_id}.json"
    if not manifest_path.exists():
        raise SystemExit(f"Manifest not found for operation {operation_id}")
    manifest_sha = compute_sha512(manifest_path)
    print(f"Manifest {manifest_path} sha512={manifest_sha}")
    print("Remote verification TBD once V:\\ replication is implemented.")


def cmd_push(args: argparse.Namespace) -> None:
    config = load_config()
    replication = config.get("replication", {})
    remote_path = replication.get("remote_path")
    rclone_remote = replication.get("rclone_remote")
    operation_id = args.operation_id
    archive_path = ARCHIVE_ROOT / f"{operation_id}{ARCHIVE_SUFFIX}"
    if not archive_path.exists():
        raise SystemExit(f"Archive not found for operation {operation_id}; run backupctl run first")
    manifest_path = MANIFEST_ROOT / f"{operation_id}.json"
    if not manifest_path.exists():
        raise SystemExit(f"Manifest not found for operation {operation_id}; run backupctl run first")
    manifest_sha = compute_sha512(manifest_path)
    archive_sha = compute_sha512(archive_path)
    print("Replication step is manual until rclone integration is implemented.")
    print("Suggested command:")
    print(
        f"  rclone copy --checksum {archive_path} "
        f"{rclone_remote or '<configure-remote>'}/{operation_id}{ARCHIVE_SUFFIX}"
    )
    print(
        f"  rclone copy --checksum {manifest_path} "
        f"{rclone_remote or '<configure-remote>'}/{operation_id}.json"
    )
    if remote_path:
        print(f"Ensure remote path `{remote_path}` is synchronised before approving destructive commands.")
    if args.exec:
        if not rclone_remote:
            raise SystemExit("rclone remote is not configured in backupctl.yml")
        for source, destination in (
            (archive_path, f"{rclone_remote}/{operation_id}{ARCHIVE_SUFFIX}"),
            (manifest_path, f"{rclone_remote}/{operation_id}.json"),
        ):
            print(f"Executing rclone copy for {source} -> {destination}")
            result = subprocess.run(
                [
                    "rclone",
                    "copy",
                    "--checksum",
                    str(source),
                    destination,
                ],
                check=False,
            )
            if result.returncode != 0:
                raise SystemExit(f"rclone copy failed for {source}")
    if args.mark:
        remote_index = load_remote_index()
        remote_index[operation_id] = {
            "operation_id": operation_id,
            "manifest_sha512": manifest_sha,
            "archive_sha512": archive_sha,
            "remote_path": remote_path,
            "recorded_at": isoformat(utcnow()),
        }
        write_remote_index(remote_index)
        append_ledger(
            {
                "operation_id": operation_id,
                "event": "replicated",
                "manifest_sha512": manifest_sha,
                "archive_sha512": archive_sha,
                "remote_path": remote_path,
                "timestamp": isoformat(utcnow()),
            }
        )
        print("Recorded remote replication in remote-index.json")
    else:
        print("Run again with --mark once replication completes to update remote index.")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Workspace backup orchestrator")
    subparsers = parser.add_subparsers(dest="command", required=True)

    run_parser = subparsers.add_parser("run", help="Execute the backup pipeline (scaffold)")
    run_parser.add_argument("--operation", required=True, help="Operation triggering the backup (e.g., destroy)")
    run_parser.add_argument("--target", required=True, help="Target sandbox or scope")
    run_parser.add_argument("--note", help="Optional note for the ledger")
    run_parser.add_argument("--id", help="Override generated operation identifier")
    run_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="Allow replacing an existing manifest with the same id",
    )
    run_parser.set_defaults(func=cmd_run)

    approve_parser = subparsers.add_parser("approve", help="Approve a backup and issue a token")
    approve_parser.add_argument("--id", required=True, help="Operation identifier to approve")
    approve_parser.add_argument("--approver", required=True, help="Approver name or handle")
    approve_parser.add_argument("--token-out", help="Custom output path for the approval token")
    approve_parser.set_defaults(func=cmd_approve)

    list_parser = subparsers.add_parser("list", help="List recent ledger entries")
    list_parser.set_defaults(func=cmd_list)

    verify_parser = subparsers.add_parser("verify", help="Recompute hashes for a manifest")
    verify_parser.add_argument("--id", required=True, help="Operation identifier to verify")
    verify_parser.set_defaults(func=cmd_verify)

    push_parser = subparsers.add_parser("push", help="Show replication instructions for an archive")
    push_parser.add_argument("operation_id", help="Operation identifier to replicate")
    push_parser.add_argument(
        "--exec",
        action="store_true",
        help="Run rclone copy commands using configured remote",
    )
    push_parser.add_argument(
        "--mark",
        action="store_true",
        help="Update remote index after replication",  # requires manual confirmation
    )
    push_parser.set_defaults(func=cmd_push)

    return parser


def main(argv: Optional[list[str]] = None) -> None:
    parser = build_parser()
    args = parser.parse_args(argv)
    args.func(args)


if __name__ == "__main__":  # pragma: no cover
    try:
        main()
    except KeyboardInterrupt:
        sys.exit(130)
