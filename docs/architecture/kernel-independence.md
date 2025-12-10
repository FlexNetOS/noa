# Kernel Independence - Selection Logic & Enforcement

- Precedence: explicit CLI/flag override > `NOA_KERNEL_MODE` env > `config` preferred mode > auto-detect (`vm` → `container` → `sandbox` → `native`); never auto-escalate without a capability grant.
- Boundary validation: every action crossing NKAL uses `BoundaryValidator::enforce` with `config/nkal-capabilities.json`; missing grants or explicit denies block the call.
- Sanitization & verification: inputs are scrubbed via `sys/core/src/nkal/sanitize.rs` before dispatch; outbound responses must pass `sys/core/src/nkal/verify.rs` to prevent secret leakage or empty responses.
- Mount enforcement: VM/container modes hydrate shared volumes from `config/kernel-mounts.json` and persist the resolved set into `.kernel-switch-state.json` alongside the source/target modes.
- Checkpoint contract: `.kernel-switch-state.json` is written on every mode change with status, mounts, and capability policy reference; `StateVerifier` must confirm status != pending before workloads resume.
- Graceful transitions: `ShutdownGuard` requires drained services (no inflight tasks) prior to switching; otherwise mode change is rejected.
- Naming consistency: accepted modes are `native`, `vm`, `container`, `sandbox`; documentation and tooling use these exact tokens to satisfy CHK012.
- Principle alignment (3.1 / FR-091-094): self-contained operation is asserted by denying host-only capabilities unless granted in the capability policy; NKAL remains the contract between NOA and the host kernel and is reviewed each release for FR-091-094 compliance (CHK011).
- Consistency verification (CHK010): each release validates this document against Constitution §3.1 and FR-091-094, recording the validation outcome in the kernel switch checkpoint reason.

## NKAL Interface Contracts (CHK031)

- Boundary validation: `sys/core/src/nkal/boundary.rs` enforces capability grants per mode/provider using `config/nkal-capabilities.json`.
- Input sanitization: `sys/core/src/nkal/sanitize.rs` strips control characters, blocks traversal/command chaining, and rejects blocked inputs before dispatch.
- Output verification: `sys/core/src/nkal/verify.rs` prevents empty responses and secret-like payloads from crossing the boundary.
- Checkpoint/state: `sys/core/src/nkal/checkpoint.rs` writes `.kernel-switch-state.json`; `sys/core/src/nkal/state.rs` validates target mode, mounts, and status before workloads resume.
- Shutdown: `sys/core/src/nkal/shutdown.rs` blocks mode transitions until drains complete; failure leaves checkpoint as pending for audit/rollback.
