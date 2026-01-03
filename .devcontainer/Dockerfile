# NOA Development Container
# Compatible with VS Code, Cursor, DevPod, and GitHub Codespaces
#
# Based on Microsoft's universal development container

FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

LABEL maintainer="NOA Platform <noa+devops@flexnetos.com>"
LABEL description="NOA Development Environment - Unified for all vibe-coding tools"
LABEL org.opencontainers.image.source="https://github.com/flexnetos/noa"

# ═══════════════════════════════════════════════════════════════════
# XDG DIRECTORIES - Created upfront for XDG compliance
# ═══════════════════════════════════════════════════════════════════
ENV NOA_ROOT=/workspaces/noa
ENV XDG_CONFIG_HOME=/workspaces/noa/etc/config
ENV XDG_CACHE_HOME=/workspaces/noa/cache
ENV XDG_DATA_HOME=/workspaces/noa/data
ENV XDG_STATE_HOME=/workspaces/noa/data/state

# Tool-specific cache locations
ENV CARGO_HOME=/workspaces/noa/cache/rust/cargo
ENV RUSTUP_HOME=/workspaces/noa/cache/rust/rustup
ENV npm_config_cache=/workspaces/noa/cache/npm
ENV PNPM_HOME=/workspaces/noa/cache/pnpm
ENV PIXI_CACHE_DIR=/workspaces/noa/cache/pixi
ENV PIP_CACHE_DIR=/workspaces/noa/cache/pip

# ═══════════════════════════════════════════════════════════════════
# SYSTEM DEPENDENCIES
# ═══════════════════════════════════════════════════════════════════
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        # Build essentials
        build-essential \
        cmake \
        ninja-build \
        pkg-config \
        # Rust dependencies
        libssl-dev \
        libsqlite3-dev \
        # Python ML dependencies
        python3-dev \
        python3-pip \
        python3-venv \
        # Git LFS
        git-lfs \
        # Other tools
        jq \
        ripgrep \
        fd-find \
        bat \
        httpie \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# ═══════════════════════════════════════════════════════════════════
# CREATE CACHE DIRECTORIES
# ═══════════════════════════════════════════════════════════════════
RUN mkdir -p \
    ${XDG_CONFIG_HOME} \
    ${XDG_CACHE_HOME} \
    ${XDG_DATA_HOME} \
    ${XDG_STATE_HOME} \
    ${CARGO_HOME} \
    ${RUSTUP_HOME} \
    ${npm_config_cache} \
    ${PNPM_HOME} \
    ${PIXI_CACHE_DIR} \
    ${PIP_CACHE_DIR}

# ═══════════════════════════════════════════════════════════════════
# FINALIZE
# ═══════════════════════════════════════════════════════════════════
WORKDIR /workspaces/noa

# Add cargo and pnpm to path
ENV PATH="${CARGO_HOME}/bin:${PNPM_HOME}:${PATH}"

# Telemetry opt-out
ENV DO_NOT_TRACK=1
ENV NEXT_TELEMETRY_DISABLED=1
