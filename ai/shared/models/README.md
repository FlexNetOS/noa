# Models Directory

This directory contains model adapter configurations and metadata.

## Purpose

Model configs define how to interact with different AI models,
including local (llama.cpp, Ollama) and cloud (OpenAI, Anthropic) providers.

## Model Configuration Format

```json
{
  "$schema": "https://noa.local/schemas/model-config.json",
  "name": "qwen2.5-7b-instruct",
  "version": "1.0.0",
  "description": "Qwen2.5 7B Instruct model for general tasks",
  "family": "qwen2.5",
  "size": "7b",
  "quantization": "Q4_K_M",
  "provider": "local",
  "backend": "llama.cpp",
  "file": {
    "name": "qwen2.5-7b-instruct-q4_k_m.gguf",
    "size_bytes": 4368000000,
    "sha256": "abc123..."
  },
  "capabilities": {
    "chat": true,
    "completion": true,
    "embedding": false,
    "function_calling": true,
    "vision": false
  },
  "parameters": {
    "context_length": 32768,
    "max_output_tokens": 8192,
    "temperature_default": 0.7,
    "top_p_default": 0.9
  },
  "recommended_for": [
    "code_generation",
    "general_chat",
    "analysis"
  ],
  "memory_requirements": {
    "minimum_vram_gb": 6,
    "recommended_vram_gb": 8,
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

