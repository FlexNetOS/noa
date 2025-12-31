#!/bin/bash

# Rust Lovable - Installation Verification Script
# This script verifies that the installation was successful and all components are working

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="$HOME/.rust-lovable"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/rust-lovable"
DATA_DIR="$HOME/.local/share/rust-lovable"
LOGS_DIR="$HOME/.local/share/rust-lovable/logs"

# Function to print colored output
print_header() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║           Rust Lovable Installation Verification             ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_section() {
    echo -e "\n${CYAN}▶ $1${NC}"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check file/directory existence and permissions
check_path() {
    local path=$1
    local type=$2
    local should_be_writable=$3
    
    if [[ "$type" == "directory" ]]; then
        if [[ -d "$path" ]]; then
            print_success "✓ Directory exists: $path"
            
            if [[ "$should_be_writable" == "true" ]]; then
                if [[ -w "$path" ]]; then
                    print_success "✓ Directory is writable: $path"
                else
                    print_error "✗ Directory is not writable: $path"
                    return 1
                fi
            fi
        else
            print_error "✗ Directory missing: $path"
            return 1
        fi
    elif [[ "$type" == "file" ]]; then
        if [[ -f "$path" ]]; then
            print_success "✓ File exists: $path"
            
            if [[ "$should_be_writable" == "true" ]]; then
                if [[ -w "$path" ]]; then
                    print_success "✓ File is writable: $path"
                else
                    print_error "✗ File is not writable: $path"
                    return 1
                fi
            fi
        else
            print_error "✗ File missing: $path"
            return 1
        fi
    fi
    
    return 0
}

# Function to verify Rust installation
verify_rust() {
    print_section "Rust Toolchain Verification"
    
    if command_exists rustc; then
        local rust_version=$(rustc --version)
        print_success "✓ Rust is installed: $rust_version"
        
        # Check cargo
        if command_exists cargo; then
            local cargo_version=$(cargo --version)
            print_success "✓ Cargo is installed: $cargo_version"
            
            # Check rustfmt
            if command_exists rustfmt; then
                print_success "✓ rustfmt is installed"
            else
                print_warning "⚠ rustfmt is not installed (optional)"
            fi
            
            # Check clippy
            if command_exists cargo-clippy; then
                print_success "✓ clippy is installed"
            else
                print_warning "⚠ clippy is not installed (optional)"
            fi
        else
            print_error "✗ Cargo is not installed"
            return 1
        fi
    else
        print_error "✗ Rust is not installed"
        return 1
    fi
    
    return 0
}

# Function to verify Rust Lovable binary
verify_binary() {
    print_section "Binary Verification"
    
    if command_exists rust-lovable; then
        print_success "✓ rust-lovable is in PATH"
        
        # Check version
        if rust-lovable --version &>/dev/null; then
            local version=$(rust-lovable --version)
            print_success "✓ Binary responds to --version: $version"
        else
            print_error "✗ Binary does not respond to --version"
            return 1
        fi
        
        # Check help
        if rust-lovable --help &>/dev/null; then
            print_success "✓ Binary responds to --help"
        else
            print_error "✗ Binary does not respond to --help"
            return 1
        fi
    else
        print_error "✗ rust-lovable is not in PATH"
        print_status "Expected location: $BIN_DIR/rust-lovable"
        return 1
    fi
    
    return 0
}

# Function to verify configuration
verify_configuration() {
    print_section "Configuration Verification"
    
    # Check config directory
    check_path "$CONFIG_DIR" "directory" "true"
    
    # Check config file
    check_path "$CONFIG_DIR/config.toml" "file" "true"
    
    # Validate TOML syntax
    if command_exists toml-test; then
        if toml-test < "$CONFIG_DIR/config.toml" &>/dev/null; then
            print_success "✓ Configuration file has valid TOML syntax"
        else
            print_error "✗ Configuration file has invalid TOML syntax"
            return 1
        fi
    else
        print_warning "⚠ toml-test not available, skipping syntax validation"
    fi
    
    return 0
}

# Function to verify data directories
verify_data_directories() {
    print_section "Data Directory Verification"
    
    check_path "$DATA_DIR" "directory" "true"
    check_path "$LOGS_DIR" "directory" "true"
    
    return 0
}

# Function to verify system dependencies
verify_system_deps() {
    print_section "System Dependencies Verification"
    
    # Check for build essentials
    if command_exists gcc; then
        print_success "✓ gcc is installed"
    else
        print_warning "⚠ gcc is not installed (may be needed for some builds)"
    fi
    
    if command_exists g++; then
        print_success "✓ g++ is installed"
    else
        print_warning "⚠ g++ is not installed (may be needed for some builds)"
    fi
    
    if command_exists pkg-config; then
        print_success "✓ pkg-config is installed"
    else
        print_warning "⚠ pkg-config is not installed (may be needed for some dependencies)"
    fi
    
    # Check for SSL development libraries
    if [[ -f /usr/include/openssl/ssl.h ]] || [[ -f /usr/local/include/openssl/ssl.h ]]; then
        print_success "✓ OpenSSL development headers are available"
    else
        print_warning "⚠ OpenSSL development headers may not be available"
    fi
    
    return 0
}

# Function to verify optional dependencies
verify_optional_deps() {
    print_section "Optional Dependencies Verification"
    
    # Check for Node.js (for web development)
    if command_exists node; then
        local node_version=$(node --version)
        print_success "✓ Node.js is installed: $node_version"
        
        if command_exists npm; then
            local npm_version=$(npm --version)
            print_success "✓ npm is installed: $npm_version"
        else
            print_warning "⚠ npm is not installed"
        fi
    else
        print_warning "⚠ Node.js is not installed (optional, for web development)"
    fi
    
    # Check for Git
    if command_exists git; then
        local git_version=$(git --version)
        print_success "✓ Git is installed: $git_version"
    else
        print_warning "⚠ Git is not installed (optional, for version control)"
    fi
    
    # Check for Docker
    if command_exists docker; then
        local docker_version=$(docker --version)
        print_success "✓ Docker is installed: $docker_version"
    else
        print_warning "⚠ Docker is not installed (optional, for containerization)"
    fi
    
    return 0
}

# Function to verify build capabilities
verify_build_capabilities() {
    print_section "Build Capabilities Verification"
    
    # Create a temporary directory for testing
    local temp_dir=$(mktemp -d)
    local test_project="$temp_dir/test_project"
    
    # Create a simple test project
    mkdir -p "$test_project/src"
    
    cat > "$test_project/Cargo.toml" << 'EOF'
[package]
name = "test_project"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = "0.6"
EOF

    cat > "$test_project/src/main.rs" << 'EOF'
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "Hello, Rust Lovable!" }
    }
}
EOF

    # Try to build the test project
    cd "$test_project"
    if cargo check &>/dev/null; then
        print_success "✓ Cargo can resolve dependencies and check the project"
        
        if cargo build &>/dev/null; then
            print_success "✓ Cargo can successfully build a simple Dioxus project"
        else
            print_warning "⚠ Cargo build failed (this may be due to missing system dependencies)"
        fi
    else
        print_error "✗ Cargo cannot check the project"
        rm -rf "$temp_dir"
        return 1
    fi
    
    # Cleanup
    rm -rf "$temp_dir"
    
    return 0
}

