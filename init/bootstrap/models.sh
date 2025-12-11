#!/bin/bash
# Bootstrap Model Download
#
# T090: Create bootstrap models.sh script
# US1: Initialize NOA Seed Environment

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"
MODELS_DIR="$NOA_ROOT/opt/models"

echo "Setting up NOA models directory..."

mkdir -p "$MODELS_DIR"

echo "Models directory created at: $MODELS_DIR"
echo ""
echo "To download models, use:"
echo "  noa models download llama-3.2-1b"
echo "  noa models download phi-3-mini"
echo ""
echo "Or manually download .gguf files to: $MODELS_DIR"

