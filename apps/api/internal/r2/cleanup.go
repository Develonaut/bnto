package r2

import (
	"context"
	"fmt"
	"log"
)

// CleanupSession deletes all input files for a session from R2.
// Objects live at uploads/{sessionID}/.
func CleanupSession(ctx context.Context, store ObjectStore, sessionID string) error {
	prefix := fmt.Sprintf("uploads/%s/", sessionID)
	return cleanupPrefix(ctx, store, prefix)
}

// CleanupOutputs deletes all output files for an execution from R2.
// Objects live at executions/{executionID}/output/.
func CleanupOutputs(ctx context.Context, store ObjectStore, executionID string) error {
	prefix := fmt.Sprintf("executions/%s/output/", executionID)
	return cleanupPrefix(ctx, store, prefix)
}

// CleanupSessionBestEffort deletes session input files, logging errors
// instead of returning them. Use after a successful download when cleanup
// failure should not abort the execution.
func CleanupSessionBestEffort(ctx context.Context, store ObjectStore, sessionID string) {
	if err := CleanupSession(ctx, store, sessionID); err != nil {
		log.Printf("warning: failed to clean up session %s: %v", sessionID, err)
	}
}

// cleanupPrefix lists all objects under prefix and deletes them one by one.
func cleanupPrefix(ctx context.Context, store ObjectStore, prefix string) error {
	keys, err := store.ListObjects(ctx, prefix)
	if err != nil {
		return fmt.Errorf("listing objects for cleanup %s: %w", prefix, err)
	}

	for _, key := range keys {
		if err := ctx.Err(); err != nil {
			return fmt.Errorf("cleanup cancelled: %w", err)
		}
		if err := store.DeleteObject(ctx, key); err != nil {
			return fmt.Errorf("deleting %s: %w", key, err)
		}
	}

	return nil
}
