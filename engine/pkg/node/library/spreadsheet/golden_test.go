package spreadsheet_test

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/spreadsheet"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_ReadCSV(t *testing.T) {
	dir := t.TempDir()
	csvPath := filepath.Join(dir, "test.csv")
	csvContent := "id,name,description\nREC-001,Alpha,Description A\nREC-002,Beta,Description B\nREC-003,Gamma,Description C"
	if err := os.WriteFile(csvPath, []byte(csvContent), 0644); err != nil {
		t.Fatal(err)
	}

	ss := spreadsheet.New()
	result, err := ss.Execute(context.Background(), map[string]any{
		"operation": "read",
		"format":    "csv",
		"path":      csvPath,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "read_csv", result)
}

func TestGolden_WriteCSV(t *testing.T) {
	dir := t.TempDir()
	outPath := filepath.Join(dir, "output.csv")

	ss := spreadsheet.New()
	result, err := ss.Execute(context.Background(), map[string]any{
		"operation": "write",
		"format":    "csv",
		"path":      outPath,
		"rows": []map[string]any{
			{"id": "REC-001", "name": "Alpha"},
			{"id": "REC-002", "name": "Beta"},
			{"id": "REC-003", "name": "Gamma"},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "write_csv", result)
}

func TestGolden_CSVRoundTrip(t *testing.T) {
	dir := t.TempDir()
	csvPath := filepath.Join(dir, "roundtrip.csv")

	ss := spreadsheet.New()

	// Write
	_, err := ss.Execute(context.Background(), map[string]any{
		"operation": "write",
		"format":    "csv",
		"path":      csvPath,
		"rows": []map[string]any{
			{"id": "A", "name": "Alpha", "value": "100"},
			{"id": "B", "name": "Beta", "value": "200"},
		},
	})
	if err != nil {
		t.Fatalf("Write failed: %v", err)
	}

	// Read back
	result, err := ss.Execute(context.Background(), map[string]any{
		"operation": "read",
		"format":    "csv",
		"path":      csvPath,
	})
	if err != nil {
		t.Fatalf("Read failed: %v", err)
	}

	testgolden.AssertGolden(t, "csv_roundtrip", result)
}

func TestGolden_EmptyCSV(t *testing.T) {
	dir := t.TempDir()
	csvPath := filepath.Join(dir, "empty.csv")
	if err := os.WriteFile(csvPath, []byte(""), 0644); err != nil {
		t.Fatal(err)
	}

	ss := spreadsheet.New()
	result, err := ss.Execute(context.Background(), map[string]any{
		"operation": "read",
		"format":    "csv",
		"path":      csvPath,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "empty_csv", result)
}
