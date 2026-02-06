// Package parallel provides concurrent task execution with worker pools.
//
// The parallel neta executes multiple tasks concurrently with configurable:
//   - Worker pool size (limit concurrency)
//   - Error handling strategies (fail fast vs collect all)
//   - Context cancellation support
//
// Example usage:
//
//	params := map[string]interface{}{
//	    "tasks": []interface{}{
//	        map[string]interface{}{"id": 1},
//	        map[string]interface{}{"id": 2},
//	    },
//	    "maxWorkers": 5,
//	    "errorStrategy": "failFast",
//	}
//	result, err := parallelNeta.Execute(ctx, params)
//
// Learn more about Go concurrency: https://go.dev/tour/concurrency/1
package parallel

import (
	"context"
	"fmt"
)

// Parallel implements the parallel neta for concurrent task execution.
type Parallel struct{}

// New creates a new parallel neta instance.
func New() *Parallel {
	return &Parallel{}
}

// Execute runs tasks concurrently with optional worker pool.
//
// Parameters:
//   - tasks: array of task definitions
//   - maxWorkers: maximum concurrent workers (default: number of tasks)
//   - errorStrategy: "failFast" or "collectAll" (default: "failFast")
//   - _onStart: callback when task starts (for testing)
//   - _onComplete: callback when task completes (for testing)
//   - _shouldError: function to determine if task should error (for testing)
//
// Returns a map with:
//   - results: array of task results
//   - errors: array of errors (if errorStrategy is "collectAll")
func (p *Parallel) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	tasks, ok := params["tasks"].([]interface{})
	if !ok {
		return nil, fmt.Errorf("tasks parameter is required and must be an array")
	}

	if len(tasks) == 0 {
		return map[string]interface{}{
			"results": []interface{}{},
		}, nil
	}

	maxWorkers := getMaxWorkers(params, len(tasks))
	errorStrategy := getErrorStrategy(params)

	// Execute tasks with worker pool
	return p.executeWithWorkerPool(ctx, tasks, maxWorkers, errorStrategy, params)
}

// getMaxWorkers extracts the maxWorkers parameter.
func getMaxWorkers(params map[string]interface{}, defaultVal int) int {
	if mw, ok := params["maxWorkers"].(int); ok && mw > 0 {
		return mw
	}
	if mw, ok := params["maxWorkers"].(float64); ok && mw > 0 {
		return int(mw)
	}
	return defaultVal
}

// getErrorStrategy extracts the errorStrategy parameter.
func getErrorStrategy(params map[string]interface{}) string {
	if es, ok := params["errorStrategy"].(string); ok {
		return es
	}
	return "failFast"
}
