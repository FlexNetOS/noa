#!/bin/bash

# Rust Lovable - Enhanced Single Click Installation Script
# Comprehensive hardware detection and platform adaptation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
RUST_MIN_VERSION="1.75.0"
INSTALL_DIR="$HOME/.rust-lovable"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/rust-lovable"
DATA_DIR="$HOME/.local/share/rust-lovable"
LOGS_DIR="$HOME/.local/share/rust-lovable/logs"

# Function to print colored output
print_header() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║               Rust Lovable Installation Wizard               ║"
    echo "║         AI-Powered Cross-Platform UI Builder                 ║"
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

# Function to detect architecture details
detect_architecture_detailed() {
    local arch=$(uname -m)
    local arch_info="{}"
    
    case "$arch" in
        x86_64)
            arch_info=$(cat << EOF
{
  "arch": "x86_64",
  "bits": 64,
  "family": "x86",
  "features": ["sse", "sse2", "avx", "avx2"],
  "vendor": "$(lscpu | grep 'Vendor ID' | awk '{print $3}' 2>/dev/null || echo 'Unknown')"
}
EOF
            )
            ;;
        aarch64|arm64)
            arch_info=$(cat << EOF
{
  "arch": "aarch64",
  "bits": 64,
  "family": "arm",
  "features": ["neon", "aes", "sha2"],
  "vendor": "ARM"
}
EOF
            )
            ;;
        *)
            arch_info=$(cat << EOF
{
  "arch": "$arch",
  "bits": 64,
  "family": "unknown",
  "features": [],
  "vendor": "Unknown"
}
EOF
            )
            ;;
    esac
    
    echo "$arch_info"
}

# Function to detect platform with detailed information
detect_platform_detailed() {
    local platform_info="{}"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [[ -f /etc/os-release ]]; then
            . /etc/os-release
            platform_info=$(cat << EOF
{
  "type": "linux",
  "distribution": "$NAME",
  "version": "$VERSION_ID",
  "id": "$ID",
  "id_like": "${ID_LIKE:-null}",
  "pretty_name": "$PRETTY_NAME"
}
EOF
            )
        else
            platform_info=$(cat << EOF
{
  "type": "linux",
  "distribution": "Unknown",
  "version": "Unknown",
  "id": "unknown",
  "id_like": null,
  "pretty_name": "Unknown Linux Distribution"
}
EOF
            )
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        local os_version=$(sw_vers -productVersion)
        local os_build=$(sw_vers -buildVersion)
        
        platform_info=$(cat << EOF
{
  "type": "macos",
  "distribution": "macOS",
  "version": "$os_version",
  "build": "$os_build",
  "id": "macos",
  "id_like": null,
  "pretty_name": "macOS $os_version"
}
EOF
        )
    else
        platform_info=$(cat << EOF
{
  "type": "unknown",
  "distribution": "Unknown",
  "version": "Unknown",
  "id": "unknown",
  "id_like": null,
  "pretty_name": "Unknown Operating System"
}
EOF
        )
    fi
    
    echo "$platform_info"
}

# Function to detect architecture with CPU features
detect_architecture_detailed() {
    local arch_info="{}"
    local arch=$(uname -m)
    
    # Detect CPU features
    local has_sse=false
    local has_avx=false
    local has_avx2=false
    local has_avx512=false
    local has_neon=false
    
    if [[ -f /proc/cpuinfo ]]; then
        has_sse=$(grep -q "sse" /proc/cpuinfo && echo true || echo false)
        has_avx=$(grep -q "avx" /proc/cpuinfo && echo true || echo false)
        has_avx2=$(grep -q "avx2" /proc/cpuinfo && echo true || echo false)
        has_avx512=$(grep -q "avx512" /proc/cpuinfo && echo true || echo false)
        has_neon=$(grep -q "neon" /proc/cpuinfo && echo true || echo false)
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS CPU feature detection
        has_sse=true
        has_avx=true
        if [[ "$arch" == "arm64" ]]; then
            has_neon=true
        fi
    fi
    
    arch_info=$(cat << EOF
{
  "arch": "$arch",
  "bits": $(getconf LONG_BIT),
  "endian": "little",
  "features": {
    "sse": $has_sse,
    "avx": $has_avx,
    "avx2": $has_avx2,
    "avx512": $has_avx512,
    "neon": $has_neon
  }
}
EOF
    )
    
    echo "$arch_info"
}

