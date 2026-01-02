# Model Inventory - ML DevOps Platform

**Location**: `N:\noa\ml_devops_platform\rust_backend\models\`  
**Total Models**: 7  
**Total Size**: 15.23 GB  
**Downloaded**: 2025-01-20

---

## 📦 Downloaded Models

### 1. DeepSeek-R1-Distill-Qwen-1.5B (Q8_0)
- **File**: `DeepSeek-R1-Distill-Qwen-1.5B-Q8_0.gguf`
- **Size**: 1,806.77 MB
- **Quantization**: Q8_0 (8-bit)
- **Source**: [unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF](https://huggingface.co/unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF)
- **Use Case**: Reasoning tasks, distilled from DeepSeek-R1

### 2. Gemma 3 1B Instruct (BF16)
- **File**: `gemma-3-1b-it-BF16.gguf`
- **Size**: 1,913.62 MB
- **Quantization**: BF16 (Brain Float 16)
- **Source**: [unsloth/gemma-3-1b-it-GGUF](https://huggingface.co/unsloth/gemma-3-1b-it-GGUF)
- **Use Case**: General instruction following, high precision

### 3. Gemma 3 4B Instruct QAT (Q4_K_XL)
- **File**: `gemma-3-4b-it-qat-UD-Q4_K_XL.gguf`
- **Size**: 2,419.98 MB
- **Quantization**: Q4_K_XL (4-bit, extra large)
- **Source**: [unsloth/gemma-3-4b-it-qat-GGUF](https://huggingface.co/unsloth/gemma-3-4b-it-qat-GGUF)
- **Use Case**: Quantization-aware trained, balanced performance

### 4. Gemma 3N E2B Instruct (Q4_K_XL)
- **File**: `gemma-3n-E2B-it-UD-Q4_K_XL.gguf`
- **Size**: 3,580.00 MB
- **Quantization**: Q4_K_XL (4-bit, extra large)
- **Source**: [unsloth/gemma-3n-E2B-it-GGUF](https://huggingface.co/unsloth/gemma-3n-E2B-it-GGUF)
- **Use Case**: Enhanced instruction following, larger variant

### 5. Phi-4 Mini Reasoning (Q4_K_XL)
- **File**: `Phi-4-mini-reasoning-UD-Q4_K_XL.gguf`
- **Size**: 2,348.59 MB
- **Quantization**: Q4_K_XL (4-bit, extra large)
- **Source**: [unsloth/Phi-4-mini-reasoning-GGUF](https://huggingface.co/unsloth/Phi-4-mini-reasoning-GGUF)
- **Use Case**: Reasoning and problem-solving tasks

### 6. Qwen3 0.6B (BF16)
- **File**: `Qwen3-0.6B-BF16.gguf`
- **Size**: 1,142.68 MB
- **Quantization**: BF16 (Brain Float 16)
- **Source**: [unsloth/Qwen3-0.6B-GGUF](https://huggingface.co/unsloth/Qwen3-0.6B-GGUF)
- **Use Case**: Lightweight, fast inference, high precision

### 7. Qwen3 4B (Q4_K_M)
- **File**: `Qwen3-4B-Q4_K_M.gguf`
- **Size**: 2,381.59 MB
- **Quantization**: Q4_K_M (4-bit, medium)
- **Source**: [unsloth/Qwen3-4B-GGUF](https://huggingface.co/unsloth/Qwen3-4B-GGUF)
- **Use Case**: General purpose, balanced size/performance

---

## 🎯 Model Selection Guide

### By Size (Smallest to Largest)
1. **Qwen3-0.6B** (1.14 GB) - Fastest, lowest memory
2. **DeepSeek-R1-Distill-Qwen-1.5B** (1.81 GB) - Reasoning focused
3. **Gemma 3 1B** (1.91 GB) - High precision
4. **Phi-4 Mini** (2.35 GB) - Reasoning tasks
5. **Qwen3-4B** (2.38 GB) - General purpose
6. **Gemma 3 4B QAT** (2.42 GB) - Balanced
7. **Gemma 3N E2B** (3.58 GB) - Most capable

### By Quantization
- **BF16** (Highest Quality): `gemma-3-1b-it`, `Qwen3-0.6B`
- **Q8_0** (High Quality): `DeepSeek-R1-Distill-Qwen-1.5B`
- **Q4_K_XL** (Balanced): `gemma-3n-E2B-it`, `Phi-4-mini-reasoning`, `gemma-3-4b-it-qat`
- **Q4_K_M** (Efficient): `Qwen3-4B`

### By Use Case
- **Reasoning**: DeepSeek-R1-Distill, Phi-4 Mini
- **General Chat**: Gemma 3 series, Qwen3 series
- **Fast Inference**: Qwen3-0.6B, Gemma 3 1B
- **Best Quality**: BF16 models (Gemma 3 1B, Qwen3-0.6B)

---

## 🚀 Loading Models

### Using the Inference Server

The server can load models dynamically. To use a specific model:

```powershell
# Example: Chat completion with Qwen3-0.6B
$body = @{
    model = "Qwen3-0.6B-BF16.gguf"
    messages = @(@{role="user"; content="Hello!"})
} | ConvertTo-Json

