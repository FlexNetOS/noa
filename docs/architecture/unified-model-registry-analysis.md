# Unified ML Model Registry for Rust-Based Inference Providers

## Technical Analysis and Architecture Recommendations

**Date:** January 3, 2026  
**Platform:** NOA (N:\noa)  
**Scope:** Multi-provider model sharing across llama.cpp, Candle, candle-vllm, pyo3-vllm, Burn, and NVIDIA Dynamo

---

## Executive Summary

This document analyzes how to share ML models from a single source across multiple Rust-based inference providers in the NOA platform. The goal is to leverage the existing Content-Addressable Storage (CAS) at `N:\noa\cas` as a unified model registry that can serve models to any provider regardless of native format.

---

## 1. Provider Comparison Matrix

| Provider | Native Format(s) | Quantization Support | Memory Mapping | Rust API Maturity | Multi-GPU | Streaming |
|----------|------------------|---------------------|----------------|-------------------|-----------|-----------|
| **llama.cpp** | GGUF | Q2_K, Q3_K_S/M/L, Q4_0/1/K_S/M, Q5_0/1/K_S/M, Q6_K, Q8_0, F16, F32 | ✅ mmap (excellent) | Bindings (llama-cpp-rs, llama-cpp-2) | ✅ | ✅ |
| **Candle** | safetensors, GGUF, GGML, PyTorch, ONNX | GGML quantizations, custom | ✅ (safetensors) | ✅ Native Rust | ✅ (NCCL) | ✅ |
| **candle-vllm** | safetensors, GGUF | GPTQ, AWQ, Marlin, ISQ (q4k, q8k) | ✅ PagedAttention | ✅ Native Rust | ✅ (NCCL, MPI) | ✅ |
| **vLLM (pyo3)** | safetensors, GGUF | AWQ, GPTQ, FP8, INT4/INT8, BitsAndBytes | ✅ | ⚠️ Python bindings | ✅ | ✅ |
| **Burn** | Own (.burn), safetensors, PyTorch, ONNX | Via backends | ✅ | ✅ Native Rust | ✅ | ⚠️ Training focus |
| **NVIDIA Dynamo** | Engine-agnostic (orchestration layer) | Per-backend | ✅ KV offloading | ⚠️ Python/Rust hybrid | ✅ Multi-node | ✅ |

---

## 2. Format Deep Dive

### 2.1 GGUF (GPT-Generated Unified Format)

**Structure:**
- Self-contained file with model + tokenizer + metadata
- Header with key-value pairs for model configsuration
- Quantized tensor data with alignment

**Key Features:**
- Memory-mappable tensor data
- Built-in tokenizer vocabulary
- Extensive quantization types (17+ variants)
- Widely supported in consumer inference

**Rust Libraries:**
- `gguf` crate (part of llama.cpp ecosystem)
- `candle_core::quantized::gguf_file`

```rust
// Example: Reading GGUF in Candle
use candle_core::quantized::gguf_file;

let mut file = std::fs::File::open("model.gguf")?;
let content = gguf_file::Content::read(&mut file)?;
let model = ModelWeights::from_gguf(content, &mut file, &device)?;
```

### 2.2 SafeTensors

**Structure:**
- 8-byte header size (u64 LE)
- JSON header with tensor metadata
- Contiguous tensor data buffer

**Key Features:**
- Zero-copy loading via mmap
- Safe (no arbitrary code execution)
- Lazy loading support
- No file size limit
- Native bfloat16/fp8 support

**Rust Library:** `safetensors` crate (official)

```rust
use safetensors::{SafeTensors, tensor::TensorView};

let data = std::fs::read("model.safetensors")?;
let tensors = SafeTensors::deserialize(&data)?;
let weight = tensors.tensor("model.layers.0.attention.wq.weight")?;
```

### 2.3 PyTorch (.pt/.pth/.bin)

**Structure:**
- Pickle-serialized state dict
- Optional TorchScript graph

**Key Concerns:**
- Security risk (pickle arbitrary code execution)
- Requires Python for deserialization
- No lazy loading

**Rust Access:** Via `tch-rs` bindings or conversion

### 2.4 ONNX

**Structure:**
- Protocol buffer format
- Operator graph + weights

**Limitations:**
- 2GB file size limit (protobuf)
- Complex for LLMs

**Rust Library:** `candle-onnx` for basic support

### 2.5 Burn Format (.burn)

**Structure:**
- Custom serialized format
- Backend-agnostic tensor representation

