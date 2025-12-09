#!/bin/bash
#
# Install CLI utilities (jq, ripgrep, fd, bat, fzf, delta) to noa_root/bin/
#
# Per NOA Constitution 3.1: Self-contained installation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/platform.sh" 2>/dev/null || true

# Paths
NOA_BIN="$NOA_ROOT/bin"
TEMP_DIR="$NOA_ROOT/tmp"
FORCE="${1:-}"

log_section "NOA CLI Tools Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

mkdir -p "$NOA_BIN" "$TEMP_DIR"

# ============================================
# jq - JSON processor
# ============================================
install_jq() {
    local version="1.7.1"

    if [[ -x "$NOA_BIN/jq" && "$FORCE" != "--force" ]]; then
        log_success "jq already installed"
        return 0
    fi

    log_info "Installing jq v$version..."

    local binary_name
    case "$os" in
        macos)
            if [[ "$arch" == "arm64" ]]; then
                binary_name="jq-macos-arm64"
            else
                binary_name="jq-macos-amd64"
            fi
            ;;
        *)
            binary_name="jq-linux-amd64"
            ;;
    esac

    local download_url="https://github.com/jqlang/jq/releases/download/jq-${version}/${binary_name}"

    curl -fsSL -o "$NOA_BIN/jq" "$download_url"
    chmod +x "$NOA_BIN/jq"

    log_success "Installed: jq v$version"
}

# ============================================
# ripgrep (rg) - Fast grep
# ============================================
install_ripgrep() {
    local version="14.1.0"

    if [[ -x "$NOA_BIN/rg" && "$FORCE" != "--force" ]]; then
        log_success "ripgrep already installed"
        return 0
    fi

    log_info "Installing ripgrep v$version..."

    local os_suffix arch_suffix
    case "$os" in
        macos)
            if [[ "$arch" == "arm64" ]]; then
                os_suffix="aarch64-apple-darwin"
            else
                os_suffix="x86_64-apple-darwin"
            fi
            ;;
        *)
            os_suffix="x86_64-unknown-linux-musl"
            ;;
    esac

    local archive_name="ripgrep-${version}-${os_suffix}.tar.gz"
    local download_url="https://github.com/BurntSushi/ripgrep/releases/download/${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    local extract_dir="$TEMP_DIR/rg-extract"
    mkdir -p "$extract_dir"
    tar -xzf "$archive_path" -C "$extract_dir"

    local rg_binary
    rg_binary=$(find "$extract_dir" -name "rg" -type f | head -1)
    mv "$rg_binary" "$NOA_BIN/rg"
    chmod +x "$NOA_BIN/rg"
    rm -rf "$extract_dir"

    log_success "Installed: ripgrep v$version"
}

# ============================================
# fd - Fast find
# ============================================
install_fd() {
    local version="10.2.0"

    if [[ -x "$NOA_BIN/fd" && "$FORCE" != "--force" ]]; then
        log_success "fd already installed"
        return 0
    fi

    log_info "Installing fd v$version..."

    local os_suffix
    case "$os" in
        macos)
            if [[ "$arch" == "arm64" ]]; then
                os_suffix="aarch64-apple-darwin"
            else
                os_suffix="x86_64-apple-darwin"
            fi
            ;;
        *)
            os_suffix="x86_64-unknown-linux-musl"
            ;;
    esac

    local archive_name="fd-v${version}-${os_suffix}.tar.gz"
    local download_url="https://github.com/sharkdp/fd/releases/download/v${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    local extract_dir="$TEMP_DIR/fd-extract"
    mkdir -p "$extract_dir"
    tar -xzf "$archive_path" -C "$extract_dir"

    local fd_binary
    fd_binary=$(find "$extract_dir" -name "fd" -type f | head -1)
    mv "$fd_binary" "$NOA_BIN/fd"
    chmod +x "$NOA_BIN/fd"
    rm -rf "$extract_dir"

    log_success "Installed: fd v$version"
}

# ============================================
# bat - Cat with syntax highlighting
# ============================================
install_bat() {
    local version="0.24.0"

    if [[ -x "$NOA_BIN/bat" && "$FORCE" != "--force" ]]; then
        log_success "bat already installed"
        return 0
    fi

    log_info "Installing bat v$version..."

    local os_suffix
    case "$os" in
        macos)
            if [[ "$arch" == "arm64" ]]; then
                os_suffix="aarch64-apple-darwin"
            else
                os_suffix="x86_64-apple-darwin"
            fi
            ;;
        *)
            os_suffix="x86_64-unknown-linux-musl"
            ;;
    esac

    local archive_name="bat-v${version}-${os_suffix}.tar.gz"
    local download_url="https://github.com/sharkdp/bat/releases/download/v${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    local extract_dir="$TEMP_DIR/bat-extract"
    mkdir -p "$extract_dir"
    tar -xzf "$archive_path" -C "$extract_dir"

    local bat_binary
    bat_binary=$(find "$extract_dir" -name "bat" -type f | head -1)
    mv "$bat_binary" "$NOA_BIN/bat"
    chmod +x "$NOA_BIN/bat"
    rm -rf "$extract_dir"

    log_success "Installed: bat v$version"
}

# ============================================
# fzf - Fuzzy finder
# ============================================
install_fzf() {
    local version="0.55.0"

    if [[ -x "$NOA_BIN/fzf" && "$FORCE" != "--force" ]]; then
        log_success "fzf already installed"
        return 0
    fi

    log_info "Installing fzf v$version..."

    local os_suffix arch_suffix
    case "$os" in
        macos) os_suffix="darwin" ;;
        *) os_suffix="linux" ;;
    esac
    case "$arch" in
        arm64) arch_suffix="arm64" ;;
        *) arch_suffix="amd64" ;;
    esac

    local archive_name="fzf-${version}-${os_suffix}_${arch_suffix}.tar.gz"
    local download_url="https://github.com/junegunn/fzf/releases/download/v${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    tar -xzf "$archive_path" -C "$NOA_BIN" fzf
    chmod +x "$NOA_BIN/fzf"

    log_success "Installed: fzf v$version"
}

# ============================================
# delta - Git diff viewer
# ============================================
install_delta() {
    local version="0.18.2"

    if [[ -x "$NOA_BIN/delta" && "$FORCE" != "--force" ]]; then
        log_success "delta already installed"
        return 0
    fi

    log_info "Installing delta v$version..."

    local os_suffix
    case "$os" in
        macos)
            if [[ "$arch" == "arm64" ]]; then
                os_suffix="aarch64-apple-darwin"
            else
                os_suffix="x86_64-apple-darwin"
            fi
            ;;
        *)
            os_suffix="x86_64-unknown-linux-musl"
            ;;
    esac

    local archive_name="delta-${version}-${os_suffix}.tar.gz"
    local download_url="https://github.com/dandavison/delta/releases/download/${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    local extract_dir="$TEMP_DIR/delta-extract"
    mkdir -p "$extract_dir"
    tar -xzf "$archive_path" -C "$extract_dir"

    local delta_binary
    delta_binary=$(find "$extract_dir" -name "delta" -type f | head -1)
    mv "$delta_binary" "$NOA_BIN/delta"
    chmod +x "$NOA_BIN/delta"
    rm -rf "$extract_dir"

    log_success "Installed: delta v$version"
}

# Install all tools
install_jq
install_ripgrep
install_fd
install_bat
install_fzf
install_delta

echo ""
log_success "CLI tools installation complete!"
echo "Tools installed to: $NOA_BIN"

