#!/usr/bin/env bash
set -euo pipefail

NOA_ROOT="${NOA_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)}"
CUDA_INSTALLER="$NOA_ROOT/opt/cuda/cuda_13.1.0_windows.exe"
CUDA_TARGET="$NOA_ROOT/opt/cuda/toolkit"
FORCE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --force) FORCE=true; shift ;;
        *) shift ;;
    esac
done

log() {
    local level=$1; shift
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $*"
}

log "INFO" "CUDA installation skipped on Unix systems"
log "INFO" "CUDA is only required for Windows GPU acceleration"
log "INFO" "For Unix systems, use ROCm or native CUDA installation"
exit 0
