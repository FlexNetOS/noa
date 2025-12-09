#!/bin/bash
#
# Install security tools (gitleaks, trivy, grype) to noa_root/bin/
#
# Per NOA Constitution 3.6: Security & Privacy

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

# Source libraries
source "$SCRIPT_DIR/../lib/logging.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/platform.sh" 2>/dev/null || true
source "$SCRIPT_DIR/../lib/download.sh" 2>/dev/null || true

# Paths
NOA_BIN="$NOA_ROOT/bin"
TEMP_DIR="$NOA_ROOT/tmp"
FORCE="${1:-}"

log_section "NOA Security Tools Installer"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Determine platform
os=$(get_os 2>/dev/null || echo "linux")
arch=$(get_arch 2>/dev/null || echo "amd64")

mkdir -p "$NOA_BIN" "$TEMP_DIR"

# ============================================
# Gitleaks
# ============================================
install_gitleaks() {
    local version="8.21.2"

    if [[ -x "$NOA_BIN/gitleaks" && "$FORCE" != "--force" ]]; then
        local installed_version
        installed_version=$("$NOA_BIN/gitleaks" version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        log_success "gitleaks already installed: v$installed_version"
        return 0
    fi

    log_info "Installing gitleaks v$version..."

    local os_suffix arch_suffix
    case "$os" in
        macos) os_suffix="darwin" ;;
        *) os_suffix="linux" ;;
    esac
    case "$arch" in
        arm64) arch_suffix="arm64" ;;
        *) arch_suffix="x64" ;;
    esac

    local archive_name="gitleaks_${version}_${os_suffix}_${arch_suffix}.tar.gz"
    local download_url="https://github.com/gitleaks/gitleaks/releases/download/v${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    tar -xzf "$archive_path" -C "$TEMP_DIR" gitleaks
    mv "$TEMP_DIR/gitleaks" "$NOA_BIN/gitleaks"
    chmod +x "$NOA_BIN/gitleaks"

    log_success "Installed: gitleaks v$version"
}

# ============================================
# Trivy
# ============================================
install_trivy() {
    local version="0.58.0"

    if [[ -x "$NOA_BIN/trivy" && "$FORCE" != "--force" ]]; then
        local installed_version
        installed_version=$("$NOA_BIN/trivy" version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        log_success "trivy already installed: v$installed_version"
        return 0
    fi

    log_info "Installing trivy v$version..."

    local os_suffix arch_suffix
    case "$os" in
        macos) os_suffix="macOS" ;;
        *) os_suffix="Linux" ;;
    esac
    case "$arch" in
        arm64) arch_suffix="ARM64" ;;
        *) arch_suffix="64bit" ;;
    esac

    local archive_name="trivy_${version}_${os_suffix}-${arch_suffix}.tar.gz"
    local download_url="https://github.com/aquasecurity/trivy/releases/download/v${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    local extract_dir="$TEMP_DIR/trivy-extract"
    mkdir -p "$extract_dir"
    tar -xzf "$archive_path" -C "$extract_dir"
    mv "$extract_dir/trivy" "$NOA_BIN/trivy"
    chmod +x "$NOA_BIN/trivy"
    rm -rf "$extract_dir"

    log_success "Installed: trivy v$version"
}

# ============================================
# Grype
# ============================================
install_grype() {
    local version="0.85.0"

    if [[ -x "$NOA_BIN/grype" && "$FORCE" != "--force" ]]; then
        local installed_version
        installed_version=$("$NOA_BIN/grype" version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        log_success "grype already installed: v$installed_version"
        return 0
    fi

    log_info "Installing grype v$version..."

    local os_suffix arch_suffix
    case "$os" in
        macos) os_suffix="darwin" ;;
        *) os_suffix="linux" ;;
    esac
    case "$arch" in
        arm64) arch_suffix="arm64" ;;
        *) arch_suffix="amd64" ;;
    esac

    local archive_name="grype_${version}_${os_suffix}_${arch_suffix}.tar.gz"
    local download_url="https://github.com/anchore/grype/releases/download/v${version}/${archive_name}"

    local archive_path="$TEMP_DIR/$archive_name"
    curl -fsSL -o "$archive_path" "$download_url"

    tar -xzf "$archive_path" -C "$TEMP_DIR" grype
    mv "$TEMP_DIR/grype" "$NOA_BIN/grype"
    chmod +x "$NOA_BIN/grype"

    log_success "Installed: grype v$version"
}

# Install all tools
install_gitleaks
install_trivy
install_grype

echo ""
log_success "Security tools installation complete!"
echo "Tools installed to: $NOA_BIN"

