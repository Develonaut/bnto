package spreadsheet_test

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/Develonaut/bento/pkg/neta/library/spreadsheet"
)

// TestSpreadsheet_ReadCSV tests reading CSV with headers
// CRITICAL FOR PHASE 8: Read product CSV
func TestSpreadsheet_ReadCSV(t *testing.T) {
	ctx := context.Background()

	// Create test CSV
	csvContent := `sku,name,description
PROD-001,Product A,Description A
PROD-002,Product B,Description B
PROD-003,Product C,Description C`

	tmpfile, err := os.CreateTemp("", "test-*.csv")
	if err != nil {
		t.Fatal(err)
	}
	defer os.Remove(tmpfile.Name())

	if _, err := tmpfile.WriteString(csvContent); err != nil {
		t.Fatal(err)
	}
	tmpfile.Close()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "csv",
		"path":      tmpfile.Name(),
	}

	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	rows, ok := output["rows"].([]map[string]interface{})
	if !ok {
		t.Fatal("Expected rows to be []map[string]interface{}")
	}

	if len(rows) != 3 {
		t.Errorf("len(rows) = %d, want 3", len(rows))
	}

	if rows[0]["sku"] != "PROD-001" {
		t.Errorf("rows[0].sku = %v, want PROD-001", rows[0]["sku"])
	}

	if rows[0]["name"] != "Product A" {
		t.Errorf("rows[0].name = %v, want Product A", rows[0]["name"])
	}

	if rows[2]["sku"] != "PROD-003" {
		t.Errorf("rows[2].sku = %v, want PROD-003", rows[2]["sku"])
	}
}

// TestSpreadsheet_WriteCSV tests writing CSV
func TestSpreadsheet_WriteCSV(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-write.csv")
	defer os.Remove(tmpfile)

	rows := []map[string]interface{}{
		{"sku": "PROD-001", "name": "Product A"},
		{"sku": "PROD-002", "name": "Product B"},
	}

	params := map[string]interface{}{
		"operation": "write",
		"format":    "csv",
		"path":      tmpfile,
		"rows":      rows,
	}

	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["written"] != 2 {
		t.Errorf("written = %v, want 2", output["written"])
	}

	// Verify file exists
	if _, err := os.Stat(tmpfile); os.IsNotExist(err) {
		t.Error("Output file not created")
	}
}

// TestSpreadsheet_ReadExcel tests reading Excel files
func TestSpreadsheet_ReadExcel(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	// Create a test Excel file
	tmpfile := filepath.Join(os.TempDir(), "test.xlsx")
	defer os.Remove(tmpfile)

	// First write an Excel file
	rows := []map[string]interface{}{
		{"sku": "PROD-001", "name": "Product A", "price": "10.50"},
		{"sku": "PROD-002", "name": "Product B", "price": "20.00"},
	}

	writeParams := map[string]interface{}{
		"operation": "write",
		"format":    "excel",
		"path":      tmpfile,
		"rows":      rows,
	}

	_, err := ss.Execute(ctx, writeParams)
	if err != nil {
		t.Fatalf("Write failed: %v", err)
	}

	// Now read it back
	readParams := map[string]interface{}{
		"operation": "read",
		"format":    "excel",
		"path":      tmpfile,
	}

	result, err := ss.Execute(ctx, readParams)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	rows2, ok := output["rows"].([]map[string]interface{})
	if !ok {
		t.Fatal("Expected rows to be []map[string]interface{}")
	}

	if len(rows2) != 2 {
		t.Errorf("len(rows) = %d, want 2", len(rows2))
	}

	if rows2[0]["sku"] != "PROD-001" {
		t.Errorf("rows[0].sku = %v, want PROD-001", rows2[0]["sku"])
	}
}

// TestSpreadsheet_EmptyCSV tests reading empty CSV file
func TestSpreadsheet_EmptyCSV(t *testing.T) {
	ctx := context.Background()

	tmpfile, err := os.CreateTemp("", "test-empty-*.csv")
	if err != nil {
		t.Fatal(err)
	}
	defer os.Remove(tmpfile.Name())
	tmpfile.Close()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "csv",
		"path":      tmpfile.Name(),
	}

	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	rows := output["rows"].([]map[string]interface{})

	if len(rows) != 0 {
		t.Errorf("len(rows) = %d, want 0", len(rows))
	}
}

// TestSpreadsheet_MalformedCSV tests handling malformed CSV
func TestSpreadsheet_MalformedCSV(t *testing.T) {
	ctx := context.Background()

	// Create malformed CSV (mismatched columns)
	csvContent := `sku,name,description
PROD-001,Product A
PROD-002,Product B,Description B,Extra`

	tmpfile, err := os.CreateTemp("", "test-malformed-*.csv")
	if err != nil {
		t.Fatal(err)
	}
	defer os.Remove(tmpfile.Name())

	if _, err := tmpfile.WriteString(csvContent); err != nil {
		t.Fatal(err)
	}
	tmpfile.Close()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "csv",
		"path":      tmpfile.Name(),
	}

	// Should still read successfully, handling missing/extra columns
	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	rows := output["rows"].([]map[string]interface{})

	if len(rows) != 2 {
		t.Errorf("len(rows) = %d, want 2", len(rows))
	}
}

// TestSpreadsheet_CSVWithHeaders tests CSV with custom headers
func TestSpreadsheet_CSVWithHeaders(t *testing.T) {
	ctx := context.Background()

	csvContent := `SKU,Product Name,Price
PROD-001,Widget,10.50
PROD-002,Gadget,20.00`

	tmpfile, err := os.CreateTemp("", "test-headers-*.csv")
	if err != nil {
		t.Fatal(err)
	}
	defer os.Remove(tmpfile.Name())

	if _, err := tmpfile.WriteString(csvContent); err != nil {
		t.Fatal(err)
	}
	tmpfile.Close()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "csv",
		"path":      tmpfile.Name(),
	}

	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	rows := output["rows"].([]map[string]interface{})

	if len(rows) != 2 {
		t.Errorf("len(rows) = %d, want 2", len(rows))
	}

	// Headers should be preserved
	if rows[0]["SKU"] != "PROD-001" {
		t.Errorf("rows[0].SKU = %v, want PROD-001", rows[0]["SKU"])
	}
}

// TestSpreadsheet_InvalidOperation tests error handling
func TestSpreadsheet_InvalidOperation(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "invalid",
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid operation, got nil")
	}
}

// TestSpreadsheet_MissingPath tests error handling for missing path
func TestSpreadsheet_MissingPath(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "csv",
		// Missing path parameter
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing path, got nil")
	}
}
