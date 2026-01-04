# ML DevOps Inference Server

High-performance Rust-based inference server for local ML model execution.

## 🎯 Features

- **OpenAI-Compatible API**: Drop-in replacement for OpenAI endpoints
- **Streaming Responses**: Server-Sent Events (SSE) for token-by-token output
- **Cross-Platform**: Linux and Windows binaries
- **Lightweight**: ~5-8 MB optimized binaries
- **Fast Startup**: <500ms initialization time
- **Mock Implementation**: Ready for RuvLLM/Candle integration

## 🚀 Quick Start

### Linux

```bash
# Start the server
./target/release/inference_server --port 8080

# Test health endpoint
curl http://localhost:8080/health
```

### Windows

```powershell
# Start the server
.\target\x86_64-pc-windows-gnu\release\inference_server.exe --port 8080

# Test health endpoint
Invoke-WebRequest http://localhost:8080/health
```

## 🏗️ Building from Source

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Linux**: GCC toolchain
- **Windows Cross-Compilation** (Linux → Windows):
  ```bash
  rustup target add x86_64-pc-windows-gnu
  sudo apt-get install mingw-w64
  ```

### Build Commands

#### Linux Binary

```bash
cd rust_backend
cargo build --release
# Output: target/release/inference_server (~4.3 MB)
```

#### Windows Binary (from Linux)

```bash
cd rust_backend
cargo build --release --target x86_64-pc-windows-gnu
# Output: target/x86_64-pc-windows-gnu/release/inference_server.exe (~7.9 MB)
```

#### Both Platforms (Automated)

```bash
cd rust_backend
chmod +x build.sh
./build.sh
```

## 📦 Project Structure

```
rust_backend/
├── inference_server/
│   ├── src/
│   │   ├── main.rs         # Entry point & CLI
│   │   ├── lib.rs          # Library exports
│   │   ├── server.rs       # Axum HTTP server
│   │   ├── models.rs       # Model management (mock)
│   │   └── types.rs        # API types (OpenAI-compatible)
│   └── Cargo.toml          # Dependencies
├── .cargo/
│   └── configs.toml         # Cross-compilation configs
├── Cargo.toml              # Workspace configs
├── build.sh                # Automated build script
└── README.md               # This file
```

## 🔧 configsuration

### Command-Line Options

```bash
./inference_server --help

Options:
  --host <HOST>       Host to bind to [default: 127.0.0.1]
  --port <PORT>       Port to listen on [default: 8080]
  --log-level <LEVEL> Log level (trace/debug/info/warn/error) [default: info]
  -h, --help          Print help
  -V, --version       Print version
```

### Environment Variables

```bash
export RUST_LOG=info                # Logging level
export INFERENCE_HOST=0.0.0.0       # Bind to all interfaces
export INFERENCE_PORT=8080          # Server port
```

## 🌐 API Endpoints

### Health Check

```bash
GET /health

Response:
{
  "status": "healthy",
  "model_loaded": false,
  "uptime_seconds": 42
}
```

### Chat Completion (OpenAI-Compatible)

```bash
POST /v1/chat/completions
Content-Type: application/json

{
  "model": "default",
  "messages": [{"role": "user", "content": "Hello!"}],
  "stream": true,
  "temperature": 0.7,
  "max_tokens": 1000
}

Response (SSE Stream):
data: {"id":"...","choices":[{"delta":{"content":"Hello"}}]}
data: {"id":"...","choices":[{"delta":{"content":" there"}}]}
data: [DONE]
```

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
# Start server
./target/release/inference_server &
SERVER_PID=$!

# Test endpoints
curl http://localhost:8080/health
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"default","messages":[{"role":"user","content":"Test"}]}'

# Cleanup
kill $SERVER_PID
```

### Windows Binary Verification

```bash
# On Linux (verify PE format)
file target/x86_64-pc-windows-gnu/release/inference_server.exe
# Expected: PE32+ executable (console) x86-64, for MS Windows

# On Windows (run binary)
.\inference_server.exe --version
```

## 🔄 Tauri Integration

The server is designed to be spawned by Tauri desktop apps:

```rust
// In src-tauri/src/lib.rs
#[tauri::command]
async fn start_inference_server(port: Option<u16>) -> Result<String, String> {
    let binary_name = if cfg!(target_os = "windows") {
        "inference_server.exe"
    } else {
        "inference_server"
    };
    
    let mut cmd = std::process::Command::new(binary_path)
        .arg("--port")
        .arg(port.unwrap_or(8080).to_string())
        .spawn()
        .map_err(|e| e.to_string())?;
    
    Ok(format!("http://127.0.0.1:{}", port.unwrap_or(8080)))
}
```

## 🔮 Future Enhancements

### Real ML Inference (RuvLLM)

Currently using mock implementation. To integrate RuvLLM:

1. Wait for `ruvllm` v0.2.4+ (fixes Candle compatibility)
2. Update `Cargo.toml`:
   ```toml
   [dependencies]
   ruvllm = "0.2.4"
   ```
3. Replace mock in `models.rs` with real inference:
   ```rust
   use ruvllm::{Model, configs};
   
   pub async fn load_model() -> Result<Model> {
       Model::from_pretrained("Qwen/Qwen2.5-7B-Instruct")
   }
   ```

### Performance Optimizations

- **GPU Acceleration**: CUDA/Metal support
- **Quantization**: 4-bit/8-bit model weights
- **Model Caching**: Pre-load models on startup
- **Batching**: Group multiple requests

## 📊 Performance

### Build Time

- **Linux**: ~36 seconds (release mode)
- **Windows** (cross-compile): ~90 seconds (release mode)

### Binary Size

- **Linux**: 4.3 MB (stripped)
- **Windows**: 7.9 MB (stripped)

### Runtime Performance

- **Startup**: <500ms
- **Health Check**: <5ms
- **Mock Inference**: ~200 tokens/s (simulated)

## 🐛 Troubleshooting

### Build Errors

**Issue**: `error: linker 'x86_64-w64-mingw32-gcc' not found`

**Solution**:
```bash
sudo apt-get install mingw-w64
```

**Issue**: `error: target 'x86_64-pc-windows-gnu' not found`

**Solution**:
```bash
rustup target add x86_64-pc-windows-gnu
```

### Runtime Errors

**Issue**: "Address already in use"

**Solution**: Change port with `--port` flag or kill existing process:
```bash
lsof -ti:8080 | xargs kill -9
```

**Issue**: Windows binary won't run

**Solution**: Ensure you have Visual C++ Redistributable installed:
- Download from [Microsoft](https://aka.ms/vs/17/release/vc_redist.x64.exe)

## 📄 License

MIT License - See project root for details.

## 🤝 Contributing

Contributions welcome! Focus areas:

1. **Real Inference**: Integrate RuvLLM/Candle/Llama.cpp
2. **GPU Support**: CUDA/Metal backends
3. **Model Management**: Download, cache, quantize
4. **Monitoring**: Prometheus metrics, OpenTelemetry
5. **Security**: Rate limiting, API keys, CORS

## 📞 Support

For issues, see main project documentation or file GitHub issues.