# Function to verify network connectivity
verify_network_connectivity() {
    print_section "Network Connectivity Verification"
    
    # Check if we can reach common package registries
    local test_urls=(
        "https://crates.io"
        "https://github.com"
        "https://static.rust-lang.org"
    )
    
    for url in "${test_urls[@]}"; do
        if curl -s --head --max-time 5 "$url" &>/dev/null; then
            print_success "✓ Can reach: $url"
        else
            print_warning "⚠ Cannot reach: $url (may affect package downloads)"
        fi
    done
    
    return 0
}

# Function to verify monitoring setup
verify_monitoring() {
    print_section "Monitoring Setup Verification"
    
    if [[ -f "$BIN_DIR/rust-lovable-monitor" ]]; then
        print_success "✓ Monitoring script exists"
        
        if [[ -x "$BIN_DIR/rust-lovable-monitor" ]]; then
            print_success "✓ Monitoring script is executable"
            
            # Try to run the monitoring script
            if "$BIN_DIR/rust-lovable-monitor" &>/dev/null; then
                print_success "✓ Monitoring script runs successfully"
            else
                print_warning "⚠ Monitoring script has execution issues"
            fi
        else
            print_error "✗ Monitoring script is not executable"
            return 1
        fi
    else
        print_warning "⚠ Monitoring script does not exist"
    fi
    
    return 0
}

