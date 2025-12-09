# Tools Directory

This directory contains MCP tool definitions for AI agents.

## Purpose

Tools are functions that AI agents can invoke to interact with external systems.
They follow the Model Context Protocol (MCP) specification for interoperability.

## Tool Definition Format

Tools are defined as JSON files following MCP schema:

```json
{
  "$schema": "https://noa.local/schemas/mcp-tool.json",
  "name": "file_read",
  "version": "1.0.0",
  "description": "Read contents of a file",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "Path to the file to read"
      },
      "encoding": {
        "type": "string",
        "default": "utf-8",
        "description": "File encoding"
      }
    },
    "required": ["path"]
  },
  "outputSchema": {
    "type": "object",
    "properties": {
      "content": {
        "type": "string"
      },
      "size": {
        "type": "integer"
      }
    }
  },
  "implementation": {
    "type": "builtin",
    "handler": "noa.tools.file_read"
  },
  "permissions": ["file:read"],
  "rate_limit": {
    "requests_per_minute": 100
  }
}
```

## Tool Categories

1. **File Operations** - Read, write, search files
2. **Git Operations** - Commit, diff, branch management
3. **Shell Commands** - Execute system commands
4. **HTTP Requests** - API calls, web scraping
5. **Database** - Query, insert, update data
6. **AI Providers** - Call other AI models

## Implementation Types

- **builtin** - Implemented in NOA core
- **script** - External script (bash, python)
- **http** - HTTP endpoint call
- **mcp** - Delegate to MCP server

## Creating a New Tool

1. Create a new JSON file: `my-tool.json`
2. Define input/output schemas
3. Specify implementation and permissions
4. Register in `../resources/resource-registry.json`

## Security Considerations

Tools require explicit permissions. Available permissions:
- `file:read`, `file:write` - File system access
- `shell:execute` - Command execution
- `network:http` - HTTP requests
- `git:read`, `git:write` - Git operations

## Related Files

- `../agents/` - Agents that use these tools
- `../workflows/` - Workflows that invoke tools
- `../resources/resource-registry.json` - Central registry

