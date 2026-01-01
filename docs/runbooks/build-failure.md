# Build Failure Runbook

Handle NOA build failures.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `build-failure` |
| **Trigger** | CI failure, local build error |
| **Impact** | Development blocked, deployment blocked |
| **Owner** | Development Team |
| **Escalation** | Tech Lead |
| **Severity** | S2 |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to source code
- [ ] Rust toolchain installed (1.83.0+)
- [ ] Build logs available

---

## Triage

### 1. Identify Build Type

| Build | Command | Common Issues |
|-------|---------|---------------|
| Base | `cargo build` | Missing deps |
| Full | `cargo build --features full` | API errors |
| Compression | `cargo build --features "full,compression"` | C lib issues |
| ML DevOps | `cargo build --features "full,ml-devops"` | ONNX linking |

### 2. Check Error Category

```bash
# Run build and capture errors
cargo build 2>&1 | head -50
```

| Error Pattern | Category | Solution |
|---------------|----------|----------|
| `error[E0433]: failed to resolve` | Missing import | Add `use` statement |
| `error[E0599]: no method named` | Wrong type/trait | Check trait bounds |
| `error: linking with` | Linker error | Check native deps |
| `error: could not compile` | Syntax/type | Fix code errors |

---

## Common Fixes

### Missing Dependencies

```bash
# Update Cargo.lock
cargo update

# Fetch dependencies
cargo fetch
```

### Linker Errors (Windows)

```bash
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# Set environment
$env:LIB = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.x\lib\x64"
```

### Feature Flag Issues

```bash
# Check available features
cargo read-manifest | jq '.features'

# Build with specific features
cargo build --no-default-features --features "cli,config,db"
```

### ONNX Runtime Issues (ml-devops)

```bash
# Set ORT path
$env:ORT_LIB_LOCATION = "path/to/onnxruntime"

# Or use dynamic loading
cargo build --features "full,ml-devops" --cfg ort_load_dynamic
```

---

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo build --features full` succeeds
- [ ] `cargo test` passes
- [ ] No new warnings introduced

---

## Prevention

1. Run `cargo check` before committing
2. Use pre-commit hooks
3. Keep dependencies updated
4. Test feature combinations

---

## See Also

- [agent-failure.md](agent-failure.md) — Runtime failures
- [QUICKSTART.md](../../QUICKSTART.md) — Build instructions
