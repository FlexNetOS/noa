# Self-Containment Boundary

- Internal dependencies: portable toolchains in `opt/`, data in `data/`, shared AI assets in `ai/shared/`, kernel artifacts under `sys/kernel/`, and NKAL policies in `config/nkal-capabilities.json` plus `config/kernel-mounts.json`.
- External interactions allowed (via NKAL): host scheduler for process placement, loopback networking, hypervisor APIs for VM mode, container engine CLI for container mode, and read-only access to host-provided certificates when explicitly mounted.
- Prohibited without grants: host package managers, writes outside `NOA_ROOT`, privileged device access, unrestricted outbound network calls, and filesystem paths not declared in the mount config.
- Measurable self-contained criteria: can boot with no network using internal toolchains; all writes stay beneath `NOA_ROOT`; `.kernel-switch-state.json` present after mode changes; capability policy blocks any host capability not listed.
- Host kernel features that may be leveraged when declared: hardware virtualization extensions, cgroups/Hyper-V resource isolation, file watching primitives, and time sync services; each use must be covered by a capability grant and recorded during checkpointing.
- Consistency (CHK022): Terminology matches Constitution §3.1 definitions and the spec glossary entries for "NOA Kernel" and "Host Kernel"; deviations trigger a documentation audit before release.
