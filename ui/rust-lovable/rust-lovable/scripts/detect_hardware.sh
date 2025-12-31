#!/bin/bash

# Hardware Detection Script for Rust Lovable
# This script provides granular hardware and system information

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to detect CPU information
detect_cpu_info() {
    local cpu_info="{}"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux CPU detection
        if command -v lscpu &> /dev/null; then
            local cpu_model=$(lscpu | grep "Model name" | cut -d':' -f2 | xargs)
            local cpu_cores=$(lscpu | grep "^CPU(s):" | cut -d':' -f2 | xargs)
            local cpu_threads=$(lscpu | grep "Thread(s) per core" | cut -d':' -f2 | xargs)
            local cpu_cores_per_socket=$(lscpu | grep "Core(s) per socket" | cut -d':' -f2 | xargs)
            local cpu_sockets=$(lscpu | grep "Socket(s):" | cut -d':' -f2 | xargs)
            local cpu_arch=$(lscpu | grep "Architecture:" | cut -d':' -f2 | xargs)
            local cpu_flags=$(lscpu | grep "Flags:" | cut -d':' -f2 | xargs)
            
            cpu_info=$(cat << EOF
{
  "model": "$cpu_model",
  "cores": $cpu_cores,
  "threads_per_core": $cpu_threads,
  "cores_per_socket": $cpu_cores_per_socket,
  "sockets": $cpu_sockets,
  "architecture": "$cpu_arch",
  "flags": "$cpu_flags",
  "little_endian": true
}
EOF
            )
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS CPU detection
        local cpu_model=$(sysctl -n machdep.cpu.brand_string)
        local cpu_cores=$(sysctl -n hw.ncpu)
        local cpu_threads=$(sysctl -n machdep.cpu.thread_count)
        local cpu_arch=$(uname -m)
        local cpu_family=$(sysctl -n machdep.cpu.family)
        local cpu_model_num=$(sysctl -n machdep.cpu.model)
        
        cpu_info=$(cat << EOF
{
  "model": "$cpu_model",
  "cores": $cpu_cores,
  "threads": ${cpu_threads:-$cpu_cores},
  "architecture": "$cpu_arch",
  "family": $cpu_family,
  "model_number": $cpu_model_num,
  "little_endian": true
}
EOF
        )
    fi
    
    echo "$cpu_info"
}

# Function to detect memory information
detect_memory_info() {
    local memory_info="{}"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux memory detection
        if command -v free &> /dev/null; then
            local mem_total_kb=$(free | grep "Mem:" | awk '{print $2}')
            local mem_used_kb=$(free | grep "Mem:" | awk '{print $3}')
            local mem_free_kb=$(free | grep "Mem:" | awk '{print $4}')
            local mem_available_kb=$(free | grep "Mem:" | awk '{print $7}')
            local swap_total_kb=$(free | grep "Swap:" | awk '{print $2}')
            local swap_used_kb=$(free | grep "Swap:" | awk '{print $3}')
            
            local mem_total_gb=$(echo "scale=2; $mem_total_kb / 1024 / 1024" | bc -l)
            local mem_used_gb=$(echo "scale=2; $mem_used_kb / 1024 / 1024" | bc -l)
            local mem_free_gb=$(echo "scale=2; $mem_free_kb / 1024 / 1024" | bc -l)
            local mem_available_gb=$(echo "scale=2; $mem_available_kb / 1024 / 1024" | bc -l)
            local swap_total_gb=$(echo "scale=2; $swap_total_kb / 1024 / 1024" | bc -l)
            
            memory_info=$(cat << EOF
{
  "total_gb": $mem_total_gb,
  "used_gb": $mem_used_gb,
  "free_gb": $mem_free_gb,
  "available_gb": $mem_available_gb,
  "swap_total_gb": $swap_total_gb,
  "swap_used_gb": $(echo "scale=2; $swap_used_kb / 1024 / 1024" | bc -l)
}
EOF
            )
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS memory detection
        local mem_bytes=$(sysctl -n hw.memsize)
        local page_size=$(sysctl -n hw.pagesize)
        local inactive_pages=$(vm_stat | grep "Pages inactive" | awk '{print $3}' | sed 's/\.//')
        local active_pages=$(vm_stat | grep "Pages active" | awk '{print $3}' | sed 's/\.//')
        local wired_pages=$(vm_stat | grep "Pages wired down" | awk '{print $4}' | sed 's/\.//')
        local free_pages=$(vm_stat | grep "Pages free" | awk '{print $3}' | sed 's/\.//')
        
        local mem_total_gb=$(echo "scale=2; $mem_bytes / 1024 / 1024 / 1024" | bc -l)
        local mem_used_pages=$((active_pages + inactive_pages + wired_pages))
        local mem_used_gb=$(echo "scale=2; $mem_used_pages * $page_size / 1024 / 1024 / 1024" | bc -l)
        local mem_free_gb=$(echo "scale=2; $free_pages * $page_size / 1024 / 1024 / 1024" | bc -l)
        
        memory_info=$(cat << EOF
{
  "total_gb": $mem_total_gb,
  "used_gb": $mem_used_gb,
  "free_gb": $mem_free_gb,
  "available_gb": $mem_free_gb,
  "swap_total_gb": 0,
  "swap_used_gb": 0
}
EOF
        )
    fi
    
    echo "$memory_info"
}

