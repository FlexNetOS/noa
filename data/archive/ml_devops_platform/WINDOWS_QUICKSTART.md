# 🪟 Windows Quick Start Guide

Get the ML DevOps Platform running on Windows 11 in under 5 minutes.

## ✅ Setup Complete (NOA Environment)

This platform has been successfully set up in the NOA environment with:

- **Rust Backend**: Built and running on `http://localhost:8080`
- **Google OAuth**: Configured in `nextjs_space/.env`
- **Python Environment**: `conda-forge` environment at `N:\noa\opt\conda\envs\noa`
- **Package Managers**:
  - `uv` (Python package manager) at `N:\noa\opt\uv\`
  - `uvx` (tool runner) available
- **HuggingFace CLI**: Installed via `uvx` at `N:\noa\ai\providers\local\hf-cli\`

All dependencies are self-contained within `N:\noa\` per NOA Constitution 3.1.

See `BUILD_SUCCESS.md` for detailed build information.

---

## Prerequisites

### Required
- **Windows 11** (Windows 10 21H2+ also works)
- **Visual C++ Redistributable** - [Download Here](https://aka.ms/vs/17/release/vc_redist.x64.exe)

### Optional (For Building from Source)
- **Rust** - Install via [rustup.rs](https://rustup.rs/) or:
  ```powershell
  winget install Rustlang.Rustup
  ```
- **Node.js** - Install via [nodejs.org](https://nodejs.org/) or:
  ```powershell
  winget install OpenJS.NodeJS
  ```

---

## Option 1: Use Pre-Built Binary (Fastest) ⚡

### Step 1: Download Binary

Download from GitHub releases or use the cross-compiled binary:
```
rust_backend/target/x86_64-pc-windows-gnu/release/inference_server.exe
```

### Step 2: Run Server

```powershell
# Open PowerShell in the directory containing inference_server.exe
.\inference_server.exe --port 8080
```

Expected output:
```
🦀 Rust Inference Server
📍 Listening on http://127.0.0.1:8080
✅ Ready to accept requests
```

### Step 3: Test It

**Health Check:**
```powershell
Invoke-WebRequest http://localhost:8080/health | Select-Object -Expand Content
```

**Chat Completion:**
```powershell
$body = @{
    model = "local"
    messages = @(
        @{role = "user"; content = "Hello!"}
    )
    stream = $false
} | ConvertTo-Json

Invoke-RestMethod -Method Post `
    -Uri http://localhost:8080/v1/chat/completions `
    -ContentType "application/json" `
    -Body $body
```

---

## Option 2: Build from Source 🛠️

### Step 1: Clone Repository

```powershell
git clone https://github.com/yourusername/ml_devops_platform.git
cd ml_devops_platform
```

### Step 2: Build Rust Backend

```powershell
cd rust_backend
cargo build --release
```

Build time: ~90 seconds  
Output: `target/release/inference_server.exe` (7.9 MB)

### Step 3: Run Server

```powershell
.\target\release\inference_server.exe --port 8080
```

---

## Option 3: Full Tauri Desktop App 🖥️

### Build Instructions

```powershell
# Navigate to Next.js directory
cd nextjs_space

# Install Node dependencies
yarn install

# Generate Prisma client
yarn prisma generate

# Build Tauri app
yarn tauri build
```

### Output

Installer created at:
```
src-tauri/target/release/bundle/msi/ML DevOps Platform_0.1.0_x64_en-US.msi
```

### Install & Run

1. Double-click the `.msi` installer
2. Follow installation wizard
3. Launch "ML DevOps Platform" from Start Menu
4. Click **"Start Inference Server"** in the UI

---

## 🔧 Configuration

### Command-Line Options

```powershell
.\inference_server.exe --help

Options:
  --host <HOST>       Bind address [default: 127.0.0.1]
  --port <PORT>       Port number [default: 8080]
  --log-level <LEVEL> Logging level [default: info]
  -h, --help          Show help
  -V, --version       Show version
```

### Examples

**Bind to all interfaces:**
```powershell
.\inference_server.exe --host 0.0.0.0 --port 8080
```

**Enable debug logging:**
```powershell
$env:RUST_LOG="debug"
.\inference_server.exe
```

---

## 🌐 API Reference

### Health Check

```powershell
GET http://localhost:8080/health
```

Response:
```json
{
  "status": "healthy",
  "model_loaded": false,
  "uptime_seconds": 42
}
```

### Chat Completion (OpenAI-Compatible)

```powershell
POST http://localhost:8080/v1/chat/completions
Content-Type: application/json

{
  "model": "local",
  "messages": [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": "Write a haiku about coding."}
  ],
  "stream": true,
  "temperature": 0.7,
  "max_tokens": 1000
}
```

