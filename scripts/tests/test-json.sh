#!/bin/bash
# test-json.sh - Cross-platform JSON testing helper
# Usage: test-json.sh FILE EXPRESSION

set -euo pipefail

FILE="$1"
EXPR="$2"

# Convert Unix path to Windows path if needed for Node.js
NODE_FILE="$FILE"
if [[ "$FILE" == /n/* ]]; then
    NODE_FILE="N:${FILE#/n}"
fi

# Try validators in order: jq, node, python3
if command -v jq >/dev/null 2>&1; then
    jq -e "$EXPR" "$FILE" >/dev/null 2>&1
elif command -v node >/dev/null 2>&1; then
    node -e "const data = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); if (!(${EXPR})) throw new Error('Assertion failed')" "$NODE_FILE" 2>/dev/null
elif command -v python3 >/dev/null 2>&1; then
    python3 -c "import json; data=json.load(open('$FILE')); assert ${EXPR}" 2>/dev/null
else
    echo "ERROR: No JSON validator available"
    exit 1
fi