# Function to detect GPU information
detect_gpu_info() {
    local gpu_info="{}"
    local gpu_available=false
    local gpu_model=""
    local gpu_memory_mb=0
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux GPU detection
        if command -v nvidia-smi &> /dev/null; then
            gpu_available=true
            gpu_model=$(nvidia-smi --query-gpu=name --format=csv,noheader,nounits | head -1)
            gpu_memory_mb=$(nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits | head -1)
        elif command -v lspci &> /dev/null; then
            # Check for AMD/Intel GPUs
            local gpu_pci=$(lspci | grep -i "vga\|3d" | head -1)
            if [[ -n "$gpu_pci" ]]; then
                gpu_available=true
                gpu_model=$(echo "$gpu_pci" | cut -d':' -f3 | xargs)
            fi
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS GPU detection
        gpu_available=true
        gpu_model=$(system_profiler SPDisplaysDataType | grep "Chipset Model" | head -1 | cut -d':' -f2 | xargs)
        if [[ -z "$gpu_model" ]]; then
            gpu_model=$(system_profiler SPDisplaysDataType | grep "Device" | head -1 | cut -d':' -f2 | xargs)
        fi
    fi
    
    gpu_info=$(cat << EOF
{
  "available": $gpu_available,
  "model": "${gpu_model:-null}",
  "memory_mb": $gpu_memory_mb
}
EOF
    )
    
    echo "$gpu_info"
}

# Function to detect storage information
detect_storage_info() {
    local storage_info="{}"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux storage detection
        if command -v df &> /dev/null; then
            local disk_info=$(df -h / | tail -1)
            local disk_total=$(echo "$disk_info" | awk '{print $2}')
            local disk_used=$(echo "$disk_info" | awk '{print $3}')
            local disk_available=$(echo "$disk_info" | awk '{print $4}')
            local disk_usage_percent=$(echo "$disk_info" | awk '{print $5}' | sed 's/%//')
            
            storage_info=$(cat << EOF
{
  "total": "$disk_total",
  "used": "$disk_used",
  "available": "$disk_available",
  "usage_percent": $disk_usage_percent,
  "filesystem": "ext4"
}
EOF
            )
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS storage detection
        local disk_info=$(df -h / | tail -1)
        local disk_total=$(echo "$disk_info" | awk '{print $2}')
        local disk_used=$(echo "$disk_info" | awk '{print $3}')
        local disk_available=$(echo "$disk_info" | awk '{print $4}')
        local disk_usage_percent=$(echo "$disk_info" | awk '{print $5}' | sed 's/%//')
        
        storage_info=$(cat << EOF
{
  "total": "$disk_total",
  "used": "$disk_used",
  "available": "$disk_available",
  "usage_percent": $disk_usage_percent,
  "filesystem": "apfs"
}
EOF
        )
    fi
    
    echo "$storage_info"
}

