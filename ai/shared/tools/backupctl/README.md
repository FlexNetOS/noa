# backupctl

`backupctl` orchestrates workspace backups to enforce the "backups-first" policy. It produces hash-verified manifests, packages archives, and issues approval tokens required by `sandboxctl` before any destructive action.

## Command Overview
- `backupctl run --operation <op> --target <name>`: Execute the full backup pipeline (collect, archive to `.tar.gz`, hash, ledger entry). Produces a manifest awaiting approval.
- `backupctl push <operation-id> [--exec] --mark`: Run (or print) the `rclone copy --checksum` commands that sync artifacts and record hashes in the remote index.
- `backupctl approve --id <op-id> --approver <name>`: Validate the manifest/archive hashes and emit a signed approval token that `sandboxctl` consumes.
- `backupctl list`: Display recent backups from the ledger file.
- `backupctl verify --id <op-id>`: Recalculate hashes against the stored manifest (remote check pending).

## File Layout
- Archives: `.backups/archives/`
- Manifests: `.backups/manifests/`
- Tokens, ledger, remote index, and approval tokens: `logs/backups/`
- Scheduled scripts: `tools/backupctl/scripts/` (invoked by systemd timers for nightly/hourly runs)

Implementation is staged; destructive actions remain blocked until backups are fully operational. See `docs/rsd/backup-procedure.md` for the detailed workflow.

Configuration lives in `backupctl.yml` and defines include/exclude globs alongside replication targets. Defaults avoid backing up the `.backups/` directory to prevent recursion.
