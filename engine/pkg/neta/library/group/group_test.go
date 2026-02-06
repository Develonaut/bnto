package group_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bento/pkg/neta/library/group"
)

// TestGroup_SequentialExecution verifies sequential execution of child neta.
//
// In sequential mode, the group executes child neta one after another,
// passing the accumulated context from each neta to the next.
//
// This is the most common execution mode for workflows.
func TestGroup_SequentialExecution(t *testing.T) {
	ctx := context.Background()

	grp := group.New()

	// For this test, we'll verify the group structure
	// Full execution will be handled by itamae in later phases
	params := map[string]interface{}{
		"mode": "sequential",
		"nodes": []interface{}{
			map[string]interface{}{
				"id":   "node-1",
				"type": "edit-fields",
			},
			map[string]interface{}{
				"id":   "node-2",
				"type": "edit-fields",
			},
		},
	}

	result, err := grp.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Verify result structure
	output, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	// Group should return metadata about execution
	if output["mode"] != "sequential" {
		t.Errorf("mode = %v, want sequential", output["mode"])
	}
}

// TestGroup_ParallelExecution verifies parallel execution mode.
//
// In parallel mode, child neta execute concurrently. This is useful for
// independent operations like fetching multiple API endpoints simultaneously.
func TestGroup_ParallelExecution(t *testing.T) {
	ctx := context.Background()

	grp := group.New()

	params := map[string]interface{}{
		"mode": "parallel",
		"nodes": []interface{}{
			map[string]interface{}{
				"id":   "node-1",
				"type": "http-request",
			},
			map[string]interface{}{
				"id":   "node-2",
				"type": "http-request",
			},
		},
	}

	result, err := grp.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	if output["mode"] != "parallel" {
		t.Errorf("mode = %v, want parallel", output["mode"])
	}
}

// TestGroup_EmptyNodes verifies behavior with no child nodes.
//
// An empty group should execute successfully and return empty results.
func TestGroup_EmptyNodes(t *testing.T) {
	ctx := context.Background()

	grp := group.New()

	params := map[string]interface{}{
		"mode":  "sequential",
		"nodes": []interface{}{},
	}

	result, err := grp.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	if output["executed"] != 0 {
		t.Errorf("executed = %v, want 0", output["executed"])
	}
}

// TestGroup_MissingMode tests default mode when not specified.
//
// Should default to sequential execution.
func TestGroup_MissingMode(t *testing.T) {
	ctx := context.Background()

	grp := group.New()

	params := map[string]interface{}{
		"nodes": []interface{}{
			map[string]interface{}{
				"id":   "node-1",
				"type": "edit-fields",
			},
		},
	}

	result, err := grp.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	// Should default to sequential
	if output["mode"] != "sequential" {
		t.Errorf("mode = %v, want sequential (default)", output["mode"])
	}
}

// TestGroup_InvalidMode tests error handling for invalid execution mode.
//
// Only "sequential" and "parallel" are valid modes.
func TestGroup_InvalidMode(t *testing.T) {
	ctx := context.Background()

	grp := group.New()

	params := map[string]interface{}{
		"mode": "invalid-mode",
		"nodes": []interface{}{
			map[string]interface{}{
				"id":   "node-1",
				"type": "edit-fields",
			},
		},
	}

	_, err := grp.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid mode")
	}
}

// TestGroup_NestedGroups verifies groups can contain other groups.
//
// This is critical for complex workflows with hierarchical structure.
func TestGroup_NestedGroups(t *testing.T) {
	ctx := context.Background()

	grp := group.New()

	params := map[string]interface{}{
		"mode": "sequential",
		"nodes": []interface{}{
			map[string]interface{}{
				"id":   "node-1",
				"type": "edit-fields",
			},
			map[string]interface{}{
				"id":   "group-1",
				"type": "group",
				"parameters": map[string]interface{}{
					"mode": "parallel",
					"nodes": []interface{}{
						map[string]interface{}{
							"id":   "nested-1",
							"type": "http-request",
						},
					},
				},
			},
		},
	}

	result, err := grp.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	if output["mode"] != "sequential" {
		t.Errorf("mode = %v, want sequential", output["mode"])
	}
}

// TestGroup_ContextCancellation verifies proper context cancellation handling.
//
// When workflow is cancelled, group execution should stop gracefully.
func TestGroup_ContextCancellation(t *testing.T) {
	// Create cancellable context
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	grp := group.New()

	params := map[string]interface{}{
		"mode": "sequential",
		"nodes": []interface{}{
			map[string]interface{}{
				"id":   "node-1",
				"type": "edit-fields",
			},
		},
	}

	_, err := grp.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error when context is cancelled")
	}

	// Verify it's a context cancellation error
	if err != context.Canceled {
		t.Errorf("Expected context.Canceled error, got: %v", err)
	}
}
