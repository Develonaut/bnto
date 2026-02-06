package logger

// Context helper functions for adding bento-specific fields to loggers.
// These functions are used by the engine to tag logs with workflow context.
//
// Usage:
//
//	logger := logger.New(logger.Config{...})
//
//	// Add context for a specific workflow
//	logger = logger.WithBentoID(logger, "my-workflow")
//	logger = logger.WithNodeID(logger, "node-1")
//
//	// Now all logs will include these fields
//	logger.Info("Executing node")

// WithBentoID adds bento_id to the logger context.
// The engine uses this to tag all logs from a specific workflow.
//
// The bento_id helps identify which workflow instance generated a log,
// which is critical for debugging parallel workflow executions.
func WithBentoID(logger *Logger, bentoID string) *Logger {
	return logger.With("bento_id", bentoID)
}

// WithNodeID adds node_id to the logger context.
// Used to track which node is currently executing.
//
// This helps identify exactly which step in a workflow generated a log,
// making it easier to debug complex workflows with many nodes.
func WithNodeID(logger *Logger, nodeID string) *Logger {
	return logger.With("node_id", nodeID)
}

// WithNodeType adds node_type to the logger context.
// Used to tag logs with the type of node being executed.
//
// Example node types: "http-request", "shell-command", "edit-fields"
// This helps filter and analyze logs by node type.
func WithNodeType(logger *Logger, nodeType string) *Logger {
	return logger.With("node_type", nodeType)
}
