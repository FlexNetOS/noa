// Package storage provides Storage service RPC implementations
//
// T655: Implement Storage.List RPC
// US6: P2P Hive-Mind Device Federation
package storage

import (
	"context"
	"path/filepath"
	"strings"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
	"os"
)

// List handles the List RPC
//
// Implements T655: Implement Storage.List RPC
func (s *StorageService) List(ctx context.Context, req *protocol.ListRequest) (*protocol.ListResponse, error) {
	limit := int(req.Limit)
	if limit <= 0 {
		limit = 100 // Default limit
	}

	var artifacts []*protocol.ArtifactInfo
	count := 0

	// Walk CAS directory
	err := filepath.Walk(s.cas.basePath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() {
			return nil
		}

		// Extract hash from path
		hash := filepath.Base(path)

		// Check prefix filter
		if req.Prefix != "" && !strings.HasPrefix(hash, req.Prefix) {
			return nil
		}

		// Read file to get metadata
		content, err := os.ReadFile(path)
		if err != nil {
			return err
		}

		// Get replica count
		replicaCount := 0
		if s.replicator != nil {
			replicas := s.replicator.GetReplicas(hash)
			replicaCount = len(replicas)
		}

		artifact := &protocol.ArtifactInfo{
			ContentHash: hash,
			Metadata: &protocol.ArtifactMetadata{
				SizeBytes: info.Size(),
				CreatedAt: info.ModTime().UnixMilli(),
				Labels:    make(map[string]string),
			},
			ReplicaCount: int32(replicaCount),
		}

		artifacts = append(artifacts, artifact)
		count++

		if count >= limit {
			return filepath.SkipAll // Stop walking
		}

		return nil
	})

	if err != nil {
		return &protocol.ListResponse{}, err
	}

	// TODO: Implement cursor-based pagination
	nextCursor := ""

	return &protocol.ListResponse{
		Artifacts: artifacts,
		NextCursor: nextCursor,
	}, nil
}

