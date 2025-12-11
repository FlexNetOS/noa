# Kernel Startup Sequence (CHK005)

1. Resolve `NOA_ROOT`, sanitize CLI/env inputs, and load `config/nkal-capabilities.json` plus `config/kernel-mounts.json`.
2. Determine target mode using precedence (CLI override → env `NOA_KERNEL_MODE` → config preferred → auto-detect `vm` → `container` → `sandbox` → `native`), then validate the action through `BoundaryValidator::enforce`.
3. Drain services with `ShutdownGuard` and block until inflight tasks reach zero.
4. Write `.kernel-switch-state.json` with status `pending`, source/target modes, capability policy reference, and resolved mounts.
5. Apply sanitized mounts for VM/container, hydrate kernel drivers, and start the selected kernel.
6. Run `StateVerifier` to confirm checkpoint target mode, mount coverage, and non-pending status; flip checkpoint status to `complete` when successful.
7. On failure, leave the checkpoint marked `pending` with the error reason to support rollback and auditing.

## Internal Tools & Paths (CHK013)

- Rust toolchain: `opt/rust/` with cargo home at `opt/rust/cargo/`.
- Go toolchain: `opt/go/` with modules cached in `cache/go/`.
- Node.js toolchain: `opt/node/` with global npm prefix `opt/node/node_modules/`.
- Python runtime: `opt/python/` with venv at `opt/venv/`.
- Security/quality tooling: binaries in `bin/` (gitleaks, trivy, grype, semgrep, golangci-lint, eslint, ruff).
- Kernel assets: VM images under `sys/kernel/vhdx/` and container definitions under `containers/`; mounts resolved from `config/kernel-mounts.json`.
