# NOA Seed Foundation - Full Stack Application

## 🚀 System Status

**OPERATIONAL** - Full stack application running successfully!

### Supported Platforms
- **Windows**: Windows 10/11 (build 19041+) or Windows Server 2019+
- **Linux**: Ubuntu 20.04+, Debian 11+, or RHEL 8+
- **macOS**: macOS 11.0+ (Big Sur or later)
- **WSL**: WSL2 on Windows (Ubuntu 20.04+ recommended)

### Running Services
- ✅ **API Server**: http://localhost:3001
- ✅ **UI Dashboard**: http://localhost:3000
- ✅ **Rust Core**: 6 crates compiled and operational
- ✅ **Database Schema**: Defined (SQLite ready)
- ⏳ **Neural Runtime**: Pending llama.cpp integration
- ⏳ **Agent Orchestrator**: Framework ready, needs activation
- ⏳ **P2P Network**: Pending libp2p integration

## 📦 Architecture

### Backend (Rust)
Located in `sys/core/`, the backend consists of 6 crates:

1. **noa-api** - REST API server with Axum
   - Health checks: `/health`
   - System status: `/api/v1/status`
   - Task management: `/api/v1/tasks` (GET, POST)

2. **noa-common** - Shared types and utilities
   - Entity IDs, timestamps
   - Agent types and states
   - Knowledge graph types

3. **noa-embedder** - Vector embedding service
   - FastEmbed integration (pending)
   - Text-to-vector conversion

4. **noa-trainer** - Model training pipeline
   - Fine-tuning capabilities

5. **noa-indexer** - Repository indexing
   - Code analysis and digestion

6. **noa-agent** - Autonomous agent system
   - CECCA orchestrator
   - MicroAgentStack framework
   - Permanent and disposable agents

### Frontend (Next.js + React)
Located in `sys/ui/`, the UI provides:

- **Real-time System Monitoring**
  - Component status dashboard
  - Health indicators
  - Version tracking

- **Chat Interface**
  - Natural language task creation
  - Real-time API integration
  - Message history

- **Modern Design**
  - Gradient backgrounds
  - Glassmorphism effects
  - Responsive layout

## 🛠️ Development

### Starting the Servers

**API Server (Terminal 1):**
```powershell
cd sys/core
cargo run --bin noa-api
```

**UI Server (Terminal 2):**
```powershell
cd sys/ui
npm run dev
```

### Building

**Rust Backend:**
```powershell
cd sys/core
cargo build --release
```

**Next.js UI:**
```powershell
cd sys/ui
npm run build
```

## 🧪 Testing

**API Health Check:**
```powershell
Invoke-WebRequest -Uri http://localhost:3001/health -UseBasicParsing
```

**System Status:**
```powershell
Invoke-WebRequest -Uri http://localhost:3001/api/v1/status -UseBasicParsing
```

**Create Task:**
```powershell
Invoke-WebRequest -Uri http://localhost:3001/api/v1/tasks `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"description":"Test task","priority":"normal"}' `
  -UseBasicParsing
```

## 📊 Database Schema

The system uses SQLite with the following tables:

- **knowledge_nodes** - Code entities and concepts
- **knowledge_edges** - Relationships between nodes
- **embeddings** - Vector representations
- **tasks** - User tasks and objectives
- **agents** - Active agent instances

## 🎯 Next Steps

1. **Integrate llama.cpp** for local LLM inference
2. **Activate database** with proper file permissions
3. **Implement embedder** with FastEmbed
4. **Build agent orchestrator** with CECCA
5. **Add P2P networking** with libp2p
6. **Create desktop app** with NDCL

## 📝 API Endpoints

### Health & Status
- `GET /health` - Server health check
- `GET /api/v1/status` - Component status

### Tasks
- `GET /api/v1/tasks` - List all tasks
- `POST /api/v1/tasks` - Create new task
  ```json
  {
    "description": "Task description",
    "priority": "normal|high|low"
  }
  ```

## 🔧 Configuration

### Environment Variables
- `RUST_LOG` - Logging level (default: `noa_api=debug`)
- `PORT` - API server port (default: 3001)

### Toolchain Versions
- Rust: 1.91.1
- Node: 20.18.1
- Go: 1.23.4
- Python: 3.12.8

### Config and Schema Policy
NOA uses a shared and centralized configuration system located in `/config/`. All configuration files follow a unified JSON schema defined in `/config/schemas/`, ensuring consistency and validation.

- **Metadata Structure**: Each config includes a `metadata` object with `version`, `description`, `updated_at`, and other provenance fields for auditability and change tracking.
- **Schema Validation**: Configurations are validated against JSON schemas to prevent errors and ensure compliance with the universal task execution policy.
- **Centralized Management**: Shared resources, providers, and features are configured centrally, with environment-specific overrides supported via `${NOA_ROOT}` variables.
- **Version Control**: Configs are versioned and changes are tracked, with automated validation on commit.

## 📚 Documentation

See `specs/001-noa-seed-foundation/` for:
- `spec.md` - Full system specification
- `plan.md` - Implementation plan
- `tasks.md` - Task breakdown
- `data-model.md` - Data structures

## 🎨 UI Features

- **System Dashboard** - Real-time component monitoring
- **Chat Interface** - Natural language task creation
- **Status Indicators** - Visual health checks
- **Responsive Design** - Works on all screen sizes

## 🚦 Current Status

### ✅ Completed
- Core Rust workspace with 6 crates
- REST API with Axum
- Next.js UI with TypeScript
- Real-time status monitoring
- Task management endpoints
- Chat interface
- Full stack integration

### ⏳ In Progress
- Database integration
- Neural runtime
- Agent orchestration
- P2P networking

### 📋 Pending
- llama.cpp integration
- FastEmbed embeddings
- CECCA activation
- libp2p federation
- Desktop app (NDCL)

---

**Built with ❤️ for autonomous AI**
