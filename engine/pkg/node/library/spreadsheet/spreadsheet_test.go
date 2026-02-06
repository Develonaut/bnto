package spreadsheet_test

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/Develonaut/bento/pkg/node/library/spreadsheet"
	"github.com/xuri/excelize/v2"
)

// TestSpreadsheet_ReadCSV tests reading CSV with headers.
func TestSpreadsheet_ReadCSV(t *testing.T) {
	ctx := context.Background()

	csvContent := `id,name,description
REC-001,Alpha,Description A
REC-002,Beta,Description B
REC-003,Gamma,Description C`

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

	if rows[0]["id"] != "REC-001" {
		t.Errorf("rows[0].id = %v, want REC-001", rows[0]["id"])
	}

	if rows[0]["name"] != "Alpha" {
		t.Errorf("rows[0].name = %v, want Alpha", rows[0]["name"])
	}

	if rows[2]["id"] != "REC-003" {
		t.Errorf("rows[2].id = %v, want REC-003", rows[2]["id"])
	}
}

// TestSpreadsheet_WriteCSV tests writing CSV
func TestSpreadsheet_WriteCSV(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-write.csv")
	defer os.Remove(tmpfile)

	rows := []map[string]interface{}{
		{"id": "REC-001", "name": "Alpha"},
		{"id": "REC-002", "name": "Beta"},
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
		{"id": "REC-001", "name": "Alpha", "value": "10.50"},
		{"id": "REC-002", "name": "Beta", "value": "20.00"},
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

	if rows2[0]["id"] != "REC-001" {
		t.Errorf("rows[0].id = %v, want REC-001", rows2[0]["id"])
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
	csvContent := `id,name,description
REC-001,Alpha
REC-002,Beta,Description B,Extra`

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

// TestSpreadsheet_CSVWithHeaders tests CSV with custom headers.
func TestSpreadsheet_CSVWithHeaders(t *testing.T) {
	ctx := context.Background()

	csvContent := `ID,Item Name,Value
REC-001,Alpha,10.50
REC-002,Beta,20.00`

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
	if rows[0]["ID"] != "REC-001" {
		t.Errorf("rows[0].ID = %v, want REC-001", rows[0]["ID"])
	}
}

// TestSpreadsheet_InvalidOperation tests error handling for invalid operation.
func TestSpreadsheet_InvalidOperation(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "invalid",
		"format":    "csv",
		"path":      "/tmp/test.csv",
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

// TestSpreadsheet_WriteExcelDirect tests writing Excel directly and verifying file.
func TestSpreadsheet_WriteExcelDirect(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-write-direct.xlsx")
	defer os.Remove(tmpfile)

	rows := []map[string]interface{}{
		{"id": "REC-001", "name": "Alpha", "value": 10.5},
		{"id": "REC-002", "name": "Beta", "value": 20.0},
		{"id": "REC-003", "name": "Gamma", "value": 30.5},
	}

	params := map[string]interface{}{
		"operation": "write",
		"format":    "excel",
		"path":      tmpfile,
		"rows":      rows,
	}

	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["written"] != 3 {
		t.Errorf("written = %v, want 3", output["written"])
	}

	// Verify file exists and has non-zero size
	info, err := os.Stat(tmpfile)
	if err != nil {
		t.Fatalf("File not created: %v", err)
	}
	if info.Size() == 0 {
		t.Error("File is empty")
	}
}

// TestSpreadsheet_WriteCSVEmpty tests writing empty rows to CSV.
func TestSpreadsheet_WriteCSVEmpty(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-empty-write.csv")
	defer os.Remove(tmpfile)

	rows := []map[string]interface{}{}

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
	if output["written"] != 0 {
		t.Errorf("written = %v, want 0", output["written"])
	}
}

// TestSpreadsheet_WriteExcelEmpty tests writing empty rows to Excel.
func TestSpreadsheet_WriteExcelEmpty(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-empty-write.xlsx")
	defer os.Remove(tmpfile)

	rows := []map[string]interface{}{}

	params := map[string]interface{}{
		"operation": "write",
		"format":    "excel",
		"path":      tmpfile,
		"rows":      rows,
	}

	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["written"] != 0 {
		t.Errorf("written = %v, want 0", output["written"])
	}
}

// TestSpreadsheet_ReadNonexistentCSV tests reading a nonexistent CSV file.
func TestSpreadsheet_ReadNonexistentCSV(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "csv",
		"path":      "/nonexistent/file.csv",
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for nonexistent file, got nil")
	}
}

// TestSpreadsheet_ReadNonexistentExcel tests reading a nonexistent Excel file.
func TestSpreadsheet_ReadNonexistentExcel(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "excel",
		"path":      "/nonexistent/file.xlsx",
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for nonexistent file, got nil")
	}
}

// TestSpreadsheet_UnsupportedReadFormat tests reading with unsupported format.
func TestSpreadsheet_UnsupportedReadFormat(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "parquet",
		"path":      "/tmp/test.parquet",
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for unsupported format, got nil")
	}
}

// TestSpreadsheet_UnsupportedWriteFormat tests writing with unsupported format.
func TestSpreadsheet_UnsupportedWriteFormat(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "write",
		"format":    "parquet",
		"path":      "/tmp/test.parquet",
		"rows":      []map[string]interface{}{{"a": "b"}},
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for unsupported format, got nil")
	}
}

