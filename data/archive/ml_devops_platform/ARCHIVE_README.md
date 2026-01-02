# ml_devops_platform Archive

## Status: Ready to Archive

**Date**: 2026-01-02  
**Reason**: Feature parity achieved with new Dioxus UI

## Features Migrated

The following features have been ported to the new unified Dioxus UI at `ui/app/crates/noa-ui-shell/`:

| Feature | Original Location | New Location |
|---------|------------------|--------------|
| Chat | `ml_devops/chat/` | `ui/app/crates/noa-ui-shell/src/chat/` |
| Inference | `ml_devops/inference/` | `ui/app/crates/noa-ui-shell/src/inference/` |
| Settings | `ml_devops/settings/` | `ui/app/crates/noa-ui-shell/src/settings/` |
| Logs | `ml_devops/logs/` | `ui/app/crates/noa-ui-shell/src/logs/` |
| Metrics | `ml_devops/metrics/` | `ui/app/crates/noa-ui-shell/src/metrics/` |
| Provider UI | `ml_devops/providers/` | `ui/app/crates/noa-ui-shell/src/settings/provider_settings.rs` |

## New Components Added

- **noa-hive**: P2P coordination layer (`ui/app/crates/noa-hive/`)
- **noa-api-client**: Shared API client (`gateway/api/client/rust/`)
- **noa-hived**: Daemon service (`ui/app/bins/noa-ui-hived/`)
- **Agent Sandbox**: Isolation framework (`sandbox/agents/`)

## Archive Instructions

To complete the archive, run:

```powershell
# Move to archive
Move-Item -Path "N:\noa\ml_devops_platform" -Destination "N:\noa\data\archive\ml_devops_platform"
```

Or using Git:

```bash
git mv ml_devops_platform data/archive/ml_devops_platform
git commit -m "chore: archive ml_devops_platform - feature parity with Dioxus UI"
```

## Rollback

If rollback is needed:

```powershell
Move-Item -Path "N:\noa\data\archive\ml_devops_platform" -Destination "N:\noa\ml_devops_platform"
```
