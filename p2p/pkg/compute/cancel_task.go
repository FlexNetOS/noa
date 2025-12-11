// Package compute provides Compute service RPC implementations
//
// T516: Implement Compute.CancelTask RPC
// US6: P2P Hive-Mind Device Federation
package compute

import (
	"context"
	"fmt"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// CancelTask handles the CancelTask RPC
//
// Implements T516: Implement Compute.CancelTask RPC
func (s *ComputeService) CancelTask(ctx context.Context, req *protocol.CancelTaskRequest) (*protocol.CancelTaskResponse, error) {
	// Cancel task via scheduler
	cancelled, err := s.scheduler.CancelTask(req.ExecutionId)
	if err != nil {
		return &protocol.CancelTaskResponse{
			Cancelled: false,
		}, err
	}

	return &protocol.CancelTaskResponse{
		Cancelled: cancelled,
	}, nil
}

