// Package sync provides Sync service RPC implementations
//
// T507: Implement Sync.PullChanges RPC
// US6: P2P Hive-Mind Device Federation
package sync

import (
	"context"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// PullChanges handles the PullChanges RPC
//
// Implements T507: Implement Sync.PullChanges RPC
func (s *SyncService) PullChanges(ctx context.Context, req *protocol.PullChangesRequest) (*protocol.PullChangesResponse, error) {
	maxChanges := int(req.MaxChanges)
	if maxChanges <= 0 {
		maxChanges = 100 // Default limit
	}

	// Get pending deltas since the requested version
	deltas := s.memorySync.GetPendingDeltas(req.SinceVersion, maxChanges)

	// Convert deltas to protocol changes
	changes := make([]*protocol.Change, len(deltas))
	for i, delta := range deltas {
		changes[i] = &protocol.Change{
			Id:          delta.ID,
			EntityId:    delta.EntityID,
			EntityType:  delta.EntityType,
			Type:        changeTypeToString(delta.Type),
			Data:        delta.Data,
			Timestamp:   delta.Timestamp,
			SourceDevice: delta.SourceDevice,
			Version:     delta.Version,
		}
	}

	// Get latest version
	clock := s.memorySync.GetVectorClock()
	var latestVersion int64 = 0
	for _, version := range clock {
		if version > latestVersion {
			latestVersion = version
		}
	}

	// Check if there are more changes
	hasMore := len(deltas) >= maxChanges

	return &protocol.PullChangesResponse{
		Changes:      changes,
		LatestVersion: latestVersion,
		HasMore:      hasMore,
	}, nil
}