# Function to run comprehensive hardware detection
run_hardware_detection() {
    print_section "System Analysis"
    
    # Download and run the hardware detection script
    local hardware_info
    if [[ -f "scripts/detect_hardware.sh" ]]; then
        hardware_info=$(bash scripts/detect_hardware.sh)
    else
        # Download the script if not available locally
        print_status "Downloading hardware detection script..."
        local temp_script=$(mktemp)
        curl -sSL "https://raw.githubusercontent.com/yourusername/rust-lovable/main/scripts/detect_hardware.sh" > "$temp_script" 2>/dev/null || {
            print_warning "Could not download hardware detection script, using basic detection"
            rm -f "$temp_script"
            hardware_info=$(basic_hardware_detection)
        }
        
        if [[ -f "$temp_script" ]]; then
            chmod +x "$temp_script"
            hardware_info=$(bash "$temp_script")
            rm -f "$temp_script"
        fi
    fi
    
    # Save hardware info for later use
    echo "$hardware_info" > "$CONFIG_DIR/hardware.json"
    
    # Extract key information for installation decisions
    local cpu_cores=$(echo "$hardware_info" | grep -o '"cpu_cores":[0-9]*' | cut -d':' -f2)
    local memory_gb=$(echo "$hardware_info" | grep -o '"memory_gb":[0-9]*' | cut -d':' -f2)
    local gpu_available=$(echo "$hardware_info" | grep -o '"gpu_available":[^,]*' | cut -d':' -f2)
    local disk_usage=$(echo "$hardware_info" | grep -o '"usage_percent":[0-9]*' | head -1 | cut -d':' -f2)
    
    print_success "Hardware analysis completed"
    print_status "CPU Cores: $cpu_cores"
    print_status "Memory: ${memory_gb}GB"
    print_status "GPU Available: $gpu_available"
    print_status "Disk Usage: ${disk_usage}%"
    
    # Return key metrics
    echo "$cpu_cores,$memory_gb,$gpu_available,$disk_usage"
}

# Function to run comprehensive hardware detection
run_hardware_detection() {
    print_status "Detecting hardware and system information..."
    
    # Get CPU information
    local cpu_cores=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "4")
    
    # Get memory information
    local memory_gb=$(free -g 2>/dev/null | awk '/^Mem:/{print $2}' || echo "8")
    
    # Check for GPU availability
    local gpu_available=false
    if command -v nvidia-smi &> /dev/null || command -v glxinfo &> /dev/null; then
        gpu_available=true
    fi
    
    # Get disk usage
    local disk_usage=$(df / | tail -1 | awk '{print $5}' | sed 's/%//')
    
    # Get CPU features for optimization
    local cpu_features=""
    if command -v lscpu &> /dev/null; then
        cpu_features=$(lscpu | grep 'Flags' | cut -d: -f2 | tr -d ' ')
    fi
    
    print_status "CPU Cores: $cpu_cores"
    print_status "Memory: ${memory_gb}GB"
    print_status "GPU Available: $gpu_available"
    print_status "Disk Usage: ${disk_usage}%"
    
    # Return key metrics
    echo "$cpu_cores,$memory_gb,$gpu_available,$disk_usage"
}