Invoke-WebRequest -Method POST -Uri http://localhost:8080/v1/chat/completions `
    -ContentType "application/json" -Body $body
```

### Model Loading Configuration

Edit `rust_backend/inference_server/src/models.rs` to set default model:

```rust
// Change the default model in ModelManager::new()
let model_name = "Qwen3-0.6B-BF16.gguf".to_string();
```

---

## 💾 CAS Integration (Content-Addressed Storage)

### Storing Models in CAS

All models should be stored in the NOA CAS for versioning and deduplication:

```bash
# Store a model in CAS
cd /n/noa
HASH=$(bash scripts/cas/store-object.sh \
  ml_devops_platform/rust_backend/models/Qwen3-0.6B-BF16.gguf \
  model \
  '{"name":"Qwen3-0.6B","quantization":"BF16","size_mb":1142.68}')

# Create version tag
bash scripts/cas/create-tag.sh qwen3-0.6b-v1 "$HASH" "Qwen3 0.6B BF16 quantization"

# Update current model pointer
bash scripts/cas/update-ref.sh models/qwen3/current "$HASH" "Deploy Qwen3 0.6B"
```

### Retrieving Models from CAS

```bash
# Retrieve by hash
bash scripts/cas/retrieve-object.sh $HASH /tmp/model.gguf

# Retrieve by tag
HASH=$(grep -oP '"object":\s*"\K[a-f0-9]{64}' cas/tags/qwen3-0.6b-v1)
bash scripts/cas/retrieve-object.sh "$HASH" /tmp/model.gguf

# Retrieve by ref
HASH=$(cat cas/refs/models/qwen3/current)
bash scripts/cas/retrieve-object.sh "$HASH" /tmp/model.gguf
```

### Batch Store All Models

```bash
# Store all models from inventory
cd /n/noa

for model in ml_devops_platform/rust_backend/models/*.gguf; do
  MODEL_NAME=$(basename "$model" .gguf)
  echo "Storing: $MODEL_NAME"

  HASH=$(bash scripts/cas/store-object.sh "$model" model "{\"name\":\"$MODEL_NAME\"}")

  # Create tag
  TAG=$(echo "$MODEL_NAME" | tr '[:upper:]' '[:lower:]' | tr '_' '-')
  bash scripts/cas/create-tag.sh "$TAG" "$HASH" "Model: $MODEL_NAME"

  echo "  Hash: $HASH"
  echo "  Tag: $TAG"
done
```

### Model Registry

Check stored models in CAS registry:

```bash
# View models registry
cat /n/noa/cas/registry/models.json

# List all model tags
ls -lh /n/noa/cas/tags/ | grep -E "(qwen|gemma|deepseek|phi)"

# List all model refs
find /n/noa/cas/refs/models -type f
```

---

## 📊 Performance Estimates

| Model | Size | Quantization | Est. RAM | Est. Speed |
|-------|------|--------------|----------|------------|
| Qwen3-0.6B | 1.14 GB | BF16 | ~2 GB | Very Fast |
| DeepSeek-R1-1.5B | 1.81 GB | Q8_0 | ~2.5 GB | Fast |
| Gemma 3 1B | 1.91 GB | BF16 | ~3 GB | Fast |
| Phi-4 Mini | 2.35 GB | Q4_K_XL | ~3 GB | Medium |
| Qwen3-4B | 2.38 GB | Q4_K_M | ~3.5 GB | Medium |
| Gemma 3 4B QAT | 2.42 GB | Q4_K_XL | ~3.5 GB | Medium |
| Gemma 3N E2B | 3.58 GB | Q4_K_XL | ~5 GB | Slower |

*Estimates assume CPU inference. GPU inference will be significantly faster.*

---

## 🔧 Maintenance

### Verify Models
```powershell
Get-ChildItem N:\noa\ml_devops_platform\rust_backend\models\*.gguf
```

### Download Additional Models
```powershell
cd N:\noa\ml_devops_platform\rust_backend\models
& N:\noa\ai\providers\local\hf-cli\bin\hf.cmd download <repo-id> <filename> --local-dir .
```

### Remove Models
```powershell
Remove-Item N:\noa\ml_devops_platform\rust_backend\models\<model-name>.gguf
```

---

## 📚 References

- **HuggingFace Hub**: [https://huggingface.co/models](https://huggingface.co/models)
- **Unsloth GGUF Models**: [https://huggingface.co/unsloth](https://huggingface.co/unsloth)
- **GGUF Format**: [https://github.com/ggerganov/ggml](https://github.com/ggerganov/ggml)

---

**Last Updated**: 2025-01-20  
**Status**: ✅ All Models Downloaded
