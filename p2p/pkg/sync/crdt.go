// Package sync provides synchronization protocols
//
// T240: Implement CRDT data types
// US6: P2P Hive-Mind Device Federation
package sync

import (
	"sync"
)

// CRDTType represents the type of CRDT
type CRDTType string

const (
	CRDTTypeORSet CRDTType = "or-set"  // Observed-Remove Set
	CRDTTypeLWW   CRDTType = "lww"     // Last-Write-Wins
	CRDTTypeGSet  CRDTType = "g-set"   // Grow-Only Set
	CRDTTypePNCounter CRDTType = "pn-counter" // Positive-Negative Counter
)

// CRDT represents a Conflict-free Replicated Data Type
type CRDT interface {
	Type() CRDTType
	Merge(other CRDT) error
	Value() interface{}
}

// ORSet is an Observed-Remove Set CRDT
type ORSet struct {
	mu    sync.RWMutex
	added map[string]map[string]bool // element -> device -> version
	tombstones map[string]map[string]bool // element -> device -> version
}

// NewORSet creates a new OR-Set
//
// Implements T240: Implement CRDT data types
func NewORSet() *ORSet {
	return &ORSet{
		added:     make(map[string]map[string]bool),
		tombstones: make(map[string]map[string]bool),
	}
}

// Type returns the CRDT type
func (s *ORSet) Type() CRDTType {
	return CRDTTypeORSet
}

// Add adds an element to the set
func (s *ORSet) Add(element string, deviceID string) {
	s.mu.Lock()
	defer s.mu.Unlock()

	if s.added[element] == nil {
		s.added[element] = make(map[string]bool)
	}
	s.added[element][deviceID] = true

	// Remove from tombstones if present
	if s.tombstones[element] != nil {
		delete(s.tombstones[element], deviceID)
		if len(s.tombstones[element]) == 0 {
			delete(s.tombstones, element)
		}
	}
}

// Remove removes an element from the set
func (s *ORSet) Remove(element string, deviceID string) {
	s.mu.Lock()
	defer s.mu.Unlock()

	if s.tombstones[element] == nil {
		s.tombstones[element] = make(map[string]bool)
	}
	s.tombstones[element][deviceID] = true
}

// Contains checks if an element is in the set
func (s *ORSet) Contains(element string) bool {
	s.mu.RLock()
	defer s.mu.RUnlock()

	added := s.added[element]
	if added == nil {
		return false
	}

	tombstones := s.tombstones[element]
	if tombstones == nil {
		return len(added) > 0
	}

	// Element is present if it's added and not fully tombstoned
	for deviceID := range added {
		if !tombstones[deviceID] {
			return true
		}
	}

	return false
}

// Value returns the set as a slice
func (s *ORSet) Value() interface{} {
	s.mu.RLock()
	defer s.mu.RUnlock()

	var result []string
	for element := range s.added {
		if s.Contains(element) {
			result = append(result, element)
		}
	}
	return result
}

// Merge merges another OR-Set into this one
func (s *ORSet) Merge(other CRDT) error {
	otherSet, ok := other.(*ORSet)
	if !ok {
		return ErrInvalidCRDTType
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	otherSet.mu.RLock()
	defer otherSet.mu.RUnlock()

	// Merge added elements
	for element, devices := range otherSet.added {
		if s.added[element] == nil {
			s.added[element] = make(map[string]bool)
		}
		for deviceID := range devices {
			s.added[element][deviceID] = true
		}
	}

	// Merge tombstones
	for element, devices := range otherSet.tombstones {
		if s.tombstones[element] == nil {
			s.tombstones[element] = make(map[string]bool)
		}
		for deviceID := range devices {
			s.tombstones[element][deviceID] = true
		}
	}

	return nil
}

// PNCounter is a Positive-Negative Counter CRDT
type PNCounter struct {
	mu      sync.RWMutex
	positive map[string]int64 // device -> positive count
	negative map[string]int64 // device -> negative count
}

// NewPNCounter creates a new PN-Counter
func NewPNCounter() *PNCounter {
	return &PNCounter{
		positive: make(map[string]int64),
		negative: make(map[string]int64),
	}
}

// Type returns the CRDT type
func (c *PNCounter) Type() CRDTType {
	return CRDTTypePNCounter
}

// Increment increments the counter
func (c *PNCounter) Increment(deviceID string, delta int64) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.positive[deviceID] += delta
}

// Decrement decrements the counter
func (c *PNCounter) Decrement(deviceID string, delta int64) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.negative[deviceID] += delta
}

// Value returns the current counter value
func (c *PNCounter) Value() interface{} {
	c.mu.RLock()
	defer c.mu.RUnlock()

	var posSum int64
	for _, v := range c.positive {
		posSum += v
	}

	var negSum int64
	for _, v := range c.negative {
		negSum += v
	}

	return posSum - negSum
}

// Merge merges another PN-Counter into this one
func (c *PNCounter) Merge(other CRDT) error {
	otherCounter, ok := other.(*PNCounter)
	if !ok {
		return ErrInvalidCRDTType
	}

	c.mu.Lock()
	defer c.mu.Unlock()

	otherCounter.mu.RLock()
	defer otherCounter.mu.RUnlock()

	// Merge positive counts
	for deviceID, value := range otherCounter.positive {
		if c.positive[deviceID] < value {
			c.positive[deviceID] = value
		}
	}

	// Merge negative counts
	for deviceID, value := range otherCounter.negative {
		if c.negative[deviceID] < value {
			c.negative[deviceID] = value
		}
	}

	return nil
}

var ErrInvalidCRDTType = &CRDTError{Message: "invalid CRDT type for merge"}

// CRDTError represents a CRDT operation error
type CRDTError struct {
	Message string
}

func (e *CRDTError) Error() string {
	return e.Message
}