# Basic hardware detection fallback
basic_hardware_detection() {
    local cpu_cores=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "4")
    local memory_gb=$(free -g 2>/dev/null | awk '/^Mem:/{print $2}' || echo "8")
    local gpu_available=false
    
    if command -v nvidia-smi &> /dev/null || command -v glxinfo &> /dev/null; then
        gpu_available=true
    fi
    
    local disk_usage=$(df / | tail -1 | awk '{print $5}' | sed 's/%//')
    
    cat << EOF
{
  "cpu": {"cores": $cpu_cores},
  "memory": {"gb": $memory_gb},
  "gpu": {"available": $gpu_available},
  "storage": {"usage_percent": $disk_usage}
}
EOF
}

# Function to check if Rust is installed and meets version requirements
check_rust() {
    if command -v rustc &> /dev/null; then
        local rust_version=$(rustc --version | awk '{print $2}')
        local min_version="1.75.0"
        
        if [[ $(printf '%s\n' "$min_version" "$rust_version" | sort -V | head -n1) = "$min_version" ]]; then
            print_success "Rust $rust_version is installed and meets requirements"
            return 0
        else
            print_warning "Rust $rust_version is installed but $min_version or higher is required"
            return 1
        fi
    else
        print_warning "Rust is not installed"
        return 1
    fi
}

# Function to install Rust
install_rust() {
    print_section "Rust Installation"
    
    if [[ -z "${CARGO_HOME}" ]]; then
        CARGO_HOME="$HOME/.cargo"
    fi
    
    if [[ ! -f "$CARGO_HOME/bin/rustc" ]]; then
        print_status "Installing Rust using rustup..."
        
        # Download and run rustup
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        
        # Source the cargo environment
        source "$CARGO_HOME/env"
        
        # Add cargo to PATH
        export PATH="$CARGO_HOME/bin:$PATH"
        
        print_success "Rust installed successfully"
    else
        print_status "Rust is already installed"
    fi
    
    # Update Rust to latest stable
    print_status "Updating Rust to latest stable version..."
    rustup update stable
    rustup default stable
    
    # Install additional components
    rustup component add rustfmt clippy
    
    print_success "Rust toolchain updated and ready"
}

# Function to install Rust Lovable
install_rust_lovable() {
    local platform_type=$1
    local arch=$2
    
    print_section "Rust Lovable Installation"
    
    # Clone or update repository
    if [[ -d "$INSTALL_DIR" ]] && [[ -f "$INSTALL_DIR/Cargo.toml" ]]; then
        print_status "Updating existing installation..."
        cd "$INSTALL_DIR"
        git pull origin main
    else
        print_status "Cloning Rust Lovable repository..."
        git clone https://github.com/yourusername/rust-lovable.git "$INSTALL_DIR"
        cd "$INSTALL_DIR"
    fi
    
    # Build based on platform
    print_status "Building Rust Lovable for $platform_type ($arch)..."
    
    case "$platform_type" in
        linux)
            cargo build --release --features="desktop,web"
            ;;
        macos)
            cargo build --release --features="desktop,web"
            ;;
        *)
            cargo build --release --features="desktop,web"
            ;;
    esac
    
    # Install binary
    cp target/release/rust-lovable "$BIN_DIR/"
    
    print_success "Rust Lovable built and installed successfully"
}

# Function to setup systemd service (Linux only)
setup_systemd_service() {
    local platform_type=$1
    
    if [[ "$platform_type" == "linux" ]] && command -v systemctl &> /dev/null; then
        print_section "Systemd Service Setup"
        
        # Create systemd service file
        cat > "$HOME/.config/systemd/user/rust-lovable.service" << EOF
[Unit]
Description=Rust Lovable - AI-Powered UI Builder
After=network.target

[Service]
Type=simple
ExecStart=$BIN_DIR/rust-lovable --release
Restart=always
RestartSec=10
Environment="RUST_LOG=info"
Environment="CONFIG_DIR=$CONFIG_DIR"

[Install]
WantedBy=default.target
EOF
        
        # Reload systemd
        systemctl --user daemon-reload
        
        print_success "Systemd service created (rust-lovable.service)"
    fi
}

