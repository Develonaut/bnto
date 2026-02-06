package engine

import (
	"context"
	"fmt"
	"time"

	"github.com/Develonaut/bento/pkg/node"
)

// executeLoop executes a loop node.
// Supports forEach, times, and while loop modes for iterating over collections or ranges.
func (i *Engine) executeLoop(
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

	// Track execution time for messenger
	start := time.Now()

	// Get loop parameters
	mode, ok := def.Parameters["mode"].(string)
	if !ok {
		err := newNodeError(def.ID, "loop", "validate",
			fmt.Errorf("missing or invalid 'mode' parameter"))
		if i.messenger != nil {
			i.messenger.SendNodeCompleted(def.ID, time.Since(start), err)
		}
		return err
	}

	var err error
	switch mode {
	case "forEach":
		err = i.executeForEach(ctx, def, execCtx, result)
	case "times":
		err = i.executeTimes(ctx, def, execCtx, result)
	case "while":
		err = i.executeWhile(ctx, def, execCtx, result)
	default:
		err = newNodeError(def.ID, "loop", "validate",
			fmt.Errorf("unknown loop mode: %s", mode))
	}

	duration := time.Since(start)

	// Send messenger event: node completed
	if i.messenger != nil {
		i.messenger.SendNodeCompleted(def.ID, duration, err)
	}

	return err
}

// executeWhile executes a while loop.
func (i *Engine) executeWhile(
	ctx context.Context,
	def *node.Definition,
	execCtx *executionContext,
	result *Result,
) error {
	// While loop implementation (stub for now)
	return newNodeError(def.ID, "loop", "execute",
		fmt.Errorf("while loops not yet implemented"))
}
