# Commands Directory

This directory contains shared command definitions for AI providers.

## Purpose

Commands define reusable operations that can be invoked by users or workflows.
They provide a consistent interface across all AI providers.

## Command Definition Format

```json
{
  "$schema": "https://noa.local/schemas/command.json",
  "name": "analyze-code",
  "version": "1.0.0",
  "description": "Analyze code for issues, patterns, and improvements",
  "aliases": ["code-review", "review"],
  "category": "development",
  "inputs": [
    {
      "name": "path",
      "type": "string",
      "description": "File or directory to analyze",
      "required": true
    },
    {
      "name": "depth",
      "type": "string",
      "enum": ["shallow", "normal", "deep"],
      "default": "normal",
      "description": "Analysis depth"
    }
  ],
  "workflow": "code-analysis-workflow",
  "agent": "code-analysis-agent",
  "output_format": {
    "type": "object",
    "properties": {
      "summary": { "type": "string" },
      "issues": { "type": "array" },
      "recommendations": { "type": "array" }
    }
  },
  "examples": [
    {
      "invocation": "/analyze-code src/main.rs --depth deep",
      "description": "Deep analysis of Rust source file"
    }
  ]
}
```

## Command Categories

- **development** - Code generation, analysis, testing
- **git** - Version control operations
- **docs** - Documentation generation
- **data** - Data operations
- **system** - System management

## Invoking Commands

Commands can be invoked:
1. Via CLI: `noa analyze-code src/`
2. Via IDE: `/analyze-code src/`
3. Via Workflow: `{ "command": "analyze-code", "inputs": {...} }`

## Creating a New Command

1. Create a new JSON file: `my-command.json`
2. Define inputs and output format
3. Link to agent or workflow
4. Register in `../resources/resource-registry.json`

## Related Files

- `../agents/` - Agents that execute commands
- `../workflows/` - Workflows triggered by commands
- `../resources/resource-registry.json` - Central registry
