// Package storage provides Storage service RPC implementations
//
// T654: Implement Storage.Exists RPC
// US6: P2P Hive-Mind Device Federation
package storage

import (
	"context"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// Exists handles the Exists RPC
//
// Implements T654: Implement Storage.Exists RPC
func (s *StorageService) Exists(ctx context.Context, req *protocol.ExistsRequest) (*protocol.ExistsResponse, error) {
	// Check if content exists locally
	exists := s.cas.Exists(req.ContentHash)

	// Get replica locations if replicator is available
	locations := make([]string, 0)
	if s.replicator != nil {
		replicas := s.replicator.GetReplicas(req.ContentHash)
		for _, peerID := range replicas {
			locations = append(locations, peerID.String())
		}
	}

	// If exists locally, add our peer ID to locations
	if exists {
		// TODO: Get our peer ID from context or service
		// locations = append(locations, "local")
	}

	return &protocol.ExistsResponse{
		Exists:    exists,
		Locations: locations,
	}, nil
}

