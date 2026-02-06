package logger

// Context helper functions for adding bento-specific fields to loggers.
// These functions are used by itamae to tag logs with workflow context.
//
// Usage:
//
//	logger := shoyu.New(shoyu.Config{...})
//
//	// Add context for a specific workflow
//	logger = shoyu.WithBentoID(logger, "my-workflow")
//	logger = shoyu.WithNetaID(logger, "node-1")
//
//	// Now all logs will include these fields
//	logger.Info("Executing neta")

// WithBentoID adds bento_id to the logger context.
// The itamae will use this to tag all logs from a specific workflow.
//
// The bento_id helps identify which workflow instance generated a log,
// which is critical for debugging parallel workflow executions.
func WithBentoID(logger *Logger, bentoID string) *Logger {
	return logger.With("bento_id", bentoID)
}

// WithNetaID adds neta_id to the logger context.
// Used to track which neta (ingredient) is currently executing.
//
// This helps identify exactly which step in a workflow generated a log,
// making it easier to debug complex workflows with many neta.
func WithNetaID(logger *Logger, netaID string) *Logger {
	return logger.With("neta_id", netaID)
}

// WithNetaType adds neta_type to the logger context.
// Used to tag logs with the type of neta being executed.
//
// Example neta types: "http-request", "shell-command", "edit-fields"
// This helps filter and analyze logs by neta type.
func WithNetaType(logger *Logger, netaType string) *Logger {
	return logger.With("neta_type", netaType)
}