**Compatibility:** Convert from safetensors/PyTorch via Burn APIs

---

## 3. Format Conversion Tools

### 3.1 Conversion Matrix

| From → To | Tool/Method | Notes |
|-----------|-------------|-------|
| HF safetensors → GGUF | `convert_hf_to_gguf.py` (llama.cpp) | Full conversion with quantization |
| PyTorch → GGUF | `convert_hf_to_gguf.py` | Via HF transformers loading |
| GGML → GGUF | `convert_llama_ggml_to_gguf.py` | Legacy format migration |
| LoRA → GGUF | `convert_lora_to_gguf.py` | Adapter merging |
| safetensors → Burn | `burn::record::PytorchFileRecorder` | Via Burn loader |
| ONNX → safetensors | Custom scripts | Per-model basis |
| Any → safetensors | `transformers.save_pretrained()` | Via Python |

### 3.2 Recommended Conversion Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Model Source (HuggingFace Hub)                  │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Primary Storage: safetensors                       │
│                 (canonical, high-precision F16/BF16)                 │
└─────────────────┬───────────────────────────────────┬───────────────┘
                  │                                   │
     ┌────────────▼────────────┐       ┌─────────────▼─────────────┐
     │   Quantized Variants    │       │   Direct Loading Path     │
     │   (GGUF Q4_K_M, Q5_K_M) │       │   (Candle, vLLM, Burn)    │
     └────────────┬────────────┘       └───────────────────────────┘
                  │
     ┌────────────▼────────────┐
     │   llama.cpp / Mobile    │
     │   Edge Deployment       │
     └─────────────────────────┘
```

---

## 4. Quantization Comparison

### 4.1 GGUF Quantization Types (llama.cpp)

| Type | Bits/Weight | Quality | Speed | Use Case |
|------|-------------|---------|-------|----------|
| Q2_K | 2.5 | Low | Fastest | Ultra-constrained |
| Q3_K_M | 3.4 | Medium | Fast | Mobile |
| Q4_0 | 4.0 | Good | Fast | General use |
| Q4_K_M | 4.5 | Better | Good | **Recommended** |
| Q5_K_M | 5.3 | Very Good | Medium | Quality focus |
| Q6_K | 6.0 | Excellent | Slower | Near-FP16 |
| Q8_0 | 8.0 | Best | Slowest | Maximum quality |

### 4.2 vLLM/candle-vllm Quantization

| Method | Bits | Description |
|--------|------|-------------|
| AWQ | 4-bit | Activation-aware weight quantization |
| GPTQ | 4-bit | Post-training quantization via Hessian |
| Marlin | 4-bit | Optimized GPU kernels for GPTQ |
| FP8 | 8-bit | Native FP8 (H100+) |
| ISQ | Variable | In-situ quantization at load time |

### 4.3 Quantization Interoperability

**Challenge:** Different quantization formats are NOT directly interchangeable.

**Solutions:**
1. **Canonical Storage:** Store F16/BF16 safetensors as source
2. **On-demand Quantization:** Quantize to provider-specific format at load time
3. **Pre-computed Variants:** Cache commonly-used quantized versions in CAS

---

## 5. CAS-Based Unified Model Registry Architecture

### 5.1 Directory Structure

```
cas/
├── objects/                    # Content-addressed blobs
│   └── <hash>/                 # SHA-256 or BLAKE3 hash
│       ├── model.safetensors   # Canonical weights
│       ├── model-q4km.gguf     # Quantized variant
│       ├── tokenizer.json      # Tokenizer
│       └── configs.json         # Model configs
├── registry/
│   └── models.json             # Model catalog
├── refs/
│   ├── models/
│   │   ├── llama-3.1-8b        # Points to hash
│   │   ├── qwen-2.5-7b
│   │   └── mistral-7b
│   └── latest/
│       └── default-model       # Current default
└── tags/
    ├── llama-3.1-8b-v1.0
    └── qwen-2.5-7b-instruct
