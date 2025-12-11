// Package discovery provides Discovery service RPC implementations
//
// T503: Implement Discovery.Ping RPC
// US6: P2P Hive-Mind Device Federation
package discovery

import (
	"context"
	"time"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// Ping handles the Ping RPC
//
// Implements T503: Implement Discovery.Ping RPC
func (s *DiscoveryService) Ping(ctx context.Context, req *protocol.PingRequest) (*protocol.PingResponse, error) {
	// Get current timestamp
	now := time.Now().UnixMilli()

	// Calculate latency if request timestamp is provided
	var latencyMs int64 = 0
	if req.Timestamp > 0 {
		latencyMs = now - req.Timestamp
	}

	return &protocol.PingResponse{
		Timestamp: now,
		LatencyMs: latencyMs,
	}, nil
}

