// Package sync provides Sync service RPC implementations
//
// T505: Implement Sync.GetState RPC
// US6: P2P Hive-Mind Device Federation
package sync

import (
	"context"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// SyncService implements the Sync gRPC service
type SyncService struct {
	protocol.UnimplementedSyncServer
	memorySync *MemorySync
}

// NewSyncService creates a new sync service
func NewSyncService(memorySync *MemorySync) *SyncService {
	return &SyncService{
		memorySync: memorySync,
	}
}

// GetState handles the GetState RPC
//
// Implements T505: Implement Sync.GetState RPC
func (s *SyncService) GetState(ctx context.Context, req *protocol.GetStateRequest) (*protocol.GetStateResponse, error) {
	// Get vector clock state
	clock := s.memorySync.GetVectorClock()

	// Calculate current version (max of all device versions)
	var currentVersion int64 = 0
	for _, version := range clock {
		if version > currentVersion {
			currentVersion = version
		}
	}

	// Get pending changes count
	pendingDeltas := s.memorySync.GetPendingDeltas(req.SinceVersion, 1000)

	// Calculate state hash (simplified)
	stateHash := make([]byte, 32) // SHA-256 hash
	// TODO: Calculate actual hash from state

	return &protocol.GetStateResponse{
		CurrentVersion: currentVersion,
		StateHash:      stateHash,
		PendingChanges: int32(len(pendingDeltas)),
	}, nil
}

