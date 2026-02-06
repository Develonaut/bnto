// Package group provides a container neta for executing child neta sequentially or in parallel.
//
// The group neta is a fundamental building block for complex workflows. It acts as a
// container that can execute other neta in two modes:
//
// # Sequential Mode (default)
//
// Child neta execute one after another, with each neta receiving the accumulated
// context from all previous neta. This is the most common execution pattern.
//
// Use sequential mode when:
//   - Neta depend on results from previous neta
//   - Order of execution matters
//   - You need deterministic execution flow
//
// # Parallel Mode
//
// Child neta execute concurrently, all receiving the same initial context.
// Results from all parallel executions are merged into the output context.
//
// Use parallel mode when:
//   - Neta are independent (no dependencies)
//   - You want to speed up execution (e.g., fetching multiple APIs)
//   - Order doesn't matter
//
// # Nested Groups
//
// Groups can contain other groups, enabling hierarchical workflow structures.
// This allows patterns like:
//   - Sequential group containing parallel groups
//   - Parallel group containing sequential groups
//   - Arbitrary nesting depth
//
// # Example Usage
//
//	grp := group.New()
//
//	params := map[string]interface{}{
//	    "mode": "sequential",
//	    "nodes": []interface{}{
//	        map[string]interface{}{
//	            "id":   "fetch-data",
//	            "type": "http-request",
//	        },
//	        map[string]interface{}{
//	            "id":   "process-data",
//	            "type": "transform",
//	        },
//	    },
//	}
//
//	result, _ := grp.Execute(ctx, params)
//
// Note: In Phase 1a, the group neta validates structure and returns metadata.
// Actual child execution is delegated to the itamae (orchestration engine) in Phase 2.
package group

import (
	"context"
	"fmt"

	"github.com/Develonaut/bento/pkg/neta"
)

// ExecutionMode represents how child neta are executed.
type ExecutionMode string

const (
	// Sequential executes child neta one after another.
	Sequential ExecutionMode = "sequential"

	// Parallel executes child neta concurrently.
	Parallel ExecutionMode = "parallel"
)

// Group implements the group neta type.
//
// It acts as a container for child neta and coordinates their execution
// either sequentially or in parallel.
type Group struct{}

// New creates a new group neta instance.
//
// The returned neta is stateless and can be reused across multiple executions.
func New() *Group {
	return &Group{}
}

// Execute validates the group structure and returns execution metadata.
//
// Parameters:
//
//	mode (optional): Execution mode - "sequential" or "parallel" (default: "sequential")
//	nodes (required): Array of child neta definitions
//	_context (optional): Execution context from previous neta
//
// In Phase 1a, this method validates the group configuration and returns metadata.
// In Phase 2, the itamae will use this metadata to orchestrate actual child execution.
//
// Returns a map containing:
//   - mode: The execution mode used
//   - executed: Number of nodes that would be executed
//   - nodes: The child nodes (for itamae to execute)
func (g *Group) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	// Check for context cancellation
	if err := ctx.Err(); err != nil {
		return nil, err
	}

	// Extract and validate execution mode
	mode, err := extractMode(params)
	if err != nil {
		return nil, err
	}

	// Extract child nodes
	nodes, err := extractNodes(params)
	if err != nil {
		return nil, err
	}

	// Build result metadata
	// Note: Actual child execution will be done by itamae in Phase 2
	result := map[string]interface{}{
		"mode":     string(mode),
		"executed": len(nodes),
		"nodes":    nodes,
	}

	return result, nil
}

// Verify Group implements neta.Executable at compile time
var _ neta.Executable = (*Group)(nil)

// extractMode gets and validates the execution mode from params.
// Returns Sequential as default if mode is not specified.
func extractMode(params map[string]interface{}) (ExecutionMode, error) {
	modeRaw, exists := params["mode"]
	if !exists {
		return Sequential, nil // Default to sequential
	}

	modeStr, ok := modeRaw.(string)
	if !ok {
		return "", fmt.Errorf("mode must be a string, got %T", modeRaw)
	}

	mode := ExecutionMode(modeStr)

	// Validate mode
	switch mode {
	case Sequential, Parallel:
		return mode, nil
	default:
		return "", fmt.Errorf("invalid execution mode '%s': must be 'sequential' or 'parallel'", modeStr)
	}
}

// extractNodes gets the child nodes array from params.
func extractNodes(params map[string]interface{}) ([]interface{}, error) {
	nodesRaw, exists := params["nodes"]
	if !exists {
		// Nodes are required, but can be empty array
		return []interface{}{}, nil
	}

	nodes, ok := nodesRaw.([]interface{})
	if !ok {
		return nil, fmt.Errorf("nodes must be an array, got %T", nodesRaw)
	}

	return nodes, nil
}
