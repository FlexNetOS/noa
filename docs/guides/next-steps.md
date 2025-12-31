# NOA Platform - Next Steps Guide

This guide walks you through the three critical setup steps for your NOA platform.

---

## 1. Workspace ID Issue

**Problem:** The "missing workspace ID" error occurs because `localStorage.currentWorkspaceId` (and `localStorage.currentOrganizationId`) are not populated.

### Option A: Seed the Database (Recommended for Development)

The Taskosaur backend includes a comprehensive seeder that creates sample data:

```powershell
# Navigate to Taskosaur directory
cd n:\noa\project-mgmt\Taskosaur

# Install dependencies if not done
npm install

# Run database migrations first
npm run db:migrate

# Seed core modules (creates users, organizations, workspaces, projects, tasks)
npm run db:seed
```

**Seeder Commands Available:**
| Command | Description |
|---------|-------------|
| `npm run db:seed` | Seed core modules (idempotent - safe to run multiple times) |
| `npm run db:seed:admin` | Seed admin user only |
| `npm run db:seed:clear` | Clear all seeded data |
| `npm run db:seed:reset` | Clear and re-seed everything |

**What Gets Seeded:**
- Users with sample credentials
- Organizations (`taskosaur-inc`, `tech-innovators`)
- Workspaces (`dev-team`, `design-ux`, `marketing`, etc.)
- Projects, Tasks, Labels, Comments, etc.

### Option B: Create Through the UI

1. **Start the Taskosaur platform:**
   ```powershell
   cd n:\noa\project-mgmt\Taskosaur
   npm run dev
   ```

2. **Sign up/Login** to create your user account

3. **Create an Organization** (required before workspace):
   - Go to Organization settings
   - Create a new organization

4. **Create a Workspace:**
   - Navigate to Workspaces page
   - Click "New Workspace"
   - Fill in name, description, and settings

5. **Select the Workspace:**
   - Click on the workspace in the selector
   - This populates `localStorage.currentWorkspaceId`

### Verifying the Fix

Open browser DevTools (F12) → Application → Local Storage:
- `currentOrganizationId` should contain a UUID
- `currentWorkspaceId` should contain a UUID

---

## 2. RPC Server Setup

Build llama.cpp with RPC support and run the distributed inference server.

### Step 1: Build llama.cpp with RPC

```powershell
# Navigate to llama.cpp directory
cd n:\noa\opt\llama.cpp

# Create build directory
mkdir build-rpc
cd build-rpc

# Configure with RPC enabled
# For CPU only:
cmake .. -DGGML_RPC=ON

# For CUDA + RPC:
cmake .. -DGGML_CUDA=ON -DGGML_RPC=ON

# For Vulkan + RPC:
cmake .. -DGGML_VULKAN=ON -DGGML_RPC=ON

# Build
cmake --build . --config Release
```

### Step 2: Run the RPC Server

```powershell
# Start RPC server (exposes all available devices)
.\bin\rpc-server.exe

# Or specify port and device:
.\bin\rpc-server.exe -p 50052 --device CUDA0

# Enable local caching for faster model loading:
.\bin\rpc-server.exe -c
```

**Expected Output:**
```
Starting RPC server v3.0.0
  endpoint       : 127.0.0.1:50052
  local cache    : n/a
Devices:
  CUDA0: NVIDIA GPU Name (VRAM info)
```

### Step 3: Connect LLM Clients

```powershell
# Run llama-cli with RPC backend
.\bin\llama-cli.exe -m model.gguf -ngl 99 --rpc 127.0.0.1:50052

# For llama-server with RPC:
.\bin\llama-server.exe -m model.gguf --rpc 127.0.0.1:50052 --host 0.0.0.0 --port 8080
```

### Multiple RPC Servers (Distributed)

For multi-machine inference:
```powershell
# Machine A: rpc-server on port 50052
# Machine B: rpc-server on port 50052

# Main host connects to both:
.\bin\llama-cli.exe -m model.gguf --rpc 192.168.1.10:50052,192.168.1.11:50052
```

### Troubleshooting RPC

```powershell
# Enable debug output
$env:GGML_RPC_DEBUG=1
.\bin\rpc-server.exe
```

