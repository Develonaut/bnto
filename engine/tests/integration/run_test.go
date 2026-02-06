// Package integration provides end-to-end integration tests for bento execution.
//
// These tests simulate real user workflows by executing actual bentos
// from the examples/ folder with real data.
package integration

import (
	"context"
	"encoding/json"
	"os"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/engine"
	"github.com/Develonaut/bento/pkg/node"
)

// TestUserSimulation_ItemFolderWorkflow simulates a real user workflow:
// 1. Load actual example bento from examples/ folder
// 2. Validate the bento (optional)
// 3. Execute the bento
// 4. Verify results
//
// This tests the complete end-to-end user experience using the ACTUAL
// example files that users see in documentation.
func TestUserSimulation_ItemFolderWorkflow(t *testing.T) {
	t.Log("=== Simulating Real User Workflow ===")
	t.Log("User goal: Create folder structure from test-data.csv")
	t.Log("")

	// User starts at project root (simulate by changing directory)
	originalWd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	// Change to project root (two levels up from tests/integration)
	projectRoot := "../../"
	if err := os.Chdir(projectRoot); err != nil {
		t.Fatalf("Failed to change to project root: %v", err)
	}
	defer func() { _ = os.Chdir(originalWd) }() // Restore original directory after test

	// Step 1: User loads the example bento
	t.Log("Step 1: User runs 'bento run examples/csv-to-folders.bento.json'")
	bentoPath := "examples/csv-to-folders.bento.json"

	// Load the actual example bento file
	data, err := os.ReadFile(bentoPath)
	if err != nil {
		t.Fatalf("Failed to load example bento: %v", err)
	}

	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		t.Fatalf("Failed to parse example bento: %v", err)
	}

	t.Logf("  ✓ Loaded bento: %s", def.Name)
	t.Log("")

	// Step 2: User validates the bento
	t.Log("Step 2: User validates bento with 'bento validate'")
	t.Log("  ✓ Validation passed (skipped in test)")
	t.Log("")

	// Step 3: User runs the bento
	t.Log("Step 3: Bento execution begins")
	t.Log("")

	// Create registry and engine
	p := createRegistry()
	eng := engine.New(p, nil)

	// Execute (this is what happens when user runs 'bento run')
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	startTime := time.Now()
	result, err := eng.Serve(ctx, &def)
	executionTime := time.Since(startTime)

	// Verify execution succeeded
	if err != nil {
		t.Fatalf("Step 3 FAILED: Execution error: %v", err)
	}

	t.Log("")
	t.Logf("Step 3: ✓ Execution completed in %s", executionTime)
	t.Logf("  Nodes executed: %d", result.NodesExecuted)
	t.Logf("  Status: %s", result.Status)
	t.Log("")

	// Step 4: User verifies the output
	t.Log("Step 4: User checks that folders were created in output/")

	expectedItems := []string{
		"Test Item A",
		"Test Item B",
		"Test Item C",
	}

	// The example bento creates folders in output/ (from project root)
	outputDir := "output"

	for _, itemName := range expectedItems {
		folderPath := outputDir + "/" + itemName
		if _, err := os.Stat(folderPath); os.IsNotExist(err) {
			t.Errorf("Step 4 FAILED: Folder not found: %s", folderPath)
		} else {
			t.Logf("  ✓ Found folder: %s", itemName)
		}
	}

	t.Log("")
	t.Log("=== User Workflow Complete ===")
	t.Logf("✓ User successfully created %d item folders from CSV", len(expectedItems))
	t.Logf("✓ Workflow completed in %s", executionTime)
	t.Log("✓ All folders verified and accessible")
}

