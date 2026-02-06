package parallel

import (
	"context"
	"fmt"
	"sync"
)

// worker processes tasks from the task channel.
func (p *Parallel) worker(
	ctx context.Context,
	taskChan <-chan taskJob,
	resultChan chan<- taskResult,
	errorChan chan<- error,
	wg *sync.WaitGroup,
	params map[string]interface{},
) {
	defer wg.Done()

	for job := range taskChan {
		if shouldStopWorker(ctx) {
			return
		}

		result, err := p.executeTask(ctx, job.task, params)

		if err != nil {
			if !sendError(ctx, errorChan, err) {
				return
			}
			continue
		}

		if !sendResult(ctx, resultChan, job.index, result) {
			return
		}
	}
}

// shouldStopWorker checks if the worker should stop.
func shouldStopWorker(ctx context.Context) bool {
	select {
	case <-ctx.Done():
		return true
	default:
		return false
	}
}

// sendError sends an error to the error channel.
func sendError(ctx context.Context, errorChan chan<- error, err error) bool {
	select {
	case errorChan <- err:
		return true
	case <-ctx.Done():
		return false
	}
}

// sendResult sends a result to the result channel.
func sendResult(ctx context.Context, resultChan chan<- taskResult, index int, result interface{}) bool {
	select {
	case resultChan <- taskResult{
		index:  index,
		result: result,
	}:
		return true
	case <-ctx.Done():
		return false
	}
}

// executeTask executes a single task.
func (p *Parallel) executeTask(
	ctx context.Context,
	task interface{},
	params map[string]interface{},
) (interface{}, error) {
	callOnStart(params)
	defer callOnComplete(params)

	if err := checkShouldError(task, params); err != nil {
		return nil, err
	}

	// In real implementation, this would execute nested neta
	// For now, just return the task data
	return task, nil
}

// callOnStart calls the onStart callback if provided.
func callOnStart(params map[string]interface{}) {
	if onStart, ok := params["_onStart"].(func()); ok {
		onStart()
	}
}

// callOnComplete calls the onComplete callback if provided.
func callOnComplete(params map[string]interface{}) {
	if onComplete, ok := params["_onComplete"].(func()); ok {
		onComplete()
	}
}

// checkShouldError checks if the task should error (for testing).
func checkShouldError(task interface{}, params map[string]interface{}) error {
	shouldError, ok := params["_shouldError"].(func(map[string]interface{}) bool)
	if !ok {
		return nil
	}

	taskMap, ok := task.(map[string]interface{})
	if !ok {
		return nil
	}

	if shouldError(taskMap) {
		return fmt.Errorf("task failed")
	}

	return nil
}
