// Package compute provides distributed compute worker implementation
//
// T249: Implement worker node
// US6: P2P Hive-Mind Device Federation
package compute

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// Worker represents a compute worker node
type Worker struct {
	mu          sync.RWMutex
	id          string
	tasks       map[string]*Task
	maxConcurrent int
	running     int
}

// NewWorker creates a new worker
//
// Implements T249: Implement worker node
func NewWorker(id string, maxConcurrent int) *Worker {
	if maxConcurrent <= 0 {
		maxConcurrent = 4 // Default
	}
	return &Worker{
		id:          id,
		tasks:       make(map[string]*Task),
		maxConcurrent: maxConcurrent,
		running:     0,
	}
}

// AcceptTask accepts a task for execution
func (w *Worker) AcceptTask(ctx context.Context, task *Task) (bool, error) {
	w.mu.Lock()
	defer w.mu.Unlock()

	// Check if we have capacity
	if w.running >= w.maxConcurrent {
		return false, fmt.Errorf("worker at capacity")
	}

	// Check if we can handle the task requirements
	// TODO: Check actual resource availability

	// Add task
	w.tasks[task.ID] = task
	task.Status = TaskStatusQueued

	return true, nil
}

// StartTask starts executing a task
func (w *Worker) StartTask(taskID string) error {
	w.mu.Lock()
	defer w.mu.Unlock()

	task, exists := w.tasks[taskID]
	if !exists {
		return fmt.Errorf("task not found: %s", taskID)
	}

	if task.Status != TaskStatusQueued {
		return fmt.Errorf("task not in queued state: %s", taskID)
	}

	now := time.Now()
	task.Status = TaskStatusRunning
	task.StartedAt = &now
	w.running++

	// Start execution in background
	go w.executeTask(task)

	return nil
}

// executeTask executes a task
func (w *Worker) executeTask(task *Task) {
	// TODO: Implement actual task execution based on task type
	// For now, simulate execution
	time.Sleep(task.Requirements.EstimatedDuration)

	w.mu.Lock()
	defer w.mu.Unlock()

	now := time.Now()
	task.CompletedAt = &now
	task.Status = TaskStatusCompleted
	task.Result = []byte("Task completed")
	w.running--

	// Remove from tasks after a delay
	go func() {
		time.Sleep(5 * time.Minute)
		w.mu.Lock()
		defer w.mu.Unlock()
		delete(w.tasks, task.ID)
	}()
}

// GetTask returns a task by ID
func (w *Worker) GetTask(taskID string) (*Task, bool) {
	w.mu.RLock()
	defer w.mu.RUnlock()
	task, exists := w.tasks[taskID]
	return task, exists
}

// CancelTask cancels a running task
func (w *Worker) CancelTask(taskID string) error {
	w.mu.Lock()
	defer w.mu.Unlock()

	task, exists := w.tasks[taskID]
	if !exists {
		return fmt.Errorf("task not found: %s", taskID)
	}

	if task.Status == TaskStatusCompleted || task.Status == TaskStatusFailed {
		return fmt.Errorf("task already finished")
	}

	task.Status = TaskStatusCancelled
	if task.Status == TaskStatusRunning {
		w.running--
	}

	return nil
}

// GetStatus returns worker status
func (w *Worker) GetStatus() WorkerStatus {
	w.mu.RLock()
	defer w.mu.RUnlock()

	return WorkerStatus{
		ID:            w.id,
		Running:      w.running,
		MaxConcurrent: w.maxConcurrent,
		Queued:       len(w.tasks) - w.running,
	}
}

// WorkerStatus represents worker status
type WorkerStatus struct {
	ID            string
	Running       int
	MaxConcurrent int
	Queued        int
}

