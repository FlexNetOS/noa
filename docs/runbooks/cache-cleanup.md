# Cache Cleanup Runbook

Clean model, pip, and npm caches.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `cache-cleanup` |
| **Trigger** | Monthly schedule, disk space alert |
| **Impact** | Next download may be slower |
| **Owner** | Platform Team |
| **Escalation** | On-call SRE |
| **Schedule** | Monthly on 1st at 04:00 UTC |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to NOA host machine
- [ ] NOA services stopped (recommended)
- [ ] Disk space information

---

## Cache Locations

| Cache | Path | Purpose |
|-------|------|---------|
| Models | `cache/models/` | GGUF, ONNX models |
| Hugging Face | `cache/huggingface/` | HF model cache |
| Pip | `cache/pip/` | Python packages |
| npm | `cache/npm/` | Node packages |
| Rust | `cache/rust/` | Cargo cache |
| Ollama | `cache/ollama/` | Ollama models |

---

## Steps

### 1. Check Cache Sizes

```powershell
# Windows
Get-ChildItem -Path "N:\noa\cache" -Recurse | 
    Group-Object Directory | 
    Select-Object Name, @{N='Size(MB)';E={[math]::Round(($_.Group | Measure-Object Length -Sum).Sum/1MB, 2)}}
```

```bash
# Linux/macOS
du -sh ~/noa/cache/*
```

### 2. Clean Unused Models

```bash
# List models with last access time
noa model list --show-access

# Remove models not used in 30 days
noa model cleanup --older-than 30d
```

### 3. Clean Pip Cache

```bash
# Clean pip cache
pip cache purge

# Or manually
rm -rf cache/pip/*
```

### 4. Clean npm Cache

```bash
# Clean npm cache
npm cache clean --force

# Or manually
rm -rf cache/npm/*
```

### 5. Clean Rust Target

```bash
# Remove old build artifacts
cargo clean --release

# Or clean entire target
rm -rf cache/rust/target
```

### 6. Verify Cleanup

```bash
# Check disk usage after
df -h ~/.noa

# Verify caches are functional
noa model list
```

---

## Verification

- [ ] Disk space recovered
- [ ] Active models still present
- [ ] NOA starts successfully
- [ ] No missing dependency errors

---

## See Also

- [database-backup.md](database-backup.md) — Database backup
- [log-rotation.md](log-rotation.md) — Log cleanup
