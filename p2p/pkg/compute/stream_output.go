// Package compute provides Compute service RPC implementations
//
// T517: Implement Compute.StreamOutput RPC (streaming)
// US6: P2P Hive-Mind Device Federation
package compute

import (
	"fmt"
	"time"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// StreamOutput handles the StreamOutput RPC (streaming)
//
// Implements T517: Implement Compute.StreamOutput RPC (streaming)
func (s *ComputeService) StreamOutput(req *protocol.StreamOutputRequest, stream protocol.Compute_StreamOutputServer) error {
	// Get task
	_, exists := s.scheduler.GetTask(req.ExecutionId)
	if !exists {
		return fmt.Errorf("task not found: %s", req.ExecutionId)
	}

	// Stream output (simplified - would need actual output streaming)
	// For now, send a single output message
	output := &protocol.TaskOutput{
		Type:      protocol.OutputType_OUTPUT_TYPE_STDOUT,
		Data:      []byte("Task output streaming (implementation pending)"),
		Timestamp: time.Now().UnixMilli(),
	}

	if err := stream.Send(output); err != nil {
		return err
	}

	return nil
}

