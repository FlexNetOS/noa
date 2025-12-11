# NOA Kernel Configuration

This directory contains configuration files for the NOA Kernel Abstraction Layer (NKAL).

## Files

| File | Description |
|------|-------------|
| `nkal.json` | Main NKAL configuration with mode selection policy |
| `vm.json` | VM mode configuration (Hyper-V, KVM, VirtFW) |
| `container.json` | Container mode configuration (Docker, Podman) |
| `sandbox.json` | Sandbox mode configuration (Windows Sandbox, bubblewrap) |
| `modules.json` | Kernel modules configuration per platform |

## Constitutional Reference

These configs implement **§4.11 Kernel Independence Policy** from the NOA Constitution:

- **Kernel Independence Modes** (selection precedence: VM > Container > Sandbox > Native)
- **Kernel Selection Policy** (FR-159, FR-160): Default mode is Native; escalation to isolated modes occurs automatically based on triggers

## Kernel Mode Precedence

| Priority | Mode | Isolation | Performance | Use Case |
|----------|------|-----------|-------------|----------|
| 1 | VM | Maximum | -15% | Untrusted code, external data |
| 2 | Container | High | -5% | Service isolation |
| 3 | Sandbox | Medium | -2% | Ephemeral tasks |
| 4 | Native | None | 0% | Trusted operations (default) |

## Runtime Parameters

Runtime kernel parameters are stored in `sys/kernel/params/current.json`.

Managed via:
```bash
# Linux/macOS
./scripts/noa-kernel-params set <param> <value>
./scripts/noa-kernel-params get <param>
./scripts/noa-kernel-params list-modes

# Windows
.\scripts\noa-kernel-params.ps1 -Action set -Param <param> -Value <value>
```

## Mode Selection

```bash
# Check available modes
./scripts/noa-kernel-params check-availability

# Select mode manually
./scripts/noa-kernel-params select-mode vm

# Auto-select based on policy
./scripts/noa-kernel-params select-mode auto

# Auto-select preferring isolated modes
./scripts/noa-kernel-params select-mode auto-isolated
```

## Related Files

- `/config/kernel-selection-policy.json` - Full policy with escalation triggers
- `/sys/kernel/params/current.json` - Runtime parameters
- `/sys/kernel/images/` - VM image builders
- `/sys/core/src/kernel/nkal.rs` - Rust NKAL implementation

## Jupyter Kernel (Sandbox-Plane)

NOA uses conda-forge ipykernel + nb_conda_kernels for portable notebook execution:

| Component | Location |
|-----------|----------|
| Conda prefix | `/opt/conda` |
| Kernel spec | `~/.local/share/jupyter/kernels/noa-base` |
| Config | `/sandbox-plane/config/jupyter.json` |
| Environment | `/sandbox-plane/config/environment.yaml` |

```bash
# List available kernels
jupyter kernelspec list

# Verify NOA kernel
/opt/conda/bin/python -c "import ipykernel; print(ipykernel.__version__)"
```

**Note**: NKAL Kernel Modes (VM/Container/Sandbox/Native) are for system-level isolation, separate from Jupyter notebook kernels.
