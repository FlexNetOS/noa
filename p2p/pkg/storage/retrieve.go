// Package storage provides Storage service RPC implementations
//
// T653: Implement Storage.Retrieve RPC
// US6: P2P Hive-Mind Device Federation
package storage

import (
	"context"
	"time"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// Retrieve handles the Retrieve RPC
//
// Implements T653: Implement Storage.Retrieve RPC
func (s *StorageService) Retrieve(ctx context.Context, req *protocol.RetrieveRequest) (*protocol.RetrieveResponse, error) {
	// Retrieve content from CAS
	content, err := s.cas.Retrieve(req.ContentHash)
	if err != nil {
		return &protocol.RetrieveResponse{}, err
	}

	// Build metadata (simplified)
	metadata := &protocol.ArtifactMetadata{
		SizeBytes:  int64(len(content)),
		CreatedAt:  time.Now().UnixMilli(),
		Labels:     make(map[string]string),
	}

	return &protocol.RetrieveResponse{
		Content:  content,
		Metadata: metadata,
	}, nil
}

