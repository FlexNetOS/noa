#!/bin/bash

# Rust Lovable - Single Click Installation Script
# This script automatically detects platform and hardware, then installs the appropriate version

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RUST_MIN_VERSION="1.75.0"
INSTALL_DIR="$HOME/.rust-lovable"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/rust-lovable"

# Function to print colored output
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

# Function to detect platform
detect_platform() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [[ -f /etc/os-release ]]; then
            . /etc/os-release
            echo "linux-$ID"
        else
            echo "linux-unknown"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Function to detect architecture
detect_architecture() {
    case "$(uname -m)" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        armv7l|arm)
            echo "armv7"
            ;;
        i386|i686)
            echo "i386"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# Function to detect hardware capabilities
detect_hardware() {
    local cpu_cores=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "4")
    local memory_gb=$(free -g 2>/dev/null | awk '/^Mem:/{print $2}' || echo "8")
    local gpu_available=false
    
    # Check for GPU (simplified)
    if command -v nvidia-smi &> /dev/null || command -v glxinfo &> /dev/null; then
        gpu_available=true
    fi
    
    echo "{\"cpu_cores\":$cpu_cores,\"memory_gb\":$memory_gb,\"gpu_available\":$gpu_available}"
}

# Function to check if Rust is installed
check_rust() {
    if command -v rustc &> /dev/null; then
        local rust_version=$(rustc --version | cut -d' ' -f2)
        print_status "Found Rust version: $rust_version"
        
        # Check if version is sufficient
        if [[ $(printf '%s\n' "$RUST_MIN_VERSION" "$rust_version" | sort -V | head -n1) = "$RUST_MIN_VERSION" ]]; then
            return 0
        else
            print_warning "Rust version $rust_version is below minimum required $RUST_MIN_VERSION"
            return 1
        fi
    else
        print_status "Rust not found"
        return 1
    fi
}

# Function to install Rust
install_rust() {
    print_status "Installing Rust..."
    
    if command -v curl &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust installed successfully"
    else
        print_error "curl not found. Please install curl and try again."
        exit 1
    fi
}

# Function to check system requirements
check_requirements() {
    local platform=$1
    local arch=$2
    local hardware=$3
    
    print_status "Checking system requirements..."
    
    # Parse hardware info
    local cpu_cores=$(echo "$hardware" | grep -o '"cpu_cores":[0-9]*' | cut -d':' -f2)
    local memory_gb=$(echo "$hardware" | grep -o '"memory_gb":[0-9]*' | cut -d':' -f2)
    
    # Minimum requirements
    local min_cores=2
    local min_memory=4
    
    if [[ $cpu_cores -lt $min_cores ]]; then
        print_warning "CPU cores ($cpu_cores) below recommended minimum ($min_cores)"
    fi
    
    if [[ $memory_gb -lt $min_memory ]]; then
        print_warning "Memory ($memory_gb GB) below recommended minimum ($min_memory GB)"
    fi
    
    # Check for required system dependencies
    local missing_deps=()
    
    case "$platform" in
        linux-*)
            if ! command -v pkg-config &> /dev/null; then
                missing_deps+=("pkg-config")
            fi
            if ! command -v gcc &> /dev/null; then
                missing_deps+=("gcc")
            fi
            ;;
        macos)
            if ! command -v xcodebuild &> /dev/null && ! command -v gcc &> /dev/null; then
                print_warning "Xcode Command Line Tools not found. Please install them."
            fi
            ;;
        windows)
            if ! command -v gcc &> /dev/null && ! command -v cl &> /dev/null; then
                print_warning "C++ compiler not found. Please install Visual Studio Build Tools or MinGW."
            fi
            ;;
    esac
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        print_warning "Missing dependencies: ${missing_deps[*]}"
        print_status "Please install them using your package manager."
    fi
}