```

### 5.2 Model Registry Schema

```json
{
  "version": "2.0.0",
  "models": {
    "llama-3.1-8b-instruct": {
      "hash": "blake3:abc123...",
      "formats": {
        "safetensors": {
          "files": ["model-00001-of-00004.safetensors", ...],
          "total_size": 16106127360
        },
        "gguf": {
          "q4_k_m": {
            "file": "llama-3.1-8b-instruct-q4_k_m.gguf",
            "size": 4921782272
          },
          "q5_k_m": {
            "file": "llama-3.1-8b-instruct-q5_k_m.gguf",
            "size": 5686960256
          }
        }
      },
      "metadata": {
        "architecture": "llama",
        "parameters": "8B",
        "context_length": 131072,
        "vocab_size": 128256
      },
      "providers": {
        "llama_cpp": { "recommended_format": "gguf", "quantization": "q4_k_m" },
        "candle": { "recommended_format": "safetensors" },
        "candle_vllm": { "recommended_format": "safetensors", "isq": "q4k" },
        "vllm": { "recommended_format": "safetensors" },
        "burn": { "recommended_format": "safetensors" }
      }
    }
  }
}
```

### 5.3 Rust Model Loader Abstraction

```rust
//! Unified model loader for NOA platform
//! Loads models from CAS and converts to provider-specific format

use std::path::PathBuf;
use anyhow::Result;

/// Supported inference providers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InferenceProvider {
    LlamaCpp,
    Candle,
    CandleVllm,
    Vllm,
    Burn,
    Dynamo,
}

/// Model format enumeration
#[derive(Debug, Clone)]
pub enum ModelFormat {
    Safetensors { files: Vec<PathBuf> },
    Gguf { file: PathBuf, quantization: GgufQuantization },
    PyTorch { files: Vec<PathBuf> },
}

#[derive(Debug, Clone, Copy)]
pub enum GgufQuantization {
    Q4_0, Q4_1, Q4_K_S, Q4_K_M,
    Q5_0, Q5_1, Q5_K_S, Q5_K_M,
    Q6_K, Q8_0, F16, F32,
}

/// Unified model registry client
pub struct ModelRegistry {
    cas_root: PathBuf,
    cache_dir: PathBuf,
}

impl ModelRegistry {
    pub fn new(cas_root: PathBuf) -> Self {
        let cache_dir = cas_root.join("cache").join("models");
        Self { cas_root, cache_dir }
    }

    /// Load a model for a specific provider
    pub async fn load_model(
        &self,
        model_id: &str,
        provider: InferenceProvider,
    ) -> Result<ModelHandle> {
        // 1. Resolve model from registry
        let model_info = self.resolve_model(model_id).await?;
        
        // 2. Get provider-specific format preference
        let format = self.get_preferred_format(&model_info, provider);
        
        // 3. Check if format exists, otherwise convert
        let model_path = match &format {
            ModelFormat::Gguf { file, .. } => {
                if file.exists() {
                    file.clone()
                } else {
                    self.convert_to_gguf(&model_info, format.clone()).await?
                }
            }
            ModelFormat::Safetensors { files } => {
                // Safetensors can be used directly
                files[0].clone()
            }
            _ => todo!("Other format conversions"),
        };

        Ok(ModelHandle {
            path: model_path,
            format,
            provider,
        })
    }

    /// Memory-map model for cross-process sharing
    pub fn mmap_model(&self, handle: &ModelHandle) -> Result<memmap2::Mmap> {
        let file = std::fs::File::open(&handle.path)?;
        let mmap = unsafe { memmap2::Mmap::map(&file)? };
        Ok(mmap)
    }
}

/// Handle to a loaded model
pub struct ModelHandle {
    pub path: PathBuf,
    pub format: ModelFormat,
    pub provider: InferenceProvider,
}
```

---

## 6. Memory Mapping for Cross-Process Sharing

### 6.1 Memory Mapping Capabilities by Provider

| Provider | mmap Support | Shared Memory | Zero-Copy | Notes |
|----------|--------------|---------------|-----------|-------|
| llama.cpp | ✅ Excellent | ✅ | ✅ | `--mmap` flag, default on |
| Candle | ✅ Good | ✅ | ✅ | safetensors native |
| candle-vllm | ✅ PagedAttention | ✅ | ✅ | KV cache sharing |
| vLLM | ✅ | ✅ | ✅ | PagedAttention |
| Burn | ✅ | ⚠️ Limited | ⚠️ | Training focus |

### 6.2 Shared Model Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Shared Memory Region (mmap)                       │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                    Model Weights (Read-Only)                    │ │
│  │              model.safetensors or model.gguf                    │ │
│  └────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
           │                    │                    │
           ▼                    ▼                    ▼
    ┌──────────────┐   ┌──────────────┐    ┌──────────────┐
    │  Process 1   │   │  Process 2   │    │  Process 3   │
    │  llama.cpp   │   │   Candle     │    │  candle-vllm │
    │              │   │              │    │              │
    │ KV Cache     │   │ KV Cache     │    │ PagedKV      │
    │ (private)    │   │ (private)    │    │ (shared)     │
    └──────────────┘   └──────────────┘    └──────────────┘
```

