#!/bin/bash
#
# configsure Docker for NOA integration.
#
# Verifies Docker installation and creates configsuration for NOA.
#
# Usage:
#   ./docker.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"

echo -e "\033[36mconfigsuring Docker integration...\033[0m"

# Check if docker is available
if ! command -v docker &> /dev/null; then
    echo -e "  \033[33m[SKIP]\033[0m Docker not found"
    echo -e "  \033[90mInstall Docker from: https://docs.docker.com/engine/install/\033[0m"
    exit 0
fi

DOCKER_PATH="$(command -v docker)"
echo -e "  \033[32mFound:\033[0m $DOCKER_PATH"

# Verify Docker daemon is running
if docker info &> /dev/null; then
    echo -e "  \033[32m[OK]\033[0m Docker daemon is running"
else
    echo -e "  \033[33m[WARN]\033[0m Docker found but daemon is not running"
    echo -e "  \033[90mStart Docker daemon to enable container features\033[0m"
fi

# Get Docker version
VERSION=$(docker --version 2>&1 | head -1)
echo -e "  \033[32m[OK]\033[0m Version: $VERSION"

# Check docker-compose
if command -v docker-compose &> /dev/null; then
    echo -e "  \033[32m[OK]\033[0m docker-compose available"
elif docker compose version &> /dev/null 2>&1; then
    echo -e "  \033[32m[OK]\033[0m docker compose (v2) available"
else
    echo -e "  \033[90m[INFO]\033[0m docker-compose not found (optional)"
fi

# Create NOA Docker configs directory
NOA_DOCKER_configs="$NOA_ROOT/etc/docker"
if [[ ! -d "$NOA_DOCKER_configs" ]]; then
    mkdir -p "$NOA_DOCKER_configs"
    echo -e "  \033[32m[OK]\033[0m Created Docker configs dir: $NOA_DOCKER_configs"
fi

echo ""
echo -e "\033[32mDocker integration configsured.\033[0m"

