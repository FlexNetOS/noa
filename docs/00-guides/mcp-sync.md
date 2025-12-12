# MCP Sync (Single Source of Truth)

This repo treats **MCP server configuration as a shared provider concern**, not an editor-specific one.

## Design
- **Shared source of truth (no secrets)**: `config/mcp/servers.json`
- **Generated per-app configs**: `scripts/sync-mcp.ps1`
- **Secrets**: injected via your **NOA profile** (local-only), not committed

## Secrets (NOA profile flow)
1. Create a local secrets file:
   - `%USERPROFILE%\.noa\secrets.ps1`
2. Add your token there:

```powershell
$env:GITHUB_PERSONAL_ACCESS_TOKEN = "github_pat_..."
```

3. Re-run setup (or regenerate your profile) so `noa-profile.ps1` loads that secrets file:
   - `scripts/setup/setup-noa.ps1` generates `noa-profile.ps1` to dot-source `%USERPROFILE%\.noa\secrets.ps1` if present.

## Generate configs
From the repo root:

```powershell
# Generate all supported configs from config/mcp/servers.json
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/sync-mcp.ps1 -NoaRoot .
```

To target one app:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/sync-mcp.ps1 -NoaRoot . -CursorOnly
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/sync-mcp.ps1 -NoaRoot . -WindsurfOnly
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/sync-mcp.ps1 -NoaRoot . -ClaudeDesktopOnly
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/sync-mcp.ps1 -NoaRoot . -VscodeOnly
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/sync-mcp.ps1 -NoaRoot . -VscodeUserOnly
```

## Validate (no secrets printed)

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/sync-mcp.ps1 -NoaRoot . -Check
```

## Notes
- VS Code user settings may be JSONC; if parsing fails, the script falls back to generating `.vscode/mcp.json`.
- If a token was ever printed to logs/terminal output, **revoke/rotate it** in GitHub.


