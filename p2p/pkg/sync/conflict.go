// Package sync provides synchronization protocols
//
// T242: Implement conflict resolution
// US6: P2P Hive-Mind Device Federation
package sync

import (
	"fmt"
	"sync"
	"time"
)

// Conflict represents a synchronization conflict
type Conflict struct {
	ID          string
	EntityID    string
	EntityType  string
	LocalChange  *Delta
	RemoteChange *Delta
	CreatedAt   time.Time
	Resolved    bool
	Resolution  Resolution
}

// Resolution represents how a conflict was resolved
type Resolution int

const (
	ResolutionUnspecified Resolution = iota
	ResolutionLocalWins
	ResolutionRemoteWins
	ResolutionMerge
	ResolutionCustom
)

// ConflictResolver manages conflict resolution
type ConflictResolver struct {
	mu        sync.RWMutex
	conflicts map[string]*Conflict
}

// NewConflictResolver creates a new conflict resolver
//
// Implements T242: Implement conflict resolution
func NewConflictResolver() *ConflictResolver {
	return &ConflictResolver{
		conflicts: make(map[string]*Conflict),
	}
}

// DetectConflict detects if two deltas conflict
func (cr *ConflictResolver) DetectConflict(local, remote *Delta) bool {
	// Conflicts occur when:
	// 1. Same entity, different versions
	// 2. Both are updates (not create/delete)
	// 3. Vector clocks are concurrent

	if local.EntityID != remote.EntityID {
		return false
	}

	if local.Type == ChangeTypeCreate || remote.Type == ChangeTypeCreate {
		return false // Creates don't conflict
	}

	if local.Type == ChangeTypeDelete || remote.Type == ChangeTypeDelete {
		return true // Delete conflicts with any other operation
	}

	// Both are updates - check if versions are concurrent
	// This is a simplified check - full implementation would use vector clocks
	return local.Version != remote.Version
}

// RegisterConflict registers a conflict for resolution
func (cr *ConflictResolver) RegisterConflict(entityID string, local, remote *Delta) (*Conflict, error) {
	if !cr.DetectConflict(local, remote) {
		return nil, fmt.Errorf("no conflict detected")
	}

	conflict := &Conflict{
		ID:          fmt.Sprintf("conflict-%s-%d", entityID, time.Now().UnixNano()),
		EntityID:    entityID,
		EntityType:  local.EntityType,
		LocalChange:  local,
		RemoteChange: remote,
		CreatedAt:   time.Now(),
		Resolved:    false,
		Resolution:  ResolutionUnspecified,
	}

	cr.mu.Lock()
	defer cr.mu.Unlock()
	cr.conflicts[conflict.ID] = conflict

	return conflict, nil
}

// ResolveConflict resolves a conflict
func (cr *ConflictResolver) ResolveConflict(conflictID string, resolution Resolution) error {
	cr.mu.Lock()
	defer cr.mu.Unlock()

	conflict, exists := cr.conflicts[conflictID]
	if !exists {
		return fmt.Errorf("conflict not found: %s", conflictID)
	}

	if conflict.Resolved {
		return fmt.Errorf("conflict already resolved")
	}

	conflict.Resolution = resolution
	conflict.Resolved = true

	return nil
}

// GetConflict returns a conflict by ID
func (cr *ConflictResolver) GetConflict(conflictID string) (*Conflict, bool) {
	cr.mu.RLock()
	defer cr.mu.RUnlock()
	conflict, exists := cr.conflicts[conflictID]
	return conflict, exists
}

// ListUnresolvedConflicts returns all unresolved conflicts
func (cr *ConflictResolver) ListUnresolvedConflicts() []*Conflict {
	cr.mu.RLock()
	defer cr.mu.RUnlock()

	var unresolved []*Conflict
	for _, conflict := range cr.conflicts {
		if !conflict.Resolved {
			unresolved = append(unresolved, conflict)
		}
	}
	return unresolved
}

// AutoResolve attempts to automatically resolve a conflict
func (cr *ConflictResolver) AutoResolve(conflict *Conflict) Resolution {
	// Simple auto-resolution strategy:
	// - Last-Write-Wins (LWW) based on timestamp
	if conflict.LocalChange.Timestamp > conflict.RemoteChange.Timestamp {
		return ResolutionLocalWins
	} else if conflict.RemoteChange.Timestamp > conflict.LocalChange.Timestamp {
		return ResolutionRemoteWins
	}
	// If timestamps are equal, default to merge
	return ResolutionMerge
}

