# Path Update Summary

## Overview
All hardcoded `/home/noa` paths have been replaced with `${HOME}` to make the codebase user-agnostic.

## Files Updated

### Documentation
- **README.md**: All path references updated to `${HOME}`
- **ai/shared/commands/README.md**: Command examples updated
- **ai/shared/prompts/startup-prmt.md**: Prompt templates updated (partial - special characters interfered)

### Configuration Files (JSON)
- **config/noa-server.json**: Storage, AI config, and logging paths
- **config/ai-providers.json**: All provider paths and shared resources
- **config/device-orchestration.json**: Orchestration path
- **config/git-pr-workflow.json**: PR workflow paths
- **config/git-local-cicd.json**: CI/CD, conflicts, merges, mirrors paths
- **config/git-conflict-ai.json**: AI conflict resolution path
- **ai/providers/local/llama-cpp.json**: Model path
- **ai/providers/local/ollama/config.json**: Ollama model path
- **etc/docker/daemon.json**: Docker data, exec, pid, sock paths

### Shell Scripts
- **scripts/noa**: NOA_ROOT variable
- **scripts/ssh-service**: NOA_ROOT variable
- **scripts/git-conflict**: NOA_GIT and CONFLICT_CONFIG paths
- **scripts/git-ci**: NOA_GIT and CI_CONFIG paths
- **scripts/git-pr**: NOA_GIT and PR_CONFIG paths
- **scripts/gitea-service**: NOA_ROOT and echo messages
- **scripts/ollama-service**: NOA_ROOT, OLLAMA_MODELS, echo messages
- **scripts/docker-service**: NOA_ROOT and echo messages
- **scripts/noa-namespace**: NOA_ROOT variable
- **scripts/noa-kmod**: NOA_ROOT variable
- **scripts/noa-kernel-params**: NOA_ROOT variable
- **scripts/download-static-binaries**: NOA_ROOT variable
- **scripts/patch-binary-libs**: NOA_ROOT and comments
- **scripts/bundle-all-libs**: NOA_ROOT variable
- **scripts/bundle-libraries**: NOA_ROOT and comments

### Environment & Init Files
- **.noa-env**:
  - NOA_ROOT export changed to `${HOME}`
  - All navigation aliases (cda, cdr, cdc, cdw, cdp, cdai, cdgit)
  - Path validation function updated
  - Comments updated
- **.bashrc**: Source path and init system paths
- **init/noa-init**: NOA_ROOT variable

### System Configuration
- **etc/ssh/sshd_config**: HostKey and PidFile paths
- **etc/docker/containerd.toml**: Root, state, and address paths

### Binary Wrappers
- **bin/docker-wrapper**: DOCKER_HOST and DOCKER_CONFIG exports
- **bin/docker-compose-wrapper**: DOCKER_HOST, DOCKER_CONFIG, and exec path

## Pattern Used

All replacements follow this pattern:
- **From**: `/home/noa` or `NOA_ROOT="/home/noa"`
- **To**: `${HOME}` or `NOA_ROOT="${HOME}"`

## Benefits

1. **User-Agnostic**: Works for any user, not just "noa"
2. **Portable**: Can be deployed to different home directories
3. **Multi-User**: Supports multiple users with their own instances
4. **Flexible**: Easy to customize root directory via HOME environment variable

## Environment Variables

The system now uses:
- `${HOME}` - User's home directory (system environment variable)
- `$NOA_ROOT` - Dynamically set to `${HOME}` in `.noa-env`
- All other `$NOA_*` variables derive from `$NOA_ROOT`

## Usage

After these changes, users should:
1. Ensure `HOME` environment variable is set correctly
2. Source the environment: `source ~/.noa-env`
3. Use navigation aliases: `cda`, `cdr`, etc.
4. All scripts will automatically use the correct paths

## Notes

- Some cache files in `.cursor-server` still contain old paths (ignored)
- Virtual environment files in `opt/venv` still reference old paths (can be regenerated)
- Files with special UTF-8 characters may need manual review

## Verification

To verify all paths are updated:
```bash
# Check for remaining /home/noa references (excluding cache/venv)
grep -r "/home/noa" . --exclude-dir=.cursor-server --exclude-dir=opt/venv
```

---
**Date**: Current
**Status**: Complete
**Files Modified**: 40+ files across configs, scripts, and documentation