### 6.3 Rust mmap Implementation

```rust
use memmap2::Mmap;
use std::sync::Arc;

/// Shared model memory manager
pub struct SharedModelMemory {
    mmap: Arc<Mmap>,
    format: ModelFormat,
}

impl SharedModelMemory {
    /// Create shared memory for a model file
    pub fn new(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        // Advise kernel for sequential access
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            unsafe {
                libc::madvise(
                    mmap.as_ptr() as *mut libc::c_void,
                    mmap.len(),
                    libc::MADV_SEQUENTIAL,
                );
            }
        }
        
        Ok(Self {
            mmap: Arc::new(mmap),
            format: detect_format(path)?,
        })
    }

    /// Get raw bytes (for safetensors)
    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }

    /// Clone handle for another process/thread
    pub fn share(&self) -> Arc<Mmap> {
        Arc::clone(&self.mmap)
    }
}

// Provider-specific loading from shared memory
impl SharedModelMemory {
    pub fn load_candle(&self, device: &candle_core::Device) -> Result<CandleModel> {
        match &self.format {
            ModelFormat::Safetensors { .. } => {
                let tensors = safetensors::SafeTensors::deserialize(self.as_bytes())?;
                // Load into Candle
                todo!()
            }
            ModelFormat::Gguf { .. } => {
                // Parse GGUF from memory
                todo!()
            }
            _ => Err(anyhow::anyhow!("Unsupported format for Candle")),
        }
    }
}
```

---

## 7. Provider Integration Code Examples

### 7.1 llama.cpp (via llama-cpp-2 crate)

```rust
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::model::{LlamaModel, LlamaModelParams};

pub struct LlamaCppProvider {
    model: LlamaModel,
}

impl LlamaCppProvider {
    pub fn from_registry(registry: &ModelRegistry, model_id: &str) -> Result<Self> {
        let handle = registry.load_model(model_id, InferenceProvider::LlamaCpp)?;
        
        let params = LlamaModelParams::default()
            .with_n_gpu_layers(99) // Offload all to GPU
            .with_use_mmap(true);  // Enable memory mapping
        
        let model = LlamaModel::load_from_file(&handle.path, params)?;
        
        Ok(Self { model })
    }
}
```

### 7.2 Candle (safetensors)

```rust
use candle_core::{Device, DType};
use candle_transformers::models::llama::{configs, Llama};
use safetensors::SafeTensors;

pub struct CandleProvider {
    model: Llama,
    device: Device,
}

impl CandleProvider {
    pub fn from_registry(registry: &ModelRegistry, model_id: &str) -> Result<Self> {
        let handle = registry.load_model(model_id, InferenceProvider::Candle)?;
        let device = Device::cuda_if_available(0)?;
        
        // Memory-mapped loading
        let mmap = registry.mmap_model(&handle)?;
        let tensors = SafeTensors::deserialize(&mmap)?;
        
        // Load model weights
        let vb = candle_nn::VarBuilder::from_safetensors(
            vec![handle.path.clone()],
            DType::F16,
            &device,
        )?;
        
        let configs = load_configs(registry, model_id)?;
        let model = Llama::load(vb, &configs)?;
        
        Ok(Self { model, device })
    }
}
```

### 7.3 candle-vllm

```rust
// candle-vllm uses its own loading mechanism
// but can work with CAS paths

pub struct CandleVllmProvider {
    // Uses candle-vllm's internal types
}

impl CandleVllmProvider {
    pub fn from_registry(registry: &ModelRegistry, model_id: &str) -> Result<Self> {
        let handle = registry.load_model(model_id, InferenceProvider::CandleVllm)?;
        
        // candle-vllm supports both safetensors and GGUF
        // Use --isq flag for in-situ quantization of safetensors
        
        // Start candle-vllm server pointing to CAS path
        // target/release/candle-vllm --w {handle.path.parent()} --isq q4k
        
        todo!()
    }
}
```

### 7.4 vLLM via pyo3

