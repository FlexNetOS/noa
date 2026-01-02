#!/bin/bash

# Rust Lovable - Dynamic Deployment Script
# Integrates with detect_hardware.sh for platform-aware deployment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_header() {
    echo -e "${CYAN}========================================${NC}"
    echo -e "${CYAN}  Rust Lovable - Dynamic Deployment${NC}"
    echo -e "${CYAN}  $(date)${NC}"
    echo -e "${CYAN}========================================${NC}"
}

print_status() { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Default configuration
DEPLOY_DIR="${DEPLOY_DIR:-$PROJECT_ROOT/deployed}"
LOG_DIR="${LOG_DIR:-$PROJECT_ROOT/logs}"
CONFIG_DIR="${CONFIG_DIR:-$PROJECT_ROOT/config}"
PORT="${PORT:-8080}"
HOST="${HOST:-127.0.0.1}"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --port)
            PORT="$2"
            shift 2
            ;;
        --host)
            HOST="$2"
            shift 2
            ;;
        --deploy-dir)
            DEPLOY_DIR="$2"
            shift 2
            ;;
        --config)
            CONFIG_DIR="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --port PORT       Server port (default: 8080)"
            echo "  --host HOST       Server host (default: 127.0.0.1)"
            echo "  --deploy-dir DIR  Deployment directory"
            echo "  --config DIR      Configuration directory"
            echo "  --help            Show this help message"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Step 1: Detect hardware
detect_hardware() {
    print_status "Detecting hardware configuration..."
    
    if [[ -f "$SCRIPT_DIR/detect_hardware.sh" ]]; then
        source "$SCRIPT_DIR/detect_hardware.sh"
        HARDWARE_INFO=$(detect_all_hardware)
        
        # Parse hardware info for deployment decisions
        CPU_CORES=$(echo "$HARDWARE_INFO" | grep -o '"cores":[0-9]*' | head -1 | cut -d':' -f2)
        MEMORY_GB=$(echo "$HARDWARE_INFO" | grep -o '"total_gb":[0-9.]*' | head -1 | cut -d':' -f2 | cut -d'.' -f1)
        GPU_AVAILABLE=$(echo "$HARDWARE_INFO" | grep -o '"available":[a-z]*' | head -1 | cut -d':' -f2)
        
        print_success "Hardware detected:"
        echo "  - CPU Cores: ${CPU_CORES:-unknown}"
        echo "  - Memory: ${MEMORY_GB:-unknown} GB"
        echo "  - GPU Available: ${GPU_AVAILABLE:-unknown}"
        
        # Save hardware info
        mkdir -p "$CONFIG_DIR"
        echo "$HARDWARE_INFO" > "$CONFIG_DIR/hardware.json"
        print_success "Hardware info saved to $CONFIG_DIR/hardware.json"
    else
        print_warning "detect_hardware.sh not found, using defaults"
        CPU_CORES=4
        MEMORY_GB=8
        GPU_AVAILABLE=false
    fi
}

