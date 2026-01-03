# Shared AI Resources

All AI providers share these resources, allowing seamless provider switching without recreating configurations.

---

## Quick Reference

### Available Tools

| Tool | Category | Description | Permissions |
|------|----------|-------------|-------------|
| [code-generation-tool](tools/code-generation-tool.json) | Development | Generate code from natural language | `file:read` |
| [reasoning-tool](tools/reasoning-tool.json) | Analysis | Complex reasoning and problem-solving | None |
| [documentation-generator](tools/documentation-generator.json) | Documentation | AI-driven docs with Litho/wiki-rs | `file:read`, `file:write` |
| [backupctl](tools/backupctl/) | Operations | Workspace backup orchestration | `file:*`, `shell:execute` |
| [sandboxctl](tools/sandboxctl/) | Operations | Sandbox lifecycle management | `shell:execute` |

### Available Agents

| Agent | Type | Description |
|-------|------|-------------|
| reasoning-agent | Analysis | Advanced reasoning and analysis |
| code-generation-agent | Development | Code generation and refactoring |
| inline-completion-agent | Development | Fast inline code completion |
| orchestration-agent | System | Multi-provider orchestration |

### Available Models (Local SLMs)

| Model | Size | Use Case | Context |
|-------|------|----------|---------|
| [Qwen3-0.6B](models/README.md#qwen3-06b) | 0.6B | Edge agents, IoT | 32K |
| [gemma-3-1b-it](models/README.md#gemma-3-1b-it) | 1B | Ultra-lightweight | 32K |
| [DeepSeek-R1-Distill-Qwen-1.5B](models/README.md#deepseek-r1-distill-qwen-15b) | 1.5B | Reasoning (<3B) | 128K |
| [Phi-4-mini-reasoning](models/README.md#phi-4-mini-reasoning) | 3.8B | Math reasoning | 128K |
| [Qwen3-4B](models/README.md#qwen3-4b) | 4B | Agent tasks, MCP | 32K+ |
| [gemma-3-4b-it-qat](models/README.md#gemma-3-4b-it-qat) | 4B | Multimodal, vision | 128K |
| [gemma-3n-E2B-it](models/README.md#gemma-3n-e2b-it) | 2B (4B) | Edge multimodal | 32K |

---

## Directory Structure

```
ai/shared/
├── agents/          # AI agent definitions
├── commands/        # Slash commands and terminal commands
├── models/          # Model configurations and adapters
├── prompts/         # Reusable prompt templates
├── resources/       # Central resource registry
├── skills/          # AI capabilities and skills
├── tools/           # MCP tool definitions
└── workflows/       # Multi-step AI workflows
```

| Directory | Purpose | Key Files |
|-----------|---------|-----------|
| `agents/` | AI agents that work across all providers | `reasoning-agent.json`, `code-generation-agent.json` |
| `commands/` | Slash commands for IDEs and terminals | `ultrathink.command.yaml` |
| `models/` | Model adapters and configs | `README.md` (model registry) |
| `prompts/` | Prompt templates with variables | `startup-prmt.md`, `ultrathink.prompt.md` |
| `resources/` | Central registry and mappings | `resource-registry.json` |
| `skills/` | Reusable skill modules | `code-generation.json` |
| `tools/` | MCP tool definitions | `README.md` (tool registry) |
| `workflows/` | Multi-step orchestration | `code-analysis-workflow.yaml` |

---

## Provider Priority

Resources are available to all providers. Fallback order per NOA Constitution §3.3:

| Priority | Provider Type | Examples | Notes |
|----------|---------------|----------|-------|
| 1 | **Local** | llama.cpp, Ollama | Always preferred, offline-capable |
| 2 | **Hybrid** | Cursor, Codex | Local-first with cloud fallback |
| 3 | **Cloud** | OpenAI, Anthropic | Used when local unavailable |

---

## Resource Registry

All resources are registered in [`resources/resource-registry.json`](resources/resource-registry.json):

```json
{
  "registry": {
    "agents": { "path": "ai/shared/agents/", "resources": [...] },
    "tools": { "path": "ai/shared/tools/", "resources": [...] },
    "commands": { "path": "ai/shared/commands/", "resources": [...] },
    "prompts": { "path": "ai/shared/prompts/", "resources": [...] },
    "workflows": { "path": "ai/shared/workflows/", "resources": [...] },
    "skills": { "path": "ai/shared/skills/", "resources": [...] },
    "models": { "path": "ai/shared/models/", "resources": [...] }
  }
}
```

---

## Usage

### Provider Switching

All providers automatically use resources from these directories. Switching providers maintains all configurations, agents, workflows, and prompts.

```bash
# Switch between providers - all tools remain available
noa config set provider llama.cpp   # Local inference
noa config set provider ollama      # Ollama server
noa config set provider openai      # Cloud fallback
```

### Tool Invocation

```bash
# Use any tool via CLI
noa tool code-generation-tool --language rust --description "HTTP server with axum"
noa tool reasoning-tool --problem "Optimize this algorithm" --depth deep

# Documentation generation
noa wiki generate --full
noa wiki generate --incremental
```

### Agent Delegation

```bash
# Invoke agents directly
noa agent reasoning-agent --task "Analyze this codebase structure"
noa agent code-generation-agent --spec "REST API for user management"
```

---

## Adding New Resources

### 1. Create Resource File

```bash
# Example: new tool
touch ai/shared/tools/my-new-tool.json
```

### 2. Define Schema

Follow the appropriate schema for the resource type:
- Tools: `https://noa.local/schemas/mcp-tool.json`
- Agents: `https://noa.local/schemas/agent.json`
- Workflows: `https://noa.local/schemas/workflow.json`

### 3. Register in Registry

Add entry to `resources/resource-registry.json`:

```json
{
  "name": "my-new-tool",
  "file": "my-new-tool.json",
  "type": "tool",
  "providers": ["all"],
  "description": "Description of my tool"
}
```

### 4. Update Documentation

- Update `tools/README.md` or `models/README.md` as appropriate
- Add to this file's quick reference tables

---

## Related Documentation

| Document | Purpose |
|----------|---------|
| [tools/README.md](tools/README.md) | Complete tool registry and documentation |
| [models/README.md](models/README.md) | Complete model registry and specifications |
| [CONSTITUTION.md](../../CONSTITUTION.md) | NOA governance principles |
| [config/litho.toml](../../config/litho.toml) | Documentation generator config |
| [config/ai-providers.json](../../config/ai-providers.json) | Provider configurations |
