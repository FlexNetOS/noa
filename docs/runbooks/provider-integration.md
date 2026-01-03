# NOA Provider Integration Setup

This document describes how AI providers (VS Code Copilot, Cursor, Claude, Codex) are unified under NOA's shared resource architecture.

## Quick Setup

```nu
# From NOA root, run the MCP config generator
nu scripts/generate-mcp-configs.nu

# Authenticate GitHub CLI (required for github MCP server)
N:/noa/bin/gh.exe auth login
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    AgentGateway (Master)                     │
│          gateway/mcp/agentgateway/config/mcp-servers.json    │
└─────────────────────────┬───────────────────────────────────┘
                          │
       ┌──────────────────┼──────────────────┐
       │                  │                  │
       ▼                  ▼                  ▼
┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│   VS Code   │   │   Cursor    │   │   Claude    │
│ .vscode/mcp │   │ .cursor/mcp │   │ etc/claude  │
└─────────────┘   └─────────────┘   └─────────────┘
```

## Shared Resources

| Resource Type | Location | Description |
|---------------|----------|-------------|
| Commands | `ai/shared/commands/` | Provider-agnostic command definitions |
| Tools | `ai/shared/tools/` | MCP tool definitions |
| Prompts | `ai/shared/prompts/` | Template prompts |
| Agents | `ai/shared/agents/` | Agent definitions |
| Chat Sessions | `data/chat-sessions/` | Unified conversation storage |

## Provider-Specific Symlinks

| Provider | Symlink | Target |
|----------|---------|--------|
| Cursor | `.cursor/commands/` | `ai/shared/commands/` |

## MCP Servers

Configured in `gateway/mcp/agentgateway/config/mcp-servers.json`:

| Server | Type | Description | Providers |
|--------|------|-------------|-----------|
| filesystem | stdio | File system access | All |
| github | stdio | GitHub API | All |
| memory | stdio | Knowledge graph | All |
| pylance | stdio | Python LSP | vscode, cursor, claude |
| noa-model-registry | stdio | Local model management | All |
| noa-hive | stdio | P2P model sharing | vscode, cursor, claude |

## Terminal Configuration

NOA uses Nushell (nu) as the default shell for portability:

```
Binary: .pixi/envs/default/bin/nu.exe
Wrapper: bin/nu.cmd
Env: etc/nushell/env.nu
```

### VS Code Terminal Profile

Add to your VS Code settings.json:

```json
{
  "terminal.integrated.profiles.windows": {
    "NOA Nu": {
      "path": "N:\\noa\\bin\\nu.cmd"
    }
  },
  "terminal.integrated.defaultProfile.windows": "NOA Nu"
}
```

## GitHub CLI Auth

The GitHub MCP server requires authentication:

```nu
# Interactive login
N:/noa/bin/gh.exe auth login

# Or set token directly
$env.GITHUB_TOKEN = "ghp_..."
```

## Chat Session Storage

All providers' chat histories can be synced to `data/chat-sessions/`:

```
data/chat-sessions/
├── config.json          # Sync configuration
├── conversations/       # Session JSON files
├── memory/             # Knowledge graph
│   └── knowledge-graph.json
└── history/            # Historical exports
```

## Regenerating Configs

When you modify the master MCP config, regenerate provider configs:

```nu
nu scripts/generate-mcp-configs.nu
```

This updates:
- `.vscode/mcp.json`
- `.cursor/mcp.json`
- `etc/claude/mcp.json`