# Function to detect package manager
detect_package_manager() {
    if command -v apt-get &> /dev/null; then
        echo "apt"
    elif command -v yum &> /dev/null; then
        echo "yum"
    elif command -v dnf &> /dev/null; then
        echo "dnf"
    elif command -v pacman &> /dev/null; then
        echo "pacman"
    elif command -v brew &> /dev/null; then
        echo "brew"
    elif command -v choco &> /dev/null; then
        echo "choco"
    else
        echo "unknown"
    fi
}

# Function to install system dependencies
install_system_deps() {
    local platform=$1
    local package_manager=$2
    
    print_status "Installing system dependencies..."
    
    case "$platform" in
        linux-ubuntu|linux-debian)
            if [[ "$package_manager" == "apt" ]]; then
                sudo apt-get update
                sudo apt-get install -y build-essential pkg-config libssl-dev
            fi
            ;;
        linux-fedora)
            if [[ "$package_manager" == "dnf" ]]; then
                sudo dnf groupinstall -y "Development Tools"
                sudo dnf install -y openssl-devel
            fi
            ;;
        linux-arch)
            if [[ "$package_manager" == "pacman" ]]; then
                sudo pacman -S --needed base-devel openssl
            fi
            ;;
        macos)
            if [[ "$package_manager" == "brew" ]]; then
                brew install openssl
            else
                print_status "Please install Xcode Command Line Tools: xcode-select --install"
            fi
            ;;
        windows)
            print_status "Please ensure Visual Studio Build Tools are installed."
            ;;
    esac
}

# Function to download and install Rust Lovable
install_rust_lovable() {
    local platform=$1
    local arch=$2
    
    print_status "Installing Rust Lovable..."
    
    # Create directories
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$BIN_DIR"
    mkdir -p "$CONFIG_DIR"
    
    # Clone or download the repository
    if [[ -d "$INSTALL_DIR/repo" ]]; then
        print_status "Updating existing installation..."
        cd "$INSTALL_DIR/repo"
        git pull origin main
    else
        print_status "Cloning repository..."
        git clone https://github.com/yourusername/rust-lovable.git "$INSTALL_DIR/repo"
        cd "$INSTALL_DIR/repo"
    fi
    
    # Build with platform-specific optimizations
    print_status "Building for $platform-$arch..."
    
    case "$platform" in
        linux-*)
            cargo build --release --features web,desktop
            ;;
        macos)
            cargo build --release --features web,desktop
            ;;
        windows)
            cargo build --release --features web,desktop
            ;;
    esac
    
    # Create symlink in bin directory
    ln -sf "$INSTALL_DIR/repo/target/release/rust-lovable" "$BIN_DIR/rust-lovable"
    
    # Make it executable
    chmod +x "$BIN_DIR/rust-lovable"
    
    print_success "Rust Lovable installed successfully!"
}

# Function to setup configuration
setup_config() {
    local platform=$1
    local hardware=$2
    
    print_status "Setting up configuration..."
    
    # Create default config
    cat > "$CONFIG_DIR/config.toml" << EOF
[general]
platform = "$platform"
auto_update = true

[ui]
theme = "dark"
language = "en"

[ai]
provider = "openai"
model = "gpt-4"
timeout = 30

[sandbox]
max_sandboxes = 5
execution_timeout = 60

[performance]
cache_enabled = true
max_cache_size = "1GB"

[hardware]
cpu_cores = $(echo "$hardware" | grep -o '"cpu_cores":[0-9]*' | cut -d':' -f2)
memory_gb = $(echo "$hardware" | grep -o '"memory_gb":[0-9]*' | cut -d':' -f2)
gpu_available = $(echo "$hardware" | grep -o '"gpu_available":[^,]*' | cut -d':' -f2)
EOF
    
    # Create environment file
    cat > "$CONFIG_DIR/.env" << EOF
# Rust Lovable Environment Configuration
RUST_LOG=info
RUST_BACKTRACE=1

# Platform-specific settings
PLATFORM=$platform
CONFIG_DIR=$CONFIG_DIR
DATA_DIR=$INSTALL_DIR/data
EOF
    
    print_success "Configuration created at $CONFIG_DIR/config.toml"
}

