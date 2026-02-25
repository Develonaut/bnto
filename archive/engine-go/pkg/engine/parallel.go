package engine

import (
	"context"
	"sync"
	"time"

	"github.com/Develonaut/bnto/pkg/node"
)

// executeParallel executes a parallel node.
// Runs all child nodes concurrently using goroutines.
func (i *Engine) executeParallel(
	ctx context.Context,
	def *node.Definition,
	execCtx *executionContext,
	result *Result,
) error {
	i.notifyProgress(def.ID, "starting")

	// Send messenger event: node started
	if i.messenger != nil {
		i.messenger.SendNodeStarted(def.ID, def.Name, def.Type)
	}

	childCount := len(def.Nodes)

	if i.logger != nil {
		i.logger.Info("⚡ Starting parallel execution",
			"parallel_id", def.ID,
			"child_count", childCount)
	}

	// Handle empty parallel
	if childCount == 0 {
		i.notifyProgress(def.ID, "completed")
		// Send messenger event: node completed
		if i.messenger != nil {
			i.messenger.SendNodeCompleted(def.ID, 0, nil)
		}
		result.NodesExecuted++
		return nil
	}

	// Track execution time for messenger
	start := time.Now()

	// Get max concurrency (default: no limit)
	maxConcurrency := 0
	if mc, ok := def.Parameters["maxConcurrency"]; ok {
		if mcFloat, ok := mc.(float64); ok {
			maxConcurrency = int(mcFloat)
		}
	}

	// Execute children in parallel
	var wg sync.WaitGroup
	var mu sync.Mutex
	var firstErr error

	// Create semaphore for concurrency control
	var sem chan struct{}
	if maxConcurrency > 0 {
		sem = make(chan struct{}, maxConcurrency)
	}

	for idx := range def.Nodes {
		child := &def.Nodes[idx]

		wg.Add(1)
		go func(node *node.Definition) {
			defer wg.Done()

			// Acquire semaphore if concurrency limited
			if sem != nil {
				sem <- struct{}{}
				defer func() { <-sem }()
			}

			// Create child context
			childCtx := execCtx.copy()

			// Execute child
			childResult := &Result{
				NodeOutputs: make(map[string]interface{}),
			}

			err := i.executeNode(ctx, node, childCtx, childResult)

			// Handle error (capture first error only)
			mu.Lock()
			if err != nil && firstErr == nil {
				firstErr = err
			}

			// Merge child outputs
			if err == nil {
				for k, v := range childResult.NodeOutputs {
					result.NodeOutputs[k] = v
				}
				result.NodesExecuted += childResult.NodesExecuted
			}
			mu.Unlock()
		}(child)
	}

	// Wait for all goroutines to complete
	wg.Wait()

	duration := time.Since(start)

	if firstErr != nil {
		err := newNodeError(def.ID, "parallel", "execute", firstErr)
		// Send messenger event: node completed with error
		if i.messenger != nil {
			i.messenger.SendNodeCompleted(def.ID, duration, err)
		}
		return err
	}

	i.notifyProgress(def.ID, "completed")
	result.NodesExecuted++

	// Send messenger event: node completed
	if i.messenger != nil {
		i.messenger.SendNodeCompleted(def.ID, duration, nil)
	}

	if i.logger != nil {
		i.logger.Info("✓ Parallel execution completed",
			"parallel_id", def.ID,
			"child_count", childCount)
	}

	return nil
}
