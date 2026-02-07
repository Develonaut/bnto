// Package engine provides the orchestration engine for executing bntos.
//
// The engine orchestrates bnto execution, managing data flow, concurrency,
// and error handling.
//
// # Usage
//
//	r := registry.New()
//	logger := logger.New(logger.Config{Level: logger.LevelInfo})
//	eng := engine.New(r, logger)
//
//	// Execute a bnto
//	result, err := eng.Serve(ctx, bntoDef)
//	if err != nil {
//	    log.Fatalf("Execution failed: %v", err)
//	}
//
// # Context Management
//
// The engine passes data between node through an execution context.
// Each node's output becomes available to downstream nodes via template variables.
//
// Learn more about context.Context: https://go.dev/blog/context
package engine

import (
	"context"
	"fmt"
	"time"

	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/registry"
	"github.com/Develonaut/bnto/pkg/logger"
)

// ProgressMessenger receives execution progress events.
// Optional - nil check before use.
type ProgressMessenger interface {
	SendNodeStarted(path, name, nodeType string)
	SendNodeCompleted(path string, duration time.Duration, err error)
	SendLoopChild(loopPath, childName string, index, total int)
}

// Engine orchestrates bnto execution.
type Engine struct {
	registry    *registry.Registry
	logger      *logger.Logger    // Optional - can be nil
	messenger   ProgressMessenger // Optional - for TUI progress updates
	onProgress  ProgressCallback
	slowMoDelay time.Duration   // Delay between node completions for animations
	state       *executionState // Progress tracking state
}

// ProgressCallback is called when a node starts/completes execution.
type ProgressCallback func(nodeID string, status string)

// Result contains the result of a bnto execution.
type Result struct {
	Status        Status                 // Execution status
	NodesExecuted int                    // Number of nodes executed
	NodeOutputs   map[string]interface{} // Output from each node
	Duration      time.Duration          // Total execution time
	Error         error                  // Error if execution failed
}

// Status represents the execution status.
type Status string

const (
	StatusSuccess   Status = "success"
	StatusFailed    Status = "failed"
	StatusCancelled Status = "cancelled"
)

// New creates a new Engine orchestrator.
func New(r *registry.Registry, log *logger.Logger) *Engine {
	return &Engine{
		registry: r,
		logger:   log,
	}
}

// NewWithMessenger creates an Engine with progress messaging.
// Both logger and messenger are optional - can be nil.
func NewWithMessenger(r *registry.Registry, log *logger.Logger, messenger ProgressMessenger) *Engine {
	// Note: Import tui here would create circular dependency, so we'll set slowMo from outside
	return &Engine{
		registry:    r,
		logger:      log,
		messenger:   messenger,
		slowMoDelay: 0, // Will be set by caller to avoid circular dependency
	}
}

// SetSlowMoDelay sets the delay between node completions for animations.
func (i *Engine) SetSlowMoDelay(delay time.Duration) {
	i.slowMoDelay = delay
}

// OnProgress registers a callback for progress updates.
func (i *Engine) OnProgress(callback ProgressCallback) {
	i.onProgress = callback
}

// Serve executes a bnto definition.
//
// Returns:
//   - *Result: Execution result with outputs from all nodes
//   - error: Any error that occurred during execution
func (i *Engine) Serve(ctx context.Context, def *node.Definition) (*Result, error) {
	start := time.Now()

	// Analyze graph structure and initialize execution state
	graph := analyzeGraph(def)
	i.state = newExecutionState(graph)

	if i.logger != nil {
		msg := msgBntoStarted(def.Name)
		i.logger.Info(msg.format())
	}

	result := &Result{
		NodeOutputs: make(map[string]interface{}),
	}

	// Create execution context
	execCtx := newExecutionContext()

	// Execute the bnto
	err := i.executeNode(ctx, def, execCtx, result)

	result.Duration = time.Since(start)

	if err != nil {
		result.Status = StatusFailed
		result.Error = err

		if i.logger != nil {
			durationStr := formatDuration(result.Duration)
			msg := msgBntoFailed(durationStr)
			i.logger.Error(msg.format())
			i.logger.Error("Error: " + err.Error())
		}

		return result, err
	}

	result.Status = StatusSuccess

	if i.logger != nil {
		durationStr := formatDuration(result.Duration)
		msg := msgBntoCompleted(durationStr)
		i.logger.Info(msg.format())
	}

	return result, nil
}

// formatDuration formats a duration to match CLI output (e.g., "6ms", "1.2s")
func formatDuration(d time.Duration) string {
	if d < time.Second {
		// Less than 1 second - show milliseconds
		ms := d.Milliseconds()
		return fmt.Sprintf("%dms", ms)
	}
	// 1 second or more - show with decimal
	s := d.Seconds()
	return fmt.Sprintf("%.1fs", s)
}
