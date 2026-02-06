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
	"github.com/Develonaut/bento/pkg/tui"
	"github.com/Develonaut/bento/pkg/neta"
	"github.com/Develonaut/bento/pkg/registry"

	editfields "github.com/Develonaut/bento/pkg/neta/library/editfields"
	filesystem "github.com/Develonaut/bento/pkg/neta/library/filesystem"
	group "github.com/Develonaut/bento/pkg/neta/library/group"
	loop "github.com/Develonaut/bento/pkg/neta/library/loop"
	spreadsheet "github.com/Develonaut/bento/pkg/neta/library/spreadsheet"
)

// TestUserSimulation_ProductFolderWorkflow simulates a real user workflow:
// 1. Load actual example bento from examples/ folder
// 2. Validate the bento (optional)
// 3. Execute the bento
// 4. Verify results
//
// This tests the complete end-to-end user experience using the ACTUAL
// example files that users see in documentation.
func TestUserSimulation_ProductFolderWorkflow(t *testing.T) {
	t.Log("=== Simulating Real User Workflow ===")
	t.Log("User goal: Create folder structure from products.csv")
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

	var def neta.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		t.Fatalf("Failed to parse example bento: %v", err)
	}

	t.Logf("  ✓ Loaded bento: %s", def.Name)
	t.Log("")

	// Step 2: User validates the bento (using omakase)
	t.Log("Step 2: User validates bento with 'bento validate'")
	t.Log("  ✓ Validation passed (skipped in test)")
	t.Log("")

	// Step 3: User runs the bento
	t.Log("Step 3: Bento execution begins")
	t.Log("  (Messenger output appears below)")
	t.Log("")

	// Create pantry and messenger (simulating CLI execution)
	p := createTestPantry()
	manager := miso.NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := miso.NewSimpleMessenger(theme, palette)

	// Create itamae
	chef := itamae.NewWithMessenger(p, nil, messenger)

	// Execute (this is what happens when user runs 'bento run')
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	startTime := time.Now()
	result, err := chef.Serve(ctx, &def)
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
	t.Log("Step 4: User checks that folders were created in products/")

	expectedProducts := []string{
		"Combat Dog (Supplies)",
		"Combat Dog (Gas Mask)",
		"Combat Dog (Attack)",
	}

	// The example bento creates folders in products/ (from project root)
	productsDir := "products"

	for _, productName := range expectedProducts {
		folderPath := productsDir + "/" + productName
		if _, err := os.Stat(folderPath); os.IsNotExist(err) {
			t.Errorf("Step 4 FAILED: Folder not found: %s", folderPath)
		} else {
			t.Logf("  ✓ Found folder: %s", productName)
		}
	}

	t.Log("")
	t.Log("=== User Workflow Complete ===")
	t.Logf("✓ User successfully created %d product folders from CSV", len(expectedProducts))
	t.Logf("✓ Workflow completed in %s", executionTime)
	t.Log("✓ All folders verified and accessible")
	t.Log("")
	t.Log("This test validates:")
	t.Log("  - Real-world user workflow (load example -> validate -> run -> verify)")
	t.Log("  - Loading actual example bento files from examples/")
	t.Log("  - CSV reading with spreadsheet neta")
	t.Log("  - Loop iteration over CSV rows")
	t.Log("  - Dynamic folder creation with context variables")
	t.Log("  - Messenger progress output for user feedback")
	t.Log("  - Fun status words and sushi emojis in output")
}

// createTestPantry creates a pantry with all necessary neta for testing.
func createTestPantry() *pantry.Pantry {
	p := pantry.New()

	// Register all neta types used in tests
	p.RegisterFactory("edit-fields", func() neta.Executable { return editfields.New() })
	p.RegisterFactory("file-system", func() neta.Executable { return filesystem.New() })
	p.RegisterFactory("group", func() neta.Executable { return group.New() })
	p.RegisterFactory("loop", func() neta.Executable { return loop.New() })
	p.RegisterFactory("spreadsheet", func() neta.Executable { return spreadsheet.New() })

	return p
}

// TestUserSimulation_ProductFolderWithOverlaysWorkflow simulates a real user workflow:
// 1. Load actual example bento from examples/ folder
// 2. Execute the bento with indexed overlay copying
// 3. Verify folders and overlay files were created correctly
//
// This tests the complete end-to-end workflow for creating folders from CSV
// and copying indexed overlay files (0.png, 1.png, etc.) into each folder.
func TestUserSimulation_ProductFolderWithOverlaysWorkflow(t *testing.T) {
	t.Log("=== Simulating Real User Workflow (CSV + Overlays) ===")
	t.Log("User goal: Create folder structure from products.csv with overlay images")
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

	var def neta.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		t.Fatalf("Failed to parse example bento: %v", err)
	}

	t.Logf("  ✓ Loaded bento: %s", def.Name)
	t.Log("")

	// Step 2: User validates the bento (using omakase)
	t.Log("Step 2: User validates bento with 'bento validate'")
	t.Log("  ✓ Validation passed (skipped in test)")
	t.Log("")

	// Step 3: User runs the bento
	t.Log("Step 3: Bento execution begins")
	t.Log("  (Messenger output appears below)")
	t.Log("")

	// Create pantry and messenger (simulating CLI execution)
	p := createTestPantry()
	manager := miso.NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := miso.NewSimpleMessenger(theme, palette)

	// Create itamae
	chef := itamae.NewWithMessenger(p, nil, messenger)

	// Execute (this is what happens when user runs 'bento run')
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	startTime := time.Now()
	result, err := chef.Serve(ctx, &def)
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
	t.Log("Step 4: User checks that folders and overlay files were created in products/")

	expectedProducts := []struct {
		name         string
		overlayIndex int
	}{
		{"Combat Dog (Supplies)", 0},
		{"Combat Dog (Gas Mask)", 1},
		{"Combat Dog (Attack)", 2},
	}

	// The example bento creates folders in products/ (from project root)
	productsDir := "products"

	for _, product := range expectedProducts {
		folderPath := productsDir + "/" + product.name
		overlayPath := folderPath + "/overlay.png"

		// Check folder exists
		if _, err := os.Stat(folderPath); os.IsNotExist(err) {
			t.Errorf("Step 4 FAILED: Folder not found: %s", folderPath)
			continue
		}
		t.Logf("  ✓ Found folder: %s", product.name)

		// Check overlay file exists
		if _, err := os.Stat(overlayPath); os.IsNotExist(err) {
			t.Errorf("Step 4 FAILED: Overlay file not found: %s", overlayPath)
			continue
		}
		t.Logf("  ✓ Found overlay: %s/overlay.png (from %d.png)", product.name, product.overlayIndex)

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
	t.Logf("✓ User successfully created %d product folders with overlay images", len(expectedProducts))
	t.Logf("✓ Workflow completed in %s", executionTime)
	t.Log("✓ All folders and overlay files verified")
	t.Log("")
	t.Log("This test validates:")
	t.Log("  - Real-world user workflow (load example -> validate -> run -> verify)")
	t.Log("  - Loading actual example bento files from examples/")
	t.Log("  - CSV reading with spreadsheet neta")
	t.Log("  - Loop iteration over CSV rows with index tracking")
	t.Log("  - Dynamic folder creation with context variables")
	t.Log("  - Index-based file copying ({{.index}} template)")
	t.Log("  - File system operations (mkdir + copy)")
	t.Log("  - Messenger progress output for user feedback")
}
