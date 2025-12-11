#!/usr/bin/env bash
#
# NOA llama.cpp Builder (Unix/macOS)
#
# Builds llama.cpp from source in noa_root/opt/llama.cpp/
#
# Constitutional Compliance: §3.1 Self-Contained & Autonomous
#
# Usage:
#   ./llama-cpp-build.sh
#   ./llama-cpp-build.sh --gpu-layers
#   ./llama-cpp-build.sh --clean

set -euo pipefail

# Defaults
GPU_LAYERS=false
CLEAN=false
NOA_ROOT="${NOA_ROOT:-}"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --gpu-layers|-g)
            GPU_LAYERS=true
            shift
            ;;
        --clean|-c)
            CLEAN=true
            shift
            ;;
        --noa-root)
            NOA_ROOT="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--gpu-layers] [--clean] [--noa-root PATH]"
            exit 1
            ;;
    esac
done

# Auto-detect NOA_ROOT
if [[ -z "$NOA_ROOT" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    # Walk up from script location to find repo root
    CURRENT="$SCRIPT_DIR"
    while [[ -n "$CURRENT" ]] && [[ ! -d "$CURRENT/.git" ]]; do
        CURRENT="$(dirname "$CURRENT")"
        # Prevent infinite loop
        if [[ "$CURRENT" == "/" ]]; then
            CURRENT=""
        fi
    done
    if [[ -n "$CURRENT" ]]; then
        NOA_ROOT="$CURRENT"
    else
        NOA_ROOT="$(pwd)"
    fi
fi

LLAMA_DIR="$NOA_ROOT/opt/llama.cpp"
BUILD_DIR="$LLAMA_DIR/build"
BIN_DIR="$NOA_ROOT/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m' # No Color

log_info() { echo -e "${NC}[..] $1${NC}"; }
log_ok() { echo -e "${GREEN}[OK] $1${NC}"; }
log_warn() { echo -e "${YELLOW}[!!] $1${NC}"; }
log_error() { echo -e "${RED}[XX] $1${NC}"; }

echo ""
echo -e "${CYAN}============================================================${NC}"
echo -e "${CYAN}NOA llama.cpp Builder${NC}"
echo -e "NOA_ROOT:   $NOA_ROOT"
echo -e "LLAMA_DIR:  $LLAMA_DIR"
echo -e "BUILD_DIR:  $BUILD_DIR"
echo -e "${CYAN}============================================================${NC}"
echo ""

# Check prerequisites
log_info "[1/5] Checking prerequisites..."

# Check CMake
if ! command -v cmake &> /dev/null; then
    log_error "CMake not found. Install via:"
    echo "  Linux: sudo apt-get install cmake  # or your package manager"
    echo "  macOS: brew install cmake"
    echo "  Or use: scripts/bootstrap/installers/cmake-portable.sh"
    exit 1
fi
CMAKE_VERSION=$(cmake --version | head -1)
log_ok "CMake: $CMAKE_VERSION"

# Check for C/C++ compiler
if command -v gcc &> /dev/null; then
    GCC_VERSION=$(gcc --version | head -1)
    log_ok "Compiler: $GCC_VERSION"
elif command -v clang &> /dev/null; then
    CLANG_VERSION=$(clang --version | head -1)
    log_ok "Compiler: $CLANG_VERSION"
else
    log_warn "No C/C++ compiler found - CMake will try to find one"
fi

# Check CUDA if GPU requested
if [[ "$GPU_LAYERS" == "true" ]]; then
    # Check for CUDA in NOA portable location first
    CUDA_PORTABLE="$NOA_ROOT/opt/cuda/toolkit"
    NVCC_PORTABLE="$CUDA_PORTABLE/bin/nvcc"

    if [[ -f "$NVCC_PORTABLE" ]]; then
        log_ok "CUDA (Portable): $CUDA_PORTABLE"
        export CUDA_PATH="$CUDA_PORTABLE"
        export CUDA_HOME="$CUDA_PORTABLE"
        export PATH="$CUDA_PORTABLE/bin:$PATH"
    else
        # Fall back to system CUDA
        if command -v nvcc &> /dev/null; then
            NVCC_VERSION=$(nvcc --version | grep "release" | head -1)
            log_ok "CUDA (System): $NVCC_VERSION"
        else
            log_warn "CUDA not found - building without GPU support"
            echo "  Install CUDA: scripts/bootstrap/installers/cuda-portable.sh"
            GPU_LAYERS=false
        fi
    fi
fi

# Check llama.cpp submodule
log_info "[2/5] Checking llama.cpp submodule..."

if [[ ! -d "$LLAMA_DIR" ]]; then
    log_error "llama.cpp not found at $LLAMA_DIR"
    echo "  Initialize submodule:"
    echo "    git submodule update --init opt/llama.cpp"
    exit 1
fi

if [[ ! -f "$LLAMA_DIR/CMakeLists.txt" ]]; then
    log_error "CMakeLists.txt not found - submodule may be empty"
    echo "  Reinitialize:"
    echo "    git submodule update --init --recursive opt/llama.cpp"
    exit 1
fi

log_ok "llama.cpp source found"

# Clean build if requested
if [[ "$CLEAN" == "true" ]] && [[ -d "$BUILD_DIR" ]]; then
    log_info "[3/5] Cleaning previous build..."
    rm -rf "$BUILD_DIR"
    log_ok "Cleaned"
else
    log_info "[3/5] Using existing build directory (if any)"
fi

# Configure with CMake
log_info "[4/5] Configuring with CMake..."

CMAKE_ARGS=(
    "-B" "$BUILD_DIR"
    "-S" "$LLAMA_DIR"
    "-DCMAKE_BUILD_TYPE=Release"
)

if [[ "$GPU_LAYERS" == "true" ]]; then
    CMAKE_ARGS+=("-DGGML_CUDA=ON")
    CMAKE_ARGS+=("-DCMAKE_CUDA_ARCHITECTURES=native")
    echo -e "  ${CYAN}CUDA GPU support: ENABLED${NC}"
else
    echo -e "  ${GRAY}CUDA GPU support: disabled (CPU only)${NC}"
fi

# Disable CURL dependency (not needed for basic inference)
CMAKE_ARGS+=("-DLLAMA_CURL=OFF")

# Build only CLI, not server (server has httplib linking issues)
CMAKE_ARGS+=("-DBUILD_SHARED_LIBS=OFF")

# Use Ninja if available for faster builds
NINJA_EXE="$NOA_ROOT/opt/ninja/ninja"
if [[ -f "$NINJA_EXE" ]]; then
    CMAKE_ARGS+=("-G" "Ninja")
    CMAKE_ARGS+=("-DCMAKE_MAKE_PROGRAM=$NINJA_EXE")
    echo -e "  ${CYAN}Build system: Ninja (fast)${NC}"
elif command -v ninja &> /dev/null; then
    CMAKE_ARGS+=("-G" "Ninja")
    echo -e "  ${CYAN}Build system: Ninja (fast)${NC}"
else
    # Use Unix Makefiles as default
    CMAKE_ARGS+=("-G" "Unix Makefiles")
    echo -e "  ${GRAY}Build system: Unix Makefiles (default)${NC}"
fi

cd "$LLAMA_DIR"
if ! cmake "${CMAKE_ARGS[@]}"; then
    log_error "CMake configuration failed"
    exit 1
fi
log_ok "Configuration complete"

# Build
log_info "[5/5] Building llama.cpp..."
echo -e "  ${GRAY}This may take several minutes...${NC}"

cd "$LLAMA_DIR"
# Use all available CPU cores for parallel build
if command -v nproc &> /dev/null; then
    JOBS=$(nproc)
elif command -v sysctl &> /dev/null; then
    JOBS=$(sysctl -n hw.ncpu)
else
    JOBS=4
fi

if ! cmake --build "$BUILD_DIR" --config Release --parallel "$JOBS"; then
    log_error "Build failed"
    exit 1
fi
log_ok "Build complete"

# Find and report built binaries
echo ""
echo -e "${CYAN}=== Build Summary ===${NC}"

# On Unix, binaries are typically in build/bin/ (no Release subdirectory)
BIN_SEARCH_DIR="$BUILD_DIR/bin"
if [[ ! -d "$BIN_SEARCH_DIR" ]]; then
    # Fallback: check if binaries are directly in build/
    BIN_SEARCH_DIR="$BUILD_DIR"
fi

if [[ -d "$BIN_SEARCH_DIR" ]]; then
    echo -e "${GREEN}Built binaries in ${BIN_SEARCH_DIR}:${NC}"

    # List all executables
    find "$BIN_SEARCH_DIR" -type f -executable -name "llama-*" 2>/dev/null | while read -r bin; do
        echo -e "  ${GRAY}- $(basename "$bin")${NC}"
    done

    # Check for key binaries
    LLAMA_SERVER="$BIN_SEARCH_DIR/llama-server"
    LLAMA_CLI="$BIN_SEARCH_DIR/llama-cli"

    if [[ -f "$LLAMA_SERVER" ]]; then
        echo ""
        log_ok "llama-server ready"
        echo -e "  ${GRAY}Wrapper: $BIN_DIR/llama-server${NC}"
    fi

    if [[ -f "$LLAMA_CLI" ]]; then
        log_ok "llama-cli ready"
        echo -e "  ${GRAY}Wrapper: $BIN_DIR/llama-cli${NC}"
    fi
else
    log_warn "No binaries found in expected location"
fi

echo ""
echo -e "${GREEN}llama.cpp build complete!${NC}"
echo ""
echo -e "${CYAN}Usage:${NC}"
echo -e "  ${GRAY}llama-server -m model.gguf --port 8080   # Start inference server${NC}"
echo -e "  ${GRAY}llama-cli -m model.gguf -p 'Hello'       # Run inference${NC}"
echo ""
echo -e "${YELLOW}Download models from: https://huggingface.co/models?other=gguf${NC}"

exit 0