# Function to detect network information
detect_network_info() {
    local network_info="{}"
    
    if command -v ip &> /dev/null; then
        # Linux network detection
        local interfaces=$(ip link show | grep -E "^[0-9]+:" | awk -F: '{print $2}' | xargs)
        local ip_address=$(ip addr show | grep "inet " | grep -v "127.0.0.1" | head -1 | awk '{print $2}' | cut -d'/' -f1)
        
        network_info=$(cat << EOF
{
  "interfaces": [$(echo "$interfaces" | sed 's/^/"/' | sed 's/$/"/' | tr ' ' ',')],
  "ip_address": "${ip_address:-null}",
  "hostname": "$(hostname)"
}
EOF
        )
    elif command -v ifconfig &> /dev/null; then
        # macOS network detection
        local interfaces=$(ifconfig | grep "^[a-zA-Z]" | awk '{print $1}' | xargs)
        local ip_address=$(ifconfig | grep "inet " | grep -v "127.0.0.1" | head -1 | awk '{print $2}')
        
        network_info=$(cat << EOF
{
  "interfaces": [$(echo "$interfaces" | sed 's/^/"/' | sed 's/$/"/' | tr ' ' ',')],
  "ip_address": "${ip_address:-null}",
  "hostname": "$(hostname)"
}
EOF
        )
    fi
    
    echo "$network_info"
}

# Function to detect development tools
detect_dev_tools() {
    local tools_info="{}"
    
    local has_git=false
    local has_node=false
    local has_npm=false
    local has_yarn=false
    local has_pnpm=false
    local has_bun=false
    local has_rust=false
    local has_cargo=false
    local has_docker=false
    local has_python=false
    
    command -v git &> /dev/null && has_git=true
    command -v node &> /dev/null && has_node=true
    command -v npm &> /dev/null && has_npm=true
    command -v yarn &> /dev/null && has_yarn=true
    command -v pnpm &> /dev/null && has_pnpm=true
    command -v bun &> /dev/null && has_bun=true
    command -v rustc &> /dev/null && has_rust=true
    command -v cargo &> /dev/null && has_cargo=true
    command -v docker &> /dev/null && has_docker=true
    command -v python3 &> /dev/null && has_python=true
    
    tools_info=$(cat << EOF
{
  "git": $has_git,
  "node": $has_node,
  "npm": $has_npm,
  "yarn": $has_yarn,
  "pnpm": $has_pnpm,
  "bun": $has_bun,
  "rust": $has_rust,
  "cargo": $has_cargo,
  "docker": $has_docker,
  "python": $has_python
}
EOF
    )
    
    echo "$tools_info"
}

# Function to detect performance capabilities
detect_performance_info() {
    local performance_info="{}"
    
    # CPU benchmark (simplified)
    local cpu_score=0
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v sysbench &> /dev/null; then
            cpu_score=$(sysbench cpu --cpu-max-prime=10000 run 2>&1 | grep "events per second" | awk '{print $4}')
        fi
    fi
    
    # Memory bandwidth (simplified)
    local memory_bandwidth=0
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v mbw &> /dev/null; then
            memory_bandwidth=$(mbw -q 100 2>&1 | grep "AVG" | awk '{print $2}')
        fi
    fi
    
    # Disk I/O (simplified)
    local disk_iops=0
    if command -v fio &> /dev/null; then
        disk_iops=$(fio --name=randread --ioengine=libaio --iodepth=32 --rw=randread --bs=4k --direct=1 --size=128M --numjobs=1 --runtime=10 --group_reporting 2>&1 | grep "iops" | awk '{print $3}' | cut -d'=' -f2)
    fi
    
    performance_info=$(cat << EOF
{
  "cpu_score": ${cpu_score:-0},
  "memory_bandwidth": ${memory_bandwidth:-0},
  "disk_iops": ${disk_iops:-0},
  "supports_sse": true,
  "supports_avx": true,
  "supports_avx2": $(grep -q avx2 /proc/cpuinfo && echo true || echo false),
  "supports_avx512": $(grep -q avx512 /proc/cpuinfo && echo true || echo false)
}
EOF
    )
    
    echo "$performance_info"
}

