package engine

import (
	"context"
	"fmt"
	"time"

	"github.com/Develonaut/bento/pkg/neta"
)

// executeTimes executes a times loop (repeat N times) AS A LEAF NODE.
func (i *Engine) executeTimes(
	ctx context.Context,
	def *neta.Definition,
	execCtx *executionContext,
	result *Result,
) error {
	count, err := i.extractTimesCount(def)
	if err != nil {
		i.state.setNodeState(def.ID, "error")
		return err
	}

	i.initializeLoopExecution(def, execCtx)

	start := time.Now()
	loopResults, err := i.executeTimesIterations(ctx, def, count, execCtx)
	if err != nil {
		i.state.setNodeState(def.ID, "error")
		return err
	}

	i.finalizeLoopExecution(def, loopResults, execCtx, result, time.Since(start))
	return nil
}

// executeTimesIterations executes all times loop iterations and returns results.
func (i *Engine) executeTimesIterations(
	ctx context.Context,
	def *neta.Definition,
	count int,
	execCtx *executionContext,
) ([]interface{}, error) {
	loopResults := make([]interface{}, 0, count)

	for iteration := 0; iteration < count; iteration++ {
		if err := i.checkLoopCancellation(ctx, def); err != nil {
			return nil, err
		}

		i.reportLoopProgress(def, iteration, count)

		iterResult, err := i.executeTimesIteration(ctx, def, iteration, count, execCtx)
		if err != nil {
			i.logLoopIterationError(def, iteration, err)
			return nil, fmt.Errorf("iteration %d failed: %w", iteration, err)
		}

		loopResults = append(loopResults, iterResult)
	}

	return loopResults, nil
}

// executeTimesIteration executes one iteration (INTERNAL - not tracked in graph).
func (i *Engine) executeTimesIteration(
	ctx context.Context,
	def *neta.Definition,
	iteration int,
	total int,
	execCtx *executionContext,
) (map[string]interface{}, error) {
	iterCtx := execCtx.withNode(def.Name)
	iterCtx.set("iteration", iteration)
	iterCtx.set("index", iteration) // Alias for consistency with forEach

	return i.executeIterationChildren(ctx, def, iteration, total, iterCtx)
}

// extractTimesCount extracts and validates count parameter for times loop.
func (i *Engine) extractTimesCount(def *neta.Definition) (int, error) {
	countParam := def.Parameters["count"]
	count, ok := countParam.(float64)
	if !ok {
		return 0, newNodeError(def.ID, "loop", "validate",
			fmt.Errorf("'count' must be a number"))
	}
	return int(count), nil
}