// TestSpreadsheet_MissingFormat tests missing format parameter.
func TestSpreadsheet_MissingFormat(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"path":      "/tmp/test.csv",
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing format, got nil")
	}
}

// TestSpreadsheet_WriteMissingRows tests write with missing rows parameter.
func TestSpreadsheet_WriteMissingRows(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "write",
		"format":    "csv",
		"path":      "/tmp/test.csv",
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing rows, got nil")
	}
}

// TestSpreadsheet_MissingOperation tests missing operation parameter.
func TestSpreadsheet_MissingOperation(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"format": "csv",
		"path":   "/tmp/test.csv",
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing operation, got nil")
	}
}

// TestSpreadsheet_CSVRoundTrip tests full CSV write+read cycle.
func TestSpreadsheet_CSVRoundTrip(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-csv-roundtrip.csv")
	defer os.Remove(tmpfile)

	rows := []map[string]interface{}{
		{"id": "A", "name": "Alpha"},
		{"id": "B", "name": "Beta"},
	}

	writeParams := map[string]interface{}{
		"operation": "write",
		"format":    "csv",
		"path":      tmpfile,
		"rows":      rows,
	}

	_, err := ss.Execute(ctx, writeParams)
	if err != nil {
		t.Fatalf("Write failed: %v", err)
	}

	readParams := map[string]interface{}{
		"operation": "read",
		"format":    "csv",
		"path":      tmpfile,
	}

	result, err := ss.Execute(ctx, readParams)
	if err != nil {
		t.Fatalf("Read failed: %v", err)
	}

	output := result.(map[string]interface{})
	readRows := output["rows"].([]map[string]interface{})

	if len(readRows) != 2 {
		t.Errorf("len(rows) = %d, want 2", len(readRows))
	}
}

