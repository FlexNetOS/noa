#!/bin/bash
#
# Configure centralized logging for NOA.
#
# Sets up log directories and rotation for all NOA components.
#
# Usage:
#   ./log-setup.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"
LOGS_DIR="$NOA_ROOT/logs"

echo -e "\033[36mConfiguring centralized logging...\033[0m"
echo -e "\033[90mNOA Root: $NOA_ROOT\033[0m"
echo -e "\033[90mLogs Dir: $LOGS_DIR\033[0m"
echo ""

# Log subdirectories by component
declare -A LOG_DIRS=(
    ["bootstrap"]="Bootstrap and setup logs"
    ["providers"]="AI provider logs"
    ["agents"]="Agent execution logs"
    ["workflows"]="Workflow execution logs"
    ["system"]="System and runtime logs"
    ["audit"]="Audit trail logs"
    ["errors"]="Error logs"
)

for log_name in "${!LOG_DIRS[@]}"; do
    log_path="$LOGS_DIR/$log_name"
    if [[ ! -d "$log_path" ]]; then
        mkdir -p "$log_path"
        echo -e "  \033[32m[CREATE]\033[0m $log_name/ (${LOG_DIRS[$log_name]})"
    else
        echo -e "  \033[90m[EXISTS]\033[0m $log_name/"
    fi
done

# Create log configuration file
LOG_CONFIG_PATH="$LOGS_DIR/log-config.json"
cat > "$LOG_CONFIG_PATH" << EOF
{
  "\$schema": "https://noa.local/schemas/log-config.json",
  "version": "1.0.0",
  "log_root": "$LOGS_DIR",
  "directories": {
    "bootstrap": {"path": "$LOGS_DIR/bootstrap", "description": "Bootstrap and setup logs"},
    "providers": {"path": "$LOGS_DIR/providers", "description": "AI provider logs"},
    "agents": {"path": "$LOGS_DIR/agents", "description": "Agent execution logs"},
    "workflows": {"path": "$LOGS_DIR/workflows", "description": "Workflow execution logs"},
    "system": {"path": "$LOGS_DIR/system", "description": "System and runtime logs"},
    "audit": {"path": "$LOGS_DIR/audit", "description": "Audit trail logs"},
    "errors": {"path": "$LOGS_DIR/errors", "description": "Error logs"}
  },
  "rotation": {
    "enabled": true,
    "max_size_mb": 100,
    "max_files": 10,
    "compress": true
  },
  "retention": {
    "default_days": 30,
    "audit_days": 365,
    "error_days": 90
  },
  "format": {
    "timestamp": "ISO8601",
    "include_process_id": true,
    "include_thread_id": false
  }
}
EOF

echo ""
echo -e "\033[32mLog configuration saved to: $LOG_CONFIG_PATH\033[0m"

# Create .gitignore for logs
GITIGNORE_PATH="$LOGS_DIR/.gitignore"
if [[ ! -f "$GITIGNORE_PATH" ]]; then
    cat > "$GITIGNORE_PATH" << 'EOF'
# Ignore all log files
*.log
*.log.*
*.gz

# Keep directory structure
!.gitignore
EOF
    echo -e "\033[32mCreated .gitignore for logs directory\033[0m"
fi

echo ""
echo -e "\033[32mLog setup complete.\033[0m"

