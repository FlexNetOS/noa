#!/bin/bash

# Build script for desktop applications (macOS, Windows, Linux)
# Prerequisites: Rust, Node.js, Yarn must be installed

set -e  # Exit on error

echo "🏗️  ML DevOps Platform - Desktop Build Script"
echo "============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0;0m' # No Color

# Check prerequisites
echo -e "\n${YELLOW}Checking prerequisites...${NC}"

if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found. Please install Node.js 18+${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Node.js found: $(node --version)${NC}"

if ! command -v yarn &> /dev/null; then
    echo -e "${RED}❌ Yarn not found. Please install Yarn${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Yarn found: $(yarn --version)${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Please install Rust toolchain${NC}"
    echo -e "${YELLOW}Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Cargo found: $(cargo --version)${NC}"

# Detect platform
PLATFORM=""
if [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macOS"
    echo -e "${GREEN}📱 Platform: macOS${NC}"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="Linux"
    echo -e "${GREEN}🐧 Platform: Linux${NC}"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    PLATFORM="Windows"
    echo -e "${GREEN}🪟 Platform: Windows${NC}"
else
    echo -e "${YELLOW}⚠️  Unknown platform: $OSTYPE${NC}"
    PLATFORM="Unknown"
fi

# Install dependencies
echo -e "\n${YELLOW}Installing dependencies...${NC}"
yarn install

# Build Next.js app
echo -e "\n${YELLOW}Building Next.js application...${NC}"
yarn build

# Build Rust backend (optional)
if [ -d "../rust_backend/inference_server" ]; then
    echo -e "\n${YELLOW}Building Rust inference server...${NC}"
    cd ../rust_backend/inference_server
    cargo build --release
    cd -
    echo -e "${GREEN}✓ Rust backend built successfully${NC}"
fi

# Build Tauri app
echo -e "\n${YELLOW}Building Tauri desktop application...${NC}"
yarn tauri:build

echo -e "\n${GREEN}✅ Build completed successfully!${NC}"

# Show output locations
echo -e "\n${YELLOW}Build artifacts:${NC}"
if [ "$PLATFORM" == "macOS" ]; then
    echo -e "  📦 DMG installer: ${GREEN}src-tauri/target/release/bundle/dmg/${NC}"
    echo -e "  📱 App bundle: ${GREEN}src-tauri/target/release/bundle/macos/${NC}"
elif [ "$PLATFORM" == "Linux" ]; then
    echo -e "  📦 AppImage: ${GREEN}src-tauri/target/release/bundle/appimage/${NC}"
    echo -e "  📦 DEB package: ${GREEN}src-tauri/target/release/bundle/deb/${NC}"
elif [ "$PLATFORM" == "Windows" ]; then
    echo -e "  📦 MSI installer: ${GREEN}src-tauri/target/release/bundle/msi/${NC}"
    echo -e "  📦 EXE: ${GREEN}src-tauri/target/release/ML\ DevOps\ Platform.exe${NC}"
fi

echo -e "\n${GREEN}🎉 Desktop build complete!${NC}"