// TestSpreadsheet_CSVHeaderOnly tests reading CSV with only a header row.
func TestSpreadsheet_CSVHeaderOnly(t *testing.T) {
	ctx := context.Background()

	tmpfile, err := os.CreateTemp("", "test-header-only-*.csv")
	if err != nil {
		t.Fatal(err)
	}
	defer os.Remove(tmpfile.Name())

	if _, err := tmpfile.WriteString("id,name,value\n"); err != nil {
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

	if len(rows) != 0 {
		t.Errorf("len(rows) = %d, want 0 (header only)", len(rows))
	}
}

// TestSpreadsheet_ReadExcelRoundTrip tests full Excel write+read cycle.
func TestSpreadsheet_ReadExcelRoundTrip(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-roundtrip.xlsx")
	defer os.Remove(tmpfile)

	// Write Excel with multiple columns
	rows := []map[string]interface{}{
		{"a": "1", "b": "2", "c": "3"},
		{"a": "4", "b": "5", "c": "6"},
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

	// Read it back
	readParams := map[string]interface{}{
		"operation": "read",
		"format":    "excel",
		"path":      tmpfile,
	}

	result, err := ss.Execute(ctx, readParams)
	if err != nil {
		t.Fatalf("Read failed: %v", err)
	}

	output := result.(map[string]interface{})
	readRows := output["rows"].([]map[string]interface{})

	if len(readRows) != 2 {
		t.Errorf("len(rows) = %d, want 2", len(readRows))
	}
}

// TestSpreadsheet_ReadEmptyExcel tests reading Excel with header but no data rows.
func TestSpreadsheet_ReadEmptyExcel(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	tmpfile := filepath.Join(os.TempDir(), "test-empty.xlsx")
	defer os.Remove(tmpfile)

	// Write Excel with just one row (becomes header on read)
	rows := []map[string]interface{}{
		{"id": "header-only"},
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

	readParams := map[string]interface{}{
		"operation": "read",
		"format":    "excel",
		"path":      tmpfile,
	}

	result, err := ss.Execute(ctx, readParams)
	if err != nil {
		t.Fatalf("Read failed: %v", err)
	}

	output := result.(map[string]interface{})
	readRows := output["rows"].([]map[string]interface{})

	// Excel has header row (row 1) and data row (row 2), so header = row 1 cols, data = remaining
	// With just one data row written, reading produces: header="id" row, data=["header-only"] row
	// Actually, writeExcel writes headers in row 1 and data starting row 2,
	// so 1 row written = header in row 1, data in row 2 = 1 data row when read back
	if len(readRows) != 1 {
		t.Errorf("len(rows) = %d, want 1", len(readRows))
	}
}

// TestSpreadsheet_WriteCSVToReadOnlyPath tests CSV write to unwritable path.
func TestSpreadsheet_WriteCSVToReadOnlyPath(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "write",
		"format":    "csv",
		"path":      "/nonexistent-dir/test.csv",
		"rows":      []map[string]interface{}{{"a": "b"}},
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for unwritable path, got nil")
	}
}

// TestSpreadsheet_ReadExcelEmptySheet tests reading Excel with completely empty sheet.
func TestSpreadsheet_ReadExcelEmptySheet(t *testing.T) {
	ctx := context.Background()

	tmpfile := filepath.Join(os.TempDir(), "test-empty-sheet.xlsx")
	defer os.Remove(tmpfile)

	// Create a valid xlsx with an empty sheet using excelize directly
	f := excelize.NewFile()
	if err := f.SaveAs(tmpfile); err != nil {
		t.Fatal(err)
	}
	f.Close()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "excel",
		"path":      tmpfile,
	}

	result, err := ss.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	rows := output["rows"].([]map[string]interface{})

	if len(rows) != 0 {
		t.Errorf("len(rows) = %d, want 0 for empty sheet", len(rows))
	}
}

// TestSpreadsheet_ReadInvalidExcel tests reading a corrupt Excel file.
func TestSpreadsheet_ReadInvalidExcel(t *testing.T) {
	ctx := context.Background()

	tmpfile := filepath.Join(os.TempDir(), "test-invalid.xlsx")
	defer os.Remove(tmpfile)

	if err := os.WriteFile(tmpfile, []byte("not xlsx content"), 0644); err != nil {
		t.Fatal(err)
	}

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "read",
		"format":    "excel",
		"path":      tmpfile,
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid Excel file, got nil")
	}
}

// TestSpreadsheet_WriteExcelToReadOnlyPath tests Excel write to unwritable path.
func TestSpreadsheet_WriteExcelToReadOnlyPath(t *testing.T) {
	ctx := context.Background()

	ss := spreadsheet.New()

	params := map[string]interface{}{
		"operation": "write",
		"format":    "excel",
		"path":      "/nonexistent-dir/test.xlsx",
		"rows":      []map[string]interface{}{{"a": "b"}},
	}

	_, err := ss.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for unwritable path, got nil")
	}
}
