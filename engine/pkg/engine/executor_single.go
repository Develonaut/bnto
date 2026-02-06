package engine

import (
	"context"
	"regexp"
	"strings"
	"time"

	"github.com/Develonaut/bento/pkg/node"
	"github.com/charmbracelet/lipgloss"
)

// executeSingle executes a single (non-group) node.
func (i *Engine) executeSingle(ctx context.Context, def *node.Definition, execCtx *executionContext, result *Result) error {
	i.logExecutionStart(def, execCtx)
	nodeImpl, err := i.loadNodeImplementation(def)
	if err != nil {
		return err
	}
	if err := i.executeAndRecordNode(ctx, def, nodeImpl, execCtx, result); err != nil {
		return err
	}
	return nil
}

// loadNodeImplementation loads node from registry with error wrapping.
func (i *Engine) loadNodeImplementation(def *node.Definition) (node.Executable, error) {
	nodeImpl, err := i.registry.GetNew(def.Type)
	if err != nil {
		return nil, newNodeError(def.ID, def.Type, "get node", err)
	}
	return nodeImpl, nil
}

// executeAndRecordNode executes node and records results.
func (i *Engine) executeAndRecordNode(ctx context.Context, def *node.Definition, nodeImpl node.Executable,
	execCtx *executionContext, result *Result) error {
	params := i.prepareNodeParams(def, execCtx)
	output, duration, err := i.executeNodeWithTiming(ctx, nodeImpl, params)
	i.sendNodeCompleted(def.ID, duration, err)
	if err != nil {
		return newNodeError(def.ID, def.Type, "execute", err)
	}
	i.storeExecutionResult(def.ID, output, execCtx, result)
	i.logExecutionComplete(def, execCtx, duration)
	return nil
}

// logExecutionStart logs and notifies at the start of node execution.
func (i *Engine) logExecutionStart(def *node.Definition, execCtx *executionContext) {
	i.notifyProgress(def.ID, "starting")

	// Mark node as executing in execution state
	i.state.setNodeState(def.ID, "executing")
	i.state.setNodeProgress(def.ID, 0, "Starting")

	if i.messenger != nil {
		i.messenger.SendNodeStarted(def.ID, def.Name, def.Type)
	}

	if i.logger != nil {
		msg := msgNodeStarted()
		i.logger.Debug(msg.format(),
			"node_id", def.ID,
			"node_type", def.Type)
	}
}

// prepareNodeParams prepares execution parameters with context resolution.
func (i *Engine) prepareNodeParams(def *node.Definition, execCtx *executionContext) map[string]interface{} {
	params := make(map[string]interface{})
	for k, v := range def.Parameters {
		params[k] = execCtx.resolveValue(v)
	}

	params["_context"] = execCtx.toMap()
	params["_onOutput"] = func(line string) {
		if i.logger != nil {
			// Stream output with breadcrumb context (no tree indentation)
			// This is used for long-running external process output
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

// executeNodeWithTiming executes a node and tracks duration.
func (i *Engine) executeNodeWithTiming(
	ctx context.Context,
	nodeImpl node.Executable,
	params map[string]interface{},
) (interface{}, time.Duration, error) {
	start := time.Now()
	output, err := nodeImpl.Execute(ctx, params)
	duration := time.Since(start)

	if i.slowMoDelay > 0 {
		time.Sleep(i.slowMoDelay)
	}

	return output, duration, err
}

// sendNodeCompleted sends messenger event for completed node.
func (i *Engine) sendNodeCompleted(nodeID string, duration time.Duration, err error) {
	if i.messenger != nil {
		i.messenger.SendNodeCompleted(nodeID, duration, err)
	}
}

// storeExecutionResult stores node output and marks node as completed.
func (i *Engine) storeExecutionResult(
	nodeID string,
	output interface{},
	execCtx *executionContext,
	result *Result,
) {
	execCtx.set(nodeID, output)
	result.NodeOutputs[nodeID] = output
	result.NodesExecuted++

	// Mark node as completed in execution state
	i.state.setNodeProgress(nodeID, 100, "Completed")
	i.state.setNodeState(nodeID, "completed")
}

// logExecutionComplete logs completion with progress tracking.
func (i *Engine) logExecutionComplete(def *node.Definition, execCtx *executionContext, duration time.Duration) {
	i.notifyProgress(def.ID, "completed")

	if i.logger != nil {
		progressPct := i.state.getProgress()
		durationStr := formatDuration(duration)
		msg := msgChildNodeCompleted(execCtx.getBreadcrumb(), def.Type, def.Name, durationStr, progressPct)
		i.logger.Info(msg.format())
	}
}

// formatStreamingOutput formats streaming output with colored breadcrumb and tool prefixes.
// Example: Node1:Node2 [TOOL] message
// Breadcrumb nodes are colored in different shades, tool prefixes are colored by type.
func formatStreamingOutput(breadcrumb, line string) string {
	var result strings.Builder

	// Style the breadcrumb trail
	if breadcrumb != "" {
		nodes := strings.Split(breadcrumb, ":")

		// Color palette for breadcrumb nodes (cyan to blue gradient)
		nodeColors := []lipgloss.Color{
			lipgloss.Color("#00D9FF"), // Bright cyan
			lipgloss.Color("#0099FF"), // Sky blue
			lipgloss.Color("#0066FF"), // Medium blue
		}

		for i, node := range nodes {
			// Cycle through colors
			color := nodeColors[i%len(nodeColors)]
			styledNode := lipgloss.NewStyle().Foreground(color).Render(node)
			if i > 0 {
				result.WriteString(":")
			}
			result.WriteString(styledNode)
		}
		result.WriteString(" ")
	}

	// Detect and style tool prefixes like [PYTHON], [NODE], etc.
	toolPrefixRegex := regexp.MustCompile(`^\[([A-Z]+)\]\s*`)
	if matches := toolPrefixRegex.FindStringSubmatch(line); matches != nil {
		toolName := matches[1]
		restOfLine := toolPrefixRegex.ReplaceAllString(line, "")

		// Define colors for different tools
		var toolColor lipgloss.Color
		switch toolName {
		case "PYTHON":
			toolColor = lipgloss.Color("#3776AB") // Python blue
		case "NODE":
			toolColor = lipgloss.Color("#339933") // Node.js green
		default:
			toolColor = lipgloss.Color("#FF9900") // Default orange
		}

		styledTool := lipgloss.NewStyle().
			Foreground(toolColor).
			Bold(true).
			Render("[" + toolName + "]")

		result.WriteString(styledTool)
		result.WriteString(" ")
		result.WriteString(restOfLine)
	} else {
		// No tool prefix, just append the line
		result.WriteString(line)
	}

	return result.String()
}
