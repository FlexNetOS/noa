// Package storage provides Storage service RPC implementations
//
// T652: §3.8 Implement Storage.Store RPC
// US6: P2P Hive-Mind Device Federation
package storage

import (
	"context"
	"fmt"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// StorageService implements the Storage gRPC service
type StorageService struct {
	protocol.UnimplementedStorageServer
	cas      *CAS
	replicator *ReplicationManager
}

// NewStorageService creates a new storage service
func NewStorageService(cas *CAS, replicator *ReplicationManager) *StorageService {
	return &StorageService{
		cas:        cas,
		replicator: replicator,
	}
}

// Store handles the Store RPC
//
// Implements T652: §3.8 Implement Storage.Store RPC
func (s *StorageService) Store(ctx context.Context, req *protocol.StoreRequest) (*protocol.StoreResponse, error) {
	// Store content in CAS
	hash, err := s.cas.Store(req.Content)
	if err != nil {
		return &protocol.StoreResponse{
			Success: false,
		}, err
	}

	// If hash provided, verify it matches
	if req.ContentHash != "" && req.ContentHash != hash {
		return &protocol.StoreResponse{
			Success: false,
		}, fmt.Errorf("content hash mismatch: expected %s, got %s", req.ContentHash, hash)
	}

	// Trigger replication if replicator is available
	if s.replicator != nil {
		go func() {
			_ = s.replicator.Replicate(context.Background(), hash, req.Content)
		}()
	}

	return &protocol.StoreResponse{
		Success:     true,
		ContentHash: hash,
	}, nil
}

