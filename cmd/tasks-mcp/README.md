# NOA Tasks MCP Server

Model Context Protocol (MCP) server for task management integration with the Ruler task system.

## Overview

This MCP server exposes task management capabilities through the standard MCP protocol, allowing AI agents to:

- List and query tasks
- Create new tasks
- Update task status
- Access specs and plans
- Validate plans against governing rules

## Installation

```bash
cd cmd/tasks-mcp
npm install
```

## Usage

### Running the server

```bash
# Development mode with hot reload
npm run dev

# Production
npm run build
npm start
```

### Testing with MCP Inspector

```bash
npm run inspector
```

### Configuring in Claude Desktop

Add to your Claude Desktop config (`%APPDATA%\Claude\claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "noa-tasks": {
      "command": "npx",
      "args": ["tsx", "N:/noa/cmd/tasks-mcp/src/index.ts"]
    }
  }
}
```

## Available Tools

### Task Management

- **list_tasks** - List tasks with filtering by status and format options
- **get_task** - Get detailed information about a specific task
- **create_task** - Create a new task
- **update_task_status** - Update task status with optional notes

### Spec/Plan Management

- **get_spec** - Get specification document content
- **validate_plan** - Validate a plan against governing rules

## Development

### Project Structure

```
src/
├── index.ts          # Main server entry point
├── tools/            # Tool implementations
│   ├── tasks/        # Task management tools
│   └── specs/        # Spec/plan tools
└── shared/           # Shared utilities and types
```

### Adding New Tools

1. Create a new tool file in the appropriate `tools/` subdirectory
2. Register the tool in `index.ts`
3. Add tests in the `tests/` directory

## Integration with Ruler

This MCP server is designed to integrate with the Ruler task management system:

- Tasks are stored in `.ruler/tasks.md` files
- Specs are stored in `.ruler/spec.md` files
- Plans are validated against the constitutional flow

## License

Apache-2.0