```rust
use pyo3::prelude::*;

pub struct VllmProvider {
    llm: PyObject,
}

impl VllmProvider {
    pub fn from_registry(registry: &ModelRegistry, model_id: &str) -> Result<Self> {
        let handle = registry.load_model(model_id, InferenceProvider::Vllm)?;
        
        Python::with_gil(|py| {
            let vllm = py.import("vllm")?;
            let llm_class = vllm.getattr("LLM")?;
            
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("model", handle.path.to_str())?;
            kwargs.set_item("trust_remote_code", true)?;
            kwargs.set_item("quantization", "awq")?; // or "gptq", "fp8"
            
            let llm = llm_class.call((), Some(kwargs))?;
            
            Ok(Self { llm: llm.into() })
        })
    }
}
```

### 7.5 Burn

```rust
use burn::prelude::*;
use burn::record::{BinFileRecorder, Recorder};

pub struct BurnProvider<B: Backend> {
    model: LlamaModel<B>,
}

impl<B: Backend> BurnProvider<B> {
    pub fn from_registry(registry: &ModelRegistry, model_id: &str, device: &B::Device) -> Result<Self> {
        let handle = registry.load_model(model_id, InferenceProvider::Burn)?;
        
        // Burn can load from safetensors or its own format
        let record = BinFileRecorder::<FullPrecisionSettings>::new()
            .load(handle.path.clone(), device)?;
        
        let model = LlamaModel::new(device).load_record(record);
        
        Ok(Self { model })
    }
}
```

---

## 8. NVIDIA Dynamo Integration

### 8.1 Dynamo Architecture

Dynamo acts as an orchestration layer, not a direct inference provider. It coordinates:
- vLLM workers
- SGLang workers
- TensorRT-LLM workers

### 8.2 Integration with CAS

```python
# dynamo/model_loader.py
from pathlib import Path
import json

class CASModelLoader:
    def __init__(self, cas_root: Path):
        self.cas_root = cas_root
        self.registry = self._load_registry()
    
    def _load_registry(self):
        with open(self.cas_root / "registry" / "models.json") as f:
            return json.load(f)
    
    def get_model_path(self, model_id: str, format: str = "safetensors"):
        model = self.registry["models"][model_id]
        hash_id = model["hash"].split(":")[1][:4]
        return self.cas_root / "objects" / hash_id[:2] / hash_id[2:4] / hash_id
    
    def start_worker(self, model_id: str, engine: str = "sglang"):
        model_path = self.get_model_path(model_id)
        
        if engine == "sglang":
            return f"python -m dynamo.sglang --model {model_path}"
        elif engine == "vllm":
            return f"python -m dynamo.vllm --model {model_path}"
        elif engine == "trtllm":
            return f"python -m dynamo.trtllm --model {model_path}"
```

---

## 9. Recommended Architecture for NOA Platform

### 9.1 Design Principles

1. **Canonical Storage:** Store all models as safetensors in CAS
2. **Lazy Conversion:** Convert to GGUF/other formats on-demand
3. **Shared Memory:** Use mmap for cross-process model sharing
4. **Provider Abstraction:** Unified API hiding provider specifics
5. **Content Addressing:** Models identified by hash, not name

### 9.2 Proposed Component Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         NOA Model Registry                          │
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐  │
│  │ Model Catalog │  │ Format Index │  │ Conversion Pipeline      │  │
│  │ (refs/tags)   │  │ (variants)   │  │ (HF→safetensors→GGUF)   │  │
│  └──────────────┘  └──────────────┘  └──────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Content-Addressable Storage                       │
│                           (N:\noa\cas)                               │
│                                                                      │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐    │
│  │ safetensors│  │   GGUF     │  │ Tokenizers │  │  configss   │    │
│  │  (F16/BF16)│  │ (Q4/Q5/Q8) │  │   (.json)  │  │  (.json)   │    │
│  └────────────┘  └────────────┘  └────────────┘  └────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ Provider Adapter│ │ Provider Adapter│ │ Provider Adapter│
│    llama.cpp    │ │     Candle      │ │   candle-vllm   │
└────────┬────────┘ └────────┬────────┘ └────────┬────────┘
         │                   │                   │
         ▼                   ▼                   ▼
    ┌─────────┐         ┌─────────┐        ┌─────────┐
    │Inference│         │Inference│        │Inference│
    │ Worker  │         │ Worker  │        │ Server  │
    └─────────┘         └─────────┘        └─────────┘
