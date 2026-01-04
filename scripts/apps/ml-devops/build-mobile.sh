#!/bin/bash

# Build script for mobile applications (Android, iOS)
# Prerequisites: Android Studio/SDK, Xcode (macOS only), Rust, Node.js, Yarn

set -e  # Exit on error

echo "📱 ML DevOps Platform - Mobile Build Script"
echo "==========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0;m' # No Color

# Parse arguments
TARGET="${1:-all}"  # Default to 'all' if no argument

if [ "$TARGET" != "android" ] && [ "$TARGET" != "ios" ] && [ "$TARGET" != "all" ]; then
    echo -e "${RED}Invalid target: $TARGET${NC}"
    echo -e "${YELLOW}Usage: $0 [android|ios|all]${NC}"
    exit 1
fi

# Check prerequisites
echo -e "\n${YELLOW}Checking prerequisites...${NC}"

if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Node.js: $(node --version)${NC}"

if ! command -v yarn &> /dev/null; then
    echo -e "${RED}❌ Yarn not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Yarn: $(yarn --version)${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Cargo: $(cargo --version)${NC}"

# Detect platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    CAN_BUILD_IOS=true
else
    CAN_BUILD_IOS=false
fi

# Install dependencies
echo -e "\n${YELLOW}Installing dependencies...${NC}"
yarn install

# Build Android
if [ "$TARGET" == "android" ] || [ "$TARGET" == "all" ]; then
    echo -e "\n${BLUE}===========================================${NC}"
    echo -e "${BLUE}Building Android APK...${NC}"
    echo -e "${BLUE}===========================================${NC}"
    
    # Check Android prerequisites
    if [ -z "$ANDROID_HOME" ]; then
        echo -e "${RED}❌ ANDROID_HOME not set${NC}"
        echo -e "${YELLOW}Please set up Android SDK and ANDROID_HOME environment variable${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Android SDK: $ANDROID_HOME${NC}"
    
    # Check if Android is initialized
    if [ ! -d "src-tauri/gen/android" ]; then
        echo -e "${YELLOW}Initializing Android project...${NC}"
        yarn tauri android init
    fi
    
    # Install Android Rust targets
    echo -e "${YELLOW}Installing Rust Android targets...${NC}"
    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
    
    # Build APK
    echo -e "${YELLOW}Building Android APK...${NC}"
    yarn tauri:android:build
    
    echo -e "${GREEN}✅ Android build completed!${NC}"
    echo -e "${YELLOW}APK location: ${GREEN}src-tauri/gen/android/app/build/outputs/apk/${NC}"
fi

# Build iOS
if [ "$TARGET" == "ios" ] || [ "$TARGET" == "all" ]; then
    echo -e "\n${BLUE}===========================================${NC}"
    echo -e "${BLUE}Building iOS App...${NC}"
    echo -e "${BLUE}===========================================${NC}"
    
    if [ "$CAN_BUILD_IOS" = false ]; then
        echo -e "${RED}❌ iOS builds are only supported on macOS${NC}"
        if [ "$TARGET" == "ios" ]; then
            exit 1
        else
            echo -e "${YELLOW}Skipping iOS build...${NC}"
        fi
    else
        # Check Xcode
        if ! command -v xcodebuild &> /dev/null; then
            echo -e "${RED}❌ Xcode not found${NC}"
            exit 1
        fi
        echo -e "${GREEN}✓ Xcode found${NC}"
        
        # Check CocoaPods
        if ! command -v pod &> /dev/null; then
            echo -e "${YELLOW}⚠️  CocoaPods not found. Installing...${NC}"
            sudo gem install cocoapods
        fi
        echo -e "${GREEN}✓ CocoaPods found${NC}"
        
        # Check if iOS is initialized
        if [ ! -d "src-tauri/gen/ios" ]; then
            echo -e "${YELLOW}Initializing iOS project...${NC}"
            yarn tauri ios init
        fi
        
        # Install iOS Rust targets
        echo -e "${YELLOW}Installing Rust iOS targets...${NC}"
        rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
        
        # Build iOS
        echo -e "${YELLOW}Building iOS app...${NC}"
        yarn tauri:ios:build
        
        echo -e "${GREEN}✅ iOS build completed!${NC}"
        echo -e "${YELLOW}iOS app location: ${GREEN}src-tauri/gen/ios/${NC}"
        echo -e "${YELLOW}To deploy to App Store:${NC}"
        echo -e "  1. Open src-tauri/gen/ios/ML\ DevOps\ Platform.xcodeproj in Xcode"
        echo -e "  2. configsure signing & capabilities"
        echo -e "  3. Select Generic iOS Device"
        echo -e "  4. Product > Archive"
    fi
fi

echo -e "\n${GREEN}🎉 Mobile build(s) complete!${NC}"
