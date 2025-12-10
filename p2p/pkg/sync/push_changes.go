// Package sync provides Sync service RPC implementations
//
// T506: Implement Sync.PushChanges RPC
// US6: P2P Hive-Mind Device Federation
package sync

import (
	"context"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// PushChanges handles the PushChanges RPC
//
// Implements T506: Implement Sync.PushChanges RPC
func (s *SyncService) PushChanges(ctx context.Context, req *protocol.PushChangesRequest) (*protocol.PushChangesResponse, error) {
	var conflicts []*protocol.Conflict

	// Apply each change
	for _, changeProto := range req.Changes {
		// Convert protocol change to Delta
		delta := &Delta{
			ID:          changeProto.Id,
			EntityID:    changeProto.EntityId,
			EntityType:  changeProto.EntityType,
			Type:        ChangeType(changeProto.Type),
			Data:        changeProto.Data,
			Timestamp:   changeProto.Timestamp,
			SourceDevice: changeProto.SourceDevice,
			Version:     changeProto.Version,
		}

		// Apply delta (may detect conflicts)
		if err := s.memorySync.ApplyDelta(ctx, delta); err != nil {
			// Check if it's a conflict
			unresolved := s.memorySync.GetUnresolvedConflicts()
			for _, conflict := range unresolved {
				if conflict.EntityID == delta.EntityID {
					conflicts = append(conflicts, &protocol.Conflict{
						Id:          conflict.ID,
						EntityId:    conflict.EntityID,
						LocalChange:  convertDeltaToChange(conflict.LocalChange),
						RemoteChange: convertDeltaToChange(conflict.RemoteChange),
					})
				}
			}
		}
	}

	// Get new version after applying changes
	clock := s.memorySync.GetVectorClock()
	var newVersion int64 = 0
	for _, version := range clock {
		if version > newVersion {
			newVersion = version
		}
	}

	accepted := len(conflicts) == 0

	return &protocol.PushChangesResponse{
		Accepted:   accepted,
		NewVersion: newVersion,
		Conflicts:  conflicts,
	}, nil
}

// convertDeltaToChange converts a Delta to protocol Change
func convertDeltaToChange(delta *Delta) *protocol.Change {
	if delta == nil {
		return nil
	}
	return &protocol.Change{
		Id:          delta.ID,
		EntityId:    delta.EntityID,
		EntityType:  delta.EntityType,
		Type:        protocol.ChangeType(delta.Type),
		Data:        delta.Data,
		Timestamp:   delta.Timestamp,
		SourceDevice: delta.SourceDevice,
		Version:     delta.Version,
	}
}

