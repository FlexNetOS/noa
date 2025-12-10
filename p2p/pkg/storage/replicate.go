// Package storage provides Storage service RPC implementations
//
// T656: Implement Storage.Replicate RPC
// US6: P2P Hive-Mind Device Federation
package storage

import (
	"context"
	"fmt"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
	"github.com/libp2p/go-libp2p/core/peer"
)

// Replicate handles the Replicate RPC
//
// Implements T656: Implement Storage.Replicate RPC
func (s *StorageService) Replicate(ctx context.Context, req *protocol.ReplicateRequest) (*protocol.ReplicateResponse, error) {
	// Retrieve content
	content, err := s.cas.Retrieve(req.ContentHash)
	if err != nil {
		return &protocol.ReplicateResponse{
			Success: false,
		}, err
	}

	// Parse target peer ID
	targetPeerID, err := peer.Decode(req.TargetPeer)
	if err != nil {
		return &protocol.ReplicateResponse{
			Success: false,
		}, fmt.Errorf("invalid peer ID: %w", err)
	}

	// Add target to replicator if not already present
	if s.replicator != nil {
		s.replicator.AddTarget(targetPeerID, 1)

		// Trigger replication
		if err := s.replicator.Replicate(ctx, req.ContentHash, content); err != nil {
			return &protocol.ReplicateResponse{
				Success: false,
			}, err
		}

		// Mark replica
		s.replicator.MarkReplica(req.ContentHash, targetPeerID)
	}

	return &protocol.ReplicateResponse{
		Success: true,
	}, nil
}

