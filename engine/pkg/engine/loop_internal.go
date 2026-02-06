package engine

import (
	"context"
	"time"

	"github.com/Develonaut/bento/pkg/node"
)

// executeNodeInternal executes a node without state tracking (for loop children).
func (i *Engine) executeNodeInternal(
	ctx context.Context,
	def *node.Definition,
	execCtx *executionContext,
) (interface{}, error) {
	i.logInternalNodeStart(def, execCtx)

	nodeImpl, err := i.loadNodeForInternal(def)
	if err != nil {
		return nil, err
	}

	params := i.prepareInternalNodeParams(def, execCtx)

	output, duration, err := i.executeInternalNode(ctx, nodeImpl, params)
	if err != nil {
		return nil, newNodeError(def.ID, def.Type, "execute", err)
	}

	i.logInternalNodeComplete(def, execCtx, duration)
	return output, nil
}

// logInternalNodeStart logs execution start for internal node.
func (i *Engine) logInternalNodeStart(def *node.Definition, execCtx *executionContext) {
	if i.logger != nil {
		msg := msgChildNodeStarted(execCtx.getBreadcrumb(), def.Type, def.Name)
		i.logger.Info(msg.format())
	}
}

// loadNodeForInternal loads node implementation for internal execution.
func (i *Engine) loadNodeForInternal(def *node.Definition) (node.Executable, error) {
	nodeImpl, err := i.registry.GetNew(def.Type)
	if err != nil {
		return nil, newNodeError(def.ID, def.Type, "get node", err)
	}
	return nodeImpl, nil
}

// prepareInternalNodeParams prepares parameters for internal node execution.
func (i *Engine) prepareInternalNodeParams(
	def *node.Definition,
	execCtx *executionContext,
) map[string]interface{} {
	params := make(map[string]interface{})
	for k, v := range def.Parameters {
		params[k] = execCtx.resolveValue(v)
	}
	params["_context"] = execCtx.toMap()
	params["_onOutput"] = func(line string) {
		if i.logger != nil {
			// Stream output with breadcrumb context
			breadcrumb := execCtx.getBreadcrumb()
			if breadcrumb != "" {
				formattedLine := formatStreamingOutput(breadcrumb, line)
				i.logger.Info(formattedLine)
			} else {
				i.logger.Info(line)
			}
		}
	}
	return params
}

// executeInternalNode executes node and tracks duration.
func (i *Engine) executeInternalNode(
	ctx context.Context,
	nodeImpl node.Executable,
	params map[string]interface{},
) (interface{}, time.Duration, error) {
	start := time.Now()
	output, err := nodeImpl.Execute(ctx, params)
	return output, time.Since(start), err
}

// logInternalNodeComplete logs completion for internal node.
func (i *Engine) logInternalNodeComplete(
	def *node.Definition,
	execCtx *executionContext,
	duration time.Duration,
) {
	if i.logger != nil {
		durationStr := formatDuration(duration)
		progressPct := i.state.getProgress()
		msg := msgChildNodeCompleted(execCtx.getBreadcrumb(), def.Type, def.Name, durationStr, progressPct)
		i.logger.Info(msg.format())
	}
}
