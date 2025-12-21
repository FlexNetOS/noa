// Package storage provides distributed storage functionality
//
// T245: Implement replication protocol
// US6: P2P Hive-Mind Device Federation
package storage

import (
	"context"
	"sync"
	"time"

	"github.com/libp2p/go-libp2p/core/peer"
)

// ReplicationTarget represents a replication target
type ReplicationTarget struct {
	PeerID    peer.ID
	Priority  int
	LastSync  time.Time
	Status    ReplicationStatus
}

// ReplicationStatus represents replication status
type ReplicationStatus int

const (
	ReplicationStatusIdle ReplicationStatus = iota
	ReplicationStatusSyncing
	ReplicationStatusError
)

// ReplicationManager manages content replication across peers
type ReplicationManager struct {
	mu       sync.RWMutex
	cas      *CAS
	targets  map[peer.ID]*ReplicationTarget
	replicas map[string][]peer.ID // content_hash -> []peer_id
	minReplicas int
}

// NewReplicationManager creates a new replication manager
//
// Implements T245: Implement replication protocol
func NewReplicationManager(cas *CAS, minReplicas int) *ReplicationManager {
	if minReplicas <= 0 {
		minReplicas = 3 // Default: 3 replicas
	}
	return &ReplicationManager{
		cas:        cas,
		targets:    make(map[peer.ID]*ReplicationTarget),
		replicas:   make(map[string][]peer.ID),
		minReplicas: minReplicas,
	}
}

// AddTarget adds a replication target
func (rm *ReplicationManager) AddTarget(peerID peer.ID, priority int) {
	rm.mu.Lock()
	defer rm.mu.Unlock()

	rm.targets[peerID] = &ReplicationTarget{
		PeerID:   peerID,
		Priority: priority,
		Status:   ReplicationStatusIdle,
	}
}

// RemoveTarget removes a replication target
func (rm *ReplicationManager) RemoveTarget(peerID peer.ID) {
	rm.mu.Lock()
	defer rm.mu.Unlock()

	delete(rm.targets, peerID)

	// Remove from all replica lists
	for hash, peers := range rm.replicas {
		newPeers := make([]peer.ID, 0)
		for _, p := range peers {
			if p != peerID {
				newPeers = append(newPeers, p)
			}
		}
		rm.replicas[hash] = newPeers
	}
}

// Replicate replicates content to peers
func (rm *ReplicationManager) Replicate(ctx context.Context, hash string, content []byte) error {
	_ = ctx
	rm.mu.Lock()
	defer rm.mu.Unlock()

	// Get current replica count
	currentReplicas := len(rm.replicas[hash])

	// If we have enough replicas, check if we need to maintain them
	if currentReplicas >= rm.minReplicas {
		// Verify all replicas are still available
		availableReplicas := 0
		for _, peerID := range rm.replicas[hash] {
			if target, exists := rm.targets[peerID]; exists && target.Status != ReplicationStatusError {
				availableReplicas++
			}
		}

		// If we still have enough, no replication needed
		if availableReplicas >= rm.minReplicas {
			return nil
		}
	}

	// Select targets for replication
	targets := rm.selectTargets(hash, rm.minReplicas-currentReplicas)

	// Replicate to each target (simplified - would use actual P2P transfer)
	for _, target := range targets {
		// TODO: Implement actual replication via P2P protocol
		rm.replicas[hash] = append(rm.replicas[hash], target.PeerID)
		target.LastSync = time.Now()
		target.Status = ReplicationStatusSyncing
	}

	return nil
}

// selectTargets selects peers for replication
func (rm *ReplicationManager) selectTargets(excludeHash string, count int) []*ReplicationTarget {
	// Get existing replicas for this hash
	existing := make(map[peer.ID]bool)
	for _, peerID := range rm.replicas[excludeHash] {
		existing[peerID] = true
	}

	// Sort targets by priority
	var candidates []*ReplicationTarget
	for _, target := range rm.targets {
		if !existing[target.PeerID] && target.Status != ReplicationStatusError {
			candidates = append(candidates, target)
		}
	}

	// Sort by priority (higher is better)
	// TODO: Implement proper sorting

	// Return top N candidates
	if len(candidates) > count {
		candidates = candidates[:count]
	}

	return candidates
}

// GetReplicas returns the list of peers that have a replica
func (rm *ReplicationManager) GetReplicas(hash string) []peer.ID {
	rm.mu.RLock()
	defer rm.mu.RUnlock()
	return rm.replicas[hash]
}

// MarkReplica marks a peer as having a replica
func (rm *ReplicationManager) MarkReplica(hash string, peerID peer.ID) {
	rm.mu.Lock()
	defer rm.mu.Unlock()

	// Check if already marked
	for _, p := range rm.replicas[hash] {
		if p == peerID {
			return // Already marked
		}
	}

	rm.replicas[hash] = append(rm.replicas[hash], peerID)
}

// RemoveReplica removes a peer from replica list
func (rm *ReplicationManager) RemoveReplica(hash string, peerID peer.ID) {
	rm.mu.Lock()
	defer rm.mu.Unlock()

	peers := rm.replicas[hash]
	newPeers := make([]peer.ID, 0)
	for _, p := range peers {
		if p != peerID {
			newPeers = append(newPeers, p)
		}
	}
	rm.replicas[hash] = newPeers
}

