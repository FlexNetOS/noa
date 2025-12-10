#!/bin/bash
# Bootstrap Directory Structure Creation
#
# T088: Create bootstrap dirs.sh script
# US1: Initialize NOA Seed Environment
# §3.1: Self-Contained & Autonomous

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"

echo "Creating NOA directory structure at: $NOA_ROOT"

# Core directories
mkdir -p "$NOA_ROOT/sys/core"
mkdir -p "$NOA_ROOT/sys/services"
mkdir -p "$NOA_ROOT/sys/ui"
mkdir -p "$NOA_ROOT/sys/digest"
mkdir -p "$NOA_ROOT/sys/kernel"

# P2P directory
mkdir -p "$NOA_ROOT/p2p"

# Optional packages
mkdir -p "$NOA_ROOT/opt"

# Initialization
mkdir -p "$NOA_ROOT/init/bootstrap"
mkdir -p "$NOA_ROOT/init/migrations"
mkdir -p "$NOA_ROOT/init/seeds"
mkdir -p "$NOA_ROOT/init/services"

# Containers
mkdir -p "$NOA_ROOT/containers"

# Configuration
mkdir -p "$NOA_ROOT/config/schemas"
mkdir -p "$NOA_ROOT/config/templates"

# Binary directory
mkdir -p "$NOA_ROOT/bin"

# AI directory
mkdir -p "$NOA_ROOT/ai/providers"
mkdir -p "$NOA_ROOT/ai/shared"

# Data directories
mkdir -p "$NOA_ROOT/data/memory"
mkdir -p "$NOA_ROOT/data/knowledge"
mkdir -p "$NOA_ROOT/data/embeddings"
mkdir -p "$NOA_ROOT/data/artifacts"
mkdir -p "$NOA_ROOT/data/modules"
mkdir -p "$NOA_ROOT/data/state"
mkdir -p "$NOA_ROOT/data/cache"
mkdir -p "$NOA_ROOT/data/backups"

# Logs
mkdir -p "$NOA_ROOT/logs"

# Temporary
mkdir -p "$NOA_ROOT/tmp"

echo "✓ Directory structure created"