// TestUserSimulation_ItemFolderWithOverlaysWorkflow simulates a real user workflow:
// 1. Load actual example bento from examples/ folder
// 2. Execute the bento with indexed overlay copying
// 3. Verify folders and overlay files were created correctly
//
// This tests the complete end-to-end workflow for creating folders from CSV
// and copying indexed overlay files (0.png, 1.png, etc.) into each folder.
func TestUserSimulation_ItemFolderWithOverlaysWorkflow(t *testing.T) {
	t.Log("=== Simulating Real User Workflow (CSV + Overlays) ===")
	t.Log("User goal: Create folder structure from test-data.csv with overlay images")
	t.Log("")

	// User starts at project root (simulate by changing directory)
	originalWd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	// Change to project root (two levels up from tests/integration)
	projectRoot := "../../"
	if err := os.Chdir(projectRoot); err != nil {
		t.Fatalf("Failed to change to project root: %v", err)
	}
	defer func() { _ = os.Chdir(originalWd) }() // Restore original directory after test

	// Step 1: User loads the example bento
	t.Log("Step 1: User runs 'bento run examples/csv-to-folders-with-overlays.bento.json'")
	bentoPath := "examples/csv-to-folders-with-overlays.bento.json"

	// Load the actual example bento file
	data, err := os.ReadFile(bentoPath)
	if err != nil {
		t.Fatalf("Failed to load example bento: %v", err)
	}

	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		t.Fatalf("Failed to parse example bento: %v", err)
	}

	t.Logf("  ✓ Loaded bento: %s", def.Name)
	t.Log("")

	// Step 2: User validates the bento
	t.Log("Step 2: User validates bento with 'bento validate'")
	t.Log("  ✓ Validation passed (skipped in test)")
	t.Log("")

	// Step 3: User runs the bento
	t.Log("Step 3: Bento execution begins")
	t.Log("")

	// Create registry and engine
	p := createRegistry()
	eng := engine.New(p, nil)

	// Execute (this is what happens when user runs 'bento run')
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	startTime := time.Now()
	result, err := eng.Serve(ctx, &def)
	executionTime := time.Since(startTime)

	// Verify execution succeeded
	if err != nil {
		t.Fatalf("Step 3 FAILED: Execution error: %v", err)
	}

	t.Log("")
	t.Logf("Step 3: ✓ Execution completed in %s", executionTime)
	t.Logf("  Nodes executed: %d", result.NodesExecuted)
	t.Logf("  Status: %s", result.Status)
	t.Log("")

	// Step 4: User verifies the output
	t.Log("Step 4: User checks that folders and overlay files were created in output/")

	expectedItems := []struct {
		name         string
		overlayIndex int
	}{
		{"Test Item A", 0},
		{"Test Item B", 1},
		{"Test Item C", 2},
	}

	// The example bento creates folders in output/ (from project root)
	outputDir := "output"

	for _, item := range expectedItems {
		folderPath := outputDir + "/" + item.name
		overlayPath := folderPath + "/overlay.png"

		// Check folder exists
		if _, err := os.Stat(folderPath); os.IsNotExist(err) {
			t.Errorf("Step 4 FAILED: Folder not found: %s", folderPath)
			continue
		}
		t.Logf("  ✓ Found folder: %s", item.name)

		// Check overlay file exists
		if _, err := os.Stat(overlayPath); os.IsNotExist(err) {
			t.Errorf("Step 4 FAILED: Overlay file not found: %s", overlayPath)
			continue
		}
		t.Logf("  ✓ Found overlay: %s/overlay.png (from %d.png)", item.name, item.overlayIndex)

		// Verify overlay file has content (not empty)
		fileInfo, err := os.Stat(overlayPath)
		if err != nil {
			t.Errorf("Step 4 FAILED: Cannot stat overlay file: %v", err)
			continue
		}
		if fileInfo.Size() == 0 {
			t.Errorf("Step 4 FAILED: Overlay file is empty: %s", overlayPath)
			continue
		}
		t.Logf("  ✓ Overlay file size: %d bytes", fileInfo.Size())
	}

	t.Log("")
	t.Log("=== User Workflow Complete ===")
	t.Logf("✓ User successfully created %d item folders with overlay images", len(expectedItems))
	t.Logf("✓ Workflow completed in %s", executionTime)
	t.Log("✓ All folders and overlay files verified")
}
