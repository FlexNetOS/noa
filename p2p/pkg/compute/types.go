// Package compute provides distributed compute types
//
// T248: Implement distributed compute scheduler
// US6: P2P Hive-Mind Device Federation
package compute

import (
	"sync"
	"time"

	"github.com/FlexNetOS/noa/p2p/pkg/tasks"
	"github.com/libp2p/go-libp2p/core/peer"
)

// TaskType represents the type of compute task
type TaskType int

const (
	TaskTypeInference TaskType = iota
	TaskTypeEmbedding
	TaskTypeParse
	TaskTypeAnalyze
	TaskTypeCustom
)

// TaskStatus represents the status of a task
type TaskStatus int

const (
	TaskStatusPending TaskStatus = iota
	TaskStatusQueued
	TaskStatusRunning
	TaskStatusCompleted
	TaskStatusFailed
	TaskStatusCancelled
)

// TaskRequirements represents resource requirements for a task
type TaskRequirements struct {
	MinMemoryMB       int64
	MinCPUCores       int64
	RequiresGPU       bool
	EstimatedDuration time.Duration
}

// Task represents a compute task
type Task struct {
	ID           string
	Type         TaskType
	Payload      []byte
	Requirements TaskRequirements
	Timeout      time.Duration
	Status       TaskStatus
	Result       []byte
	Error        string
	CreatedAt    time.Time
	StartedAt    *time.Time
	CompletedAt  *time.Time
}

// Scheduler manages task scheduling and distribution
type Scheduler struct {
	mu              sync.RWMutex
	tasks           map[string]*Task
	queue           []*Task
	workers         []*Worker
	router          *TaskRouter
	offloadProtocol *tasks.OffloadProtocol
}

// TaskRouter routes tasks to peers (simplified)
type TaskRouter struct {
	peers map[peer.ID][]string // peer -> capabilities
}

// NewTaskRouter creates a new task router
func NewTaskRouter() *TaskRouter {
	return &TaskRouter{
		peers: make(map[peer.ID][]string),
	}
}

// RouteTask selects a peer for a task
func (r *TaskRouter) RouteTask(reqs tasks.ResourceRequirements, taskType string) (peer.ID, error) {
	// Simple routing - return first capable peer
	for peerID := range r.peers {
		return peerID, nil
	}
	return "", nil
}

// SetRouter sets the task router
func (s *Scheduler) SetRouter(router *TaskRouter) {
	s.router = router
}

// SetOffloadProtocol sets the offload protocol
func (s *Scheduler) SetOffloadProtocol(op *tasks.OffloadProtocol) {
	s.offloadProtocol = op
}

// RegisterWorker registers a worker with the scheduler
func (s *Scheduler) RegisterWorker(worker *Worker) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.workers = append(s.workers, worker)
}
