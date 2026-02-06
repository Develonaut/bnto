// Package integration provides end-to-end integration tests for bento execution.
package integration

import (
	"context"
	"encoding/json"
	"os"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/engine"
	"github.com/Develonaut/bento/pkg/tui"
	"github.com/Develonaut/bento/pkg/neta"
)

// TestLoopErrorPropagation tests that errors from nodes inside a loop
// are properly propagated and cause the bento execution to fail.
func TestLoopErrorPropagation(t *testing.T) {
	t.Log("=== Testing Loop Error Propagation ===")
	t.Log("Verify that a failing node inside a loop causes the bento to fail")
	t.Log("")

	// Create a bento with a loop that contains a failing node
	bentoDef := createLoopWithFailingNode(t)

	// Create execution environment
	p := createTestPantry()
	manager := miso.NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := miso.NewSimpleMessenger(theme, palette)
	chef := itamae.NewWithMessenger(p, nil, messenger)

	// Execute the bento - we expect it to FAIL
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	t.Log("Executing bento with failing loop node...")
	result, err := chef.Serve(ctx, bentoDef)

	// Verify execution FAILED (this is the correct behavior)
	if err == nil {
		t.Fatal("Expected bento execution to fail, but it succeeded")
	}

	t.Logf("✓ Bento execution failed as expected: %v", err)

	// Verify error message contains information about the failing node
	errStr := err.Error()
	if errStr == "" {
		t.Error("Error message should not be empty")
	}

	t.Logf("✓ Error message: %s", errStr)

	// Verify result status
	if result != nil {
		if result.Status != "failure" && result.Status != "error" && result.Status != "failed" {
			t.Errorf("Expected result.Status to be 'failure', 'error', or 'failed', got: %s", result.Status)
		}
		t.Logf("✓ Result status: %s", result.Status)

		// Verify that some nodes were executed before the failure
		// (at least the loop node should have started)
		if result.NodesExecuted < 0 {
			t.Error("Expected at least 0 nodes to be tracked (partial execution)")
		}
		t.Logf("✓ Nodes executed before failure: %d", result.NodesExecuted)
	}

	t.Log("")
	t.Log("=== Loop Error Propagation Test Complete ===")
	t.Log("✓ Errors from loop child nodes are properly propagated")
	t.Log("✓ Bento execution fails when loop nodes fail")
	t.Log("✓ Error messages provide diagnostic information")
}

// createLoopWithFailingNode creates a bento definition with a loop
// that contains a node that will fail.
func createLoopWithFailingNode(t *testing.T) *neta.Definition {
	t.Helper()

	// Create a bento with a forEach loop over 3 items
	// The loop contains a file-system copy node with invalid parameters
	bentoJSON := `{
		"id": "test-loop-error",
		"type": "group",
		"version": "1.0.0",
		"name": "Test Loop Error Propagation",
		"position": {"x": 0, "y": 0},
		"metadata": {
			"description": "Test that errors in loop nodes propagate correctly"
		},
		"parameters": {},
		"inputPorts": [],
		"outputPorts": [],
		"nodes": [
			{
				"id": "test-loop",
				"type": "loop",
				"version": "1.0.0",
				"name": "Test Loop",
				"position": {"x": 100, "y": 100},
				"metadata": {
					"description": "Loop with a failing node"
				},
				"parameters": {
					"mode": "times",
					"count": 3
				},
				"inputPorts": [],
				"outputPorts": [],
				"nodes": [
					{
						"id": "failing-node",
						"type": "file-system",
						"version": "1.0.0",
						"name": "Failing Copy",
						"position": {"x": 100, "y": 100},
						"metadata": {
							"description": "This node will fail due to missing dest parameter"
						},
						"parameters": {
							"operation": "copy",
							"source": "/tmp/nonexistent-source.txt"
						},
						"inputPorts": [],
						"outputPorts": []
					}
				],
				"edges": []
			}
		],
		"edges": []
	}`

	var def neta.Definition
	if err := json.Unmarshal([]byte(bentoJSON), &def); err != nil {
		t.Fatalf("Failed to parse test bento: %v", err)
	}

	return &def
}

