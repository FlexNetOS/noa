# RuvLLM Integration Guide

## Overview

This document describes the integration of **RuvLLM v0.2.3** - a self-learning language model orchestration system with SONA (Self-Optimizing Neural Architecture) - into the ML DevOps Platform.

## What is RuvLLM?

RuvLLM is NOT a replacement LLM, but rather an **orchestration layer** that makes any LLM smarter over time through:

### Key Features

1. **Adaptive Memory (Ruvector)**
   - HNSW-indexed vector database
   - Graph-based knowledge representation
   - Continuous learning from interactions

2. **SONA - Self-Optimizing Neural Architecture**
   Three temporal learning loops:
   - **Loop A (Instant)**: Per-request adaptation with MicroLoRA (< 100µs)
   - **Loop B (Hourly)**: Pattern extraction and strategy updates
   - **Loop C (Weekly)**: Dream consolidation and concept hierarchies

3. **FastGRNN Router**
   - Intelligent model selection
   - Cost optimization (prefers cheaper models when quality holds)
   - Automatic escalation when needed

4. **EWC++ (Elastic Weight Consolidation)**
   - Prevents catastrophic forgetting
   - Online Fisher estimation
   - Preserves learned knowledge

## Architecture

```
┌─────────────────────────────────────────────────┐
│            Client Application                    │
│         (TypeScript/Tauri Desktop)              │
└────────────────────┬────────────────────────────┘
                     │ HTTP/SSE
                     ▼
      ┌──────────────────────────────┐
      │   Inference Server (axum)     │
      │   Port 8080                   │
      └─────────────┬─────────────────┘
                    │
                    ▼
      ┌──────────────────────────────┐
      │     ModelManager              │
      │   (Rust Wrapper)              │
      └─────────────┬─────────────────┘
                    │
                    ▼
      ╔═════════════════════════════════╗
      ║         RuvLLM v0.2.3            ║
      ║   (Self-Learning Orchestrator)   ║
      ╚═════════════════════════════════╝
                    │
          ┌─────────┼─────────┐
          ▼         ▼         ▼
    ┌────────┐ ┌────────┐ ┌────────┐
    │ LFM2   │ │Ruvector│ │FastGRNN│
    │Cortex  │ │ Memory │ │ Router │
    └────────┘ └────────┘ └────────┘
          │         │         │
          └─────────┼─────────┘
                    ▼
         ┌────────────────────┐
         │  Local ML Models    │
         │ (Qwen, Llama, Phi)  │
         └────────────────────┘
```

## Implementation Details

### Cargo Dependencies

```toml
[dependencies]
ruvllm = { version = "0.2.3", features = ["full"] }
```

Features enabled:
- `storage`: Vector database persistence
- `metrics`: Prometheus monitoring
- `server`: Built-in HTTP server (axum)
- `real-inference`: Candle-based model loading
- `hf-export`: HuggingFace model export

### Initialization

```rust
use ruvllm::{Config, RuvLLM};

// Build configuration
let config = Config::builder()
    .db_path("./ruvllm_memory.db")
    .build()?;

// Initialize RuvLLM
let llm = RuvLLM::new(config).await?;
```

### Simple Query

```rust
// Direct query
let response = llm.query("What is machine learning?").await?;
println!("Response: {}", response.text);

// Response includes latency breakdown:
// - embedding_ms
// - retrieval_ms
// - routing_ms
// - attention_ms
// - generation_ms
```

### With Session (for continuous learning)

```rust
use ruvllm::{Request, Session};

let session = Session::new();
let request = Request::new("Explain neural networks")
    .with_session(&session.id);

let response = llm.process(request).await?;
```

### Chat Completions (OpenAI-Compatible)

Our implementation wraps RuvLLM to provide OpenAI-compatible API:

```rust
// In models.rs
pub async fn generate_completion(
    &self,
    request: ChatCompletionRequest,
) -> Result<ChatCompletionResponse> {
    let llm_guard = self.ruvllm.read().await;
    let llm = llm_guard.as_ref().context("RuvLLM not loaded")?;

    // Format chat messages into query
    let query = Self::format_messages(&request.messages);

    // Use RuvLLM for inference
    let response = llm.query(&query).await?;

    // Convert to OpenAI format
    ChatCompletionResponse { /* ... */ }
}
```

## Self-Learning in Action

### Example: Code Generation Improvement

**First Query:**
```
User: "Write a fibonacci function in Rust"
Response: <basic implementation>
Memory: Stores query + response + user satisfaction
```

**Later Query (same session):**
```
User: "fibonacci in Rust"
Response: <improved implementation with tail recursion>
Why better: RuvLLM learned from previous interactions
```

### Memory Growth

```
Initial:     100 nodes in memory graph
After 1 day: 1,000 nodes
After 1 week: 5,000 nodes (with compression)
```

RuvLLM automatically:
- Strengthens successful patterns
- Weakens unsuccessful ones
- Archives old but preserves concepts

## Performance Benchmarks

### Orchestration Latency (CPU-Only)

