#!/bin/bash
#
# Initialize the shared execution memory database.
#
# Creates the SQLite database for shared provider execution memory.
# This enables context sharing and reasoning state across AI providers.
#
# Usage:
#   ./execution-memory.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"

SHARED_RESOURCES_DIR="$NOA_ROOT/ai/shared/resources"
SCHEMA_DIR="$SHARED_RESOURCES_DIR/schema"
DB_PATH="$SHARED_RESOURCES_DIR/execution-memory.db"
SCHEMA_PATH="$SCHEMA_DIR/execution-memory.sql"

echo -e "\033[36mInitializing shared execution memory database...\033[0m"
echo -e "\033[90mDatabase path: $DB_PATH\033[0m"
echo ""

# Ensure directories exist
mkdir -p "$SHARED_RESOURCES_DIR" "$SCHEMA_DIR"

# Check if database already exists
if [[ -f "$DB_PATH" ]]; then
    echo -e "  \033[90m[EXISTS]\033[0m Execution memory database already initialized"
    exit 0
fi

# Check for SQLite
if ! command -v sqlite3 &> /dev/null; then
    echo -e "  \033[33m[WARN]\033[0m SQLite not found. Database will be created on first use."
    echo -e "  \033[90mSchema file: $SCHEMA_PATH\033[0m"

    # Create placeholder file
    echo "-- Execution memory database placeholder" > "$DB_PATH"
    echo "-- Initialize with: sqlite3 execution-memory.db < schema/execution-memory.sql" >> "$DB_PATH"
    exit 0
fi

# Check if schema exists
if [[ ! -f "$SCHEMA_PATH" ]]; then
    echo -e "\033[31mError:\033[0m Schema file not found: $SCHEMA_PATH. Run create-directories first."
    exit 1
fi

# Initialize database with schema
echo -e "  Creating database with schema..."
sqlite3 "$DB_PATH" < "$SCHEMA_PATH"

echo -e "  \033[32m[OK]\033[0m Execution memory database initialized"