# Step 2: Configure based on hardware
configure_deployment() {
    print_status "Configuring deployment based on hardware..."
    
    # Determine optimal configuration
    if [[ "${CPU_CORES:-4}" -ge 8 && "${MEMORY_GB:-8}" -ge 16 ]]; then
        DEPLOYMENT_PROFILE="high-performance"
        WORKER_THREADS=$((CPU_CORES / 2))
        MAX_CONNECTIONS=10000
        CACHE_SIZE_MB=$((MEMORY_GB * 100))
    elif [[ "${CPU_CORES:-4}" -ge 4 && "${MEMORY_GB:-8}" -ge 8 ]]; then
        DEPLOYMENT_PROFILE="standard"
        WORKER_THREADS=4
        MAX_CONNECTIONS=5000
        CACHE_SIZE_MB=500
    else
        DEPLOYMENT_PROFILE="minimal"
        WORKER_THREADS=2
        MAX_CONNECTIONS=1000
        CACHE_SIZE_MB=256
    fi
    
    print_success "Deployment profile: $DEPLOYMENT_PROFILE"
    echo "  - Worker threads: $WORKER_THREADS"
    echo "  - Max connections: $MAX_CONNECTIONS"
    echo "  - Cache size: ${CACHE_SIZE_MB}MB"
    
    # Create runtime configuration
    cat > "$CONFIG_DIR/runtime.json" << EOF
{
  "profile": "$DEPLOYMENT_PROFILE",
  "server": {
    "host": "$HOST",
    "port": $PORT,
    "worker_threads": $WORKER_THREADS,
    "max_connections": $MAX_CONNECTIONS
  },
  "cache": {
    "size_mb": $CACHE_SIZE_MB,
    "enabled": true
  },
  "gpu": {
    "enabled": ${GPU_AVAILABLE:-false},
    "acceleration": ${GPU_AVAILABLE:-false}
  },
  "logging": {
    "level": "info",
    "directory": "$LOG_DIR",
    "max_size_mb": 100,
    "max_files": 10
  },
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
    
    print_success "Runtime configuration saved to $CONFIG_DIR/runtime.json"
}

# Step 3: Prepare deployment directory
prepare_deployment() {
    print_status "Preparing deployment directory..."
    
    mkdir -p "$DEPLOY_DIR"
    mkdir -p "$LOG_DIR"
    mkdir -p "$CONFIG_DIR"
    
    # Copy binary
    if [[ -f "$PROJECT_ROOT/dist/rust-lovable" ]]; then
        cp "$PROJECT_ROOT/dist/rust-lovable" "$DEPLOY_DIR/"
        chmod +x "$DEPLOY_DIR/rust-lovable"
    elif [[ -f "$PROJECT_ROOT/dist/rust-lovable.exe" ]]; then
        cp "$PROJECT_ROOT/dist/rust-lovable.exe" "$DEPLOY_DIR/"
    elif [[ -f "$PROJECT_ROOT/target/release/rust-lovable" ]]; then
        cp "$PROJECT_ROOT/target/release/rust-lovable" "$DEPLOY_DIR/"
        chmod +x "$DEPLOY_DIR/rust-lovable"
    elif [[ -f "$PROJECT_ROOT/target/release/rust-lovable.exe" ]]; then
        cp "$PROJECT_ROOT/target/release/rust-lovable.exe" "$DEPLOY_DIR/"
    else
        print_error "Binary not found! Run prepare-deployment.sh first."
        exit 1
    fi
    
    # Copy configuration files
    cp -r "$CONFIG_DIR"/* "$DEPLOY_DIR/config/" 2>/dev/null || mkdir -p "$DEPLOY_DIR/config"
    cp "$CONFIG_DIR/runtime.json" "$DEPLOY_DIR/config/" 2>/dev/null || true
    cp "$CONFIG_DIR/hardware.json" "$DEPLOY_DIR/config/" 2>/dev/null || true
    
    print_success "Deployment directory prepared: $DEPLOY_DIR"
}

# Step 4: Create systemd service (Linux only)
create_systemd_service() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        print_status "Creating systemd service..."
        
        SERVICE_FILE="/etc/systemd/system/rust-lovable.service"
        
        if [[ -w "/etc/systemd/system" ]]; then
            cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Rust Lovable - Conversational UI Builder
After=network.target

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=$DEPLOY_DIR
ExecStart=$DEPLOY_DIR/rust-lovable
Restart=on-failure
RestartSec=5
Environment=RUST_LOVABLE_ADDRESS=$HOST:$PORT
Environment=RUST_LOVABLE_CONFIG_DIR=$DEPLOY_DIR/config
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
            
            systemctl daemon-reload
            print_success "Systemd service created"
        else
            print_warning "Cannot create systemd service (no write permission)"
        fi
    fi
}

# Step 5: Create launcher scripts
create_launchers() {
    print_status "Creating launcher scripts..."
    
    # Bash launcher
    cat > "$DEPLOY_DIR/start.sh" << 'LAUNCHER'
#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export RUST_LOVABLE_CONFIG_DIR="$SCRIPT_DIR/config"
export RUST_LOG="${RUST_LOG:-info}"

# Read port from runtime config
if [[ -f "$SCRIPT_DIR/config/runtime.json" ]]; then
    PORT=$(grep -o '"port":[0-9]*' "$SCRIPT_DIR/config/runtime.json" | cut -d':' -f2)
    HOST=$(grep -o '"host":"[^"]*"' "$SCRIPT_DIR/config/runtime.json" | cut -d'"' -f4)
    export RUST_LOVABLE_ADDRESS="${HOST:-127.0.0.1}:${PORT:-8080}"
fi

echo "Starting Rust Lovable..."
echo "Server address: ${RUST_LOVABLE_ADDRESS:-127.0.0.1:8080}"
echo "Config directory: $RUST_LOVABLE_CONFIG_DIR"
echo "Log level: $RUST_LOG"
echo ""

if [[ -f "$SCRIPT_DIR/rust-lovable" ]]; then
    "$SCRIPT_DIR/rust-lovable" "$@"
elif [[ -f "$SCRIPT_DIR/rust-lovable.exe" ]]; then
    "$SCRIPT_DIR/rust-lovable.exe" "$@"
else
    echo "Error: Binary not found!"
    exit 1
fi
LAUNCHER
    chmod +x "$DEPLOY_DIR/start.sh"
    
    # PowerShell launcher for Windows
    cat > "$DEPLOY_DIR/start.ps1" << 'PWSH_LAUNCHER'
# Rust Lovable - PowerShell Launcher
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$env:RUST_LOVABLE_CONFIG_DIR = "$ScriptDir\config"
$env:RUST_LOG = if ($env:RUST_LOG) { $env:RUST_LOG } else { "info" }

# Read port from runtime config
$RuntimeConfig = "$ScriptDir\config\runtime.json"
if (Test-Path $RuntimeConfig) {
    $Config = Get-Content $RuntimeConfig | ConvertFrom-Json
    $Host = $Config.server.host
    $Port = $Config.server.port
    $env:RUST_LOVABLE_ADDRESS = "${Host}:${Port}"
}

Write-Host "Starting Rust Lovable..." -ForegroundColor Cyan
Write-Host "Server address: $env:RUST_LOVABLE_ADDRESS" -ForegroundColor Green
Write-Host "Config directory: $env:RUST_LOVABLE_CONFIG_DIR" -ForegroundColor Green
Write-Host "Log level: $env:RUST_LOG" -ForegroundColor Green
Write-Host ""

$Binary = "$ScriptDir\rust-lovable.exe"
if (Test-Path $Binary) {
    & $Binary @args
} else {
    Write-Host "Error: Binary not found!" -ForegroundColor Red
    exit 1
}
PWSH_LAUNCHER
    
    # Stop script
    cat > "$DEPLOY_DIR/stop.sh" << 'STOP_SCRIPT'
#!/bin/bash
echo "Stopping Rust Lovable..."
pkill -f "rust-lovable" || echo "Not running"
echo "Stopped."
STOP_SCRIPT
    chmod +x "$DEPLOY_DIR/stop.sh"
    
    print_success "Launcher scripts created"
}

# Step 6: Deploy and start
deploy_and_start() {
    print_status "Deploying and starting application..."
    
    # Set environment variables
    export RUST_LOVABLE_ADDRESS="$HOST:$PORT"
    export RUST_LOVABLE_CONFIG_DIR="$DEPLOY_DIR/config"
    export RUST_LOG="${RUST_LOG:-info}"
    
    # Start the application
    cd "$DEPLOY_DIR"
    
    if [[ -f "./rust-lovable" ]]; then
        print_status "Starting server on $HOST:$PORT..."
        nohup ./rust-lovable > "$LOG_DIR/rust-lovable.log" 2>&1 &
        echo $! > "$DEPLOY_DIR/rust-lovable.pid"
        STARTED_PID=$!
    elif [[ -f "./rust-lovable.exe" ]]; then
        print_status "Starting server on $HOST:$PORT..."
        # On Windows/Git Bash, start in background
        ./rust-lovable.exe > "$LOG_DIR/rust-lovable.log" 2>&1 &
        echo $! > "$DEPLOY_DIR/rust-lovable.pid"
        STARTED_PID=$!
    else
        print_error "Binary not found in deployment directory!"
        exit 1
    fi
    
    # Wait for startup
    sleep 2
    
    # Check if running
    if kill -0 $STARTED_PID 2>/dev/null; then
        print_success "Application started successfully (PID: $STARTED_PID)"
        print_success "Server running at http://$HOST:$PORT"
    else
        print_error "Failed to start application. Check logs at $LOG_DIR/rust-lovable.log"
        exit 1
    fi
}

# Main deployment flow
main() {
    print_header
    
    detect_hardware
    echo ""
    configure_deployment
    echo ""
    prepare_deployment
    echo ""
    create_launchers
    echo ""
    
    # Only create systemd service on Linux
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        create_systemd_service
        echo ""
    fi
    
    deploy_and_start
    
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}  Deployment Complete!${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo "Deployment directory: $DEPLOY_DIR"
    echo "Log directory: $LOG_DIR"
    echo "Configuration: $DEPLOY_DIR/config/"
    echo ""
    echo "Useful commands:"
    echo "  Start:  $DEPLOY_DIR/start.sh"
    echo "  Stop:   $DEPLOY_DIR/stop.sh"
    echo "  Logs:   tail -f $LOG_DIR/rust-lovable.log"
    echo ""
    echo "Server URL: http://$HOST:$PORT"
}

main "$@"
