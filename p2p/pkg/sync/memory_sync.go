// Package sync provides synchronization protocols
//
// T243: §3.7 Implement memory sync protocol
// US6: P2P Hive-Mind Device Federation
// FR-020: P2P state synchronization
package sync

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// MemorySync manages memory synchronization across devices
type MemorySync struct {
	mu          sync.RWMutex
	vectorClock *VectorClock
	deltaSync   *DeltaSync
	conflictResolver *ConflictResolver
	deviceID    string
}

// NewMemorySync creates a new memory sync manager
//
// Implements T243: §3.7 Implement memory sync protocol
func NewMemorySync(deviceID string) *MemorySync {
	return &MemorySync{
		vectorClock:     NewVectorClock(deviceID),
		deltaSync:       NewDeltaSync(1000),
		conflictResolver: NewConflictResolver(),
		deviceID:        deviceID,
	}
}

// SyncMemory synchronizes memory with a remote device
func (ms *MemorySync) SyncMemory(ctx context.Context, remoteDeviceID string, remoteVersion int64) error {
	ms.mu.Lock()
	defer ms.mu.Unlock()

	// Get our current version
	localVersion := ms.vectorClock.Get()

	// If remote is ahead, we need to pull changes
	if remoteVersion > localVersion {
		return fmt.Errorf("need to pull changes from remote device")
	}

	// If we're ahead, we need to push changes
	if localVersion > remoteVersion {
		return fmt.Errorf("need to push changes to remote device")
	}

	// Versions match - no sync needed
	return nil
}

// CreateMemoryDelta creates a delta for a memory change
func (ms *MemorySync) CreateMemoryDelta(entityID, entityType string, changeType ChangeType, data []byte) (*Delta, error) {
	ms.mu.Lock()
	defer ms.mu.Unlock()

	// Increment vector clock
	version := ms.vectorClock.Tick()

	delta := &Delta{
		ID:          fmt.Sprintf("delta-%s-%d", entityID, time.Now().UnixNano()),
		EntityID:    entityID,
		EntityType:  entityType,
		Type:        changeType,
		Data:        data,
		Timestamp:   time.Now().UnixMilli(),
		SourceDevice: ms.deviceID,
		Version:     version,
	}

	// Add to delta sync
	if err := ms.deltaSync.AddDelta(delta); err != nil {
		return nil, fmt.Errorf("failed to add delta: %w", err)
	}

	return delta, nil
}

// GetPendingDeltas returns deltas that need to be synced
func (ms *MemorySync) GetPendingDeltas(sinceVersion int64, maxCount int) []*Delta {
	ms.mu.RLock()
	defer ms.mu.RUnlock()
	return ms.deltaSync.GetDeltasSince(sinceVersion, maxCount)
}

// ApplyDelta applies a delta from a remote device
func (ms *MemorySync) ApplyDelta(ctx context.Context, delta *Delta) error {
	ms.mu.Lock()
	defer ms.mu.Unlock()

	// Check for conflicts
	localDelta, exists := ms.deltaSync.GetDelta(delta.EntityID)
	if exists && ms.conflictResolver.DetectConflict(localDelta, delta) {
		// Register conflict
		conflict, err := ms.conflictResolver.RegisterConflict(delta.EntityID, localDelta, delta)
		if err != nil {
			return fmt.Errorf("failed to register conflict: %w", err)
		}

		// Auto-resolve if possible
		resolution := ms.conflictResolver.AutoResolve(conflict)
		if err := ms.conflictResolver.ResolveConflict(conflict.ID, resolution); err != nil {
			return fmt.Errorf("failed to resolve conflict: %w", err)
		}

		// Apply resolution
		switch resolution {
		case ResolutionLocalWins:
			// Keep local delta, ignore remote
			return nil
		case ResolutionRemoteWins:
			// Apply remote delta
			ms.deltaSync.RemoveDelta(delta.EntityID)
			ms.deltaSync.AddDelta(delta)
			ms.vectorClock.Update(map[string]int64{delta.SourceDevice: delta.Version})
		case ResolutionMerge:
			// Merge both deltas (simplified - would need entity-specific merge logic)
			// For now, apply remote and keep local
			ms.deltaSync.AddDelta(delta)
			ms.vectorClock.Update(map[string]int64{delta.SourceDevice: delta.Version})
		}
	} else {
		// No conflict - apply delta
		ms.deltaSync.AddDelta(delta)
		ms.vectorClock.Update(map[string]int64{delta.SourceDevice: delta.Version})
	}

	return nil
}

// GetVectorClock returns the current vector clock state
func (ms *MemorySync) GetVectorClock() map[string]int64 {
	ms.mu.RLock()
	defer ms.mu.RUnlock()
	return ms.vectorClock.GetClock()
}

// GetUnresolvedConflicts returns all unresolved conflicts
func (ms *MemorySync) GetUnresolvedConflicts() []*Conflict {
	ms.mu.RLock()
	defer ms.mu.RUnlock()
	return ms.conflictResolver.ListUnresolvedConflicts()
}