---

## 3. AI Features Setup

### Step 1: Configure AI Providers

Your `config/ai-providers.json` is already configured for local-first providers:

```json
{
  "providerPriority": ["local", "hybrid", "cloud"],
  "providers": {
    "local": {
      "enabled": true,
      "types": ["llama.cpp", "ollama", "local-llm"]
    }
  }
}
```

### Step 2: Start Local AI Services

**Option A: Ollama (Simpler)**
```powershell
# Install Ollama from https://ollama.ai
# Pull a model
ollama pull llama3.2

# Start Ollama server (usually auto-starts)
ollama serve
# Runs on http://localhost:11434
```

**Option B: llama.cpp Server**
```powershell
cd n:\noa\opt\llama.cpp\build-rpc

# Start the server with your model
.\bin\llama-server.exe `
  -m "path/to/your/model.gguf" `
  --host 0.0.0.0 `
  --port 8080 `
  --ctx-size 4096
# OpenAI-compatible API at http://localhost:8080/v1
```

### Step 3: Enable AI in Taskosaur

1. **Start Taskosaur:**
   ```powershell
   cd n:\noa\project-mgmt\Taskosaur
   npm run dev
   ```

2. **Configure AI Settings:**
   - Navigate to Settings → AI Assistant Settings
   - Set the following:

   | Setting | Value for Local LLM |
   |---------|---------------------|
   | **API URL** | `http://localhost:8080/v1` (llama-server) or `http://localhost:11434/v1` (Ollama) |
   | **Model** | Model name (e.g., `llama3.2` for Ollama, or check your server) |
   | **API Key** | Can be any string for local servers (e.g., `local-key`) |
   | **Enable AI Chat** | Toggle ON |

3. **Test Connection:**
   - Click "Test Connection" in the AI Settings modal
   - Should return success if services are running

### AI Provider Quick Reference

| Provider | API URL | Notes |
|----------|---------|-------|
| Local llama-server | `http://localhost:8080/v1` | OpenAI-compatible |
| Ollama | `http://localhost:11434/v1` | OpenAI-compatible mode |
| OpenRouter | `https://openrouter.ai/api/v1` | 100+ models, free tier |
| OpenAI | `https://api.openai.com/v1` | GPT models |
| Anthropic | `https://api.anthropic.com/v1` | Claude models |

---

## Quick Start Checklist

```
[ ] 1. WORKSPACE ID
    [ ] Run: cd n:\noa\project-mgmt\Taskosaur && npm run db:seed
    [ ] OR: Create workspace through UI
    [ ] Verify: Check localStorage in DevTools

[ ] 2. RPC SERVER
    [ ] Build: cmake .. -DGGML_RPC=ON && cmake --build . --config Release
    [ ] Run: .\bin\rpc-server.exe
    [ ] Verify: See "Starting RPC server" message

[ ] 3. AI FEATURES
    [ ] Start Ollama: ollama serve
    [ ] OR Start llama-server: .\bin\llama-server.exe -m model.gguf
    [ ] Configure Taskosaur: Settings → AI Assistant
    [ ] Test: Click "Test Connection"
```

---

## Environment Variables

Create or update `.env` file in `n:\noa\project-mgmt\Taskosaur`:

```env
# Database
DATABASE_URL="postgresql://user:pass@localhost:5432/taskosaur"

# AI Services (optional, can use UI settings)
AI_API_URL="http://localhost:8080/v1"
AI_MODEL="your-model-name"

# App URL (for AI provider headers)
APP_URL="http://localhost:3000"
```

---

## Troubleshooting

### "Missing workspace ID" persists
- Clear browser localStorage and re-login
- Ensure database seeding completed without errors
- Check that you've selected a workspace in the UI

### RPC server won't start
- Ensure no other process is using port 50052
- Check CUDA/Vulkan drivers are installed if using GPU
- Run with `GGML_RPC_DEBUG=1` for verbose output

### AI chat not responding
- Verify Ollama/llama-server is running: `curl http://localhost:8080/v1/models`
- Check API key is set (even `local-key` works for local servers)
- Look at backend logs for API errors

---

*Generated: December 3, 2025*
