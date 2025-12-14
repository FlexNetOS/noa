// Package sync provides Sync service RPC implementations
//
// T508: Implement Sync.ResolveConflict RPC
// US6: P2P Hive-Mind Device Federation
package sync

import (
	"context"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// ResolveConflict handles the ResolveConflict RPC
//
// Implements T508: Implement Sync.ResolveConflict RPC
func (s *SyncService) ResolveConflict(ctx context.Context, req *protocol.ResolveConflictRequest) (*protocol.ResolveConflictResponse, error) {
	// Convert protocol resolution to internal resolution
	var resolution Resolution
	switch req.Resolution {
	case protocol.Resolution_RESOLUTION_LOCAL_WINS:
		resolution = ResolutionLocalWins
	case protocol.Resolution_RESOLUTION_REMOTE_WINS:
		resolution = ResolutionRemoteWins
	case protocol.Resolution_RESOLUTION_MERGE:
		resolution = ResolutionMerge
	default:
		resolution = ResolutionUnspecified
	}

	// Resolve the conflict
	if err := s.memorySync.conflictResolver.ResolveConflict(req.ConflictId, resolution); err != nil {
		return &protocol.ResolveConflictResponse{
			Success: false,
		}, err
	}

	// Get new version after resolution
	clock := s.memorySync.GetVectorClock()
	var newVersion int64 = 0
	for _, version := range clock {
		if version > newVersion {
			newVersion = version
		}
	}

	return &protocol.ResolveConflictResponse{
		Success:    true,
		NewVersion: newVersion,
	}, nil
}

