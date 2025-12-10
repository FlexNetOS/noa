// Package tasks provides task offloading and distribution
//
// T246: §3.3 Implement task offloading protocol
// US6: P2P Hive-Mind Device Federation
package tasks

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/libp2p/go-libp2p/core/peer"
)

// OffloadRequest represents a task offload request
type OffloadRequest struct {
	TaskID      string
	TaskType    string
	Payload     []byte
	Requirements ResourceRequirements
	TargetPeer  peer.ID
	Timeout     time.Duration
}

// OffloadResponse represents the response to an offload request
type OffloadResponse struct {
	Accepted    bool
	ExecutionID string
	EstimatedMs int64
	Error       error
}

// ResourceRequirements represents resource requirements for a task
type ResourceRequirements struct {
	MinMemoryMB int64
	MinCPUCores int64
	RequiresGPU bool
	EstimatedDuration time.Duration
}

// OffloadProtocol manages task offloading to peers
type OffloadProtocol struct {
	peers map[peer.ID]*PeerInfo
}

// PeerInfo holds information about a peer
type PeerInfo struct {
	ID          peer.ID
	Capabilities []string
	Resources   ResourceInfo
	LastSeen    time.Time
}

// ResourceInfo holds peer resource information
type ResourceInfo struct {
	CPUCores        int64
	MemoryMB        int64
	StorageMB       int64
	GPUMemoryMB     int64
	CPUAvailable    float64
	MemoryAvailable float64
	StorageAvailable float64
}

// NewOffloadProtocol creates a new offload protocol
//
// Implements T246: §3.3 Implement task offloading protocol
func NewOffloadProtocol() *OffloadProtocol {
	return &OffloadProtocol{
		peers: make(map[peer.ID]*PeerInfo),
	}
}

// RegisterPeer registers a peer for task offloading
func (op *OffloadProtocol) RegisterPeer(peerID peer.ID, capabilities []string, resources ResourceInfo) {
	op.mu.Lock()
	defer op.mu.Unlock()
	op.peers[peerID] = &PeerInfo{
		ID:          peerID,
		Capabilities: capabilities,
		Resources:   resources,
		LastSeen:    time.Now(),
	}
}

// OffloadTask offloads a task to a peer
func (op *OffloadProtocol) OffloadTask(ctx context.Context, req *OffloadRequest) (*OffloadResponse, error) {
	op.mu.RLock()
	// Check if target peer is available
	peerInfo, exists := op.peers[req.TargetPeer]
	op.mu.RUnlock()

	if !exists {
		return &OffloadResponse{
			Accepted: false,
			Error:    fmt.Errorf("peer not found: %s", req.TargetPeer),
		}, nil
	}

	// Check if peer has required resources
	if !op.checkResources(peerInfo, req.Requirements) {
		return &OffloadResponse{
			Accepted: false,
			Error:    fmt.Errorf("peer does not have required resources"),
		}, nil
	}

	// Check if peer has required capabilities
	if !op.checkCapabilities(peerInfo, req.TaskType) {
		return &OffloadResponse{
			Accepted: false,
			Error:    fmt.Errorf("peer does not have required capabilities"),
		}, nil
	}

	// TODO: Actually send task to peer via P2P protocol
	// For now, return success
	return &OffloadResponse{
		Accepted:    true,
		ExecutionID: fmt.Sprintf("exec-%s-%d", req.TaskID, time.Now().UnixNano()),
		EstimatedMs: int64(req.Requirements.EstimatedDuration.Milliseconds()),
	}, nil
}

// checkResources checks if peer has required resources
func (op *OffloadProtocol) checkResources(peer *PeerInfo, req ResourceRequirements) bool {
	// Check CPU
	if req.MinCPUCores > 0 && int64(peer.Resources.CPUAvailable*float64(peer.Resources.CPUCores)) < req.MinCPUCores {
		return false
	}

	// Check memory
	if req.MinMemoryMB > 0 && int64(peer.Resources.MemoryAvailable*float64(peer.Resources.MemoryMB)) < req.MinMemoryMB {
		return false
	}

	// Check GPU
	if req.RequiresGPU && peer.Resources.GPUMemoryMB == 0 {
		return false
	}

	return true
}

// checkCapabilities checks if peer has required capabilities
func (op *OffloadProtocol) checkCapabilities(peer *PeerInfo, taskType string) bool {
	for _, cap := range peer.Capabilities {
		if cap == taskType || cap == "compute" {
			return true
		}
	}
	return false
}

// FindSuitablePeers finds peers suitable for a task
func (op *OffloadProtocol) FindSuitablePeers(req ResourceRequirements, taskType string) []peer.ID {
	op.mu.RLock()
	defer op.mu.RUnlock()

	var suitable []peer.ID

	for peerID, peerInfo := range op.peers {
		if op.checkResources(peerInfo, req) && op.checkCapabilities(peerInfo, taskType) {
			suitable = append(suitable, peerID)
		}
	}

	return suitable
}

