# Shared AI Resources

All AI providers share these resources, allowing seamless provider switching without recreating configurations.

## Directory Structure

- **agents/**: AI agents that work across all providers
- **workflows/**: Multi-step AI workflows
- **prompts/**: Reusable prompt templates
- **skills/**: AI capabilities and skills
- **tools/**: AI tools and utilities
- **models/**: Model files (for local providers like llama.cpp)
- **commands/**: Slash commands and terminal commands

## Provider Priority

1. **Local providers** (llama.cpp, ollama) - Highest priority
2. **Hybrid providers** - Local-first with cloud fallback
3. **Cloud providers** - Used when local unavailable

## Usage

All providers automatically use resources from these directories. Switching providers maintains all configurations, agents, workflows, and prompts.
