package parallel

import (
	"context"
	"sync"
)

// taskJob represents a task to be executed.
type taskJob struct {
	index int
	task  interface{}
}

// taskResult represents the result of a task execution.
type taskResult struct {
	index  int
	result interface{}
}

// executeWithWorkerPool runs tasks using a worker pool.
func (p *Parallel) executeWithWorkerPool(
	ctx context.Context,
	tasks []interface{},
	maxWorkers int,
	errorStrategy string,
	params map[string]interface{},
) (interface{}, error) {
	taskChan := make(chan taskJob, len(tasks))
	resultChan := make(chan taskResult, len(tasks))
	errorChan := make(chan error, len(tasks))

	workerCtx, cancel := context.WithCancel(ctx)
	defer cancel()

	var wg sync.WaitGroup
	startWorkers(&wg, workerCtx, maxWorkers, p, taskChan, resultChan, errorChan, params)

	sendTasksToWorkers(taskChan, tasks)

	results, errors, err := collectResults(workerCtx, resultChan, errorChan, &wg, len(tasks), errorStrategy, cancel)
	if err != nil {
		return nil, err
	}

	return buildOutput(results, errors), nil
}

// startWorkers starts the worker goroutines.
func startWorkers(
	wg *sync.WaitGroup,
	ctx context.Context,
	maxWorkers int,
	p *Parallel,
	taskChan <-chan taskJob,
	resultChan chan<- taskResult,
	errorChan chan<- error,
	params map[string]interface{},
) {
	for i := 0; i < maxWorkers; i++ {
		wg.Add(1)
		go p.worker(ctx, taskChan, resultChan, errorChan, wg, params)
	}
}

// sendTasksToWorkers sends all tasks to the task channel.
func sendTasksToWorkers(taskChan chan<- taskJob, tasks []interface{}) {
	go func() {
		for i, task := range tasks {
			taskChan <- taskJob{
				index: i,
				task:  task,
			}
		}
		close(taskChan)
	}()
}

// buildOutput builds the output map with results and errors.
func buildOutput(results []interface{}, errors []interface{}) map[string]interface{} {
	output := map[string]interface{}{
		"results": results,
	}

	if len(errors) > 0 {
		output["errors"] = errors
	}

	return output
}
