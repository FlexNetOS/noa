// Package protocol contains gRPC protocol buffer types for P2P communication
//
// This package should normally be generated from proto files. This is a
// placeholder implementation until the proper proto generation is set up.
package protocol

import (
	"context"
)

// SubmitTaskRequest is the request for task submission
type SubmitTaskRequest struct {
	TaskId       string                `json:"task_id"`
	TaskType     string                `json:"task_type"`
	Payload      []byte                `json:"payload"`
	Requirements *ResourceRequirements `json:"requirements"`
	TimeoutMs    int64                 `json:"timeout_ms"`
}

// SubmitTaskResponse is the response for task submission
type SubmitTaskResponse struct {
	ExecutionId  string `json:"execution_id"`
	Status       string `json:"status"`
	Message      string `json:"message"`
	Accepted     bool   `json:"accepted"`
	EstimatedMs  int64  `json:"estimated_ms"`
}

// GetTaskStatusRequest is the request for getting task status
type GetTaskStatusRequest struct {
	ExecutionId string `json:"execution_id"`
}

// GetTaskStatusResponse is the response for task status
type GetTaskStatusResponse struct {
	ExecutionId string     `json:"execution_id"`
	Status      TaskStatus `json:"status"`
	Progress    float32    `json:"progress"`
	Output      []byte     `json:"output"`
	Error       string     `json:"error"`
}

// CancelTaskRequest is the request for canceling a task
type CancelTaskRequest struct {
	ExecutionId string `json:"execution_id"`
}

// CancelTaskResponse is the response for task cancellation
type CancelTaskResponse struct {
	Success   bool   `json:"success"`
	Cancelled bool   `json:"cancelled"`
	Message   string `json:"message"`
}

// StreamOutputRequest is the request for streaming task output
type StreamOutputRequest struct {
	ExecutionId string `json:"execution_id"`
}

// OutputChunk is a chunk of streaming output
type OutputChunk struct {
	Data      []byte `json:"data"`
	Timestamp int64  `json:"timestamp"`
	Done      bool   `json:"done"`
}

// ResourceRequirements specifies resource requirements for a task
type ResourceRequirements struct {
	CpuCores            int32   `json:"cpu_cores"`
	MinCpuCores         int64   `json:"min_cpu_cores"`
	MemoryMb            int32   `json:"memory_mb"`
	MinMemoryMb         int64   `json:"min_memory_mb"`
	GpuUnits            int32   `json:"gpu_units"`
	RequiresGpu         bool    `json:"requires_gpu"`
	DiskMb              int64   `json:"disk_mb"`
	NetworkMb           float64 `json:"network_mb"`
	EstimatedDurationMs int64   `json:"estimated_duration_ms"`
}

// OutputType represents the type of output
type OutputType int

const (
	OutputType_OUTPUT_TYPE_UNSPECIFIED OutputType = iota
	OutputType_OUTPUT_TYPE_STDOUT
	OutputType_OUTPUT_TYPE_STDERR
)

// TaskStatus represents the status of a task
type TaskStatus int

const (
	TaskStatus_TASK_STATUS_UNSPECIFIED TaskStatus = iota
	TaskStatus_TASK_STATUS_QUEUED
	TaskStatus_TASK_STATUS_RUNNING
	TaskStatus_TASK_STATUS_COMPLETED
	TaskStatus_TASK_STATUS_FAILED
	TaskStatus_TASK_STATUS_CANCELLED
)

// TaskOutput represents streaming task output
type TaskOutput struct {
	Type      OutputType `json:"type"`
	Data      []byte     `json:"data"`
	Timestamp int64      `json:"timestamp"`
}

// Compute_StreamOutputServer is the streaming interface for output
type Compute_StreamOutputServer interface {
	Send(*TaskOutput) error
}

// ComputeServer is the server interface for Compute service
type ComputeServer interface {
	SubmitTask(ctx context.Context, req *SubmitTaskRequest) (*SubmitTaskResponse, error)
	GetTaskStatus(ctx context.Context, req *GetTaskStatusRequest) (*GetTaskStatusResponse, error)
	CancelTask(ctx context.Context, req *CancelTaskRequest) (*CancelTaskResponse, error)
	StreamOutput(req *StreamOutputRequest, stream Compute_StreamOutputServer) error
}

// UnimplementedComputeServer can be embedded to have forward compatible implementations
type UnimplementedComputeServer struct{}

func (UnimplementedComputeServer) SubmitTask(ctx context.Context, req *SubmitTaskRequest) (*SubmitTaskResponse, error) {
	return nil, nil
}

func (UnimplementedComputeServer) GetTaskStatus(ctx context.Context, req *GetTaskStatusRequest) (*GetTaskStatusResponse, error) {
	return nil, nil
}

func (UnimplementedComputeServer) CancelTask(ctx context.Context, req *CancelTaskRequest) (*CancelTaskResponse, error) {
	return nil, nil
}

func (UnimplementedComputeServer) StreamOutput(req *StreamOutputRequest, stream Compute_StreamOutputServer) error {
	return nil
}