# Function to add Rust Lovable to PATH
add_to_path() {
    print_section "PATH Configuration"
    
    # Add to shell profile
    local shell_profile=""
    if [[ -n "${SHELL}" ]]; then
        case "$(basename "$SHELL")" in
            bash)
                shell_profile="$HOME/.bashrc"
                ;;
            zsh)
                shell_profile="$HOME/.zshrc"
                ;;
            fish)
                shell_profile="$HOME/.config/fish/config.fish"
                ;;
            *)
                shell_profile="$HOME/.profile"
                ;;
        esac
    fi
    
    if [[ -n "$shell_profile" ]] && [[ ! -f "$shell_profile" ]]; then
        print_status "Creating shell profile: $shell_profile"
        touch "$shell_profile"
    fi
    
    if [[ -n "$shell_profile" ]] && ! grep -q "$BIN_DIR" "$shell_profile"; then
        print_status "Adding $BIN_DIR to PATH in $shell_profile"
        echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$shell_profile"
    fi
    
    # Add cargo to PATH if not already there
    if [[ -n "$shell_profile" ]] && ! grep -q "CARGO_HOME" "$shell_profile"; then
        echo 'export CARGO_HOME="$HOME/.cargo"' >> "$shell_profile"
        echo 'export PATH="$CARGO_HOME/bin:$PATH"' >> "$shell_profile"
    fi
    
    print_success "PATH configuration updated"
}

# Function to check and install system dependencies with platform-specific handling
install_system_deps_enhanced() {
    local platform_info=$1
    local cpu_cores=$2
    local memory_gb=$3
    
    print_section "System Dependencies"
    
    local platform_type=$(echo "$platform_info" | grep -o '"type":"[^"]*"' | cut -d'"' -f4)
    local distribution=$(echo "$platform_info" | grep -o '"distribution":"[^"]*"' | cut -d'"' -f4)
    
    case "$platform_type" in
        linux)
            install_linux_deps "$distribution" "$cpu_cores" "$memory_gb"
            ;;
        macos)
            install_macos_deps
            ;;
        *)
            print_warning "Unknown platform type: $platform_type"
            return
            ;;
    esac
}

# Linux-specific dependency installation
install_linux_deps() {
    local distribution=$1
    local cpu_cores=$2
    local memory_gb=$3
    
    # Detect package manager
    local pkg_mgr=""
    if command -v apt-get &> /dev/null; then
        pkg_mgr="apt"
    elif command -v yum &> /dev/null; then
        pkg_mgr="yum"
    elif command -v dnf &> /dev/null; then
        pkg_mgr="dnf"
    elif command -v pacman &> /dev/null; then
        pkg_mgr="pacman"
    fi
    
    print_status "Using package manager: $pkg_mgr"
    
    case "$pkg_mgr" in
        apt)
            sudo apt-get update
            sudo apt-get install -y build-essential pkg-config libssl-dev curl git
            
            # Install additional development tools
            sudo apt-get install -y cmake libsqlite3-dev libpq-dev
            
            # Install optional performance tools
            if [[ $cpu_cores -ge 4 ]] && [[ $memory_gb -ge 8 ]]; then
                print_status "Installing optional performance tools..."
                sudo apt-get install -y htop iotop sysstat
            fi
            ;;
        yum|dnf)
            sudo $pkg_mgr groupinstall -y "Development Tools"
            sudo $pkg_mgr install -y openssl-devel curl git
            
            # Install additional development tools
            sudo $pkg_mgr install -y cmake sqlite-devel postgresql-devel
            ;;
        pacman)
            sudo pacman -S --needed --noconfirm base-devel openssl curl git
            
            # Install additional development tools
            sudo pacman -S --needed --noconfirm cmake sqlite postgresql-libs
            ;;
    esac
}

# macOS-specific dependency installation
install_macos_deps() {
    if command -v brew &> /dev/null; then
        print_status "Installing dependencies with Homebrew..."
        brew install openssl cmake sqlite postgresql
    else
        print_warning "Homebrew not found. Please install it from https://brew.sh/"
        print_status "Or ensure Xcode Command Line Tools are installed: xcode-select --install"
    fi
}

