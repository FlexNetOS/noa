# ML DevOps Platform - Quick Reference

## 🚀 Server Control

### Start Inference Server
```powershell
cd N:\noa\ml_devops_platform\rust_backend
.\target\release\inference_server.exe --port 8080
```

### Check Server Status
```powershell
Invoke-WebRequest http://localhost:8080/health | Select-Object -Expand Content
```

### Stop Server
Press `Ctrl+C` in the server terminal

---

## 📦 Package Management

### uv (Python Package Manager)
```powershell
# Install package
N:\noa\opt\uv\uv.exe pip install <package>

# Create virtual environment
N:\noa\opt\uv\uv.exe venv

# Run tool in isolated environment
N:\noa\opt\uv\uvx.exe <tool>
```

### HuggingFace CLI
```powershell
# Download model
N:\noa\ai\providers\local\hf-cli\bin\hf.cmd download <model-id>

# Login to HuggingFace
N:\noa\ai\providers\local\hf-cli\bin\hf.cmd auth login

# Check login status
N:\noa\ai\providers\local\hf-cli\bin\hf.cmd auth whoami
```

### Conda Environment
```powershell
# Activate environment
N:\noa\opt\conda\micromamba.exe activate N:\noa\opt\conda\envs\noa

# Install package
N:\noa\opt\conda\micromamba.exe install -p N:\noa\opt\conda\envs\noa <package>

# Run Python
N:\noa\opt\conda\micromamba.exe run -p N:\noa\opt\conda\envs\noa python
```

---

## 🔧 API Endpoints

### Health Check
```powershell
GET http://localhost:8080/health
```

### Chat Completion (OpenAI-compatible)
```powershell
POST http://localhost:8080/v1/chat/completions
Content-Type: application/json

{
  "model": "llama-3.2-1b",
  "messages": [
    {"role": "user", "content": "Hello!"}
  ],
  "temperature": 0.7,
  "max_tokens": 1024
}
```

### List Models
```powershell
GET http://localhost:8080/v1/models
```

---

## 📁 Key Locations

| Component | Path |
|-----------|------|
| Inference Server | `N:\noa\ml_devops_platform\rust_backend\target\release\inference_server.exe` |
| Next.js App | `N:\noa\ml_devops_platform\nextjs_space\` |
| Google OAuth Config | `N:\noa\ml_devops_platform\nextjs_space\.env` |
| Rust Toolchain | `N:\noa\opt\rust\` |
| Node.js | `N:\noa\opt\node\` |
| uv Package Manager | `N:\noa\opt\uv\` |
| Conda Environment | `N:\noa\opt\conda\envs\noa\` |
| HuggingFace CLI | `N:\noa\ai\providers\local\hf-cli\` |
| Models Directory | `N:\noa\ml_devops_platform\rust_backend\models\` |

---

## 🛠️ Installers

| Tool | Script |
|------|--------|
| Rust | `N:\noa\scripts\bootstrap\installers\rust-portable.ps1` |
| Node.js | `N:\noa\scripts\bootstrap\installers\node-portable.ps1` |
| uv | `N:\noa\scripts\bootstrap\installers\uv-portable.ps1` |
| HuggingFace CLI | `N:\noa\scripts\bootstrap\installers\hf-cli-portable.ps1` |
| Conda | `N:\noa\scripts\conda\bootstrap-micromamba.ps1` |

---

## 📚 Documentation

- **Setup Summary**: `SETUP_COMPLETE.md`
- **Build Report**: `BUILD_SUCCESS.md`
- **Windows Guide**: `WINDOWS_QUICKSTART.md`
- **NOA Constitution**: `N:\noa\CONSTITUTION.md`

---

## ⚡ Quick Commands

```powershell
# Load NOA environment
. N:\noa\noa-env.ps1

# Test all components
pwsh -NoProfile -Command "
  Write-Host 'Testing Rust Backend...' -ForegroundColor Yellow;
  Invoke-WebRequest http://localhost:8080/health | Select-Object -Expand Content;
  Write-Host 'Testing uv...' -ForegroundColor Yellow;
  & N:\noa\opt\uv\uv.exe --version;
  Write-Host 'Testing HuggingFace CLI...' -ForegroundColor Yellow;
  & N:\noa\ai\providers\local\hf-cli\bin\hf.cmd version;
"

# Rebuild Rust backend
cd N:\noa\ml_devops_platform\rust_backend
cargo build --release

# Start Next.js dev server
cd N:\noa\ml_devops_platform\nextjs_space
npm run dev
```

---

**Last Updated**: 2025-01-20  
**Status**: ✅ All Systems Operational
