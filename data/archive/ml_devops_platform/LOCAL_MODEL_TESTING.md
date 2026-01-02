# Test Local GGUF Model Loading

**Purpose**: Test script to verify local GGUF models can be loaded by the inference server.

## Current Status

The inference server is designed to download models from HuggingFace Hub. To use local GGUF files, we have two options:

### Option 1: Modify Server to Load Local Files (Recommended)

Update `models.rs` to accept local file paths:

```rust
// Add to ModelManager
pub async fn load_local_model(&self, model_path: &str, tokenizer_path: Option<&str>) -> Result<()> {
    let model_path = std::path::Path::new(model_path);
    
    // For GGUF files, we can extract tokenizer from the model itself
    // or use a separate tokenizer file if provided
    
    if !model_path.exists() {
        return Err(anyhow::anyhow!("Model file not found: {:?}", model_path));
    }
    
    // Load GGUF model
    self.load_gguf_model(model_path, tokenizer_path).await?;
    
    *self.model_loaded.write().await = true;
    *self.use_candle.write().await = true;
    *self.model_name.write().await = model_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("local-model")
        .to_string();
    
    Ok(())
}
```

### Option 2: Use HuggingFace Hub Cache (Current Approach)

The downloaded models are already cached by HuggingFace Hub. The server can access them via:

```rust
// Models are cached in: N:\noa\ml_devops_platform\rust_backend\models\.cache\huggingface\hub\
```

## Testing with Current Setup

Since the models are downloaded, we can test inference with the intelligent fallback:

```powershell
# Test chat completion
$body = @{
    model = "Qwen3-0.6B-BF16.gguf"
    messages = @(@{role="user"; content="What is 2+2?"})
    temperature = 0.7
    max_tokens = 100
} | ConvertTo-Json

Invoke-WebRequest -Method POST -Uri http://localhost:8080/v1/chat/completions `
    -ContentType "application/json" -Body $body | Select-Object -Expand Content
```

## Next Steps

1. **Modify `models.rs`** to add `load_local_model()` function
2. **Update `main.rs`** to accept `--model-path` argument for local files
3. **Rebuild server**: `cargo build --release`
4. **Test with local model**: `inference_server.exe --model-path "models/Qwen3-0.6B-BF16.gguf"`

## Model Files Available

All models are in: `N:\noa\ml_devops_platform\rust_backend\models\`

```
DeepSeek-R1-Distill-Qwen-1.5B-Q8_0.gguf       (1.81 GB)
gemma-3-1b-it-BF16.gguf                       (1.91 GB)
gemma-3-4b-it-qat-UD-Q4_K_XL.gguf             (2.42 GB)
gemma-3n-E2B-it-UD-Q4_K_XL.gguf               (3.58 GB)
Phi-4-mini-reasoning-UD-Q4_K_XL.gguf          (2.35 GB)
Qwen3-0.6B-BF16.gguf                          (1.14 GB)
Qwen3-4B-Q4_K_M.gguf                          (2.38 GB)
```

## Current Behavior

The server is running with "intelligent fallback" mode, which means:
- ✅ API endpoints are functional
- ✅ OpenAI-compatible responses
- ⚠️ Responses are mock/fallback (not real inference)
- 🔄 Real inference requires code modification to load local GGUF files

See `MODEL_INVENTORY.md` for detailed model information.
