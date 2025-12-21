// Package compute provides Compute service RPC implementations
//
// T514: §3.3 Implement Compute.SubmitTask RPC
// US6: P2P Hive-Mind Device Federation
package compute

import (
	"context"
	"time"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
	"github.com/google/uuid"
)

// ComputeService implements the Compute gRPC service
type ComputeService struct {
	protocol.UnimplementedComputeServer
	scheduler *Scheduler
	workers   map[string]*Worker
}

// NewComputeService creates a new compute service
func NewComputeService(scheduler *Scheduler) *ComputeService {
	return &ComputeService{
		scheduler: scheduler,
		workers:   make(map[string]*Worker),
	}
}

// SubmitTask handles the SubmitTask RPC
//
// Implements T514: §3.3 Implement Compute.SubmitTask RPC
func (s *ComputeService) SubmitTask(ctx context.Context, req *protocol.SubmitTaskRequest) (*protocol.SubmitTaskResponse, error) {
	// Generate execution ID
	executionID := uuid.New().String()
	if req.TaskId != "" {
		executionID = req.TaskId
	}

	// Create task
	task := &Task{
		ID:          executionID,
		Type:        TaskType(req.TaskType),
		Payload:     req.Payload,
		Requirements: convertResourceRequirements(req.Requirements),
		Timeout:     time.Duration(req.TimeoutMs) * time.Millisecond,
		Status:      TaskStatusQueued,
		CreatedAt:   time.Now(),
	}

	// Submit to scheduler
	accepted, estimatedDuration, err := s.scheduler.SubmitTask(ctx, task)
	if err != nil {
		return &protocol.SubmitTaskResponse{
			Accepted: false,
		}, err
	}

	return &protocol.SubmitTaskResponse{
		Accepted:     accepted,
		ExecutionId:  executionID,
		EstimatedMs: int64(estimatedDuration.Milliseconds()),
	}, nil
}

// Task represents a compute task
type Task struct {
	ID          string
	Type        TaskType
	Payload     []byte
	Requirements ResourceRequirements
	Timeout     time.Duration
	Status      TaskStatus
	CreatedAt   time.Time
	StartedAt   *time.Time
	CompletedAt *time.Time
	Result      []byte
	Error       error
}

// TaskType represents the type of task
type TaskType int

const (
	TaskTypeInference TaskType = iota
	TaskTypeEmbedding
	TaskTypeParse
	TaskTypeAnalyze
	TaskTypeCustom
)

// TaskStatus represents task status
type TaskStatus int

const (
	TaskStatusQueued TaskStatus = iota
	TaskStatusRunning
	TaskStatusCompleted
	TaskStatusFailed
	TaskStatusCancelled
)

// ResourceRequirements represents resource requirements
type ResourceRequirements struct {
	MinMemoryMB int64
	MinCPUCores int64
	RequiresGPU bool
	EstimatedDuration time.Duration
}

// convertResourceRequirements converts protocol requirements
func convertResourceRequirements(req *protocol.ResourceRequirements) ResourceRequirements {
	if req == nil {
		return ResourceRequirements{}
	}
	return ResourceRequirements{
		MinMemoryMB: req.MinMemoryMb,
		MinCPUCores: req.MinCpuCores,
		RequiresGPU: req.RequiresGpu,
		EstimatedDuration: time.Duration(req.EstimatedDurationMs) * time.Millisecond,
	}
}