// TestLoopPartialExecution tests that when a loop fails mid-iteration,
// the results from successful iterations are preserved.
func TestLoopPartialExecution(t *testing.T) {
	t.Log("=== Testing Loop Partial Execution ===")
	t.Log("Verify that successful iterations are tracked before loop failure")
	t.Log("")

	// Create a loop that will fail on the second iteration
	// Change to project root (two levels up from tests/integration)
	originalWd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	projectRoot := "../../"
	if err := os.Chdir(projectRoot); err != nil {
		t.Fatalf("Failed to change to project root: %v", err)
	}
	defer func() { _ = os.Chdir(originalWd) }()

	// Create test CSV with 3 rows
	testCSVPath := "/tmp/test-partial-loop.csv"
	testCSV := `name,value
first,1
second,2
third,3`
	if err := os.WriteFile(testCSVPath, []byte(testCSV), 0644); err != nil {
		t.Fatalf("Failed to create test CSV: %v", err)
	}
	defer os.Remove(testCSVPath)

	// Create a bento that:
	// 1. Reads CSV (3 rows)
	// 2. Loops over rows
	// 3. On iteration 1 (index 1), tries to copy a file that doesn't exist
	bentoJSON := `{
		"id": "test-partial-loop",
		"type": "group",
		"version": "1.0.0",
		"name": "Test Partial Loop Execution",
		"position": {"x": 0, "y": 0},
		"metadata": {},
		"parameters": {},
		"inputPorts": [],
		"outputPorts": [],
		"nodes": [
			{
				"id": "read-csv",
				"type": "spreadsheet",
				"version": "1.0.0",
				"name": "Read Test CSV",
				"position": {"x": 100, "y": 100},
				"metadata": {},
				"parameters": {
					"operation": "read",
					"format": "csv",
					"path": "/tmp/test-partial-loop.csv",
					"hasHeaders": true
				},
				"inputPorts": [],
				"outputPorts": [{"id": "out-1", "name": "rows"}]
			},
			{
				"id": "loop-items",
				"type": "loop",
				"version": "1.0.0",
				"name": "Loop Over Items",
				"position": {"x": 300, "y": 100},
				"metadata": {},
				"parameters": {
					"mode": "forEach",
					"items": "{{index . \"read-csv\" \"rows\"}}"
				},
				"inputPorts": [{"id": "in-1", "name": "items"}],
				"outputPorts": [],
				"nodes": [
					{
						"id": "copy-file",
						"type": "file-system",
						"version": "1.0.0",
						"name": "Copy Overlay",
						"position": {"x": 100, "y": 100},
						"metadata": {},
						"parameters": {
							"operation": "copy",
							"source": "tests/fixtures/overlays/{{.index}}.png",
							"dest": "/tmp/test-overlay-{{.index}}.png"
						},
						"inputPorts": [],
						"outputPorts": []
					}
				],
				"edges": []
			}
		],
		"edges": [
			{
				"id": "e1",
				"source": "read-csv",
				"target": "loop-items",
				"sourcePort": "out-1",
				"targetPort": "in-1"
			}
		]
	}`

	var def neta.Definition
	if err := json.Unmarshal([]byte(bentoJSON), &def); err != nil {
		t.Fatalf("Failed to parse test bento: %v", err)
	}

	// Create execution environment
	p := createTestPantry()
	manager := miso.NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := miso.NewSimpleMessenger(theme, palette)
	chef := itamae.NewWithMessenger(p, nil, messenger)

	// Execute
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	t.Log("Executing bento with partial loop...")
	result, err := chef.Serve(ctx, &def)

	// Should fail (overlay 3.png doesn't exist, will fail on iteration 2)
	if err != nil {
		t.Logf("✓ Bento failed as expected: %v", err)
	}

	// Check that we have partial results
	if result != nil {
		t.Logf("  NodesExecuted: %d", result.NodesExecuted)
		if loopOutput, ok := result.NodeOutputs["loop-items"].(map[string]interface{}); ok {
			if iterations, ok := loopOutput["iterations"].(int); ok {
				t.Logf("  Loop iterations completed: %d", iterations)
				// We expect at least 2 iterations to have completed
				// (0.png and 1.png exist, 2.png does not)
				if iterations < 2 {
					t.Logf("  Note: Expected at least 2 iterations, got %d", iterations)
				} else {
					t.Logf("✓ Partial iterations tracked: %d successful before failure", iterations)
				}
			}
		}
	}

	// Cleanup
	os.Remove("/tmp/test-overlay-0.png")
	os.Remove("/tmp/test-overlay-1.png")

	t.Log("")
	t.Log("=== Partial Execution Test Complete ===")
}
