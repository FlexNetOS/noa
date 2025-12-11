// Package resources provides resource and capability detection
//
// T237: §3.8 Implement resource capability detection
// US6: P2P Hive-Mind Device Federation
package resources

import (
	"runtime"
	"sync"
	"time"

	"github.com/shirou/gopsutil/v3/cpu"
	"github.com/shirou/gopsutil/v3/mem"
	"github.com/shirou/gopsutil/v3/disk"
)

// Capability represents a device capability
type Capability string

const (
	CapabilityInference Capability = "inference"
	CapabilityDigest    Capability = "digest"
	CapabilityStorage   Capability = "storage"
	CapabilityCompute   Capability = "compute"
	CapabilityGPU       Capability = "gpu"
)

// ResourceInfo holds device resource information
type ResourceInfo struct {
	CPUCores        int64   `json:"cpu_cores"`
	MemoryMB        int64   `json:"memory_mb"`
	StorageMB       int64   `json:"storage_mb"`
	GPUMemoryMB     int64   `json:"gpu_memory_mb"`
	CPUAvailable    float64 `json:"cpu_available"`    // 0.0 - 1.0
	MemoryAvailable float64 `json:"memory_available"` // 0.0 - 1.0
	StorageAvailable float64 `json:"storage_available"` // 0.0 - 1.0
}

// CapabilityInfo holds device capability information
type CapabilityInfo struct {
	Capabilities []Capability `json:"capabilities"`
	Resources    ResourceInfo `json:"resources"`
	LastUpdated  time.Time    `json:"last_updated"`
}

// Detector detects device capabilities and resources
type Detector struct {
	mu          sync.RWMutex
	lastInfo    *CapabilityInfo
	updateInterval time.Duration
}

// NewDetector creates a new capability detector
//
// Implements T237: §3.8 Implement resource capability detection
func NewDetector(updateInterval time.Duration) *Detector {
	if updateInterval == 0 {
		updateInterval = 30 * time.Second
	}
	return &Detector{
		updateInterval: updateInterval,
	}
}

// Detect detects current device capabilities and resources
func (d *Detector) Detect() (*CapabilityInfo, error) {
	// Detect CPU
	cpuCount, err := cpu.Counts(true)
	if err != nil {
		return nil, err
	}

	// Detect memory
	memInfo, err := mem.VirtualMemory()
	if err != nil {
		return nil, err
	}

	// Detect storage
	diskInfo, err := disk.Usage("/")
	if err != nil {
		return nil, err
	}

	// Detect GPU (simplified - would need actual GPU detection)
	gpuMemoryMB := int64(0)
	// TODO: Implement actual GPU detection using CUDA/OpenCL APIs

	// Calculate available resources
	cpuAvailable := 1.0 // TODO: Get actual CPU usage
	memoryAvailable := float64(memInfo.Available) / float64(memInfo.Total)
	storageAvailable := float64(diskInfo.Free) / float64(diskInfo.Total)

	// Determine capabilities based on resources
	capabilities := []Capability{
		CapabilityStorage, // Always available
		CapabilityCompute, // Always available
	}

	// Inference capability if sufficient memory
	if memInfo.Total > 4*1024*1024*1024 { // 4GB
		capabilities = append(capabilities, CapabilityInference)
	}

	// Digest capability if sufficient storage
	if diskInfo.Free > 10*1024*1024*1024 { // 10GB
		capabilities = append(capabilities, CapabilityDigest)
	}

	// GPU capability if GPU detected
	if gpuMemoryMB > 0 {
		capabilities = append(capabilities, CapabilityGPU)
	}

	info := &CapabilityInfo{
		Capabilities: capabilities,
		Resources: ResourceInfo{
			CPUCores:        int64(cpuCount),
			MemoryMB:        int64(memInfo.Total / 1024 / 1024),
			StorageMB:        int64(diskInfo.Total / 1024 / 1024),
			GPUMemoryMB:     gpuMemoryMB,
			CPUAvailable:    cpuAvailable,
			MemoryAvailable: memoryAvailable,
			StorageAvailable: storageAvailable,
		},
		LastUpdated: time.Now(),
	}

	d.mu.Lock()
	d.lastInfo = info
	d.mu.Unlock()

	return info, nil
}

// GetLastInfo returns the last detected capability info
func (d *Detector) GetLastInfo() *CapabilityInfo {
	d.mu.RLock()
	defer d.mu.RUnlock()
	return d.lastInfo
}

// GetPlatform returns the current platform
func GetPlatform() string {
	return runtime.GOOS
}

// GetArchitecture returns the current architecture
func GetArchitecture() string {
	return runtime.GOARCH
}

