#!/bin/bash
#
# configsure centralized cache directories for all NOA toolchains.
#
# Creates cache directory structure and configsures environment variables
# to centralize all tool caches under noa_root/cache/.
#
# Usage:
#   ./cache-setup.sh
#   NOA_ROOT=/path/to/noa ./cache-setup.sh

set -euo pipefail

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"
CACHE_DIR="$NOA_ROOT/cache"

echo -e "\033[36mSetting up centralized cache directories...\033[0m"
echo -e "\033[90mNOA Root: $NOA_ROOT\033[0m"
echo -e "\033[90mCache Dir: $CACHE_DIR\033[0m"
echo ""

# Create main cache directory
if [[ ! -d "$CACHE_DIR" ]]; then
    mkdir -p "$CACHE_DIR"
    echo -e "  \033[32m[CREATE]\033[0m $CACHE_DIR"
fi

# Define cache subdirectories
declare -A CACHE_DIRS=(
    ["rust"]="Rust/Cargo registry cache"
    ["go"]="Go module cache"
    ["npm"]="npm package cache"
    ["pip"]="pip package cache"
    ["models"]="AI model cache (llama.cpp, HuggingFace)"
    ["ollama"]="Ollama model cache"
    ["huggingface"]="HuggingFace Hub cache"
    ["downloads"]="Downloaded archives/installers"
)

for cache_name in "${!CACHE_DIRS[@]}"; do
    cache_path="$CACHE_DIR/$cache_name"
    if [[ ! -d "$cache_path" ]]; then
        mkdir -p "$cache_path"
        echo -e "  \033[32m[CREATE]\033[0m $cache_path (${CACHE_DIRS[$cache_name]})"
    else
        echo -e "  \033[90m[EXISTS]\033[0m $cache_path"
    fi
done

echo ""
echo -e "\033[32mCache directory setup complete.\033[0m"
echo ""

# Print recommended environment variables
echo -e "\033[33mRecommended environment variables for cache centralization:\033[0m"
echo ""
echo -e "  \033[36m# Rust/Cargo\033[0m"
echo "  export CARGO_HOME=\"$NOA_ROOT/opt/rust/cargo\""
echo "  # Note: Registry cache is at \$CARGO_HOME/registry"
echo ""
echo -e "  \033[36m# Go\033[0m"
echo "  export GOCACHE=\"$CACHE_DIR/go\""
echo "  export GOMODCACHE=\"$NOA_ROOT/opt/go/pkg/mod\""
echo ""
echo -e "  \033[36m# npm\033[0m"
echo "  export npm_configs_cache=\"$CACHE_DIR/npm\""
echo ""
echo -e "  \033[36m# pip\033[0m"
echo "  export PIP_CACHE_DIR=\"$CACHE_DIR/pip\""
echo ""
echo -e "  \033[36m# HuggingFace\033[0m"
echo "  export HF_HOME=\"$CACHE_DIR/huggingface\""
echo ""
echo -e "  \033[36m# Ollama\033[0m"
echo "  export OLLAMA_MODELS=\"$CACHE_DIR/ollama\""
echo ""

# Create a cache configs file for reference
CACHE_configs_PATH="$CACHE_DIR/cache-configs.json"
cat > "$CACHE_configs_PATH" << EOF
{
  "noa_root": "$NOA_ROOT",
  "cache_root": "$CACHE_DIR",
  "created_at": "$(date -Iseconds)",
  "directories": {
    "rust": "$CACHE_DIR/rust",
    "go": "$CACHE_DIR/go",
    "npm": "$CACHE_DIR/npm",
    "pip": "$CACHE_DIR/pip",
    "models": "$CACHE_DIR/models",
    "ollama": "$CACHE_DIR/ollama",
    "huggingface": "$CACHE_DIR/huggingface",
    "downloads": "$CACHE_DIR/downloads"
  },
  "env_vars": {
    "CARGO_HOME": "$NOA_ROOT/opt/rust/cargo",
    "GOCACHE": "$CACHE_DIR/go",
    "GOMODCACHE": "$NOA_ROOT/opt/go/pkg/mod",
    "npm_configs_cache": "$CACHE_DIR/npm",
    "PIP_CACHE_DIR": "$CACHE_DIR/pip",
    "HF_HOME": "$CACHE_DIR/huggingface",
    "OLLAMA_MODELS": "$CACHE_DIR/ollama"
  }
}
EOF
echo -e "\033[32mCache configsuration saved to: $CACHE_configs_PATH\033[0m"