# Function to optimize Rust build based on hardware
optimize_rust_build() {
    local cpu_cores=$1
    local memory_gb=$2
    local gpu_available=$3
    
    print_section "Build Optimization"
    
    # Set number of parallel jobs based on CPU cores
    local jobs=$((cpu_cores / 2))
    if [[ $jobs -lt 1 ]]; then
        jobs=1
    fi
    if [[ $jobs -gt 8 ]]; then
        jobs=8
    fi
    
    print_status "Using $jobs parallel build jobs"
    export CARGO_BUILD_JOBS=$jobs
    
    # Set memory limits for large projects
    if [[ $memory_gb -lt 8 ]]; then
        print_status "Low memory detected, setting conservative build options"
        export CARGO_BUILD_PIPELINING=false
    fi
    
    # Enable hardware-specific optimizations
    if [[ "$gpu_available" == "true" ]]; then
        print_status "GPU detected, enabling GPU-accelerated features"
        # Add GPU-specific build flags here if needed
    fi
}

# Function to create optimized configuration
create_optimized_config() {
    local hardware_info=$1
    local install_location=$2
    
    print_section "Configuration Optimization"
    
    local cpu_cores=$(echo "$hardware_info" | grep -o '"cpu_cores":[0-9]*' | cut -d':' -f2)
    local memory_gb=$(echo "$hardware_info" | grep -o '"memory_gb":[0-9]*' | cut -d':' -f2)
    local gpu_available=$(echo "$hardware_info" | grep -o '"gpu_available":[^,]*' | cut -d':' -f2)
    
    # Create optimized config based on hardware
    cat > "$CONFIG_DIR/config.toml" << EOF
[general]
platform = "universal"
auto_update = true
install_location = "$install_location"

[performance]
# Optimized for detected hardware
worker_threads = $((cpu_cores / 2 > 0 ? cpu_cores / 2 : 1))
max_sandboxes = $((cpu_cores > 4 ? 5 : 3))
cache_enabled = true
max_cache_size = "$((memory_gb / 2 > 1 ? memory_gb / 2 : 1))GB"
enable_gpu_acceleration = $gpu_available

[ai]
provider = "openai"
model = "gpt-4"
timeout = 30
max_concurrent_requests = $((cpu_cores > 4 ? 4 : 2))

[sandbox]
max_sandboxes = $((cpu_cores > 4 ? 5 : 3))
execution_timeout = 60
memory_limit_mb = $((memory_gb * 256))
cpu_limit_percent = 80

[database]
type = "sqlite"
path = "$DATA_DIR/data.db"
max_connections = $((cpu_cores * 10))

[logging]
level = "info"
file_path = "$LOGS_DIR/app.log"
max_size_mb = 100
max_files = 5

[hardware]
cpu_cores = $cpu_cores
memory_gb = $memory_gb
gpu_available = $gpu_available
EOF
    
    print_success "Optimized configuration created at $CONFIG_DIR/config.toml"
}

