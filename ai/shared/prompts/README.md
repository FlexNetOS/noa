# Prompts Directory

This directory contains prompt templates shared across all AI providers.

## Purpose

Prompts defined here can be referenced by any agent or workflow.
This enables consistent prompt engineering and easy iteration.

## Prompt Format

Prompts are stored as Markdown files with optional frontmatter:

```markdown
---
name: code-review-prompt
version: "1.0.0"
variables:
  - name: language
    type: string
    required: true
  - name: code
    type: string
    required: true
  - name: context
    type: string
    required: false
---

# Code Review Instructions

You are reviewing {{language}} code.

## Code to Review

```{{language}}
{{code}}
```

{{#if context}}
## Additional Context

{{context}}
{{/if}}

## Review Checklist

1. Check for bugs and logic errors
2. Evaluate code clarity and maintainability
3. Suggest performance improvements
4. Verify security best practices
```

## Prompt Types

1. **System Prompts** - Define agent personality and capabilities
2. **Task Prompts** - Instructions for specific tasks
3. **Template Prompts** - Reusable prompts with variable substitution

### System Prompt Injection

System prompts can be automatically injected into provider contexts:

| Frontmatter Field | Description |
|-------------------|-------------|
| `type: system` | Marks as a system prompt |
| `inject: always` | Always inject (e.g., `ultrathink.md`) |
| `inject: on-demand` | Inject based on triggers |
| `priority: 1` | Lower = higher precedence |
| `providers: ["all"]` | Which providers receive this prompt |

**Example frontmatter:**
```yaml
---
name: ultrathink
version: "1.0.0"
type: system
providers: ["all"]
inject: always
priority: 1
---
```

Provider configs inherit system prompts via:
```json
"systemPrompts": {
  "enabled": true,
  "inherit": ["ultrathink"],
  "promptPath": "${NOA_ROOT}/ai/shared/prompts"
}
```

## Variable Substitution

Prompts support Handlebars-style templating:
- `{{variable}}` - Simple substitution
- `{{#if condition}}...{{/if}}` - Conditional blocks
- `{{#each items}}...{{/each}}` - Iteration

## Creating a New Prompt

1. Create a new Markdown file: `my-prompt.md`
2. Add frontmatter with name, version, and variables
3. Write the prompt content with template variables
4. Register in `../resources/resource-registry.json`

## Related Files

- `../agents/` - Agents that reference these prompts
- `../resources/resource-registry.json` - Central registry

