# AI Agents Directory

This directory contains unified AI agent definitions shared across all providers.

## Purpose

Agents defined here are accessible to all AI providers (local, cloud, hybrid, IDE)
through the shared resource system. This eliminates provider-specific duplication
and ensures consistent agent behavior across the NOA ecosystem.

## Agent Definition Format

Each agent is defined as a JSON file following this schema:

```json
{
  "$schema": "https://noa.local/schemas/agent.json",
  "name": "reasoning-agent",
  "version": "1.0.0",
  "description": "Advanced reasoning and analysis agent",
  "capabilities": [
    "complex_reasoning",
    "step_by_step_analysis",
    "multi_perspective_evaluation"
  ],
  "system_prompt": "./prompts/reasoning-system.md",
  "tools": ["search", "calculate", "summarize"],
  "memory": {
    "type": "conversation",
    "persistence": "session"
  },
  "provider_hints": {
    "preferred": ["claude-code", "cursor"],
    "fallback": ["ollama", "llama-server"]
  }
}
```

## Creating a New Agent

1. Create a new JSON file: `my-agent.json`
2. Define capabilities and system prompt
3. Reference any required tools from `../tools/`
4. Register in `../resources/resource-registry.json`

## Provider Aliasing

Legacy provider-specific agent names are aliased in `../resources/resource-aliases.json`.
This maintains backward compatibility while encouraging unified naming.

## Related Files

- `../resources/resource-registry.json` - Central registry of all agents
- `../resources/resource-aliases.json` - Provider-specific name mappings
- `../prompts/` - System prompts referenced by agents
- `../tools/` - Tools available to agents

