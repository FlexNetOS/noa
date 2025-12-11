// Package sync provides synchronization protocols
//
// T241: Implement delta sync protocol
// US6: P2P Hive-Mind Device Federation
package sync

import (
	"sync"
	"time"
)

// Delta represents a change in state
type Delta struct {
	ID          string
	EntityID    string
	EntityType  string
	Type        ChangeType
	Data        []byte
	Timestamp   int64
	SourceDevice string
	Version     int64
}

// ChangeType represents the type of change
type ChangeType int

const (
	ChangeTypeCreate ChangeType = iota
	ChangeTypeUpdate
	ChangeTypeDelete
)

// DeltaSync manages delta synchronization
type DeltaSync struct {
	mu          sync.RWMutex
	pendingDeltas map[string]*Delta // entity_id -> delta
	maxDeltas   int
}

// NewDeltaSync creates a new delta sync manager
//
// Implements T241: Implement delta sync protocol
func NewDeltaSync(maxDeltas int) *DeltaSync {
	if maxDeltas <= 0 {
		maxDeltas = 1000 // Default limit
	}
	return &DeltaSync{
		pendingDeltas: make(map[string]*Delta),
		maxDeltas:    maxDeltas,
	}
}

// AddDelta adds a delta to the pending list
func (ds *DeltaSync) AddDelta(delta *Delta) error {
	ds.mu.Lock()
	defer ds.mu.Unlock()

	// Check if we need to evict old deltas
	if len(ds.pendingDeltas) >= ds.maxDeltas {
		// Remove oldest delta
		var oldestKey string
		var oldestTime int64 = time.Now().UnixMilli()
		for key, d := range ds.pendingDeltas {
			if d.Timestamp < oldestTime {
				oldestTime = d.Timestamp
				oldestKey = key
			}
		}
		if oldestKey != "" {
			delete(ds.pendingDeltas, oldestKey)
		}
	}

	ds.pendingDeltas[delta.EntityID] = delta
	return nil
}

// GetDeltasSince returns all deltas since a given version
func (ds *DeltaSync) GetDeltasSince(sinceVersion int64, maxCount int) []*Delta {
	ds.mu.RLock()
	defer ds.mu.RUnlock()

	var deltas []*Delta
	count := 0

	for _, delta := range ds.pendingDeltas {
		if delta.Version > sinceVersion {
			deltas = append(deltas, delta)
			count++
			if count >= maxCount {
				break
			}
		}
	}

	return deltas
}

// GetDelta returns a delta for an entity
func (ds *DeltaSync) GetDelta(entityID string) (*Delta, bool) {
	ds.mu.RLock()
	defer ds.mu.RUnlock()
	delta, exists := ds.pendingDeltas[entityID]
	return delta, exists
}

// RemoveDelta removes a delta (after it's been synced)
func (ds *DeltaSync) RemoveDelta(entityID string) {
	ds.mu.Lock()
	defer ds.mu.Unlock()
	delete(ds.pendingDeltas, entityID)
}

// Clear clears all pending deltas
func (ds *DeltaSync) Clear() {
	ds.mu.Lock()
	defer ds.mu.Unlock()
	ds.pendingDeltas = make(map[string]*Delta)
}

