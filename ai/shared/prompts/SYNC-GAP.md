# Missing Sync Mechanism: Shared Prompts to Cursor Commands

## Problem

Shared prompts in `ai/shared/prompts/` are registered in `resource-registry.json` but are **not automatically synced** to provider-specific command directories like `.cursor/commands/`.

## Current State

1. **Shared Prompts**: Exist in `ai/shared/prompts/` (e.g., `ultrathink.md`)
2. **Resource Registry**: Prompts are registered in `ai/shared/resources/resource-registry.json`
3. **Provider Configs**: Provider configs reference shared resources via `sharedResources` paths
4. **Missing**: No automatic sync from `ai/shared/prompts/` → `.cursor/commands/`

## Why This Matters

- **Cursor** requires commands to be in `.cursor/commands/` as `.md` files to be available as slash commands
- **Other providers** may have similar requirements (e.g., `.claude/commands/`, `.github/agents/`)
- The `sync-resource` command is defined but **not implemented** to actually perform the sync

## Current Workaround

Manually create command files in provider-specific directories:
- `.cursor/commands/ultrathink.md` (created manually)
- `.claude/commands/ultrathink.md` (if needed)

## Required Solution

The `sync-resource` command (or a dedicated sync script) should:

1. Read `resource-registry.json` to find all prompts marked for `["all"]` or specific providers
2. For each provider type:
   - **Cursor**: Copy prompts from `ai/shared/prompts/` → `.cursor/commands/` (convert to command format)
   - **Claude**: Copy prompts from `ai/shared/prompts/` → `.claude/commands/` (convert to command format)
   - **Other providers**: Follow provider-specific patterns
3. Convert prompt format to command format:
   - Add frontmatter with `description`
   - Add `$ARGUMENTS` placeholder if needed
   - Preserve original content

## Implementation Notes

- The `sync-resource.json` command definition exists but needs an actual implementation
- Could be implemented as:
  - PowerShell script: `scripts/sync-shared-prompts.ps1`
  - Bash script: `scripts/sync-shared-prompts.sh`
  - Or as part of the `sync-resource` command workflow

## Related Files

- `ai/shared/commands/sync-resource.json` - Command definition (not implemented)
- `scripts/bootstrap/installers/shared-resources/provider-sync.ps1` - Only updates configs, doesn't sync files
- `ai/shared/resources/resource-registry.json` - Registry of shared resources
- `.cursor/commands/ultrathink.md` - Manually created command file

