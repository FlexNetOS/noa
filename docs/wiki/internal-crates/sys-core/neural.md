# neural Module

Neural network inference and model management.

**Location**: `sys/core/src/neural/`  
**Feature**: `full`

## Overview

Provides local AI inference capabilities:

- Model loading and caching
- Inference execution
- Embedding generation
- Model format support (GGUF, ONNX)

## Key Types

### NeuralModule

Central neural processing unit.

```rust
pub struct NeuralModule {
    models: ModelRegistry,
    inference_engine: InferenceEngine,
    config: NeuralConfig,
}

impl NeuralModule {
    pub async fn load_model(&mut self, path: &Path) -> NoaResult<ModelId>;
    pub async fn generate(&self, model: ModelId, prompt: &str) -> NoaResult<String>;
    pub async fn embed(&self, model: ModelId, text: &str) -> NoaResult<Vec<f32>>;
}
```

### InferenceEngine

Backend abstraction for inference.

```rust
pub enum InferenceEngine {
    LlamaCpp(LlamaCppBackend),
    Ort(OrtBackend),  // ONNX Runtime (ml-devops feature)
}
```

### ModelConfig

Model configuration.

```rust
pub struct ModelConfig {
    pub path: PathBuf,
    pub context_size: usize,
    pub gpu_layers: u32,
    pub threads: usize,
    pub batch_size: usize,
}
```

## Supported Models

| Format | Extension | Backend |
|--------|-----------|---------|
| GGUF | `.gguf` | llama.cpp |
| ONNX | `.onnx` | ORT |
| SafeTensors | `.safetensors` | ORT |

## Default Models

| Model | Purpose | Size |
|-------|---------|------|
| `qwen2.5-coder-7b` | Code generation | 7B |
| `nomic-embed-text` | Text embeddings | 137M |
| `whisper-large-v3` | Speech-to-text | 1.5B |

## Usage

```rust
use noa_core::neural::{NeuralModule, ModelConfig};

async fn example() -> NoaResult<()> {
    let mut neural = NeuralModule::new(NeuralConfig::default());
    
    // Load model
    let model = neural.load_model("models/qwen2.5-coder-7b.gguf").await?;
    
    // Generate text
    let response = neural.generate(model, "Write a Rust function").await?;
    println!("{}", response);
    
    // Generate embeddings
    let embeddings = neural.embed(model, "Hello world").await?;
    
    Ok(())
}
```

## Feature Flags

```toml
[features]
ml-devops = ["ort"]  # Enables ONNX Runtime
```

## See Also

- [vector module](vector.md) — Vector storage
- [learning module](learning.md) — Adaptive learning
- [memory module](memory.md) — Semantic memory
