package parallel

import (
	"context"
	"sync"
)

// collectResults collects results and errors from workers.
func collectResults(
	ctx context.Context,
	resultChan <-chan taskResult,
	errorChan <-chan error,
	wg *sync.WaitGroup,
	numTasks int,
	errorStrategy string,
	cancel context.CancelFunc,
) ([]interface{}, []interface{}, error) {
	collector := newResultCollector(numTasks)
	go closeChannelsWhenDone(wg, resultChan, errorChan)

	for !collector.isComplete() {
		if err := collector.collectNext(ctx, resultChan, errorChan, errorStrategy, cancel); err != nil {
			return collector.results, collector.errors, err
		}
	}

	return collector.results, collector.errors, nil
}

// resultCollector manages collection of results and errors.
type resultCollector struct {
	results   []interface{}
	errors    []interface{}
	completed int
	numTasks  int
}

// newResultCollector creates a new result collector.
func newResultCollector(numTasks int) *resultCollector {
	return &resultCollector{
		results:  make([]interface{}, numTasks),
		errors:   make([]interface{}, 0),
		numTasks: numTasks,
	}
}

// isComplete checks if all tasks are complete.
func (c *resultCollector) isComplete() bool {
	return c.completed >= c.numTasks
}

// collectNext collects the next result or error.
func (c *resultCollector) collectNext(
	ctx context.Context,
	resultChan <-chan taskResult,
	errorChan <-chan error,
	errorStrategy string,
	cancel context.CancelFunc,
) error {
	select {
	case <-ctx.Done():
		return ctx.Err()
	case result, ok := <-resultChan:
		return c.handleResult(ctx, result, ok)
	case err, ok := <-errorChan:
		return c.handleError(ctx, err, ok, errorStrategy, cancel)
	}
}

// handleResult processes a received result.
func (c *resultCollector) handleResult(ctx context.Context, result taskResult, ok bool) error {
	if !ok {
		if err := ctx.Err(); err != nil {
			return err
		}
		return nil
	}
	c.results[result.index] = result.result
	c.completed++
	return nil
}

// handleError processes a received error.
func (c *resultCollector) handleError(
	ctx context.Context,
	err error,
	ok bool,
	errorStrategy string,
	cancel context.CancelFunc,
) error {
	if !ok {
		if ctxErr := ctx.Err(); ctxErr != nil {
			return ctxErr
		}
		return nil
	}

	if errorStrategy == "failFast" {
		cancel()
		return err
	}

	c.errors = append(c.errors, err.Error())
	c.completed++
	return nil
}

// closeChannelsWhenDone closes result and error channels when workers are done.
func closeChannelsWhenDone(wg *sync.WaitGroup, resultChan <-chan taskResult, errorChan <-chan error) {
	wg.Wait()
	// Channels are closed by type assertion in the collector
}
