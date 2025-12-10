// Package sync provides synchronization protocols
//
// T239: §3.8 Implement vector clock sync
// US6: P2P Hive-Mind Device Federation
// FR-020: P2P state synchronization

package sync

import (
	"sync"
	"time"
)

// VectorClock represents a vector clock for distributed versioning
type VectorClock struct {
	mu      sync.RWMutex
	clocks  map[string]int64 // device_id -> version
	deviceID string
}

// NewVectorClock creates a new vector clock
//
// Implements T239: §3.8 Implement vector clock sync
func NewVectorClock(deviceID string) *VectorClock {
	return &VectorClock{
		clocks:  make(map[string]int64),
		deviceID: deviceID,
	}
}

// Tick increments the clock for this device
func (vc *VectorClock) Tick() int64 {
	vc.mu.Lock()
	defer vc.mu.Unlock()
	vc.clocks[vc.deviceID]++
	return vc.clocks[vc.deviceID]
}

// Get returns the current version for this device
func (vc *VectorClock) Get() int64 {
	vc.mu.RLock()
	defer vc.mu.RUnlock()
	return vc.clocks[vc.deviceID]
}

// GetClock returns the full clock state
func (vc *VectorClock) GetClock() map[string]int64 {
	vc.mu.RLock()
	defer vc.mu.RUnlock()
	result := make(map[string]int64)
	for k, v := range vc.clocks {
		result[k] = v
	}
	return result
}

// Update merges another vector clock into this one
func (vc *VectorClock) Update(other map[string]int64) {
	vc.mu.Lock()
	defer vc.mu.Unlock()
	for deviceID, version := range other {
		current, exists := vc.clocks[deviceID]
		if !exists || version > current {
			vc.clocks[deviceID] = version
		}
	}
	// Ensure our own clock is at least 1
	if vc.clocks[vc.deviceID] == 0 {
		vc.clocks[vc.deviceID] = 1
	}
}

// HappensBefore checks if this clock happens before another
func (vc *VectorClock) HappensBefore(other map[string]int64) bool {
	vc.mu.RLock()
	defer vc.mu.RUnlock()

	allBefore := true
	atLeastOneBefore := false

	for deviceID, otherVersion := range other {
		ourVersion, exists := vc.clocks[deviceID]
		if !exists {
			ourVersion = 0
		}

		if ourVersion > otherVersion {
			return false // We have a version that's after
		}
		if ourVersion < otherVersion {
			atLeastOneBefore = true
		}
	}

	// Check if we have any devices the other doesn't
	for deviceID, ourVersion := range vc.clocks {
		_, exists := other[deviceID]
		if !exists && ourVersion > 0 {
			return false
		}
	}

	return allBefore && atLeastOneBefore
}

// Concurrent checks if two clocks are concurrent (neither happens before the other)
func (vc *VectorClock) Concurrent(other map[string]int64) bool {
	return !vc.HappensBefore(other) && !vc.happensAfter(other)
}

// happensAfter checks if this clock happens after another
func (vc *VectorClock) happensAfter(other map[string]int64) bool {
	vc.mu.RLock()
	defer vc.mu.RUnlock()

	allAfter := true
	atLeastOneAfter := false

	for deviceID, otherVersion := range other {
		ourVersion, exists := vc.clocks[deviceID]
		if !exists {
			ourVersion = 0
		}

		if ourVersion < otherVersion {
			return false // We have a version that's before
		}
		if ourVersion > otherVersion {
			atLeastOneAfter = true
		}
	}

	// Check if other has any devices we don't
	for deviceID, otherVersion := range other {
		_, exists := vc.clocks[deviceID]
		if !exists && otherVersion > 0 {
			return false
		}
	}

	return allAfter && atLeastOneAfter
}

// Timestamp returns a timestamp for the current state
func (vc *VectorClock) Timestamp() int64 {
	return time.Now().UnixMilli()
}

