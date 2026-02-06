package engine

import (
	"context"
	"fmt"
	"time"

	"github.com/Develonaut/bento/pkg/neta"
)

// executeGroup executes a group neta (container with child nodes).
func (i *Engine) executeGroup(ctx context.Context, def *neta.Definition, execCtx *executionContext, result *Result) error {
	i.logGroupStart(def, execCtx)
	if len(def.Nodes) == 0 {
		return i.handleEmptyGroup(def)
	}
	start := time.Now()
	if err := i.executeGroupNodes(ctx, def, execCtx, result, start); err != nil {
		return err
	}
	i.logGroupComplete(def, execCtx, time.Since(start))
	return nil
}

// executeGroupNodes builds graph and executes all child nodes.
func (i *Engine) executeGroupNodes(
	ctx context.Context,
	def *neta.Definition,
	execCtx *executionContext,
	result *Result,
	start time.Time,
) error {
	g, err := i.buildAndValidateGraph(def, start)
	if err != nil {
		return err
	}
	return i.executeGroupGraph(ctx, def, g, execCtx, result, start)
}

// logGroupStart logs and notifies at the start of group execution.
func (i *Engine) logGroupStart(def *neta.Definition, execCtx *executionContext) {
	i.notifyProgress(def.ID, "starting")

	if i.messenger != nil {
		i.messenger.SendNodeStarted(def.ID, def.Name, def.Type)
	}

	if i.logger != nil {
		msg := msgGroupStarted(execCtx.getBreadcrumb(), def.Name)
		i.logger.Info(msg.format())
	}
}

// handleEmptyGroup handles execution of an empty group.
func (i *Engine) handleEmptyGroup(def *neta.Definition) error {
	i.notifyProgress(def.ID, "completed")
	if i.messenger != nil {
		i.messenger.SendNodeCompleted(def.ID, 0, nil)
	}
	return nil
}

// buildAndValidateGraph builds execution graph and validates for cycles.
func (i *Engine) buildAndValidateGraph(def *neta.Definition, start time.Time) (*graph, error) {
	g, err := i.buildGraphWithErrorHandling(def, start)
	if err != nil {
		return nil, err
	}

	if err := i.validateNoCycles(g, def, start); err != nil {
		return nil, err
	}

	return g, nil
}

// buildGraphWithErrorHandling builds graph with messenger notification on error.
func (i *Engine) buildGraphWithErrorHandling(def *neta.Definition, start time.Time) (*graph, error) {
	g, err := buildGraph(def)
	if err != nil {
		i.notifyGraphError(def.ID, start, err)
		return nil, newNodeError(def.ID, "group", "build graph", err)
	}
	return g, nil
}

// validateNoCycles checks for circular dependencies.
func (i *Engine) validateNoCycles(g *graph, def *neta.Definition, start time.Time) error {
	if g.hasCycle() {
		err := newNodeError(def.ID, "group", "validate", fmt.Errorf("circular dependency detected"))
		i.notifyGraphError(def.ID, start, err)
		return err
	}
	return nil
}

// notifyGraphError sends error notification to messenger.
func (i *Engine) notifyGraphError(nodeID string, start time.Time, err error) {
	if i.messenger != nil {
		i.messenger.SendNodeCompleted(nodeID, time.Since(start), err)
	}
}

// executeGroupGraph executes the group's graph in topological order.
func (i *Engine) executeGroupGraph(
	ctx context.Context,
	def *neta.Definition,
	g *graph,
	execCtx *executionContext,
	result *Result,
	start time.Time,
) error {
	childCtx := execCtx.withNode(def.Name)
	if err := i.executeGraph(ctx, g, childCtx, result); err != nil {
		if i.messenger != nil {
			i.messenger.SendNodeCompleted(def.ID, time.Since(start), err)
		}
		return err
	}
	return nil
}

// logGroupComplete logs completion of group execution.
func (i *Engine) logGroupComplete(def *neta.Definition, execCtx *executionContext, duration time.Duration) {
	i.notifyProgress(def.ID, "completed")

	if i.messenger != nil {
		i.messenger.SendNodeCompleted(def.ID, duration, nil)
	}

	if i.logger != nil {
		durationStr := formatDuration(duration)
		msg := msgGroupCompleted(execCtx.getBreadcrumb(), def.Name, durationStr)
		i.logger.Info(msg.format())
	}
}
