// Package compute provides distributed compute scheduling
//
// T248: Implement distributed compute scheduler
// US6: P2P Hive-Mind Device Federation
package compute

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/FlexNetOS/noa/p2p/pkg/tasks"
)

// Scheduler manages local execution and optional offloading of compute tasks.
//
// Implements T248: distributed compute scheduler (initial scaffolding).
type Scheduler struct {
	mu sync.RWMutex

	tasks map[string]*Task
	queue []*Task

	// Local workers able to execute tasks.
	workers map[string]*Worker

	// Offload routing (best-effort). These are optional and can be nil.
	offloadProtocol *tasks.OffloadProtocol
	router          *tasks.Router
}

// NewScheduler creates a new scheduler
//
// Implements T248: Implement distributed compute scheduler
func NewScheduler() *Scheduler {
	offload := tasks.NewOffloadProtocol()
	return &Scheduler{
		tasks: make(map[string]*Task),
		queue: make([]*Task, 0),
		workers: make(map[string]*Worker),
		offloadProtocol: offload,
		router: tasks.NewRouter(offload),
	}
}

// SubmitTask submits a task for execution
func (s *Scheduler) SubmitTask(ctx context.Context, task *Task) (bool, time.Duration, error) {
	s.mu.Lock()
	s.tasks[task.ID] = task
	s.queue = append(s.queue, task)
	s.mu.Unlock()

	// Try to schedule immediately
	go s.scheduleTask(ctx, task)

	// Estimate duration
	estimatedDuration := task.Requirements.EstimatedDuration
	if estimatedDuration == 0 {
		estimatedDuration = 30 * time.Second
	}

	return true, estimatedDuration, nil
}

// scheduleTask attempts to schedule a task
func (s *Scheduler) scheduleTask(ctx context.Context, task *Task) {
	// First, try local workers
	s.mu.RLock()
	for _, worker := range s.workers {
		accepted, err := worker.AcceptTask(ctx, task)
		if err == nil && accepted {
			s.mu.RUnlock()
			_ = worker.StartTask(task.ID)
			return
		}
	}
	s.mu.RUnlock()

	// If no local worker available, try to offload
	req := tasks.ResourceRequirements{
		MinMemoryMB:      task.Requirements.MinMemoryMB,
		MinCPUCores:      task.Requirements.MinCPUCores,
		RequiresGPU:      task.Requirements.RequiresGPU,
		EstimatedDuration: task.Requirements.EstimatedDuration,
	}

	// Route to best peer
	peerID, err := s.router.RouteTask(req, taskTypeToString(task.Type))
	if err == nil {
		// Offload to peer
		offloadReq := &tasks.OffloadRequest{
			TaskID:      task.ID,
			TaskType:    taskTypeToString(task.Type),
			Payload:     task.Payload,
			Requirements: req,
			TargetPeer:  peerID,
			Timeout:     task.Timeout,
		}

		resp, err := s.offloadProtocol.OffloadTask(ctx, offloadReq)
		if err == nil && resp.Accepted {
			task.Status = TaskStatusRunning
			return
		}
	}

	// If offloading failed, keep in queue for later
}

// GetTask returns a task by ID
func (s *Scheduler) GetTask(id string) (*Task, bool) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	task, exists := s.tasks[id]
	return task, exists
}

// CancelTask cancels a task
func (s *Scheduler) CancelTask(id string) (bool, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	task, exists := s.tasks[id]
	if !exists {
		return false, fmt.Errorf("task not found: %s", id)
	}

	if task.Status == TaskStatusCompleted || task.Status == TaskStatusFailed {
		return false, fmt.Errorf("task already finished")
	}

	task.Status = TaskStatusCancelled
	return true, nil
}

// NextTask returns the next task from the queue
func (s *Scheduler) NextTask() *Task {
	s.mu.Lock()
	defer s.mu.Unlock()

	if len(s.queue) == 0 {
		return nil
	}

	task := s.queue[0]
	s.queue = s.queue[1:]
	return task
}

// taskTypeToString converts TaskType to string
func taskTypeToString(t TaskType) string {
	switch t {
	case TaskTypeInference:
		return "inference"
	case TaskTypeEmbedding:
		return "embedding"
	case TaskTypeParse:
		return "parse"
	case TaskTypeAnalyze:
		return "analyze"
	case TaskTypeCustom:
		return "custom"
	default:
		return "unknown"
	}
}

