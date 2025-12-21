#!/bin/bash
# Cross-platform build script for ML DevOps Inference Server
# Builds both Linux and Windows binaries

set -e

echo "🦀 Building ML DevOps Inference Server..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not found. Please install from https://rustup.rs/${NC}"
    exit 1
fi

# Build Linux binary (default)
echo -e "${YELLOW}Building for Linux (x86_64-unknown-linux-gnu)...${NC}"
time cargo build --release
LINUX_SIZE=$(du -h target/release/inference_server | cut -f1)
echo -e "${GREEN}✓ Linux binary: target/release/inference_server (${LINUX_SIZE})${NC}"
echo ""

# Check if Windows target is installed
if rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo -e "${YELLOW}Building for Windows (x86_64-pc-windows-gnu)...${NC}"
    
    # Check if mingw-w64 is installed
    if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        echo -e "${RED}Warning: mingw-w64 not found. Install with:${NC}"
        echo "  sudo apt-get install mingw-w64"
        echo "Skipping Windows build..."
    else
        time cargo build --release --target x86_64-pc-windows-gnu
        WIN_SIZE=$(du -h target/x86_64-pc-windows-gnu/release/inference_server.exe | cut -f1)
        echo -e "${GREEN}✓ Windows binary: target/x86_64-pc-windows-gnu/release/inference_server.exe (${WIN_SIZE})${NC}"
    fi
else
    echo -e "${YELLOW}Windows target not installed. To enable Windows builds:${NC}"
    echo "  rustup target add x86_64-pc-windows-gnu"
    echo "  sudo apt-get install mingw-w64"
fi

echo ""

# Check if macOS targets are installed
echo -e "${YELLOW}Checking macOS targets...${NC}"

# macOS Intel (x86_64)
if rustup target list | grep -q "x86_64-apple-darwin (installed)"; then
    echo -e "${YELLOW}Building for macOS Intel (x86_64-apple-darwin)...${NC}"
    # Note: Cross-compilation from Linux to macOS requires osxcross
    if command -v x86_64-apple-darwin-gcc &> /dev/null; then
        time cargo build --release --target x86_64-apple-darwin
        MACOS_INTEL_SIZE=$(du -h target/x86_64-apple-darwin/release/inference_server | cut -f1)
        echo -e "${GREEN}✓ macOS Intel binary: target/x86_64-apple-darwin/release/inference_server (${MACOS_INTEL_SIZE})${NC}"
    else
        echo -e "${RED}Warning: osxcross not found. Cross-compilation to macOS requires osxcross.${NC}"
        echo "See: https://github.com/tpoechtrager/osxcross"
        echo "Skipping macOS Intel build..."
    fi
else
    echo -e "${YELLOW}macOS Intel target not installed. To enable:${NC}"
    echo "  rustup target add x86_64-apple-darwin"
fi

echo ""

# macOS Apple Silicon (aarch64)
if rustup target list | grep -q "aarch64-apple-darwin (installed)"; then
    echo -e "${YELLOW}Building for macOS Apple Silicon (aarch64-apple-darwin)...${NC}"
    # Note: Cross-compilation from Linux to macOS requires osxcross
    if command -v aarch64-apple-darwin-gcc &> /dev/null; then
        time cargo build --release --target aarch64-apple-darwin
        MACOS_ARM_SIZE=$(du -h target/aarch64-apple-darwin/release/inference_server | cut -f1)
        echo -e "${GREEN}✓ macOS Apple Silicon binary: target/aarch64-apple-darwin/release/inference_server (${MACOS_ARM_SIZE})${NC}"
    else
        echo -e "${RED}Warning: osxcross not found for ARM64. Cross-compilation requires osxcross.${NC}"
        echo "See: https://github.com/tpoechtrager/osxcross"
        echo "Skipping macOS ARM build..."
    fi
else
    echo -e "${YELLOW}macOS Apple Silicon target not installed. To enable:${NC}"
    echo "  rustup target add aarch64-apple-darwin"
fi

echo ""
echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo "Available binaries:"
ls -lh target/release/inference_server 2>/dev/null || true
ls -lh target/x86_64-pc-windows-gnu/release/inference_server.exe 2>/dev/null || true
ls -lh target/x86_64-apple-darwin/release/inference_server 2>/dev/null || true
ls -lh target/aarch64-apple-darwin/release/inference_server 2>/dev/null || true

echo ""
echo -e "${YELLOW}Note: macOS cross-compilation from Linux requires osxcross.${NC}"
echo "On native macOS, simply run: cargo build --release"
