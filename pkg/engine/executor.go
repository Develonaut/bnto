package engine

import (
	"context"

	"github.com/Develonaut/bento/pkg/neta"
)

// executeNode executes a single node (handles all node types).
func (i *Engine) executeNode(
	ctx context.Context,
	def *neta.Definition,
	execCtx *executionContext,
	result *Result,
) error {
	if err := i.checkContextCancellation(ctx); err != nil {
		return err
	}
	return i.dispatchNodeType(ctx, def, execCtx, result)
}

// checkContextCancellation checks if context is cancelled.
func (i *Engine) checkContextCancellation(ctx context.Context) error {
	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
		return nil
	}
}

// dispatchNodeType routes to appropriate executor based on node type.
func (i *Engine) dispatchNodeType(
	ctx context.Context,
	def *neta.Definition,
	execCtx *executionContext,
	result *Result,
) error {
	switch def.Type {
	case "group":
		return i.executeGroup(ctx, def, execCtx, result)
	case "loop":
		return i.executeLoop(ctx, def, execCtx, result)
	case "parallel":
		return i.executeParallel(ctx, def, execCtx, result)
	default:
		return i.executeSingle(ctx, def, execCtx, result)
	}
}

// executeGraph executes all nodes in a graph in topological order.
func (i *Engine) executeGraph(ctx context.Context, g *graph, execCtx *executionContext, result *Result) error {
	executed := make(map[string]bool)
	queue := g.getStartNodes()
	for len(queue) > 0 {
		node, newQueue, err := i.processNextNode(ctx, queue, executed, g, execCtx, result)
		if err != nil {
			return err
		}
		queue = newQueue
		if node != nil {
			queue = i.enqueueReadyTargets(g, node.ID, executed, queue)
		}
	}
	return nil
}

// processNextNode processes the next node in the queue.
func (i *Engine) processNextNode(ctx context.Context, queue []*neta.Definition, executed map[string]bool,
	g *graph, execCtx *executionContext, result *Result) (*neta.Definition, []*neta.Definition, error) {
	if err := i.checkContextCancellation(ctx); err != nil {
		return nil, queue, err
	}
	node, newQueue := i.dequeueNode(queue)
	if executed[node.ID] {
		return nil, newQueue, nil
	}
	if err := i.executeAndMarkNode(ctx, node, g, executed, execCtx, result); err != nil {
		return nil, newQueue, err
	}
	return node, newQueue, nil
}

// dequeueNode removes and returns the first node from queue.
func (i *Engine) dequeueNode(queue []*neta.Definition) (*neta.Definition, []*neta.Definition) {
	return queue[0], queue[1:]
}

// executeAndMarkNode executes a node and marks it as executed.
func (i *Engine) executeAndMarkNode(
	ctx context.Context,
	node *neta.Definition,
	g *graph,
	executed map[string]bool,
	execCtx *executionContext,
	result *Result,
) error {
	if err := i.executeNode(ctx, node, execCtx, result); err != nil {
		return err
	}
	executed[node.ID] = true
	g.markExecuted(node.ID)
	return nil
}

// enqueueReadyTargets adds ready target nodes to the queue.
func (i *Engine) enqueueReadyTargets(
	g *graph,
	nodeID string,
	executed map[string]bool,
	queue []*neta.Definition,
) []*neta.Definition {
	for _, target := range g.getTargets(nodeID) {
		if g.isReady(target.ID) && !executed[target.ID] {
			queue = append(queue, target)
		}
	}
	return queue
}

// notifyProgress calls the progress callback if set.
func (i *Engine) notifyProgress(nodeID string, status string) {
	if i.onProgress != nil {
		i.onProgress(nodeID, status)
	}
}