# Function to verify systemd service (Linux only)
verify_systemd_service() {
    if [[ "$OSTYPE" == "linux-gnu"* ]] && command -v systemctl &>/dev/null; then
        print_section "Systemd Service Verification"
        
        if [[ -f "$HOME/.config/systemd/user/rust-lovable.service" ]]; then
            print_success "✓ Systemd service file exists"
            
            if systemctl --user is-enabled rust-lovable.service &>/dev/null; then
                print_success "✓ Service is enabled"
            else
                print_warning "⚠ Service is not enabled"
            fi
            
            if systemctl --user is-active rust-lovable.service &>/dev/null; then
                print_success "✓ Service is running"
            else
                print_warning "⚠ Service is not running"
            fi
        else
            print_warning "⚠ Systemd service file does not exist"
        fi
    fi
    
    return 0
}

# Function to run a quick smoke test
run_smoke_test() {
    print_section "Smoke Test"
    
    # Try to run rust-lovable with --help to ensure it works
    if timeout 5 rust-lovable --help &>/dev/null; then
        print_success "✓ Binary responds within 5 seconds"
    else
        print_warning "⚠ Binary took too long to respond or failed"
    fi
    
    return 0
}

# Function to generate a verification report
generate_report() {
    print_section "Verification Report"
    
    local total_checks=0
    local passed_checks=0
    local failed_checks=0
    local warning_count=0
    
    # Count results (this would be populated by the actual verification functions)
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}                    VERIFICATION SUMMARY                   ${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
    
    echo -e "\n${GREEN}✓ PASSED CHECKS${NC}"
    echo -e "${YELLOW}⚠ WARNINGS${NC}"
    echo -e "${RED}✗ ERRORS${NC}"
    
    echo -e "\n${BLUE}RECOMMENDATIONS:${NC}"
    echo "1. Address any errors above before using Rust Lovable"
    echo "2. Optional dependencies can be installed later as needed"
    echo "3. Run 'rust-lovable --help' to get started"
    echo "4. Check the logs at $LOGS_DIR for detailed information"
    
    echo -e "\n${GREEN}For support, visit: https://github.com/yourusername/rust-lovable${NC}"
}

# Main verification function
main() {
    print_header
    
    local exit_code=0
    
    # Run all verification steps
    verify_rust || exit_code=1
    verify_binary || exit_code=1
    verify_configuration || exit_code=1
    verify_data_directories || exit_code=1
    verify_system_deps || exit_code=1
    verify_optional_deps || exit_code=1
    verify_build_capabilities || exit_code=1
    verify_network_connectivity || exit_code=1
    verify_monitoring || exit_code=1
    verify_systemd_service || exit_code=1
    run_smoke_test || exit_code=1
    
    generate_report
    
    if [[ $exit_code -eq 0 ]]; then
        echo -e "\n${GREEN}🎉 Installation verification completed successfully!${NC}"
        echo -e "${GREEN}Rust Lovable is ready to use.${NC}"
    else
        echo -e "\n${YELLOW}⚠ Installation verification completed with some issues.${NC}"
        echo -e "${YELLOW}Please address the errors above.${NC}"
    fi
    
    exit $exit_code
}

# Run main function
main "$@"