### List Models

```powershell
GET http://localhost:8080/v1/models
```

---

## 🐛 Troubleshooting

### "VCRUNTIME140.dll was not found"

**Solution:** Install Visual C++ Redistributable:
```powershell
# Download and run
Start-Process "https://aka.ms/vs/17/release/vc_redist.x64.exe"
```

### "Address already in use"

**Solution:** Kill existing process or use different port:
```powershell
# Find process using port 8080
Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess

# Kill it
Stop-Process -Id <PID> -Force

# Or use different port
.\inference_server.exe --port 8081
```

### "Access Denied" when starting server

**Solution:** Run PowerShell as Administrator or add firewall exception:
```powershell
# As Administrator
New-NetFirewallRule -DisplayName "ML Inference Server" `
    -Direction Inbound `
    -LocalPort 8080 `
    -Protocol TCP `
    -Action Allow
```

### Slow Performance

**Note:** Current version uses mock inference. Real performance requires:
1. Actual model weights (Qwen2.5-7B ~15GB)
2. RuvLLM integration (coming in v0.2.4+)
3. GPU acceleration (CUDA/DirectML)

---

## 📊 Performance Metrics

### Startup Time
- Cold start: ~500ms
- Warm start: ~200ms

### Binary Size
- inference_server.exe: 7.9 MB (stripped)
- Full Tauri app: ~30 MB (including Next.js bundle)

### Memory Usage
- Idle: ~15 MB
- Active (mock inference): ~50 MB
- With real model: ~8-16 GB (depending on quantization)

---

## 🚀 Next Steps

### Integrate with Existing Apps

The server is OpenAI-compatible, so you can use existing libraries:

**Python:**
```python
import openai

client = openai.OpenAI(
    base_url="http://localhost:8080/v1",
    api_key="dummy"  # Not required for local
)

response = client.chat.completions.create(
    model="local",
    messages=[{"role": "user", "content": "Hello!"}]
)
```

**JavaScript:**
```javascript
import OpenAI from 'openai';

const client = new OpenAI({
    baseURL: 'http://localhost:8080/v1',
    apiKey: 'dummy'
});

const response = await client.chat.completions.create({
    model: 'local',
    messages: [{role: 'user', content: 'Hello!'}]
});
```

### Enable Real Inference

1. Download model weights:
   ```powershell
   # Using Hugging Face CLI
   pip install huggingface-hub
   huggingface-cli download Qwen/Qwen2.5-7B-Instruct
   ```

2. Wait for RuvLLM v0.2.4+ (fixes Candle compatibility)

3. Rebuild with real inference enabled

### Deploy to Production

**As Windows Service:**
```powershell
# Using NSSM (Non-Sucking Service Manager)
choco install nssm
nssm install MLInferenceServer "C:\path\to\inference_server.exe"
nssm set MLInferenceServer AppParameters "--port 8080"
nssm start MLInferenceServer
```

**With Docker Desktop:**
```dockerfile
FROM mcr.microsoft.com/windows/servercore:ltsc2022
COPY inference_server.exe /app/
EXPOSE 8080
CMD ["C:\\app\\inference_server.exe"]
```

---

## 💡 Tips & Best Practices

1. **Use PowerShell 7+**: Better performance and features
   ```powershell
   winget install Microsoft.PowerShell
   ```

2. **Add to PATH**: For easy access from any directory
   ```powershell
   $env:Path += ";C:\path\to\inference_server"
   [Environment]::SetEnvironmentVariable("Path", $env:Path, "User")
   ```

3. **Create Desktop Shortcut**:
   - Right-click `inference_server.exe`
   - Send to → Desktop (create shortcut)
   - Right-click shortcut → Properties
   - Add `--port 8080` to "Target" field

4. **Monitor Performance**:
   ```powershell
   # CPU & Memory usage
   Get-Process inference_server | Format-List *
   
   # Network connections
   Get-NetTCPConnection -LocalPort 8080
   ```

---

## 📝 Additional Resources

- **Main Documentation**: `/README.md`
- **Rust Integration Guide**: `/E2_INTEGRATION_COMPLETE.md`
- **Tauri Setup**: `/TAURI_SETUP.md`
- **Architecture**: `/ARCHITECTURE.md`
- **Issues & Support**: GitHub Issues

---

## 🤝 Contributing

Found a Windows-specific bug? Have optimization ideas?

1. Fork the repository
2. Create feature branch: `git checkout -b feature/windows-optimization`
3. Commit changes: `git commit -am 'Improve Windows performance'`
4. Push: `git push origin feature/windows-optimization`
5. Open Pull Request

---

## 📄 License

MIT License - See project root for details.

---

**Questions?** Open an issue or check the main documentation!
