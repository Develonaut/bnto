package engine

import (
	"context"
	"fmt"
	"sync"

	"github.com/Develonaut/bnto/pkg/node"
)

// executeForEachIterations executes all loop iterations and returns results.
// Supports concurrent execution via maxConcurrency parameter.
func (i *Engine) executeForEachIterations(
	ctx context.Context,
	def *node.Definition,
	items []interface{},
	execCtx *executionContext,
) ([]interface{}, error) {
	// Check for maxConcurrency parameter
	maxConcurrency := i.getLoopConcurrency(def)

	if maxConcurrency > 1 {
		return i.executeForEachConcurrent(ctx, def, items, execCtx, maxConcurrency)
	}

	// Sequential execution (original behavior)
	return i.executeForEachSequential(ctx, def, items, execCtx)
}

// getLoopConcurrency extracts maxConcurrency parameter from loop definition.
func (i *Engine) getLoopConcurrency(def *node.Definition) int {
	if mc, ok := def.Parameters["maxConcurrency"]; ok {
		if mcInt, ok := mc.(int); ok {
			return mcInt
		}
		if mcFloat, ok := mc.(float64); ok {
			return int(mcFloat)
		}
	}
	return 1 // Default: sequential
}

// executeForEachSequential executes iterations sequentially (original behavior).
func (i *Engine) executeForEachSequential(
	ctx context.Context,
	def *node.Definition,
	items []interface{},
	execCtx *executionContext,
) ([]interface{}, error) {
	loopResults := make([]interface{}, 0, len(items))
	totalItems := len(items)
	continueOnError := i.getLoopContinueOnError(def)

	for idx, item := range items {
		if err := i.checkLoopCancellation(ctx, def); err != nil {
			return nil, err
		}

		i.reportLoopProgress(def, idx, totalItems)

		iterResult, err := i.executeLoopIteration(ctx, def, item, idx, totalItems, execCtx)
		if err != nil {
			i.logLoopIterationError(def, idx, err)
			if continueOnError {
				// Skip this iteration but continue with others
				loopResults = append(loopResults, nil)
				continue
			}
			// Fail fast
			return nil, fmt.Errorf("iteration %d failed: %w", idx, err)
		}

		loopResults = append(loopResults, iterResult)
	}

	return loopResults, nil
}

// executeForEachConcurrent executes iterations concurrently with worker pool.
func (i *Engine) executeForEachConcurrent(
	ctx context.Context,
	def *node.Definition,
	items []interface{},
	execCtx *executionContext,
	maxConcurrency int,
) ([]interface{}, error) {
	totalItems := len(items)
	results := make([]interface{}, totalItems)
	continueOnError := i.getLoopContinueOnError(def)

	var wg sync.WaitGroup
	var mu sync.Mutex
	var firstErr error
	var completedCount int

	// Create semaphore for concurrency control
	sem := make(chan struct{}, maxConcurrency)

	for idx, item := range items {
		// Check cancellation before starting new iteration
		if err := ctx.Err(); err != nil {
			return nil, err
		}

		wg.Add(1)
		go func(index int, itm interface{}) {
			defer wg.Done()

			// Acquire semaphore
			sem <- struct{}{}
			defer func() { <-sem }()

			// Execute iteration
			iterResult, err := i.executeLoopIteration(ctx, def, itm, index, totalItems, execCtx)

			// Update results and progress
			mu.Lock()
			defer mu.Unlock()

			if err != nil {
				if continueOnError {
					// Log error but continue with other iterations
					i.logLoopIterationError(def, index, err)
					results[index] = nil // Mark as skipped
				} else {
					// Fail fast - capture first error
					if firstErr == nil {
						firstErr = fmt.Errorf("iteration %d failed: %w", index, err)
						i.logLoopIterationError(def, index, err)
					}
				}
			} else {
				results[index] = iterResult
			}

			completedCount++
			i.reportLoopProgress(def, completedCount-1, totalItems)
		}(idx, item)
	}

	// Wait for all iterations to complete
	wg.Wait()

	if firstErr != nil {
		return nil, firstErr
	}

	return results, nil
}

// getLoopContinueOnError checks if loop should continue on iteration errors.
func (i *Engine) getLoopContinueOnError(def *node.Definition) bool {
	if coe, ok := def.Parameters["continueOnError"]; ok {
		if coeBool, ok := coe.(bool); ok {
			return coeBool
		}
	}
	return false
}

// executeLoopIteration executes one iteration (INTERNAL - not tracked in graph).
func (i *Engine) executeLoopIteration(
	ctx context.Context,
	def *node.Definition,
	item interface{},
	idx int,
	total int,
	execCtx *executionContext,
) (map[string]interface{}, error) {
	iterCtx := execCtx.withNode(def.Name)
	iterCtx.set("item", item)
	iterCtx.set("index", idx)

	return i.executeIterationChildren(ctx, def, idx, total, iterCtx)
}

// executeIterationChildren executes all child nodes for one iteration.
func (i *Engine) executeIterationChildren(
	ctx context.Context,
	def *node.Definition,
	idx int,
	total int,
	iterCtx *executionContext,
) (map[string]interface{}, error) {
	iterResult := make(map[string]interface{})

	for j := range def.Nodes {
		childDef := &def.Nodes[j]

		// Notify messenger which child is currently executing
		if i.messenger != nil {
			i.messenger.SendLoopChild(def.ID, childDef.Name, idx, total)
		}

		output, err := i.executeNodeInternal(ctx, childDef, iterCtx)
		if err != nil {
			return nil, err
		}

		iterResult[childDef.ID] = output
		iterCtx.set(childDef.ID, output)
	}

	return iterResult, nil
}
