# NOA Quick Start Guide

## 🚀 Start the Full Stack Application

### Prerequisites
Toolchains are installed in `N:\noa\opt\`:
- Rust 1.91.1
- Node 20.18.1
- Go 1.23.4
- Python 3.12.8

### Environment (Kernels + Conda-forge for notebooks)

**Windows (PowerShell):**
```powershell
. N:\noa\noa-env.ps1
```

This sets:
- `NOA_KERNEL=N:\noa\sys\kernel`
- toolchains on `PATH`
- helper commands for a self-contained conda-forge environment

**Bash (Linux/macOS/WSL):**
```bash
source /path/to/noa/.noa-env
```

#### Conda-forge runtime (recommended for notebooks)

NOA uses a self-contained micromamba environment under `N:\noa\opt\conda` (or `$NOA_OPT/conda`) intended for Jupyter/notebook workflows.

Bootstrap (requires you to place `micromamba`/`micromamba.exe` in `opt/conda/`):

- Windows:
  ```powershell
  . N:\noa\noa-env.ps1
  .\scripts\conda\bootstrap-micromamba.ps1
  noa-conda-activate
  ```

- Bash:
  ```bash
  source /path/to/noa/.noa-env
  ./scripts/conda/bootstrap-micromamba.sh
  noa_conda_activate
  ```

> Legacy note: `.noa-env` still supports activating `$NOA_OPT/venv` if it exists, but conda-forge is the preferred cross-platform strategy.

### Start Servers

**Terminal 1 - API Server:**
```powershell
. N:\noa\noa-env.ps1
cd N:\noa\sys\core
cargo run -p noa-api --bin noa-api
```

**Terminal 2 - UI Server:**
```powershell
. N:\noa\noa-env.ps1
cd N:\noa\sys\ui
npm run dev
```

### Access the Application

- **UI Dashboard**: http://localhost:3000
- **API Server**: http://localhost:3001
- **API Health**: http://localhost:3001/health
- **System Status**: http://localhost:3001/api/v1/status

## 🎯 Features

### UI Dashboard (Port 3000)
- Real-time system status monitoring
- Component health indicators
- Chat interface for task creation
- Modern glassmorphism design

### API Server (Port 3001)
- RESTful endpoints
- Task management
- System health checks
- CORS enabled for UI integration

## 📝 Quick Test

**Create a task via API:**
```powershell
$body = @{
    description = "Test task"
    priority = "high"
} | ConvertTo-Json

Invoke-WebRequest `
    -Uri http://localhost:3001/api/v1/tasks `
    -Method POST `
    -ContentType "application/json" `
    -Body $body `
    -UseBasicParsing
```

**Check system status:**
```powershell
Invoke-WebRequest `
    -Uri http://localhost:3001/api/v1/status `
    -UseBasicParsing |
    Select-Object -ExpandProperty Content
```

## 🏗️ Architecture

```
NOA Seed Foundation
├── sys/core/          # Rust backend
│   ├── crates/
│   │   ├── api/       # REST API server
│   │   ├── common/    # Shared types
│   │   ├── embedder/  # Vector embeddings
│   │   ├── trainer/   # Model training
│   │   ├── indexer/   # Code indexing
│   │   └── agent/     # Agent orchestration
│   └── target/        # Build artifacts
│
└── sys/ui/            # Next.js frontend
    ├── src/
    │   ├── app/       # Pages
    │   └── components/# React components
    └── node_modules/  # Dependencies
```

## 🔧 Development

**Build Rust backend:**
```powershell
cd sys/core
cargo build --release
```

**Build Next.js UI:**
```powershell
cd sys/ui
npm run build
```

**Run tests:**
```powershell
cd sys/core
cargo test
```

## 📊 System Components

| Component | Status | Port | Description |
|-----------|--------|------|-------------|
| API Server | ✅ Ready | 3001 | REST API with Axum |
| UI Dashboard | ✅ Ready | 3000 | Next.js + React |
| Database | ⏳ Pending | - | SQLite schema defined |
| Neural Runtime | ⏳ Pending | - | llama.cpp integration |
| Agent Orchestrator | ⏳ Pending | - | CECCA framework |
| P2P Network | ⏳ Pending | - | libp2p federation |

## 📚 Next Steps

1. Integrate llama.cpp for local LLM inference
2. Activate database with SQLite
3. Build embedder with FastEmbed
4. Implement CECCA agent orchestrator
5. Add P2P networking with libp2p

---

**Status**: ✅ Full stack operational and ready for development!