# Function to setup systemd service (Linux)
setup_systemd_service() {
    local platform=$1
    
    if [[ "$platform" == linux-* ]]; then
        if command -v systemctl &> /dev/null; then
            print_status "Setting up systemd service..."
            
            cat > "$HOME/.config/systemd/user/rust-lovable.service" << EOF
[Unit]
Description=Rust Lovable Development Server
After=network.target

[Service]
Type=simple
ExecStart=$BIN_DIR/rust-lovable
Restart=always
RestartSec=10
Environment="RUST_LOG=info"
Environment="CONFIG_DIR=$CONFIG_DIR"

[Install]
WantedBy=default.target
EOF
            
            systemctl --user daemon-reload
            print_success "Systemd service created. Start with: systemctl --user start rust-lovable"
        fi
    fi
}

# Function to add to PATH
add_to_path() {
    print_status "Adding to PATH..."
    
    local shell_rc=""
    if [[ -n "${BASH_VERSION:-}" ]]; then
        shell_rc="$HOME/.bashrc"
    elif [[ -n "${ZSH_VERSION:-}" ]]; then
        shell_rc="$HOME/.zshrc"
    elif [[ -n "${FISH_VERSION:-}" ]]; then
        shell_rc="$HOME/.config/fish/config.fish"
    fi
    
    if [[ -n "$shell_rc" && ! -f "$shell_rc" ]]; then
        echo "# Rust Lovable" >> "$shell_rc"
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$shell_rc"
        print_success "Added to PATH in $shell_rc"
        print_status "Please run: source $shell_rc"
    fi
}

# Function to run health check
health_check() {
    print_status "Running health check..."
    
    if "$BIN_DIR/rust-lovable" --version &> /dev/null; then
        print_success "Health check passed!"
        return 0
    else
        print_error "Health check failed!"
        return 1
    fi
}

# Function to print final instructions
print_final_instructions() {
    local platform=$1
    
    print_success "Installation complete! 🎉"
    echo
    echo "Rust Lovable has been installed successfully."
    echo
    echo "Quick start:"
    echo "  rust-lovable --help"
    echo "  rust-lovable --version"
    echo
    echo "Development mode:"
    echo "  rust-lovable --dev"
    echo
    echo "Configuration file:"
    echo "  $CONFIG_DIR/config.toml"
    echo
    echo "Log files:"
    echo "  $CONFIG_DIR/logs/"
    echo
    echo "Uninstall:"
    echo "  rm -rf $INSTALL_DIR"
    echo "  rm $BIN_DIR/rust-lovable"
    echo
    
    case "$platform" in
        linux-*)
            echo "Systemd service:"
            echo "  systemctl --user start rust-lovable"
            echo "  systemctl --user enable rust-lovable"
            ;;
        macos)
            echo "LaunchAgent service:"
            echo "  launchctl load $HOME/Library/LaunchAgents/rust-lovable.plist"
            ;;
    esac
}

# Main installation function
main() {
    print_status "Starting Rust Lovable installation..."
    
    # Detect platform and hardware
    local platform=$(detect_platform)
    local arch=$(detect_architecture)
    local hardware=$(detect_hardware)
    local package_manager=$(detect_package_manager)
    
    print_status "Detected platform: $platform"
    print_status "Detected architecture: $arch"
    print_status "Detected hardware: $hardware"
    print_status "Detected package manager: $package_manager"
    
    # Check requirements
    check_requirements "$platform" "$arch" "$hardware"
    
    # Install Rust if needed
    if ! check_rust; then
        install_rust
    fi
    
    # Install system dependencies
    install_system_deps "$platform" "$package_manager"
    
    # Install Rust Lovable
    install_rust_lovable "$platform" "$arch"
    
    # Setup configuration
    setup_config "$platform" "$hardware"
    
    # Setup systemd service
    setup_systemd_service "$platform"
    
    # Add to PATH
    add_to_path
    
    # Run health check
    if health_check; then
        print_final_instructions "$platform"
    else
        print_error "Installation completed but health check failed."
        print_status "Please check the logs and try again."
        exit 1
    fi
}

# Run main function
main "$@"