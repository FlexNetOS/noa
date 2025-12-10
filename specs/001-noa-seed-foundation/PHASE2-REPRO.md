# Phase 2 Reproduction Guide: Database & Storage Infrastructure

**Phase**: Phase 2 - Foundational Database & Storage Infrastructure
**Date**: 2025-12-10
**Purpose**: Exact environment and commands for reproducing Phase 2 implementation

---

## Environment Requirements

### Operating System
- **Windows**: Windows 10/11 (tested on Windows 10.0.26220)
- **Linux**: Ubuntu 22.04+ or equivalent
- **macOS**: macOS 12+ (Monterey or later)

### Required Tools

| Tool | Version | Installation |
|------|---------|--------------|
| Rust | 1.83+ | `rustup install stable` |
| Cargo | 1.83+ | Included with Rust |
| Git | 2.40+ | System package manager |
| Node.js | 20+ | `nvm install 20` or system package |
| Python | 3.12+ | `pyenv install 3.12` or system package |
| SQLite | 3.40+ | System package manager |

### Environment Variables

```bash
# Required
export NOA_ROOT="${HOME}/noa"  # or set to your NOA installation path

# Optional (with defaults)
export NOA_PG_USER="noa"        # PostgreSQL username (if using PostgreSQL)
export NOA_PG_PASSWORD=""       # PostgreSQL password (if using PostgreSQL)
```

---

## Step-by-Step Reproduction

### 1. Clone Repository

```bash
git clone https://github.com/FlexNetOS/noa.git
cd noa
export NOA_ROOT="$(pwd)"
```

### 2. Verify Prerequisites

**Bash (Linux/macOS)**:
```bash
./scripts/bash/check-prerequisites.sh --json --require-tasks --include-tasks
```

**PowerShell (Windows)**:
```powershell
.\scripts\powershell\check-prerequisites.ps1 -Json -RequireTasks -IncludeTasks
```

**Expected Output**: JSON with all tools listed as installed

### 3. Create Data Directories

```bash
# Create Phase 2 data directories
mkdir -p "${NOA_ROOT}/data/memory"
mkdir -p "${NOA_ROOT}/data/knowledge"
mkdir -p "${NOA_ROOT}/data/embeddings"
mkdir -p "${NOA_ROOT}/data/artifacts"
```

**Windows PowerShell**:
```powershell
New-Item -ItemType Directory -Force -Path "${env:NOA_ROOT}\data\memory"
New-Item -ItemType Directory -Force -Path "${env:NOA_ROOT}\data\knowledge"
New-Item -ItemType Directory -Force -Path "${env:NOA_ROOT}\data\embeddings"
New-Item -ItemType Directory -Force -Path "${env:NOA_ROOT}\data\artifacts"
```

### 4. Verify Configuration Files

```bash
# Verify all Phase 2 config files exist
test -f "${NOA_ROOT}/config/database.yaml" && echo "✓ database.yaml"
test -f "${NOA_ROOT}/config/minio.yaml" && echo "✓ minio.yaml"
test -f "${NOA_ROOT}/config/qdrant.yaml" && echo "✓ qdrant.yaml"
test -f "${NOA_ROOT}/config/quickwit.yaml" && echo "✓ quickwit.yaml"
test -f "${NOA_ROOT}/containers/oci/registry.yaml" && echo "✓ registry.yaml"
```

### 5. Verify Database Schema Files

```bash
# Verify migration files exist
test -f "${NOA_ROOT}/init/migrations/001_initial.sql" && echo "✓ 001_initial.sql"
test -f "${NOA_ROOT}/init/migrations/002_indexes.sql" && echo "✓ 002_indexes.sql"
test -f "${NOA_ROOT}/init/migrations/003_vectors.sql" && echo "✓ 003_vectors.sql"
```

### 6. Build Rust Core

```bash
cd "${NOA_ROOT}/sys/core"
cargo build --release
```

**Expected Output**: Build completes with exit code 0

### 7. Run Database Migration

```bash
cd "${NOA_ROOT}/sys/core"
cargo run -- db migrate
```

**Expected Output**: Database created at `${NOA_ROOT}/data/noa.db`

### 8. Verify Database Schema

```bash
cd "${NOA_ROOT}/sys/core"
cargo run -- db check
```

**Expected Output**: All tables verified, exit code 0

### 9. Run Smoke Tests

**Bash (Linux/macOS)**:
```bash
cd "${NOA_ROOT}"
./scripts/test/smoke-test-phase2.sh
```

**PowerShell (Windows)**:
```powershell
cd "${env:NOA_ROOT}"
.\scripts\test\smoke-test-phase2.ps1
```

**Expected Output**: All checks pass, exit code 0

### 10. Test CSV Export

```bash
cd "${NOA_ROOT}/sys/core"
mkdir -p exports
cargo run -- db export --format csv --output ./exports
```

**Expected Output**: CSV files created in `exports/` directory

### 11. Test API Server

```bash
# Terminal 1: Start server
cd "${NOA_ROOT}/sys/core"
cargo run -- start

# Terminal 2: Test health endpoint
curl http://localhost:8080/api/v1/health
```

**Expected Output**: `{"status":"ok"}` with HTTP 200

---

## Verification Checklist

- [ ] Prerequisites verified (all tools installed)
- [ ] Data directories created
- [ ] Configuration files exist
- [ ] Database schema files exist
- [ ] Rust project builds successfully
- [ ] Database migration runs successfully
- [ ] Database schema verified
- [ ] Smoke tests pass
- [ ] CSV export works
- [ ] API server starts and responds

---

## Troubleshooting

### Database Migration Fails

**Error**: `Failed to open database`
**Solution**: Ensure `${NOA_ROOT}/data/` directory exists and is writable

### Build Fails

**Error**: `error: failed to compile`
**Solution**:
1. Run `cargo clean`
2. Update Rust: `rustup update stable`
3. Rebuild: `cargo build`

### Smoke Tests Fail

**Error**: `❌ FAIL: <file> exists`
**Solution**: Verify file paths match expected locations in `tasks.md`

### API Server Won't Start

**Error**: `Address already in use`
**Solution**:
1. Check if port 8080 is in use: `netstat -an | grep 8080`
2. Change port in `config/noa-server.json`
3. Or stop existing server

---

## Expected File Structure

```
${NOA_ROOT}/
├── data/
│   ├── memory/
│   ├── knowledge/
│   ├── embeddings/
│   └── artifacts/
├── config/
│   ├── database.yaml
│   ├── minio.yaml
│   ├── qdrant.yaml
│   ├── quickwit.yaml
│   └── observability.yaml
├── containers/
│   └── oci/
│       └── registry.yaml
├── init/
│   └── migrations/
│       ├── 001_initial.sql
│       ├── 002_indexes.sql
│       └── 003_vectors.sql
└── sys/
    └── core/
        └── src/
            ├── db/
            ├── config/
            ├── export/
            ├── api/
            └── cli/
```

---

## Exit Codes

All commands should return exit code 0 on success:
- `cargo build`: 0
- `cargo run -- db migrate`: 0
- `cargo run -- db check`: 0
- `./scripts/test/smoke-test-phase2.sh`: 0
- `curl http://localhost:8080/api/v1/health`: 0 (HTTP 200)

---

*Reproduction guide for Phase 2 - Database & Storage Infrastructure*
*Last Updated: 2025-12-10*

