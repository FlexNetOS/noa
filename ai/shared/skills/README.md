# Skills Directory

This directory contains reusable skill modules for AI agents.

## Purpose

Skills are discrete capabilities that can be composed into agents.
They provide modular functionality like code generation, analysis, or transformation.

## Skill Definition Format

Skills are defined as JSON files:

```json
{
  "$schema": "https://noa.local/schemas/skill.json",
  "name": "code-generation",
  "version": "1.0.0",
  "description": "Generate code from natural language descriptions",
  "category": "development",
  "inputs": [
    {
      "name": "description",
      "type": "string",
      "required": true
    },
    {
      "name": "language",
      "type": "string",
      "required": true,
      "enum": ["python", "typescript", "rust", "go"]
    }
  ],
  "outputs": [
    {
      "name": "code",
      "type": "string"
    },
    {
      "name": "explanation",
      "type": "string"
    }
  ],
  "prompt_template": "../prompts/code-generation.md",
  "tools_required": ["file_write", "syntax_check"],
  "examples": [
    {
      "input": {
        "description": "Create a function that reverses a string",
        "language": "python"
      },
      "output": {
        "code": "def reverse_string(s: str) -> str:\n    return s[::-1]",
        "explanation": "Uses Python slice notation with step -1 to reverse"
      }
    }
  ]
}
```

## Skill Categories

- **development** - Code generation, refactoring, testing
- **analysis** - Code review, security analysis, performance
- **documentation** - README generation, API docs, comments
- **data** - Data transformation, validation, parsing
- **communication** - Summarization, translation, formatting

## Creating a New Skill

1. Create a new JSON file: `my-skill.json`
2. Define inputs, outputs, and prompt template
3. Add examples for few-shot learning
4. Register in `../resources/resource-registry.json`

## Composing Skills into Agents

Agents reference skills to define their capabilities:

```json
{
  "name": "coding-assistant",
  "skills": [
    "code-generation",
    "code-review",
    "test-generation"
  ]
}
```

## Related Files

- `../agents/` - Agents that use these skills
- `../prompts/` - Prompt templates for skills
- `../tools/` - Tools required by skills