# Function to setup monitoring and health checks
setup_monitoring() {
    print_section "Monitoring Setup"
    
    # Create monitoring script
    cat > "$BIN_DIR/rust-lovable-monitor" << 'EOF'
#!/bin/bash

# Rust Lovable Health Monitor

CONFIG_DIR="$HOME/.config/rust-lovable"
LOGS_DIR="$HOME/.local/share/rust-lovable/logs"

# Check if service is running
check_service() {
    if systemctl --user is-active --quiet rust-lovable; then
        echo "✓ Service is running"
        return 0
    else
        echo "✗ Service is not running"
        return 1
    fi
}

# Check disk space
check_disk_space() {
    local usage=$(df "$HOME" | tail -1 | awk '{print $5}' | sed 's/%//')
    if [[ $usage -lt 80 ]]; then
        echo "✓ Disk space OK ($usage%)"
    elif [[ $usage -lt 90 ]]; then
        echo "⚠ Disk space warning ($usage%)"
    else
        echo "✗ Disk space critical ($usage%)"
    fi
}

# Check memory usage
check_memory() {
    if command -v free &> /dev/null; then
        local used_percent=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
        if [[ $used_percent -lt 80 ]]; then
            echo "✓ Memory usage OK ($used_percent%)"
        elif [[ $used_percent -lt 90 ]]; then
            echo "⚠ Memory usage warning ($used_percent%)"
        else
            echo "✗ Memory usage critical ($used_percent%)"
        fi
    fi
}

# Check recent logs for errors
check_logs() {
    if [[ -f "$LOGS_DIR/app.log" ]]; then
        local error_count=$(grep -c "ERROR" "$LOGS_DIR/app.log" 2>/dev/null || echo 0)
        if [[ $error_count -eq 0 ]]; then
            echo "✓ No recent errors in logs"
        else
            echo "⚠ Found $error_count recent errors in logs"
        fi
    fi
}

# Main check function
main() {
    echo "Rust Lovable Health Check"
    echo "========================"
    
    check_service
    check_disk_space
    check_memory
    check_logs
    
    echo ""
    echo "For detailed logs, run: journalctl --user -u rust-lovable -f"
}

main "$@"
EOF
    
    chmod +x "$BIN_DIR/rust-lovable-monitor"
    
    # Create systemd timer for health checks (Linux only)
    if [[ "$OSTYPE" == "linux-gnu"* ]] && command -v systemctl &> /dev/null; then
        cat > "$HOME/.config/systemd/user/rust-lovable-health.timer" << EOF
[Unit]
Description=Run Rust Lovable health check every hour

[Timer]
OnCalendar=hourly
Persistent=true

[Install]
WantedBy=timers.target
EOF
        
        cat > "$HOME/.config/systemd/user/rust-lovable-health.service" << EOF
[Unit]
Description=Rust Lovable Health Check

[Service]
Type=oneshot
ExecStart=$BIN_DIR/rust-lovable-monitor
EOF
    fi
    
    print_success "Monitoring setup completed"
}

# Function to perform post-installation verification
post_install_verification() {
    print_section "Post-Installation Verification"
    
    # Check binary
    if "$BIN_DIR/rust-lovable" --version &> /dev/null; then
        print_success "✓ Binary is executable and responds to --version"
    else
        print_error "✗ Binary verification failed"
        return 1
    fi
    
    # Check configuration
    if [[ -f "$CONFIG_DIR/config.toml" ]]; then
        print_success "✓ Configuration file exists"
    else
        print_error "✗ Configuration file missing"
        return 1
    fi
    
    # Check directories
    for dir in "$DATA_DIR" "$LOGS_DIR"; do
        if [[ -d "$dir" ]]; then
            print_success "✓ Directory $dir exists"
        else
            print_error "✗ Directory $dir missing"
            return 1
        fi
    done
    
    # Check permissions
    if [[ -w "$CONFIG_DIR" ]] && [[ -w "$DATA_DIR" ]] && [[ -w "$LOGS_DIR" ]]; then
        print_success "✓ All directories are writable"
    else
        print_error "✗ Some directories are not writable"
        return 1
    fi
    
    print_success "✓ All post-installation checks passed"
    return 0
}

