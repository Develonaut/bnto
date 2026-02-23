// Package execution tracks asynchronous workflow execution state.
//
// The Manager holds in-memory state for all running and recently completed
// executions. Completed executions are expired after a configurable TTL
// by a background cleanup goroutine.
package execution

import (
	"sync"
	"time"

	"github.com/Develonaut/bnto/pkg/api"
)

// Status constants for execution lifecycle.
const (
	StatusPending   = "pending"
	StatusRunning   = "running"
	StatusCompleted = "completed"
	StatusFailed    = "failed"
)

// Execution represents the state of an async workflow run.
type Execution struct {
	ID          string          `json:"id"`
	Status      string          `json:"status"`
	Progress    []NodeProgress  `json:"progress"`
	Result      *api.RunResult  `json:"result,omitempty"`
	Error       string          `json:"error,omitempty"`
	OutputFiles []OutputFile    `json:"outputFiles,omitempty"`
	StartedAt   time.Time       `json:"startedAt"`
	CompletedAt *time.Time      `json:"completedAt,omitempty"`
}

// OutputFile describes a file produced by workflow execution.
type OutputFile struct {
	Key         string `json:"key"`
	Name        string `json:"name"`
	SizeBytes   int64  `json:"sizeBytes"`
	ContentType string `json:"contentType"`
}

// NodeProgress tracks the status of an individual node during execution.
type NodeProgress struct {
	NodeID string `json:"nodeId"`
	Status string `json:"status"`
}

// Manager stores and retrieves execution state in memory.
type Manager struct {
	mu         sync.RWMutex
	executions map[string]*Execution
}

// NewManager creates a Manager ready for use.
func NewManager() *Manager {
	return &Manager{
		executions: make(map[string]*Execution),
	}
}

// Create registers a new pending execution and returns it.
func (m *Manager) Create(id string) *Execution {
	exec := &Execution{
		ID:        id,
		Status:    StatusPending,
		Progress:  []NodeProgress{},
		StartedAt: time.Now(),
	}
	m.mu.Lock()
	m.executions[id] = exec
	m.mu.Unlock()
	return exec
}

// Get returns a snapshot of an execution by ID, or nil if not found.
// The returned copy is safe to serialize without holding the lock.
func (m *Manager) Get(id string) *Execution {
	m.mu.RLock()
	defer m.mu.RUnlock()
	exec, ok := m.executions[id]
	if !ok {
		return nil
	}
	snapshot := *exec
	snapshot.Progress = make([]NodeProgress, len(exec.Progress))
	copy(snapshot.Progress, exec.Progress)
	if exec.OutputFiles != nil {
		snapshot.OutputFiles = make([]OutputFile, len(exec.OutputFiles))
		copy(snapshot.OutputFiles, exec.OutputFiles)
	}
	return &snapshot
}

// SetRunning marks an execution as running.
func (m *Manager) SetRunning(id string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if exec, ok := m.executions[id]; ok {
		exec.Status = StatusRunning
	}
}

// AddProgress appends a node progress update.
func (m *Manager) AddProgress(id string, nodeID, status string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if exec, ok := m.executions[id]; ok {
		exec.Progress = append(exec.Progress, NodeProgress{
			NodeID: nodeID,
			Status: status,
		})
	}
}

// Complete marks an execution as completed with a result.
func (m *Manager) Complete(id string, result *api.RunResult) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if exec, ok := m.executions[id]; ok {
		now := time.Now()
		exec.Status = StatusCompleted
		exec.Result = result
		exec.CompletedAt = &now
	}
}

// CompleteWithFiles marks an execution as completed with a result and output files.
func (m *Manager) CompleteWithFiles(id string, result *api.RunResult, files []OutputFile) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if exec, ok := m.executions[id]; ok {
		now := time.Now()
		exec.Status = StatusCompleted
		exec.Result = result
		exec.OutputFiles = files
		exec.CompletedAt = &now
	}
}

// Fail marks an execution as failed with an error message.
func (m *Manager) Fail(id string, errMsg string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if exec, ok := m.executions[id]; ok {
		now := time.Now()
		exec.Status = StatusFailed
		exec.Error = errMsg
		exec.CompletedAt = &now
	}
}

// CleanupBefore removes executions that completed before the given cutoff.
func (m *Manager) CleanupBefore(cutoff time.Time) int {
	m.mu.Lock()
	defer m.mu.Unlock()
	removed := 0
	for id, exec := range m.executions {
		if exec.CompletedAt != nil && exec.CompletedAt.Before(cutoff) {
			delete(m.executions, id)
			removed++
		}
	}
	return removed
}
