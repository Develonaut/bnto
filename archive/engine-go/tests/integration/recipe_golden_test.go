package integration

import (
	"bytes"
	"encoding/csv"
	"os"
	"path/filepath"
	"sort"
	"testing"

	"github.com/Develonaut/bnto/pkg/testgolden"
)

// TestRecipeGolden_CompressImages verifies compress-images recipe output properties.
func TestRecipeGolden_CompressImages(t *testing.T) {
	root := projectRoot(t)
	outputDir := t.TempDir()

	result := loadAndExecute(t, filepath.Join(root, "tests/fixtures/workflows/compress-images.bnto.json"), map[string]string{
		"INPUT_DIR":  filepath.Join(root, "tests/fixtures/images"),
		"OUTPUT_DIR": outputDir,
	})

	if result.Status != "success" {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	matches, _ := filepath.Glob(filepath.Join(outputDir, "*"))
	if len(matches) == 0 {
		t.Fatal("No output files produced")
	}

	for _, m := range matches {
		// Compress recipe may output WebP (optimize converts to WebP for better compression)
		testgolden.AssertImageProperties(t, m, testgolden.ImageProperties{})
	}
}

// TestRecipeGolden_ResizeImages verifies resize-images recipe produces correctly sized output.
func TestRecipeGolden_ResizeImages(t *testing.T) {
	root := projectRoot(t)
	outputDir := t.TempDir()

	result := loadAndExecute(t, filepath.Join(root, "tests/fixtures/workflows/resize-images.bnto.json"), map[string]string{
		"INPUT_DIR":  filepath.Join(root, "tests/fixtures/images"),
		"OUTPUT_DIR": outputDir,
	})

	if result.Status != "success" {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	matches, _ := filepath.Glob(filepath.Join(outputDir, "*"))
	if len(matches) == 0 {
		t.Fatal("No resized output files produced")
	}

	for _, m := range matches {
		testgolden.AssertImageProperties(t, m, testgolden.ImageProperties{
			Width:  200,
			Format: "png",
		})
	}
}

// TestRecipeGolden_ConvertImageFormat verifies convert-image-format recipe produces WebP output.
func TestRecipeGolden_ConvertImageFormat(t *testing.T) {
	root := projectRoot(t)
	outputDir := t.TempDir()

	result := loadAndExecute(t, filepath.Join(root, "tests/fixtures/workflows/convert-image-format.bnto.json"), map[string]string{
		"INPUT_DIR":  filepath.Join(root, "tests/fixtures/images"),
		"OUTPUT_DIR": outputDir,
	})

	if result.Status != "success" {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	matches, _ := filepath.Glob(filepath.Join(outputDir, "*.webp"))
	if len(matches) == 0 {
		t.Fatal("No WebP output files produced")
	}

	for _, m := range matches {
		testgolden.AssertImageProperties(t, m, testgolden.ImageProperties{
			Format: "webp",
		})
	}
}

// TestRecipeGolden_RenameFiles verifies rename-files recipe renames with correct prefix.
func TestRecipeGolden_RenameFiles(t *testing.T) {
	root := projectRoot(t)
	inputDir := t.TempDir()
	outputDir := t.TempDir()

	srcFiles, _ := filepath.Glob(filepath.Join(root, "tests/fixtures/images/*.png"))
	for _, src := range srcFiles {
		data, err := os.ReadFile(src)
		if err != nil {
			t.Fatalf("Failed to read %s: %v", src, err)
		}
		if err := os.WriteFile(filepath.Join(inputDir, filepath.Base(src)), data, 0644); err != nil {
			t.Fatal(err)
		}
	}

	result := loadAndExecute(t, filepath.Join(root, "tests/fixtures/workflows/rename-files.bnto.json"), map[string]string{
		"INPUT_DIR":  inputDir,
		"OUTPUT_DIR": outputDir,
	})

	if result.Status != "success" {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	matches, _ := filepath.Glob(filepath.Join(outputDir, "renamed-*"))
	if len(matches) == 0 {
		t.Fatal("No renamed output files produced")
	}

	testgolden.AssertGolden(t, "recipe_rename_files", map[string]any{
		"status":        result.Status,
		"nodesExecuted": result.NodesExecuted,
		"fileCount":     len(matches),
	})
}

// TestRecipeGolden_CleanCSV verifies clean-csv recipe output content.
func TestRecipeGolden_CleanCSV(t *testing.T) {
	root := projectRoot(t)
	outputCSV := filepath.Join(t.TempDir(), "clean.csv")

	result := loadAndExecute(t, filepath.Join(root, "tests/fixtures/workflows/clean-csv.bnto.json"), map[string]string{
		"INPUT_CSV":  filepath.Join(root, "tests/fixtures/test-data.csv"),
		"OUTPUT_CSV": outputCSV,
	})

	if result.Status != "success" {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	data, err := os.ReadFile(outputCSV)
	if err != nil {
		t.Fatalf("Failed to read output CSV: %v", err)
	}

	// Parse into sorted rows for stable comparison (column order may vary)
	rows := parseCSVSorted(t, data)

	testgolden.AssertGolden(t, "recipe_clean_csv", map[string]any{
		"status":        result.Status,
		"nodesExecuted": result.NodesExecuted,
		"rowCount":      len(rows),
		"rows":          rows,
	})
}

// TestRecipeGolden_RenameCSVColumns verifies rename-csv-columns recipe output.
func TestRecipeGolden_RenameCSVColumns(t *testing.T) {
	root := projectRoot(t)
	outputCSV := filepath.Join(t.TempDir(), "renamed.csv")

	result := loadAndExecute(t, filepath.Join(root, "tests/fixtures/workflows/rename-csv-columns.bnto.json"), map[string]string{
		"INPUT_CSV":  filepath.Join(root, "tests/fixtures/test-data.csv"),
		"OUTPUT_CSV": outputCSV,
	})

	if result.Status != "success" {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	data, err := os.ReadFile(outputCSV)
	if err != nil {
		t.Fatalf("Failed to read output CSV: %v", err)
	}

	rows := parseCSVSorted(t, data)

	testgolden.AssertGolden(t, "recipe_rename_csv_columns", map[string]any{
		"status":        result.Status,
		"nodesExecuted": result.NodesExecuted,
		"rowCount":      len(rows),
		"rows":          rows,
	})
}

// parseCSVSorted reads CSV data and returns rows as sorted key-value maps.
// This normalizes non-deterministic column ordering from Go map iteration.
func parseCSVSorted(t *testing.T, data []byte) []map[string]string {
	t.Helper()

	r := csv.NewReader(bytes.NewReader(data))
	records, err := r.ReadAll()
	if err != nil {
		t.Fatalf("Failed to parse CSV: %v", err)
	}

	if len(records) < 2 {
		return nil
	}

	headers := make([]string, len(records[0]))
	copy(headers, records[0])
	sort.Strings(headers)

	// Build header index from original order
	headerIdx := make(map[string]int)
	for i, h := range records[0] {
		headerIdx[h] = i
	}

	rows := make([]map[string]string, 0, len(records)-1)
	for _, record := range records[1:] {
		row := make(map[string]string, len(headers))
		for _, h := range headers {
			idx := headerIdx[h]
			if idx < len(record) {
				row[h] = record[idx]
			}
		}
		rows = append(rows, row)
	}

	return rows
}
