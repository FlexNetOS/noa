# NOA Quick Start Guide

## 🚀 Start the Full Stack Application

### Prerequisites
All toolchains are already installed in `N:\noa\opt\`:
- Rust 1.91.1
- Node 20.18.1
- Go 1.23.4
- Python 3.12.8

### Start Servers

**Terminal 1 - API Server:**
```powershell
cd N:\noa\sys\core
$env:PATH = "N:\noa\opt\rust\cargo\bin;$env:PATH"
cargo run --bin noa-api
```

**Terminal 2 - UI Server:**
```powershell
cd N:\noa\sys\ui
$env:PATH = "N:\noa\opt\node;$env:PATH"
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

## 🎨 UI Features

1. **System Dashboard**
   - Real-time component status
   - Health indicators with color coding
   - Version information

2. **Chat Interface**
   - Natural language input
   - Task creation via API
   - Message history
   - Loading states

3. **Responsive Design**
   - Works on desktop and mobile
   - Dark theme with gradients
   - Smooth animations

## 🔌 API Endpoints

### Health & Status
- `GET /health` - Server health check
  ```json
  {"status":"healthy","version":"0.1.0"}
  ```

- `GET /api/v1/status` - Component status
  ```json
  {
    "status":"operational",
    "components":{
      "api":true,
      "database":true,
      "embedder":false,
      "agents":false,
      "p2p":false
    }
  }
  ```

### Tasks
- `GET /api/v1/tasks` - List all tasks
  ```json
  {"tasks":[]}
  ```

- `POST /api/v1/tasks` - Create new task
  ```json
  {
    "description": "Task description",
    "priority": "normal"
  }
  ```
  Response:
  ```json
  {
    "task_id": "uuid",
    "status": "queued"
  }
  ```

## 🐛 Troubleshooting

**API server won't start:**
- Check if port 3001 is available
- Verify Rust toolchain is in PATH
- Check `sys/core/target/debug/` for build artifacts

**UI server won't start:**
- Check if port 3000 is available
- Verify Node is in PATH
- Run `npm install` in `sys/ui/`

**CORS errors:**
- API server has CORS enabled for all origins
- Check browser console for specific errors

## 📚 Next Steps

1. **Integrate llama.cpp** for local LLM inference
2. **Activate database** with SQLite
3. **Build embedder** with FastEmbed
4. **Implement CECCA** agent orchestrator
5. **Add P2P networking** with libp2p

## 🎓 Learn More

- See `README.md` for full documentation
- Check `specs/001-noa-seed-foundation/` for specifications
- Review `sys/core/crates/` for Rust implementation
- Explore `sys/ui/src/` for UI components

---

**Status**: ✅ Full stack operational and ready for development!
xplore `sys/ui/src/` for UI components

---

**Status**: ✅ Full stack operational and ready for development!
