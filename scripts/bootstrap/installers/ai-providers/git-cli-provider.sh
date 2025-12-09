#!/bin/bash
#
# Git CLI Provider setup for NOA bootstrap (Unix)
#
# Configures Git as an AI provider for local operations.
# Per NOA Constitution §3.3: Agentic Orchestration

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")")}"

# Source libraries
[[ -f "$SCRIPT_DIR/../../lib/logging.sh" ]] && source "$SCRIPT_DIR/../../lib/logging.sh"

# Paths
NOA_BIN="$NOA_ROOT/bin"
PROVIDER_CONFIG="$NOA_ROOT/ai/providers/local/git-cli/config.json"

log_section "NOA Git CLI Provider Setup"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

mkdir -p "$NOA_BIN" "$(dirname "$PROVIDER_CONFIG")"

# Check if Git is available
if ! command -v git &>/dev/null; then
    log_error "Git is not installed"
    echo "Please install Git first: ./git.sh"
    exit 1
fi

GIT_PATH=$(command -v git)
GIT_VERSION=$(git --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)

log_success "Git found: v$GIT_VERSION"
log_info "Git path: $GIT_PATH"

# Create symlink in NOA bin if needed
if [[ ! -L "$NOA_BIN/git" && ! -f "$NOA_BIN/git" ]]; then
    ln -sf "$GIT_PATH" "$NOA_BIN/git"
    log_success "Created symlink: $NOA_BIN/git"
fi

# Create provider config
cat > "$PROVIDER_CONFIG" <<EOF
{
  "name": "git-cli",
  "type": "local",
  "priority": 6,
  "enabled": true,
  "description": "Git CLI as AI provider for version control operations",
  "version": "$GIT_VERSION",
  "cli": {
    "command": "git",
    "binaryPath": {
      "unix": "$NOA_BIN/git"
    }
  },
  "capabilities": {
    "versionControl": true,
    "diffGeneration": true,
    "commitHistory": true,
    "branchManagement": true,
    "mergeConflictResolution": true
  },
  "workflows": {
    "commit": {
      "description": "AI-assisted commit message generation",
      "command": "git commit"
    },
    "diff": {
      "description": "Semantic diff analysis",
      "command": "git diff"
    },
    "log": {
      "description": "Commit history analysis",
      "command": "git log"
    },
    "conflict": {
      "description": "AI-assisted merge conflict resolution",
      "script": "\${NOA_ROOT}/scripts/git-conflict.sh"
    }
  },
  "sharedResourcePath": "\${NOA_ROOT}/ai/shared"
}
EOF

log_success "Created provider config: $PROVIDER_CONFIG"

echo ""
log_success "Git CLI provider setup complete!"

