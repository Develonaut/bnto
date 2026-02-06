package integration

import (
	"path/filepath"
	"strings"
	"testing"
)

// TestCSVReader_Integration tests the CSV reader bento end-to-end.
// This is the first real integration test validating that:
// - Spreadsheet node works with real CSV files
// - Bento definitions load and execute correctly
// - Test infrastructure from Phase 8.1 is functional
func TestCSVReader_Integration(t *testing.T) {
	// Path is relative to project root since RunBento sets working directory
	bentoPath := filepath.Join("examples", "csv-reader.bento.json")

	// Run the CSV reader bento
	output, err := RunBento(t, bentoPath, nil)
	if err != nil {
		t.Fatalf("RunBento failed: %v\nOutput: %s", err, output)
	}

	// Verify successful execution by checking output contains success messages
	if !strings.Contains(output, "Delicious! Bento executed successfully") {
		t.Errorf("Expected success message in output. Got: %s", output)
	}

	// Verify the bento executed at least 1 node (the CSV reader)
	if !strings.Contains(output, "1 nodes executed") {
		t.Errorf("Expected '1 nodes executed' in output. Got: %s", output)
	}

	// Verify the CSV reader node was executed
	if !strings.Contains(output, "node_id=read-csv") {
		t.Errorf("Expected 'node_id=read-csv' in output. Got: %s", output)
	}

	// Verify the node type is spreadsheet
	if !strings.Contains(output, "node_type=spreadsheet") {
		t.Errorf("Expected 'node_type=spreadsheet' in output. Got: %s", output)
	}

	// Verify the node completed successfully
	if !strings.Contains(output, "✓ Node completed") {
		t.Errorf("Expected '✓ Node completed' in output. Got: %s", output)
	}

	t.Log("CSV Reader bento executed successfully")
	t.Log("Validated:")
	t.Log("  - Spreadsheet node loads and executes")
	t.Log("  - CSV file (tests/fixtures/products-test.csv) is read")
	t.Log("  - Bento definition parses correctly")
	t.Log("  - Integration test infrastructure works")
}