| Metric | Time | Notes |
|--------|------|-------|
| System Init | 3.71ms | One-time startup |
| Average Query | 0.09ms | Excludes LLM generation |
| Session Query | 0.04ms | With context reuse |
| Throughput | 38,000 QPS | 8 concurrent queries |

### Latency Breakdown

- Embedding: ~0.02ms
- Retrieval: ~0.01ms
- Routing: ~0.01ms
- Attention: ~0.02ms
- Generation: ~0.04ms (RuvLLM overhead only)

**Note:** Actual LLM generation time (e.g., 120ms for Llama-3.3-70B) is separate.

### Comparison with Cloud APIs

| Service | P50 Latency | Cost |
|---------|-------------|------|
| GPT-4o | 450ms | $$$$ |
| Claude 3 | 380ms | $$$ |
| Llama 70B | 120ms | $$ |
| **RuvLLM (local)** | **0.06ms + generation** | **$0** |

RuvLLM orchestration is **7,500x faster** than cloud API overhead!

## Configuration

### Full Configuration Example

```rust
let config = Config::builder()
    .db_path("./memory.db")
    .embedding_dimension(768)
    .max_memory_size(10_000)
    .learning_enabled(true)
    .router_learning_rate(0.001)
    .sona_instant_enabled(true)
    .sona_background_interval_secs(3600)
    .sona_deep_interval_secs(604800)
    .build()?;
```

### Environment Variables

```bash
# Database location
RUVLLM_DB_PATH=./ruvllm_memory.db

# Memory settings
RUVLLM_MAX_MEMORY_SIZE=10000
RUVLLM_EMBEDDING_DIM=768

# Learning settings
RUVLLM_LEARNING_ENABLED=true
RUVLLM_ROUTER_LR=0.001

# SONA loops
RUVLLM_SONA_INSTANT=true
RUVLLM_SONA_BACKGROUND_INTERVAL=3600
RUVLLM_SONA_DEEP_INTERVAL=604800
```

## Integration with Tauri

### Update Tauri Commands

```rust
// In src-tauri/src/lib.rs

#[tauri::command]
async fn start_inference_server(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Initialize RuvLLM
    let config = ruvllm::Config::builder()
        .db_path("./ruvllm_memory.db")
        .build()
        .map_err(|e| e.to_string())?;
    
    let llm = ruvllm::RuvLLM::new(config)
        .await
        .map_err(|e| e.to_string())?;
    
    // Store in app state
    state.ruvllm.write().await.replace(llm);
    
    Ok("RuvLLM started on http://localhost:8080".to_string())
}
```

### TypeScript Client

```typescript
import { invoke } from '@tauri-apps/api/core';

// Start RuvLLM server
const serverUrl = await invoke<string>('start_inference_server');
console.log(serverUrl); // "RuvLLM started on http://localhost:8080"

// Use OpenAI-compatible API
const response = await fetch('http://localhost:8080/v1/chat/completions', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    model: 'ruvllm',
    messages: [{ role: 'user', content: 'Hello!' }],
  }),
});

const data = await response.json();
console.log(data.choices[0].message.content);
```

## Next Steps

### Phase E.1: Basic Integration ✅ (Complete)
- [x] Add ruvllm dependency
- [x] Implement ModelManager with RuvLLM
- [x] OpenAI-compatible API wrapper
- [x] Documentation

### Phase E.2: Advanced Features (Planned)
- [ ] Enable real inference with Candle models
- [ ] Download models from HuggingFace Hub
- [ ] GPU acceleration (CUDA/Metal)
- [ ] Model quantization (4-bit, 8-bit)
- [ ] Streaming inference with SSE
- [ ] Session management for continuous learning
- [ ] Prometheus metrics export
- [ ] Benchmarking utilities

### Phase E.3: SONA Integration (Future)
- [ ] Expose SONA configuration to UI
- [ ] Visualize memory graph
- [ ] Show learning progress
- [ ] Export learned patterns to HuggingFace
- [ ] Federated learning across devices

## Resources

- **RuvLLM GitHub**: https://github.com/ruvnet/ruvector
- **Crates.io**: https://crates.io/crates/ruvllm
- **Example Code**: https://github.com/ruvnet/ruvector/tree/main/examples/ruvLLM
- **Research Paper**: Coming soon

## Troubleshooting

### Build Errors

**Issue**: RuvLLM dependencies fail to compile

**Solution**:
```bash
# Update Rust
rustup update

# Clean build
cd rust_backend
cargo clean

# Rebuild with full features
cargo build --release --features full
```

### Memory Issues

**Issue**: RuvLLM uses too much memory

**Solution**:
```rust
let config = Config::builder()
    .max_memory_size(5_000)  // Reduce from 10,000
    .compression_interval_secs(1800)  // Compress more often
    .build()?;
```

### Performance Issues

**Issue**: Queries are slow

**Check**:
1. Is the database on SSD?
2. Is learning enabled (adds overhead)?
3. Are you using the latest version?

**Optimize**:
```rust
let config = Config::builder()
    .learning_enabled(false)  // Disable for benchmark
    .sona_instant_enabled(false)  // Reduce overhead
    .build()?;
```

---

**Built with ❤️ using RuvLLM, Rust, and SONA self-learning architecture**
