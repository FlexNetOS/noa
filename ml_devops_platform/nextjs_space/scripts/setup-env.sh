#!/bin/bash

# Environment setup script for ML DevOps Platform
# Sets up development environment and dependencies

set -e

echo "🔧 ML DevOps Platform - Environment Setup"
echo "========================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0;m'

# Check and install Node.js
if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}Installing Node.js...${NC}"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install node
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
        sudo apt-get install -y nodejs
    fi
fi
echo -e "${GREEN}✓ Node.js: $(node --version)${NC}"

# Check and install Yarn
if ! command -v yarn &> /dev/null; then
    echo -e "${YELLOW}Installing Yarn...${NC}"
    npm install -g yarn
fi
echo -e "${GREEN}✓ Yarn: $(yarn --version)${NC}"

# Check and prompt for Rust installation
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}⚠️  Rust not found${NC}"
    echo -e "${YELLOW}To build desktop/mobile apps, Rust is required.${NC}"
    read -p "Install Rust now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source "$HOME/.cargo/env"
    fi
fi

if command -v cargo &> /dev/null; then
    echo -e "${GREEN}✓ Cargo: $(cargo --version)${NC}"
else
    echo -e "${YELLOW}⚠️  Rust/Cargo not installed. Desktop/mobile builds will not work.${NC}"
fi

# Install project dependencies
echo -e "\n${YELLOW}Installing project dependencies...${NC}"
yarn install

# Setup environment variables
if [ ! -f ".env" ]; then
    echo -e "\n${YELLOW}Creating .env file...${NC}"
    if [ -f ".env.example" ]; then
        cp .env.example .env
        echo -e "${GREEN}✓ .env file created from .env.example${NC}"
        echo -e "${YELLOW}⚠️  Please update .env with your configuration${NC}"
    fi
fi

# Setup database
echo -e "\n${YELLOW}Setting up database...${NC}"
if [ -n "$DATABASE_URL" ] || grep -q "DATABASE_URL=" .env; then
    yarn prisma generate
    yarn prisma db push
    echo -e "${GREEN}✓ Database setup complete${NC}"
else
    echo -e "${YELLOW}⚠️  DATABASE_URL not configured. Skipping database setup.${NC}"
fi

# Install Playwright browsers (optional)
read -p "Install Playwright browsers for E2E testing? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    yarn playwright install chromium
    echo -e "${GREEN}✓ Playwright browsers installed${NC}"
fi

echo -e "\n${GREEN}✅ Environment setup complete!${NC}"
echo -e "\n${YELLOW}Next steps:${NC}"
echo -e "  1. Configure .env file with your settings"
echo -e "  2. Run 'yarn dev' to start development server"
echo -e "  3. Visit http://localhost:3000"
echo -e "\n${YELLOW}For desktop builds:${NC}"
echo -e "  Run: ./scripts/build-desktop.sh"
echo -e "\n${YELLOW}For mobile builds:${NC}"
echo -e "  Run: ./scripts/build-mobile.sh [android|ios|all]"
