#!/usr/bin/env bash
#
# Install all required NOA toolchains in a self-contained way under NOA_ROOT.
# Targets: rust/cargo, go, protoc, golangci-lint, eslint, ruff, gitleaks, trivy, grype, semgrep, gh, node, python
# Default: containment only; set ALLOW_GLOBAL=1 to allow system package managers.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
NOA_ROOT="${NOA_ROOT:-$REPO_ROOT}"
BIN_DIR="$NOA_ROOT/bin"
OPT_DIR="$NOA_ROOT/opt"
DEVTOOLS_DIR="$OPT_DIR/dev-tools"
ALLOW_GLOBAL="${ALLOW_GLOBAL:-0}"
UPDATE_EXISTING="${UPDATE_EXISTING:-0}"

mkdir -p "$BIN_DIR" "$DEVTOOLS_DIR"

info() { printf '[INFO] %s\n' "$*"; }
ok()   { printf '[OK]   %s\n' "$*"; }
warn() { printf '[WARN] %s\n' "$*" >&2; }
err()  { printf '[ERR]  %s\n' "$*" >&2; }

download_and_extract() {
  local url="$1" dest="$2" strip_root="${3:-}"
  local tmp
  tmp="$(mktemp).tar.gz"
  info "Downloading $url"
  curl -fsSL "$url" -o "$tmp"
  mkdir -p "$dest"
  tar -xzf "$tmp" -C "$dest"
  rm -f "$tmp"
  if [[ -n "$strip_root" && -d "$dest/$strip_root" ]]; then
    mv "$dest/$strip_root"/* "$dest"/
    rm -rf "$dest/$strip_root"
  fi
}

link_bin() {
  local source="$1" link_name="$2"
  local target="$BIN_DIR/$link_name"
  rm -f "$target"
  ln -s "$source" "$target"
  ok "Linked $link_name -> $source"
}

should_install() {
  # usage: should_install name1 [name2...]; returns 0 to install, 1 to skip
  local name
  for name in "$@"; do
    if [[ -x "$BIN_DIR/$name" ]]; then
      [[ "$UPDATE_EXISTING" == "1" ]] || return 1
    fi
  done
  return 0
}

install_rust() {
  if ! should_install rustc cargo; then
    info "rust/cargo already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local rust_home="$OPT_DIR/rust"
  mkdir -p "$rust_home"/{rustup,cargo}
  export RUSTUP_HOME="$rust_home/rustup"
  export CARGO_HOME="$rust_home/cargo"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --profile default --default-toolchain stable
  local cargo_bin="$CARGO_HOME/bin"
  link_bin "$cargo_bin/rustc" rustc
  link_bin "$cargo_bin/cargo" cargo
  link_bin "$cargo_bin/rustfmt" rustfmt || true
  link_bin "$cargo_bin/cargo-clippy" cargo-clippy || true
}

install_go() {
  if ! should_install go; then
    info "go already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/go"
  download_and_extract "https://go.dev/dl/go1.23.0.linux-amd64.tar.gz" "$DEVTOOLS_DIR" "go"
  link_bin "$tool_dir/bin/go" go
}

install_protoc() {
  if ! should_install protoc; then
    info "protoc already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/protobuf"
  download_and_extract "https://github.com/protocolbuffers/protobuf/releases/download/v28.0/protoc-28.0-linux-x86_64.zip" "$tool_dir"
  link_bin "$tool_dir/bin/protoc" protoc
}

install_golangci() {
  if ! should_install golangci-lint; then
    info "golangci-lint already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/golangci-lint"
  download_and_extract "https://github.com/golangci/golangci-lint/releases/download/v1.62.0/golangci-lint-1.62.0-linux-amd64.tar.gz" "$tool_dir" "golangci-lint-1.62.0-linux-amd64"
  link_bin "$tool_dir/golangci-lint" golangci-lint
}

install_ruff() {
  if ! should_install ruff; then
    info "ruff already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/ruff"
  download_and_extract "https://github.com/astral-sh/ruff/releases/download/v0.8.1/ruff-0.8.1-linux-x86_64.tar.gz" "$tool_dir"
  link_bin "$tool_dir/ruff" ruff
}

install_gitleaks() {
  if ! should_install gitleaks; then
    info "gitleaks already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/gitleaks"
  download_and_extract "https://github.com/gitleaks/gitleaks/releases/download/v8.21.2/gitleaks_8.21.2_linux_x64.tar.gz" "$tool_dir"
  link_bin "$tool_dir/gitleaks" gitleaks
}

install_trivy() {
  if ! should_install trivy; then
    info "trivy already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/trivy"
  download_and_extract "https://github.com/aquasecurity/trivy/releases/download/v0.57.1/trivy_0.57.1_Linux-64bit.tar.gz" "$tool_dir"
  link_bin "$tool_dir/trivy" trivy
}

install_grype() {
  if ! should_install grype; then
    info "grype already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/grype"
  download_and_extract "https://github.com/anchore/grype/releases/download/v0.84.0/grype_0.84.0_linux_amd64.tar.gz" "$tool_dir"
  link_bin "$tool_dir/grype" grype
}

install_semgrep() {
  if ! should_install semgrep; then
    info "semgrep already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/semgrep"
  mkdir -p "$tool_dir"
  curl -fsSL "https://semgrep.dev/api/cli_v1/releases/latest/download?os=linux&arch=amd64" -o "$tool_dir/semgrep"
  chmod +x "$tool_dir/semgrep"
  link_bin "$tool_dir/semgrep" semgrep
}

install_gh() {
  if ! should_install gh; then
    info "gh already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/gh"
  download_and_extract "https://github.com/cli/cli/releases/download/v2.53.0/gh_2.53.0_linux_amd64.tar.gz" "$tool_dir" "gh_2.53.0_linux_amd64"
  link_bin "$tool_dir/bin/gh" gh
}

install_git() {
  warn "Portable Git on Linux is typically provided by the OS. Install git via package manager or place git in $NOA_BIN."
}

install_gitlfs() {
  if ! should_install git-lfs; then
    info "git-lfs already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/git-lfs"
  download_and_extract "https://github.com/git-lfs/git-lfs/releases/download/v3.5.1/git-lfs-linux-amd64-v3.5.1.tar.gz" "$tool_dir"
  if [[ -x "$tool_dir/git-lfs" ]]; then
    link_bin "$tool_dir/git-lfs" git-lfs
  else
    warn "git-lfs binary not found after extract; install manually."
  fi
}

install_node() {
  if ! should_install node npm npx; then
    info "node/npm already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/node"
  download_and_extract "https://nodejs.org/dist/v20.18.0/node-v20.18.0-linux-x64.tar.gz" "$tool_dir" "node-v20.18.0-linux-x64"
  link_bin "$tool_dir/bin/node" node
  link_bin "$tool_dir/bin/npm" npm
  link_bin "$tool_dir/bin/npx" npx
}

install_python() {
  if ! should_install python; then
    info "python already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  local tool_dir="$DEVTOOLS_DIR/python"
  download_and_extract "https://www.python.org/ftp/python/3.12.7/Python-3.12.7.tgz" "$tool_dir" "Python-3.12.7"
  if command -v make >/dev/null 2>&1; then
    pushd "$tool_dir/Python-3.12.7" >/dev/null
    ./configure --prefix="$tool_dir/python-build" >/dev/null
    make -s -j"$(nproc)" >/dev/null
    make -s install >/dev/null
    popd >/dev/null
    link_bin "$tool_dir/python-build/bin/python3" python
    link_bin "$tool_dir/python-build/bin/pip3" pip
  else
    warn "make not available; skipping python build. Install manually."
  fi
}

install_eslint() {
  if [[ ! -x "$BIN_DIR/node" && ! -x "$BIN_DIR/node.exe" ]]; then
    warn "Node not found in $BIN_DIR. Install node portable before eslint."
    return
  fi
  local npm_bin
  npm_bin="$(dirname "$(command -v npm)")"
  NPM_CONFIG_PREFIX="$DEVTOOLS_DIR/npm-global"
  mkdir -p "$NPM_CONFIG_PREFIX"
  PATH="$NPM_CONFIG_PREFIX/bin:$PATH"
  npm install -g eslint@9.13.0
  if [[ -x "$NPM_CONFIG_PREFIX/bin/eslint" ]]; then
    link_bin "$NPM_CONFIG_PREFIX/bin/eslint" eslint
  fi
}

#region AI Provider CLIs (FR-039)

install_claude_code() {
  if ! should_install claude; then
    info "Claude Code CLI already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  info "Installing Claude Code CLI..."

  # First, try to use the bootstrap installer if available
  local bootstrap_installer="$REPO_ROOT/scripts/bootstrap/installers/ai-providers/claude-code.sh"
  if [[ -x "$bootstrap_installer" ]]; then
    info "  Using bootstrap installer..."
    "$bootstrap_installer" --noa-root "$NOA_ROOT" --method npm
    return
  fi

  # Fallback: install via npm
  if [[ ! -x "$BIN_DIR/node" ]]; then
    warn "Node not found in $BIN_DIR. Install node portable before Claude Code CLI."
    return
  fi

  local npm_cache="$OPT_DIR/npm-cache"
  local npm_prefix="$DEVTOOLS_DIR/npm-global"
  mkdir -p "$npm_cache" "$npm_prefix"
  NPM_CONFIG_CACHE="$npm_cache"
  NPM_CONFIG_PREFIX="$npm_prefix"
  PATH="$npm_prefix/bin:$PATH"

  npm install -g @anthropic-ai/claude-code

  if [[ -x "$npm_prefix/bin/claude" ]]; then
    link_bin "$npm_prefix/bin/claude" claude
    ok "Claude Code CLI installed"
  fi

  # Ensure provider config directory exists
  local provider_config_dir="$NOA_ROOT/ai/providers/cloud/claude-code"
  if [[ ! -d "$provider_config_dir" ]]; then
    mkdir -p "$provider_config_dir"
    info "  Created provider config directory: $provider_config_dir"
  fi
}

install_codex_cli() {
  if ! should_install codex; then
    info "Codex CLI already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  info "Installing Codex CLI..."

  # First, try to use the bootstrap installer if available
  local bootstrap_installer="$REPO_ROOT/scripts/bootstrap/installers/ai-providers/codex-cli.sh"
  if [[ -x "$bootstrap_installer" ]]; then
    info "  Using bootstrap installer..."
    "$bootstrap_installer" --noa-root "$NOA_ROOT" --method npm
    return
  fi

  # Fallback: install via npm
  if [[ ! -x "$BIN_DIR/node" ]]; then
    warn "Node not found in $BIN_DIR. Install node portable before Codex CLI."
    return
  fi

  local npm_cache="$OPT_DIR/npm-cache"
  local npm_prefix="$DEVTOOLS_DIR/npm-global"
  mkdir -p "$npm_cache" "$npm_prefix"
  NPM_CONFIG_CACHE="$npm_cache"
  NPM_CONFIG_PREFIX="$npm_prefix"
  PATH="$npm_prefix/bin:$PATH"

  # Try @openai/codex first, fallback to codex-cli
  npm install -g @openai/codex 2>/dev/null || npm install -g codex-cli

  if [[ -x "$npm_prefix/bin/codex" ]]; then
    link_bin "$npm_prefix/bin/codex" codex
    ok "Codex CLI installed"
  else
    warn "Codex CLI install pending (binary not found after npm). Will retry on next run."
  fi

  # Ensure provider config directory exists
  local provider_config_dir="$NOA_ROOT/ai/providers/cloud/codex"
  if [[ ! -d "$provider_config_dir" ]]; then
    mkdir -p "$provider_config_dir"
    info "  Created provider config directory: $provider_config_dir"
  fi
}

install_cursor_cli() {
  info "Installing Cursor CLI..."

  local tool_dir="$DEVTOOLS_DIR/cursor-cli"
  mkdir -p "$tool_dir"

  # Note: Cursor CLI requires manual download or is bundled with Cursor IDE
  warn "Cursor CLI requires manual installation from https://cursor.com"
  warn "Once installed, add cursor to PATH or link to $BIN_DIR"
}

install_abacus_cli() {
  if ! should_install abacusai; then
    info "Abacus AI CLI already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  info "Installing Abacus AI CLI..."

  if [[ ! -x "$BIN_DIR/node" ]]; then
    warn "Node not found in $BIN_DIR. Install node portable before Abacus CLI."
    return
  fi

  local npm_cache="$OPT_DIR/npm-cache"
  local npm_prefix="$DEVTOOLS_DIR/npm-global"
  mkdir -p "$npm_cache" "$npm_prefix"
  NPM_CONFIG_CACHE="$npm_cache"
  NPM_CONFIG_PREFIX="$npm_prefix"
  PATH="$npm_prefix/bin:$PATH"

  npm install -g @abacus-ai/cli

  if [[ -x "$npm_prefix/bin/abacusai" ]]; then
    link_bin "$npm_prefix/bin/abacusai" abacusai
    ok "Abacus AI CLI installed"
  fi
}

install_vscode_with_copilot() {
  if ! should_install code; then
    info "VS Code already present; skipping (set UPDATE_EXISTING=1 to force)"
    return
  fi
  info "Installing VS Code (portable) with GitHub Copilot..."

  local tool_dir="$DEVTOOLS_DIR/vscode"
  mkdir -p "$tool_dir"

  # Detect platform
  local os_type
  case "$(uname -s)" in
    Linux*)  os_type="linux-x64" ;;
    Darwin*)
      if [[ "$(uname -m)" == "arm64" ]]; then
        os_type="darwin-arm64"
      else
        os_type="darwin-x64"
      fi
      ;;
    *) warn "Unsupported OS for VS Code portable"; return ;;
  esac

  local vscode_url="https://code.visualstudio.com/sha/download?build=stable&os=$os_type"
  local tmp_archive="/tmp/vscode-portable.tar.gz"

  info "  Downloading VS Code portable for $os_type..."
  if curl -fsSL "$vscode_url" -o "$tmp_archive"; then
    info "  Extracting to $tool_dir..."
    tar -xzf "$tmp_archive" -C "$tool_dir" --strip-components=1 2>/dev/null || \
      unzip -q "$tmp_archive" -d "$tool_dir" 2>/dev/null
    rm -f "$tmp_archive"

    # Create portable mode marker
    mkdir -p "$tool_dir/data"

    local code_path="$tool_dir/bin/code"
    if [[ -x "$code_path" ]]; then
      link_bin "$code_path" code
      ok "VS Code portable installed"

      # Install GitHub Copilot extensions
      info "  Installing GitHub Copilot extensions..."
      "$code_path" --install-extension GitHub.copilot --force 2>/dev/null || true
      "$code_path" --install-extension GitHub.copilot-chat --force 2>/dev/null || true
      ok "GitHub Copilot extensions installed"
    fi
  else
    warn "VS Code download failed. Manual install: https://code.visualstudio.com/download"
  fi

  # Ensure provider config directory exists
  local provider_config_dir="$NOA_ROOT/ai/providers/ide/vscode-copilot"
  if [[ ! -d "$provider_config_dir" ]]; then
    mkdir -p "$provider_config_dir"
    info "  Created provider config directory: $provider_config_dir"
  fi
}

install_git_cli_provider() {
  # First ensure Git is installed
  if ! should_install git; then
    info "Git CLI already present as provider"
  else
    install_git
  fi

  info "Configuring Git CLI as AI provider (Priority 6)..."

  # Ensure provider config directory exists
  local provider_config_dir="$NOA_ROOT/ai/providers/local/git-cli"
  if [[ ! -d "$provider_config_dir" ]]; then
    mkdir -p "$provider_config_dir"
    info "  Created provider config directory: $provider_config_dir"
  fi

  ok "Git CLI configured as AI provider"
}

install_all_ai_providers() {
  info "Installing AI Provider CLIs (FR-039)..."

  # Ensure Node is installed first (required for npm-based CLIs)
  if [[ ! -x "$BIN_DIR/node" ]]; then
    info "  Installing Node.js first (required for AI provider CLIs)..."
    install_node
  fi

  # Install in priority order (from plan.md Provider Priority table)
  # Priority 1: llama.cpp (handled separately as submodule)
  install_cursor_cli          # Priority 2 (manual - requires Cursor IDE)
  install_claude_code         # Priority 3
  install_codex_cli           # Priority 4
  install_vscode_with_copilot # Priority 5 (IDE with extension)
  install_git_cli_provider    # Priority 6 (local)
  install_abacus_cli          # Priority 7

  # Install shared resources after all providers
  install_shared_resources

  ok "AI Provider CLI installation complete"
}

#endregion

#region Shared Provider Resources (FR-037 to FR-042)

install_shared_resources() {
  info "Installing Shared Provider Resources (FR-037 to FR-042)..."

  local shared_dir="$NOA_ROOT/ai/shared"

  # Create all shared resource directories
  local -a shared_dirs=(
    "agents"
    "workflows"
    "prompts"
    "skills"
    "tools"
    "models"
    "commands"
    "resources"
    "resources/context"
    "resources/state"
  )

  for dir in "${shared_dirs[@]}"; do
    local full_path="$shared_dir/$dir"
    if [[ ! -d "$full_path" ]]; then
      mkdir -p "$full_path"
      info "  Created: ai/shared/$dir"
    fi
  done

  # Create shared execution memory database (SQLite)
  local db_path="$shared_dir/resources/execution-memory.db"
  if [[ ! -f "$db_path" ]]; then
    touch "$db_path"
    info "  Created execution memory database: $db_path"
  fi

  # Create shared resources config
  local config_path="$NOA_ROOT/config/shared-resources.json"
  if [[ ! -f "$config_path" ]]; then
    cat > "$config_path" << 'EOF'
{
  "version": "1.0.0",
  "basePath": "${NOA_ROOT}/ai/shared",
  "executionMemory": {
    "enabled": true,
    "path": "${NOA_ROOT}/ai/shared/resources/execution-memory.db"
  },
  "directories": {
    "agents": "${NOA_ROOT}/ai/shared/agents",
    "workflows": "${NOA_ROOT}/ai/shared/workflows",
    "prompts": "${NOA_ROOT}/ai/shared/prompts",
    "skills": "${NOA_ROOT}/ai/shared/skills",
    "tools": "${NOA_ROOT}/ai/shared/tools",
    "models": "${NOA_ROOT}/ai/shared/models",
    "commands": "${NOA_ROOT}/ai/shared/commands",
    "resources": "${NOA_ROOT}/ai/shared/resources"
  }
}
EOF
    info "  Created shared resources config: $config_path"
  fi

  # Update provider configs to reference shared resources
  local -a providers=(
    "ai/providers/cloud/claude-code"
    "ai/providers/cloud/codex"
    "ai/providers/cloud/abacus"
    "ai/providers/hybrid/cursor"
    "ai/providers/ide/vscode-copilot"
    "ai/providers/local/git-cli"
  )

  for provider_path in "${providers[@]}"; do
    local full_provider_path="$NOA_ROOT/$provider_path"
    if [[ ! -d "$full_provider_path" ]]; then
      mkdir -p "$full_provider_path"
      info "  Created provider directory: $provider_path"
    fi
  done

  ok "Shared Provider Resources installed"
}

#endregion

TOOLS_ALL=(
  rust go protoc golangci-lint eslint ruff gitleaks trivy grype semgrep gh git gitlfs node python
  claude-code codex-cli cursor-cli abacus-cli vscode-copilot git-cli ai-providers shared-resources
)
if [[ $# -gt 0 ]]; then
  TO_INSTALL=("$@")
else
  TO_INSTALL=("${TOOLS_ALL[@]}")
fi

for t in "${TO_INSTALL[@]}"; do
  case "$t" in
    rust) install_rust ;;
    go) install_go ;;
    protoc) install_protoc ;;
    golangci-lint) install_golangci ;;
    eslint) install_eslint ;;
    ruff) install_ruff ;;
    gitleaks) install_gitleaks ;;
    trivy) install_trivy ;;
    grype) install_grype ;;
    semgrep) install_semgrep ;;
    gh) install_gh ;;
    git) install_git ;;
    gitlfs) install_gitlfs ;;
    node) install_node ;;
    python) install_python ;;
    # AI Provider CLIs (FR-039)
    claude-code) install_claude_code ;;
    codex-cli) install_codex_cli ;;
    cursor-cli) install_cursor_cli ;;
    abacus-cli) install_abacus_cli ;;
    vscode-copilot) install_vscode_with_copilot ;;
    git-cli) install_git_cli_provider ;;
    ai-providers) install_all_ai_providers ;;
    # Shared Provider Resources (FR-037 to FR-042)
    shared-resources) install_shared_resources ;;
    *)
      warn "Unknown tool: $t"
      ;;
  esac
done

ok "Install finished. Ensure $BIN_DIR is first in PATH for this session."
info "AI Provider configs located at: $NOA_ROOT/ai/providers/"
info "Shared resources at: $NOA_ROOT/ai/shared/"

