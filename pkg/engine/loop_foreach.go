package engine

import (
	"context"
	"fmt"
	"time"

	"github.com/Develonaut/bento/pkg/neta"
)

// executeForEach executes a forEach loop AS A LEAF NODE.
// The loop is a single unit in the progress graph, with children executing internally.
func (i *Engine) executeForEach(
	ctx context.Context,
	def *neta.Definition,
	execCtx *executionContext,
	result *Result,
) error {
	items, err := i.extractLoopItems(def, execCtx)
	if err != nil {
		i.state.setNodeState(def.ID, "error")
		return err
	}

	i.initializeLoopExecution(def, execCtx)

	start := time.Now()
	loopResults, err := i.executeForEachIterations(ctx, def, items, execCtx)
	if err != nil {
		i.state.setNodeState(def.ID, "error")
		return err
	}

	i.finalizeLoopExecution(def, loopResults, execCtx, result, time.Since(start))
	return nil
}

// initializeLoopExecution sets up loop state and logging.
func (i *Engine) initializeLoopExecution(def *neta.Definition, execCtx *executionContext) {
	i.state.setNodeState(def.ID, "executing")
	i.state.setNodeProgress(def.ID, 0, "Starting loop")

	if i.logger != nil {
		msg := msgLoopStarted(execCtx.getBreadcrumb(), def.Name)
		i.logger.Info(msg.format())
	}
}

// finalizeLoopExecution completes loop execution and stores results.
func (i *Engine) finalizeLoopExecution(
	def *neta.Definition,
	loopResults []interface{},
	execCtx *executionContext,
	result *Result,
	duration time.Duration,
) {
	i.state.setNodeProgress(def.ID, 100, "Completed")
	i.state.setNodeState(def.ID, "completed")

	execCtx.set(def.ID, loopResults)
	result.NodeOutputs[def.ID] = loopResults
	result.NodesExecuted++

	if i.logger != nil {
		durationStr := formatDuration(duration)
		progressPct := i.state.getProgress()
		msg := msgLoopCompleted(execCtx.getBreadcrumb(), def.Name, durationStr, progressPct)
		i.logger.Info(msg.format())
	}

	i.notifyProgress(def.ID, "completed")
}

// checkLoopCancellation checks if context is cancelled.
func (i *Engine) checkLoopCancellation(ctx context.Context, def *neta.Definition) error {
	select {
	case <-ctx.Done():
		i.state.setNodeState(def.ID, "error")
		return ctx.Err()
	default:
		return nil
	}
}

// reportLoopProgress reports partial progress for current iteration.
func (i *Engine) reportLoopProgress(def *neta.Definition, idx, total int) {
	progress := (idx * 100) / total
	message := fmt.Sprintf("Iteration %d/%d", idx+1, total)
	i.state.setNodeProgress(def.ID, progress, message)

	if i.logger != nil {
		i.logger.Debug("Loop iteration",
			"loop_id", def.ID,
			"iteration", idx+1,
			"total", total)
	}
}

// logLoopIterationError logs an error during loop iteration.
func (i *Engine) logLoopIterationError(def *neta.Definition, idx int, err error) {
	if i.logger != nil {
		i.logger.Error("│  │   ✗ Loop iteration failed",
			"loop_id", def.ID,
			"iteration", idx,
			"error", err)
	}
}
