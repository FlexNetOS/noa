#!/bin/bash
#
# NOA AppData Directory Initialization (Unix)
#
# Creates all necessary AppData/XDG directories within noa_root for FR-001 compliance.
# Ensures all application data stays contained within NOA.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")}"

echo "NOA AppData Directory Initialization"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Define all AppData/XDG directories (FR-001: Self-contained operation)
directories=(
    # Unix XDG directories
    "data"                          # XDG_DATA_HOME
    "etc"                           # XDG_CONFIG_HOME
    "data/cache"                    # XDG_CACHE_HOME
    "data/state"                    # XDG_STATE_HOME
    "tmp/runtime"                   # XDG_RUNTIME_DIR

    # Windows AppData structure (for cross-platform compatibility)
    "data/appdata/roaming"          # APPDATA (when running under Wine/WSL)
    "data/appdata/local"            # LOCALAPPDATA

    # Temp directories
    "tmp"                           # TMPDIR, TEMP, TMP

    # NOA-specific data directories
    "data/memory"                   # Memory store
    "data/knowledge"                # Knowledge graphs
    "data/embeddings"               # Vector embeddings
    "data/artifacts"                # CAS artifact store
    "data/backups"                  # Backup storage
    "data/archives"                 # Archived data

    # Log directories
    "logs"                          # Application logs
    "logs/bootstrap"                # Bootstrap logs
    "logs/audit"                    # Audit logs

    # Common app-specific directories
    "etc/claude"                    # Claude Desktop config
    "etc/abacus"                    # Abacus Desktop config
    "etc/chatgpt"                   # ChatGPT Desktop config
    "data/cache/claude"             # Claude cache
    "data/cache/abacus"             # Abacus cache
    "data/cache/chatgpt"            # ChatGPT cache
)

created=0
existed=0

for dir in "${directories[@]}"; do
    full_path="$NOA_ROOT/$dir"

    if [[ -d "$full_path" ]]; then
        echo "  [EXISTS] $dir"
        ((existed++))
    else
        if mkdir -p "$full_path" 2>/dev/null; then
            echo "  [CREATED] $dir"
            ((created++))
        else
            echo "  [ERROR] Failed to create $dir" >&2
        fi
    fi
done

# Set proper permissions for runtime directory
if [[ -d "$NOA_ROOT/tmp/runtime" ]]; then
    chmod 700 "$NOA_ROOT/tmp/runtime" 2>/dev/null || true
fi

echo ""
echo "Summary:"
echo "  Created: $created directories"
echo "  Existed: $existed directories"
echo "  Total:   $((created + existed)) directories"
echo ""
echo "AppData directory structure initialized!"
echo "All application data will be contained in: $NOA_ROOT/data/"