# Function to detect security features
detect_security_info() {
    local security_info="{}"
    
    local has_selinux=false
    local has_apparmor=false
    local has_firewalld=false
    local has_ufw=false
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        [[ -f /etc/selinux/config ]] && has_selinux=true
        [[ -d /etc/apparmor.d ]] && has_apparmor=true
        systemctl is-active --quiet firewalld && has_firewalld=true
        systemctl is-active --quiet ufw && has_ufw=true
    fi
    
    security_info=$(cat << EOF
{
  "selinux": $has_selinux,
  "apparmor": $has_apparmor,
  "firewalld": $has_firewalld,
  "ufw": $has_ufw,
  "firewall_active": $has_firewalld || $has_ufw
}
EOF
    )
    
    echo "$security_info"
}

# Function to detect virtualization/containerization
detect_virtualization_info() {
    local virt_info="{}"
    
    local is_container=false
    local is_vm=false
    local container_type="none"
    
    # Check for containerization
    if [[ -f /.dockerenv ]] || grep -q docker /proc/1/cgroup 2>/dev/null; then
        is_container=true
        container_type="docker"
    elif [[ -n "${container:-}" ]] || [[ -n "${CONTAINER:-}" ]]; then
        is_container=true
        container_type="podman"
    fi
    
    # Check for virtualization
    if [[ -d /proc/vz ]] && [[ ! -d /proc/bc ]]; then
        is_vm=true
    elif [[ -f /proc/cpuinfo ]] && grep -q "vmx\|svm" /proc/cpuinfo; then
        # Could be VM or bare metal with VT-x/AMD-V
        if [[ $(dmesg 2>/dev/null | grep -i "hypervisor" | wc -l) -gt 0 ]]; then
            is_vm=true
        fi
    fi
    
    virt_info=$(cat << EOF
{
  "is_container": $is_container,
  "is_vm": $is_vm,
  "container_type": "$container_type"
}
EOF
    )
    
    echo "$virt_info"
}

# Main function to detect all hardware information
detect_all_hardware() {
    print_status "Detecting hardware and system information..."
    
    local os_info=$(detect_os_info)
    local cpu_info=$(detect_cpu_info)
    local memory_info=$(detect_memory_info)
    local gpu_info=$(detect_gpu_info)
    local storage_info=$(detect_storage_info)
    local network_info=$(detect_network_info)
    local dev_tools=$(detect_dev_tools)
    local performance_info=$(detect_performance_info)
    local security_info=$(detect_security_info)
    local virtualization_info=$(detect_virtualization_info)
    
    local full_hardware_info=$(cat << EOF
{
  "os": $os_info,
  "cpu": $cpu_info,
  "memory": $memory_info,
  "gpu": $gpu_info,
  "storage": $storage_info,
  "network": $network_info,
  "tools": $dev_tools,
  "performance": $performance_info,
  "security": $security_info,
  "virtualization": $virtualization_info,
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
    )
    
    print_success "Hardware detection completed"
    echo "$full_hardware_info"
}

# Function to detect OS information
detect_os_info() {
    local os_name="Unknown"
    local os_version="Unknown"
    local kernel_version="Unknown"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        os_name="Linux"
        if [[ -f /etc/os-release ]]; then
            . /etc/os-release
            os_name=$NAME
            os_version=$VERSION_ID
        fi
        kernel_version=$(uname -r)
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        os_name="macOS"
        os_version=$(sw_vers -productVersion)
        kernel_version=$(uname -r)
    fi
    
    cat << EOF
{
  "name": "$os_name",
  "version": "$os_version",
  "kernel": "$kernel_version",
  "hostname": "$(hostname)",
  "uptime_seconds": $(awk '{print $1}' /proc/uptime 2>/dev/null || echo 0)
}
EOF
}

# Run the detection if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    detect_all_hardware
fi