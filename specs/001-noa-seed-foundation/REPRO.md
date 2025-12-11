# Reproduction Instructions: Phase 1 Implementation

**Purpose**: Exact environment and commands for reproducing Phase 1 setup
**Created**: 2025-01-09
**Phase**: Phase 1 - Setup (Shared Infrastructure)

---

## Environment Requirements

### Operating System
- **Windows**: Windows 10/11 (build 19041+) or Windows Server 2019+
- **Linux**: Ubuntu 20.04+, Debian 11+, or RHEL 8+
- **macOS**: macOS 11.0+ (Big Sur or later)
- **WSL**: WSL2 on Windows (Ubuntu 20.04+ recommended)

### Prerequisites (Auto-detected by check-prereqs scripts)

#### Required Tools
- **Git**: >= 2.40.0
- **Rust**: >= 1.83.0 (via rustup)
- **Go**: >= 1.23.0
- **Node.js**: >= 20.0.0
- **Python**: >= 3.12.0
- **protoc**: >= 28.0.0

#### Quality Tools
- **rustfmt**: >= 1.0.0
- **clippy**: >= 0.1.0
- **golangci-lint**: >= 1.62.0
- **eslint**: >= 9.0.0
- **ruff**: >= 0.8.0

#### Security Tools
- **Gitleaks**: >= 8.21.0
- **Trivy**: >= 0.57.0
- **Grype**: >= 0.84.0
- **Semgrep**: >= 1.97.0

---

## Step-by-Step Reproduction

### Step 1: Clone Repository

```bash
# Windows (PowerShell)
git clone https://github.com/FlexNetOS/noa.git
cd noa

# Linux/macOS/WSL (Bash)
git clone https://github.com/FlexNetOS/noa.git
cd noa
```

### Step 2: Verify Prerequisites

**Windows (PowerShell):**
```powershell
.\scripts\powershell\check-prerequisites.ps1 -Json
```

**Linux/macOS/WSL (Bash):**
```bash
./scripts/bash/check-prerequisites.sh --json
```

**Expected Output**: JSON with tool status (✅ installed, ❌ missing, ⚠️ version warning)

### Step 3: Verify Directory Structure

**Windows (PowerShell):**
```powershell
# Check all Phase 1 directories exist
Test-Path sys/core, sys/ui, sys/digest, sys/kernel
Test-Path p2p/discovery, p2p/sync, p2p/compute, p2p/storage
Test-Path opt/llama.cpp, opt/llama-cpp-rs, opt/ollama
Test-Path init/migrations, init/seeds
Test-Path containers/oci, containers/compose
Test-Path config
Test-Path bin
Test-Path ai/providers, ai/models, ai/prompts, ai/grammars
```

**Linux/macOS/WSL (Bash):**
```bash
# Check all Phase 1 directories exist
test -d sys/core && test -d sys/ui && test -d sys/digest && test -d sys/kernel
test -d p2p/discovery && test -d p2p/sync && test -d p2p/compute && test -d p2p/storage
test -d opt/llama.cpp && test -d opt/llama-cpp-rs && test -d opt/ollama
test -d init/migrations && test -d init/seeds
test -d containers/oci && test -d containers/compose
test -d config
test -d bin
test -d ai/providers && test -d ai/models && test -d ai/prompts && test -d ai/grammars
```

### Step 4: Verify Project Initialization

**Rust Workspace:**
```bash
cd sys/core
cargo check
```

**Go Module:**
```bash
cd p2p
go mod verify
```

**TypeScript/Next.js:**
```bash
cd sys/ui
npm install
npm run type-check
```

**Python Project:**
```bash
cd sys/digest
python -m pip install -e .
```

### Step 5: Run Build Scripts

**Windows (PowerShell):**
```powershell
.\scripts\powershell\build.ps1
```

**Linux/macOS/WSL (Bash):**
```bash
./scripts/bash/build.sh
```

### Step 6: Verify CI Pipeline

```bash
# Check GitHub Actions workflow exists
test -f .github/workflows/ci.yml
```

---

## Verification Commands

### Smoke Test

**Windows (PowerShell):**
```powershell
# Run all Phase 1 smoke tests
.\scripts\powershell\check-prerequisites.ps1
Test-Path sys/core/Cargo.toml
Test-Path p2p/go.mod
Test-Path sys/ui/package.json
Test-Path README.md
```

**Linux/macOS/WSL (Bash):**
```bash
# Run all Phase 1 smoke tests
./scripts/bash/check-prerequisites.sh
test -f sys/core/Cargo.toml
test -f p2p/go.mod
test -f sys/ui/package.json
test -f README.md
```

**Expected Exit Code**: 0 (all checks pass)

---

## Troubleshooting

### Issue: Prerequisites Missing
**Solution**: Run bootstrap installer:
```powershell
# Windows
.\scripts\bootstrap\bootstrap.ps1 -InstallAllTools

# Linux/macOS/WSL
./scripts/bootstrap/bootstrap.sh --install-all-tools
```

### Issue: Directory Structure Missing
**Solution**: Run setup script:
```powershell
# Windows
.\scripts\setup\setup-noa.ps1

# Linux/macOS/WSL
./scripts/setup/setup-noa.sh
```

### Issue: Build Failures
**Solution**: Check individual project builds:
1. Rust: `cd sys/core && cargo build`
2. Go: `cd p2p && go build ./...`
3. TypeScript: `cd sys/ui && npm run build`

---

## Expected Results

After completing all steps:
- ✅ All prerequisite tools detected and verified
- ✅ All Phase 1 directories created
- ✅ All project initialization files present
- ✅ Rust workspace compiles
- ✅ Go module validates
- ✅ TypeScript project type-checks
- ✅ Build scripts execute successfully
- ✅ CI pipeline configuration present

---

*Last Updated: 2025-01-09*