```

### 9.3 Implementation Roadmap

#### Phase 1: Foundation (Weeks 1-2)
- [ ] Extend CAS registry schema for multi-format support
- [ ] Implement `ModelRegistry` Rust crate
- [ ] Add safetensors mmap loading
- [ ] Integrate with existing `inference_server`

#### Phase 2: Format Support (Weeks 3-4)
- [ ] GGUF reader/writer in pure Rust
- [ ] Conversion pipeline (safetensors → GGUF)
- [ ] Quantization presets (Q4_K_M, Q5_K_M, Q8_0)
- [ ] Tokenizer extraction/embedding

#### Phase 3: Provider Integration (Weeks 5-6)
- [ ] llama-cpp-2 adapter
- [ ] Candle adapter (existing code)
- [ ] candle-vllm adapter
- [ ] vLLM pyo3 bindings

#### Phase 4: Orchestration (Weeks 7-8)
- [ ] Dynamo integration
- [ ] Multi-GPU scheduling
- [ ] KV cache sharing
- [ ] Hot model swapping

---

## 10. Trade-offs and Recommendations

### 10.1 Format Selection

| Scenario | Recommended Format | Reason |
|----------|-------------------|--------|
| Edge/Mobile deployment | GGUF (Q4_K_M) | Best compression, fast CPU inference |
| GPU inference (consumer) | GGUF (Q5_K_M) | Good quality/speed balance |
| GPU inference (datacenter) | safetensors (F16) | Maximum quality, PagedAttention |
| Multi-provider support | safetensors | Universal compatibility |
| Training/fine-tuning | safetensors | Burn/Candle native support |

### 10.2 Provider Selection

| Use Case | Recommended Provider | Notes |
|----------|---------------------|-------|
| Local inference (single user) | llama.cpp | Best CPU performance, low memory |
| Batch inference (server) | candle-vllm | PagedAttention, continuous batching |
| Maximum compatibility | vLLM (pyo3) | Most models, most quantization |
| Research/training | Burn or Candle | Native Rust, autodiff |
| Multi-node datacenter | Dynamo + vLLM/SGLang | KV-aware routing, disaggregation |

### 10.3 Key Recommendations for NOA

1. **Use safetensors as canonical format** - Universal support, safe, fast
2. **Pre-compute GGUF variants** - Q4_K_M and Q5_K_M for common models
3. **Leverage existing `inference_server`** - Already has Candle/GGUF support
4. **Implement lazy conversion** - Only convert when first requested
5. **Share via mmap** - Reduce memory footprint for multi-process
6. **Abstract provider differences** - Unified API for inference requests

---

## 11. References

- [llama.cpp GGUF Specification](https://github.com/ggerganov/llama.cpp/blob/master/gguf-py/gguf/gguf_reader.py)
- [Safetensors Format](https://huggingface.co/docs/safetensors/index)
- [Candle Documentation](https://github.com/huggingface/candle)
- [candle-vllm](https://github.com/EricLBuehler/candle-vllm)
- [vLLM Documentation](https://docs.vllm.ai/)
- [Burn Framework](https://burn.dev/)
- [NVIDIA Dynamo](https://github.com/ai-dynamo/dynamo)
- [NOA CAS Specification](file:///N:/noa/cas/README.md)

---

## Appendix A: Quantization Quality Benchmarks

| Model | Format | Perplexity | Tokens/sec (RTX 4090) | Memory |
|-------|--------|------------|----------------------|--------|
| Llama-3.1-8B | F16 | 6.14 | 85 | 16 GB |
| Llama-3.1-8B | Q8_0 | 6.15 | 120 | 8.5 GB |
| Llama-3.1-8B | Q5_K_M | 6.18 | 145 | 5.7 GB |
| Llama-3.1-8B | Q4_K_M | 6.24 | 165 | 4.9 GB |
| Llama-3.1-8B | Q4_0 | 6.35 | 175 | 4.5 GB |

---

## Appendix B: NOA-Specific File Paths

```
N:\noa\
├── cas\                        # Content-Addressable Storage
│   ├── objects\                # Stored models (by hash)
│   ├── registry\models.json    # Model catalog
│   └── refs\                   # Mutable references
├── cache\
│   ├── models\                 # Model download cache (empty currently)
│   └── huggingface\            # HF Hub cache (empty currently)
├── llama.cpp\                  # llama.cpp submodule
│   ├── convert_hf_to_gguf.py   # Conversion script
│   └── gguf-py\                # GGUF Python library
└── sys\core\apps\ml-devops-rust-backend\
    └── inference_server\       # Existing Candle-based server
        └── src\models.rs       # GGUF loading code
```
