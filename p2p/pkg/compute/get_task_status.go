// Package compute provides Compute service RPC implementations
//
// T515: Implement Compute.GetTaskStatus RPC
// US6: P2P Hive-Mind Device Federation
package compute

import (
	"context"
	"fmt"
	"time"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// GetTaskStatus handles the GetTaskStatus RPC
//
// Implements T515: Implement Compute.GetTaskStatus RPC
func (s *ComputeService) GetTaskStatus(ctx context.Context, req *protocol.GetTaskStatusRequest) (*protocol.GetTaskStatusResponse, error) {
	// Get task from scheduler
	task, exists := s.scheduler.GetTask(req.ExecutionId)
	if !exists {
		return &protocol.GetTaskStatusResponse{
			Status: protocol.TaskStatus_TASK_STATUS_UNSPECIFIED,
		}, fmt.Errorf("task not found: %s", req.ExecutionId)
	}

	// Convert status
	var status protocol.TaskStatus
	switch task.Status {
	case TaskStatusQueued:
		status = protocol.TaskStatus_TASK_STATUS_QUEUED
	case TaskStatusRunning:
		status = protocol.TaskStatus_TASK_STATUS_RUNNING
	case TaskStatusCompleted:
		status = protocol.TaskStatus_TASK_STATUS_COMPLETED
	case TaskStatusFailed:
		status = protocol.TaskStatus_TASK_STATUS_FAILED
	case TaskStatusCancelled:
		status = protocol.TaskStatus_TASK_STATUS_CANCELLED
	default:
		status = protocol.TaskStatus_TASK_STATUS_UNSPECIFIED
	}

	// Calculate progress (simplified)
	progress := float32(0.0)
	if task.Status == TaskStatusCompleted {
		progress = 1.0
	} else if task.Status == TaskStatusRunning && task.StartedAt != nil {
		elapsed := time.Since(*task.StartedAt)
		if task.Requirements.EstimatedDuration > 0 {
			progress = float32(elapsed) / float32(task.Requirements.EstimatedDuration)
			if progress > 1.0 {
				progress = 1.0
			}
		}
	}

	// Get error message
	errorMsg := ""
	if task.Error != nil {
		errorMsg = task.Error.Error()
	}

	return &protocol.GetTaskStatusResponse{
		Status:   status,
		Progress: progress,
		Result:   task.Result,
		Error:    errorMsg,
	}, nil
}

