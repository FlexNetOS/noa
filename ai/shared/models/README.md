# Models Directory

This directory contains model adapter configurations and metadata.

## Purpose

Model configs define how to interact with different AI models,
including local (llama.cpp, Ollama) and cloud (OpenAI, Anthropic) providers.

---

## Supported Models

The following models are supported for local inference via llama.cpp/Ollama. All models use GGUF format from Unsloth's optimized quantizations.

### Model Registry Table

| Model | Family | Size | Quantization | Context | Capabilities | License | HuggingFace |
|-------|--------|------|--------------|---------|--------------|---------|-------------|
| [gemma-3n-E2B-it](#gemma-3n-e2b-it) | Gemma 3n | 2B (4B total) | Q4_K_XL | 32K | Text, Vision, Audio | Gemma | [unsloth/gemma-3n-E2B-it-GGUF](https://huggingface.co/unsloth/gemma-3n-E2B-it-GGUF) |
| [Phi-4-mini-reasoning](#phi-4-mini-reasoning) | Phi-4 | 3.8B | Q4_K_XL | 128K | Math, Reasoning, Code | MIT | [unsloth/Phi-4-mini-reasoning-GGUF](https://huggingface.co/unsloth/Phi-4-mini-reasoning-GGUF) |
| [DeepSeek-R1-Distill-Qwen-1.5B](#deepseek-r1-distill-qwen-15b) | DeepSeek R1 | 1.5B | Q8_0 | 128K | Reasoning, Math, Code | Apache-2.0 | [unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF](https://huggingface.co/unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF) |
| [gemma-3-4b-it-qat](#gemma-3-4b-it-qat) | Gemma 3 | 4B | Q4_K_XL | 128K | Text, Vision, Multilingual | Gemma | [unsloth/gemma-3-4b-it-qat-GGUF](https://huggingface.co/unsloth/gemma-3-4b-it-qat-GGUF) |
| [gemma-3-1b-it](#gemma-3-1b-it) | Gemma 3 | 1B | BF16 | 32K | Text, Multilingual | Gemma | [unsloth/gemma-3-1b-it-GGUF](https://huggingface.co/unsloth/gemma-3-1b-it-GGUF) |
| [Qwen3-0.6B](#qwen3-06b) | Qwen3 | 0.6B | BF16 | 32K | Reasoning, Agent, Multilingual | Apache-2.0 | [unsloth/Qwen3-0.6B-GGUF](https://huggingface.co/unsloth/Qwen3-0.6B-GGUF) |
| [Qwen3-4B](#qwen3-4b) | Qwen3 | 4B | Q4_K_M | 32K | Reasoning, Agent, Code | Apache-2.0 | [unsloth/Qwen3-4B-GGUF](https://huggingface.co/unsloth/Qwen3-4B-GGUF) |

### Memory Requirements

| Model | Q4 Size | Q8 Size | BF16 Size | Min VRAM | CPU Fallback |
|-------|---------|---------|-----------|----------|--------------|
| gemma-3n-E2B-it | 3.75 GB | 4.79 GB | 8.92 GB | 4 GB | ✅ |
| Phi-4-mini-reasoning | 2.49 GB | 4.08 GB | 7.68 GB | 4 GB | ✅ |
| DeepSeek-R1-Distill-Qwen-1.5B | 1.12 GB | 1.89 GB | 3.56 GB | 2 GB | ✅ |
| gemma-3-4b-it-qat | 2.54 GB | 4.13 GB | 7.77 GB | 4 GB | ✅ |
| gemma-3-1b-it | 0.81 GB | 1.07 GB | 2.01 GB | 2 GB | ✅ |
| Qwen3-0.6B | 0.40 GB | 0.64 GB | 1.20 GB | 1 GB | ✅ |
| Qwen3-4B | 2.50 GB | 4.28 GB | 8.05 GB | 4 GB | ✅ |

---

## Model Details

### gemma-3n-E2B-it

**Google Gemma 3n E2B Instruct** - Edge-optimized multimodal model with selective parameter activation.

- **Parameters**: 4B total, 2B active (E2B = Effective 2B)
- **Architecture**: gemma3n (transformer with selective activation)
- **Training Data**: 11T tokens, 140+ languages, images, audio
- **Context Length**: 32K tokens
- **Capabilities**: Text generation, image understanding, audio transcription
- **Recommended For**: Edge deployment, mobile devices, resource-constrained environments
- **Quantization**: Q4_K_XL (Dynamic 2.0 - SOTA accuracy)

```bash
# Ollama
ollama run hf.co/unsloth/gemma-3n-E2B-it-GGUF:Q4_K_XL

# llama.cpp
./llama-cli -m gemma-3n-E2B-it-UD-Q4_K_XL.gguf --temp 1.0 --top-k 64 --top-p 0.95
```

---

### Phi-4-mini-reasoning

**Microsoft Phi-4 Mini Reasoning** - Compact reasoning model optimized for mathematical problem-solving.

- **Parameters**: 3.8B
- **Architecture**: phi3 (dense decoder-only transformer)
- **Training Data**: 150B tokens synthetic math data (distilled from DeepSeek-R1)
- **Context Length**: 128K tokens
- **Capabilities**: Multi-step reasoning, math, code, formal proofs
- **Benchmarks**: AIME 2024: 57.5%, MATH-500: 94.6%, GPQA Diamond: 52.0%
- **Recommended For**: Mathematical reasoning, educational tutoring, edge deployment

```bash
# llama.cpp
./llama-cli -m Phi-4-mini-reasoning-UD-Q4_K_XL.gguf \
  --temp 0.8 --top-p 0.95 \
  --prompt '<|system|>Your name is Phi, an AI math expert.<|end|><|user|>Solve x^2 + 4x - 5 = 0<|end|><|assistant|>'
```

---

### DeepSeek-R1-Distill-Qwen-1.5B

**DeepSeek R1 Distill Qwen 1.5B** - Reasoning model distilled from DeepSeek-R1 (671B).

- **Parameters**: 1.5B
- **Architecture**: qwen2 (based on Qwen2.5-Math-1.5B)
- **Training Data**: Distilled from DeepSeek-R1 reasoning data
- **Context Length**: 128K tokens
- **Capabilities**: Chain-of-thought reasoning, self-verification, reflection
- **Benchmarks**: AIME 2024: 28.9%, MATH-500: 83.9%
- **Recommended For**: Lightweight reasoning, edge devices, NOA SLM tasks (<3B requirement)

```bash
# llama.cpp (with Q8 KV cache for quality)
./llama-cli -m DeepSeek-R1-Distill-Qwen-1.5B-Q8_0.gguf \
  --cache-type-k q8_0 --threads 16 \
  --prompt '<｜User｜>What is 1+1?<｜Assistant｜>' -no-cnv
```

---

### gemma-3-4b-it-qat

**Google Gemma 3 4B Instruct QAT** - Quantization-Aware Trained multimodal model.

- **Parameters**: 4B
- **Architecture**: gemma3 (multimodal transformer)
- **Training Data**: 4T tokens, 140+ languages, vision data
- **Context Length**: 128K tokens
- **Capabilities**: Text, vision (896x896 images), multilingual (140+ languages)
- **Benchmarks**: MMLU: 74.5%, GSM8K: 71.0%, HumanEval: 45.7%
- **Recommended For**: General-purpose tasks, document understanding, multilingual applications

```bash
# Ollama
ollama run hf.co/unsloth/gemma-3-4b-it-qat-GGUF:Q4_K_XL
```

---

### gemma-3-1b-it

**Google Gemma 3 1B Instruct** - Smallest Gemma 3 model for ultra-lightweight deployment.

- **Parameters**: 1B
- **Architecture**: gemma3_text (text-only transformer)
- **Training Data**: 2T tokens, 140+ languages
- **Context Length**: 32K tokens
- **Capabilities**: Text generation, summarization, Q&A
- **Benchmarks**: MMLU: 59.6%, GSM8K: 38.4%, HumanEval: 36.0%
- **Recommended For**: Ultra-lightweight deployment, IoT, embedded systems

```bash
# Ollama
ollama run hf.co/unsloth/gemma-3-1b-it-GGUF:BF16
```

---

### Qwen3-0.6B

**Alibaba Qwen3 0.6B** - Smallest Qwen3 model with thinking/non-thinking mode switching.

- **Parameters**: 0.6B (0.44B non-embedding)
- **Architecture**: qwen3 (dense transformer with GQA)
- **Training Data**: Pre-training + post-training
- **Context Length**: 32K tokens
- **Capabilities**: Thinking mode, agent capabilities, 100+ languages, tool calling
- **Recommended For**: Edge agents, IoT, real-time applications, NOA ultra-light tasks

```bash
# Ollama with thinking mode
ollama run hf.co/unsloth/Qwen3-0.6B-GGUF:BF16

# User prompt examples
# Enable thinking: "How many r's in strawberries? /think"
# Disable thinking: "Who are you? /no_think"
```

---

### Qwen3-4B

**Alibaba Qwen3 4B** - Balanced Qwen3 model with full feature set.

- **Parameters**: 4B (3.6B non-embedding)
- **Architecture**: qwen3 (dense transformer, 36 layers, GQA)
- **Training Data**: Pre-training + post-training
- **Context Length**: 32K native, 131K with YaRN
- **Capabilities**: Thinking mode, agent capabilities, 100+ languages, MCP tool calling
- **Recommended For**: General-purpose, agent tasks, code generation, multi-turn dialogue

```bash
# vLLM deployment
vllm serve Qwen/Qwen3-4B --enable-reasoning --reasoning-parser deepseek_r1

# llama.cpp with YaRN for 128K context
./llama-server -m Qwen3-4B-Q4_K_M.gguf \
  --rope-scaling yarn --rope-scale 4 --yarn-orig-ctx 32768
```

---

## NOA Recommended Configurations

### Primary SLM (<3B params per Constitution §3.3)

| Use Case | Model | Quantization | Notes |
|----------|-------|--------------|-------|
| **Documentation Generation** | Qwen3-0.6B | BF16 | Ultra-fast, agent-capable |
| **Code Analysis** | DeepSeek-R1-Distill-Qwen-1.5B | Q8_0 | Best reasoning at <2B |
| **General Tasks** | gemma-3-1b-it | BF16 | Multilingual, balanced |

### Extended Models (when resources allow)

| Use Case | Model | Quantization | Notes |
|----------|-------|--------------|-------|
| **Mathematical Reasoning** | Phi-4-mini-reasoning | Q4_K_XL | SOTA math at 3.8B |
| **Multimodal Tasks** | gemma-3n-E2B-it | Q4_K_XL | Vision + Audio |
| **Agent/Tool Calling** | Qwen3-4B | Q4_K_M | Full MCP support |
| **Document Understanding** | gemma-3-4b-it-qat | Q4_K_XL | 128K context |

---

## Model Configuration Format

```json
{
  "$schema": "https://noa.local/schemas/model-config.json",
  "name": "qwen3-4b",
  "version": "1.0.0",
  "description": "Qwen3 4B for agent tasks and general reasoning",
  "family": "qwen3",
  "size": "4b",
  "quantization": "Q4_K_M",
  "provider": "local",
  "backend": "llama.cpp",
  "file": {
    "name": "Qwen3-4B-Q4_K_M.gguf",
    "size_bytes": 2500000000,
    "sha256": "..."
  },
  "capabilities": {
    "chat": true,
    "completion": true,
    "embedding": false,
    "function_calling": true,
    "vision": false,
    "thinking_mode": true
  },
  "parameters": {
    "context_length": 32768,
    "max_output_tokens": 32768,
    "temperature_default": 0.6,
    "top_p_default": 0.95,
    "top_k_default": 20
  },
  "recommended_for": [
    "code_generation",
    "agent_tasks",
    "reasoning"
  ],
  "memory_requirements": {
    "minimum_vram_gb": 4,
    "recommended_vram_gb": 6,
    "cpu_fallback": true
  }
}
```

## Model Categories

1. **Local Models** - Run on local hardware via llama.cpp/Ollama
2. **Cloud Models** - Accessed via API (OpenAI, Anthropic, etc.)
3. **Hybrid Models** - Local with cloud fallback

## Model Selection

The ModelSelector agent uses these configs to choose optimal models:
- Match task requirements to model capabilities
- Consider hardware constraints
- Balance speed vs quality
- Prefer SLMs (<3B) per NOA Constitution

## Sideloading Models

For offline scenarios (FR-152):
1. Copy `.gguf` file to `noa_root/ai/models/`
2. Create matching `model-name.sha256` checksum file
3. System auto-registers on next startup

## Creating a New Model Config

1. Create a new JSON file: `my-model.json`
2. Define capabilities and parameters
3. Specify file location and checksum
4. Register in `../resources/resource-registry.json`

## Related Files

- `noa_root/ai/models/` - Actual model files (not in shared/)
- `../resources/resource-registry.json` - Central registry
- `noa_root/config/ai-providers.json` - Provider configuration
- `noa_root/config/litho.toml` - Litho documentation generator model config