# Function to print final installation summary
print_installation_summary() {
    print_header
    
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                  Installation Complete! 🎉                   ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════════╝${NC}"
    echo
    
    echo -e "${CYAN}Quick Start:${NC}"
    echo -e "  ${BLUE}rust-lovable --help${NC}     Show help information"
    echo -e "  ${BLUE}rust-lovable --dev${NC}      Start in development mode"
    echo -e "  ${BLUE}rust-lovable --release${NC}  Start in production mode"
    echo
    
    echo -e "${CYAN}Configuration:${NC}"
    echo -e "  Config:   ${CONFIG_DIR}/config.toml"
    echo -e "  Data:     $DATA_DIR"
    echo -e "  Logs:     $LOGS_DIR"
    echo -e "  Binary:   $BIN_DIR/rust-lovable"
    echo
    
    echo -e "${CYAN}Service Management (Linux):${NC}"
    echo -e "  ${BLUE}systemctl --user start rust-lovable${NC}    Start service"
    echo -e "  ${BLUE}systemctl --user enable rust-lovable${NC}   Enable auto-start"
    echo -e "  ${BLUE}systemctl --user status rust-lovable${NC}   Check status"
    echo
    
    echo -e "${CYAN}Monitoring:${NC}"
    echo -e "  ${BLUE}rust-lovable-monitor${NC}                  Run health check"
    echo -e "  ${BLUE}journalctl --user -u rust-lovable -f${NC}  View logs"
    echo
    
    echo -e "${CYAN}Documentation:${NC}"
    echo -e "  Wiki:     https://github.com/yourusername/rust-lovable/wiki"
    echo -e "  Issues:   https://github.com/yourusername/rust-lovable/issues"
    echo -e "  Discord:  https://discord.gg/rust-lovable"
    echo
    
    echo -e "${GREEN}Enjoy building UIs with Rust Lovable! 🚀${NC}"
}

# Main installation function
main() {
    print_header
    
    # Detect platform
    local platform_info=$(detect_platform_detailed)
    local platform_type=$(echo "$platform_info" | grep -o '"type":"[^"]*"' | cut -d'"' -f4)
    
    # Detect architecture
    local arch_info=$(detect_architecture_detailed)
    local arch=$(echo "$arch_info" | grep -o '"arch":"[^"]*"' | cut -d'"' -f4)
    
    print_status "Detected Platform: $(echo "$platform_info" | grep -o '"pretty_name":"[^"]*"' | cut -d'"' -f4)"
    print_status "Detected Architecture: $arch"
    
    # Run hardware detection
    local hardware_metrics=$(run_hardware_detection)
    local cpu_cores=$(echo "$hardware_metrics" | cut -d',' -f1)
    local memory_gb=$(echo "$hardware_metrics" | cut -d',' -f2)
    local gpu_available=$(echo "$hardware_metrics" | cut -d',' -f3)
    local disk_usage=$(echo "$hardware_metrics" | cut -d',' -f4)
    
    # Check requirements
    if [[ $cpu_cores -lt 2 ]]; then
        print_warning "CPU cores ($cpu_cores) below recommended minimum (2)"
    fi
    
    if [[ $memory_gb -lt 4 ]]; then
        print_warning "Memory (${memory_gb}GB) below recommended minimum (4GB)"
    fi
    
    if [[ $disk_usage -gt 90 ]]; then
        print_error "Disk usage critical (${disk_usage}%), please free up space"
        exit 1
    fi
    
    # Create directories
    print_section "Directory Setup"
    mkdir -p "$INSTALL_DIR" "$BIN_DIR" "$CONFIG_DIR" "$DATA_DIR" "$LOGS_DIR"
    print_success "Created necessary directories"
    
    # Install system dependencies
    install_system_deps_enhanced "$platform_info" "$cpu_cores" "$memory_gb"
    
    # Check and install Rust
    if ! check_rust; then
        install_rust
    fi
    
    # Optimize build based on hardware
    optimize_rust_build "$cpu_cores" "$memory_gb" "$gpu_available"
    
    # Install Rust Lovable
    install_rust_lovable "$platform_type" "$arch"
    
    # Create optimized configuration
    create_optimized_config "$hardware_metrics" "$INSTALL_DIR"
    
    # Setup monitoring
    setup_monitoring
    
    # Setup systemd service (Linux only)
    setup_systemd_service "$platform_type"
    
    # Add to PATH
    add_to_path
    
    # Run post-installation verification
    if post_install_verification; then
        print_installation_summary
    else
        print_error "Installation completed but verification failed"
        print_status "Please check the errors above and try again"
        exit 1
    fi
}

# Run main function with error handling
trap 'print_error "Installation failed. Check the logs above for details."; exit 1' ERR
main "$@"