package neta

import "context"

// Executable is the core interface that all neta types must implement.
//
// The Execute method runs the neta's logic and returns a result.
// The result is passed as context to connected neta in the workflow.
//
// # Interface Implementation
//
// Go interfaces are implicit - if a type has an Execute method with this
// signature, it automatically implements Executable. You don't need to
// explicitly declare that a type implements an interface.
//
// # Example Implementation
//
//	type HTTPNeta struct {
//	    client *http.Client
//	}
//
//	func (h *HTTPNeta) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
//	    url := params["url"].(string)
//	    resp, err := h.client.Get(url)
//	    if err != nil {
//	        return nil, err
//	    }
//	    // ... process response
//	    return responseData, nil
//	}
//
// # Context Usage
//
// The context.Context parameter enables:
//   - Cancellation: Stop execution when workflow is cancelled
//   - Timeouts: Automatically timeout long-running operations
//   - Deadlines: Enforce maximum execution time
//
// Always respect context cancellation:
//
//	if ctx.Err() != nil {
//	    return nil, ctx.Err()
//	}
//
// Learn more about Go interfaces: https://go.dev/tour/methods/9
// Learn more about context: https://go.dev/blog/context
type Executable interface {
	// Execute runs the neta's logic.
	//
	// Parameters:
	//   ctx: Go context for cancellation, timeouts, and deadlines
	//   params: Neta-specific parameters plus execution context from previous neta
	//
	// The params map typically contains:
	//   - Neta-specific configuration (from Definition.Parameters)
	//   - "_context" key with accumulated data from previous neta
	//   - Other special keys prefixed with "_" (e.g., "_onOutput" for streaming)
	//
	// Returns:
	//   - interface{}: Result data (usually map[string]interface{})
	//   - error: Any error that occurred during execution
	//
	// The result is typically a map that gets merged into the workflow's
	// execution context and passed to downstream neta.
	Execute(ctx context.Context, params map[string]interface{}) (interface{}, error)
}

// ExecutionContext contains data passed between neta during workflow execution.
//
// This structure is managed by the itamae (orchestration engine) and tracks
// the accumulated state as a workflow executes.
//
// The execution context flows through the workflow graph, with each neta
// adding its results to the Data map.
type ExecutionContext struct {
	// Data contains accumulated results from all previous neta in the workflow.
	//
	// When a neta executes, its result is merged into this map.
	// Downstream neta can access results from upstream neta through this data.
	//
	// Example:
	//   If neta "fetch-user" returns {"user": {"id": 123, "name": "Alice"}},
	//   then downstream neta can access it via context.Data["user"]
	Data map[string]interface{}

	// NodeID is the ID of the currently executing neta.
	//
	// This helps with debugging and tracing workflow execution.
	NodeID string

	// Depth is the execution depth for nested groups.
	//
	// Top-level neta have depth 0.
	// Neta inside a group have depth 1.
	// Neta inside a group inside a group have depth 2, etc.
	//
	// This is useful for:
	//   - Debugging nested workflows
	//   - Limiting recursion depth
	//   - Indented logging output
	Depth int
}
