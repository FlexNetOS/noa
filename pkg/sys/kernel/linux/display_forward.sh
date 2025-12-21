#!/usr/bin/env bash
# NDCL display forwarding for Linux containers/VMs (Phase 19 T875)
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: display_forward.sh [--wayland] [--x11] [--gpus all|auto|none] [--container <name>]

Sets up environment variables and volume mounts for forwarding the host display
into a container or VM. Defaults to X11 with GPU passthrough when available.
USAGE
}

X11=true
WAYLAND=false
GPUS="all"
CONTAINER=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --wayland) WAYLAND=true; X11=false; shift ;;
    --x11) X11=true; WAYLAND=false; shift ;;
    --gpus) GPUS="$2"; shift 2 ;;
    --container) CONTAINER="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown argument: $1"; usage; exit 1 ;;
  esac
done

if $X11; then
  export DISPLAY=${DISPLAY:-:0}
  export XAUTHORITY=${XAUTHORITY:-"$HOME/.Xauthority"}
  echo "[NDCL] X11 forwarding enabled: DISPLAY=${DISPLAY}"
  echo "Mount /tmp/.X11-unix into the target container/VM."
fi

if $WAYLAND; then
  export WAYLAND_DISPLAY=${WAYLAND_DISPLAY:-"wayland-0"}
  export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-"/run/user/$(id -u)"}
  echo "[NDCL] Wayland forwarding enabled: ${XDG_RUNTIME_DIR}/${WAYLAND_DISPLAY}"
fi

if [[ -n "${CONTAINER}" ]]; then
  echo "[NDCL] Example docker run:"
  echo "  docker run -e DISPLAY=$DISPLAY \\"
  echo "    -v /tmp/.X11-unix:/tmp/.X11-unix \\"
  if $WAYLAND; then
    echo "    -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY \\"
    echo "    -v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:/tmp/$WAYLAND_DISPLAY \\"
  fi
  echo "    --gpus ${GPUS} ${CONTAINER}"
fi
