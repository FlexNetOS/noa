#!/bin/bash
#
# configsure provider state synchronization.
#
# Sets up the provider synchronization system that enables state sharing
# between multiple AI providers during collaborative execution.
#
# Usage:
#   ./provider-sync.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"

SHARED_RESOURCES_DIR="$NOA_ROOT/ai/shared/resources"
SYNC_configs_PATH="$SHARED_RESOURCES_DIR/provider-sync-configs.json"

echo -e "\033[36mconfigsuring provider state synchronization...\033[0m"
echo -e "\033[90mconfigs path: $SYNC_configs_PATH\033[0m"
echo ""

# Ensure directory exists
mkdir -p "$SHARED_RESOURCES_DIR"

# Check if configs already exists
if [[ -f "$SYNC_configs_PATH" ]]; then
    echo -e "  \033[90m[EXISTS]\033[0m Provider sync configs already configsured"
    exit 0
fi

# Create sync configsuration
cat > "$SYNC_configs_PATH" << 'EOF'
{
  "$schema": "https://noa.local/schemas/provider-sync.json",
  "version": "1.0.0",
  "description": "Provider state synchronization configsuration (§3.8)",
  "enabled": true,
  "synchronization": {
    "enabled": true,
    "interval_ms": 1000,
    "batch_size": 100,
    "retry_count": 3,
    "retry_delay_ms": 500
  },
  "state_types": {
    "context": {
      "sync": true,
      "ttl_seconds": 3600,
      "max_size_bytes": 1048576
    },
    "reasoning": {
      "sync": true,
      "ttl_seconds": 7200,
      "max_size_bytes": 524288
    },
    "task_state": {
      "sync": true,
      "ttl_seconds": 86400,
      "max_size_bytes": 262144
    },
    "provider_status": {
      "sync": true,
      "ttl_seconds": 60,
      "max_size_bytes": 4096
    }
  },
  "conflict_resolution": {
    "strategy": "last_write_wins",
    "notify_on_conflict": true,
    "log_conflicts": true
  },
  "providers": {
    "auto_discover": true,
    "discovery_paths": [
      "${NOA_ROOT}/ai/providers/local",
      "${NOA_ROOT}/ai/providers/cloud",
      "${NOA_ROOT}/ai/providers/hybrid",
      "${NOA_ROOT}/ai/providers/ide"
    ]
  },
  "database": {
    "path": "${NOA_ROOT}/ai/shared/resources/execution-memory.db",
    "wal_mode": true,
    "busy_timeout_ms": 5000
  }
}
EOF

echo -e "  \033[32m[OK]\033[0m Created provider sync configs: $SYNC_configs_PATH"

# Verify execution memory database exists
DB_PATH="$SHARED_RESOURCES_DIR/execution-memory.db"
if [[ ! -f "$DB_PATH" ]]; then
    echo -e "  \033[33m[WARN]\033[0m Execution memory database not found. Run execution-memory.sh first."
fi

echo ""
echo -e "\033[32mProvider sync configsuration complete.\033[0m"

