// Package tasks provides task offloading and distribution
//
// T247: Implement resource-based routing
// US6: P2P Hive-Mind Device Federation
package tasks

import (
	"fmt"
	"sort"
	"time"

	"github.com/libp2p/go-libp2p/core/peer"
)

// Router provides resource-based task routing
type Router struct {
	offloadProtocol *OffloadProtocol
}

// NewRouter creates a new router
//
// Implements T247: Implement resource-based routing
func NewRouter(offloadProtocol *OffloadProtocol) *Router {
	return &Router{
		offloadProtocol: offloadProtocol,
	}
}

// RouteTask routes a task to the best available peer
func (r *Router) RouteTask(req ResourceRequirements, taskType string) (peer.ID, error) {
	// Find suitable peers
	suitable := r.offloadProtocol.FindSuitablePeers(req, taskType)

	if len(suitable) == 0 {
		return "", fmt.Errorf("no suitable peers found")
	}

	// Score peers based on resources and availability
	type peerScore struct {
		peerID peer.ID
		score  float64
	}

	scores := make([]peerScore, 0, len(suitable))
	r.offloadProtocol.mu.RLock()
	defer r.offloadProtocol.mu.RUnlock()

	for _, peerID := range suitable {
		peerInfo, exists := r.offloadProtocol.peers[peerID]
		if !exists {
			continue
		}

		score := r.calculateScore(peerInfo, req)
		scores = append(scores, peerScore{
			peerID: peerID,
			score:  score,
		})
	}

	// Sort by score (highest first)
	sort.Slice(scores, func(i, j int) bool {
		return scores[i].score > scores[j].score
	})

	if len(scores) == 0 {
		return "", fmt.Errorf("no suitable peers found")
	}

	return scores[0].peerID, nil
}

// calculateScore calculates a routing score for a peer
func (r *Router) calculateScore(peer *PeerInfo, req ResourceRequirements) float64 {
	score := 0.0

	// CPU availability score
	if req.MinCPUCores > 0 {
		availableCores := peer.Resources.CPUAvailable * float64(peer.Resources.CPUCores)
		if availableCores >= float64(req.MinCPUCores) {
			score += availableCores / float64(req.MinCPUCores) * 0.3
		}
	}

	// Memory availability score
	if req.MinMemoryMB > 0 {
		availableMemory := peer.Resources.MemoryAvailable * float64(peer.Resources.MemoryMB)
		if availableMemory >= float64(req.MinMemoryMB) {
			score += availableMemory / float64(req.MinMemoryMB) * 0.3
		}
	}

	// GPU availability score
	if req.RequiresGPU && peer.Resources.GPUMemoryMB > 0 {
		score += 0.2
	}

	// Recency score (prefer recently seen peers)
	timeSinceLastSeen := time.Since(peer.LastSeen)
	if timeSinceLastSeen < 5*time.Minute {
		score += 0.2
	} else if timeSinceLastSeen < 30*time.Minute {
		score += 0.1
	}

	return score
}

