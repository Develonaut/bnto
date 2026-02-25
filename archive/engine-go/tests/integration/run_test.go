// Package integration provides end-to-end integration tests for bnto execution.
//
// These tests execute the Tier 1 bnto fixtures against the engine
// to verify each workflow produces correct output.
package integration

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/engine"
	"github.com/Develonaut/bnto/pkg/node"
)

// loadAndExecute loads a fixture, sets env vars, and runs it through the engine.
func loadAndExecute(t *testing.T, fixturePath string, envVars map[string]string) *engine.Result {
	t.Helper()

	data, err := os.ReadFile(fixturePath)
	if err != nil {
		t.Fatalf("Failed to load fixture %s: %v", fixturePath, err)
	}

	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		t.Fatalf("Failed to parse fixture %s: %v", fixturePath, err)
	}

	for k, v := range envVars {
		os.Setenv(k, v)
		defer os.Unsetenv(k)
	}

	reg := createRegistry()
	eng := engine.New(reg, nil)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	result, err := eng.Serve(ctx, &def)
	if err != nil {
		t.Fatalf("Execution failed: %v", err)
	}

	return result
}

// projectRoot returns the engine root (two levels up from tests/integration).
func projectRoot(t *testing.T) string {
	t.Helper()
	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}
	return filepath.Join(wd, "../..")
}

func TestTier1_CompressImages(t *testing.T) {
	root := projectRoot(t)
	outputDir := t.TempDir()

	result := loadAndExecute(t, filepath.Join(root, "tests/fixtures/workflows/compress-images.bnto.json"), map[string]string{
		"INPUT_DIR":  filepath.Join(root, "tests/fixtures/images"),
		"OUTPUT_DIR": outputDir,
	})

	if result.Status != "success" {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	// Verify compressed output exists
	matches, _ := filepath.Glob(filepath.Join(outputDir, "*.png"))
	if len(matches) == 0 {
		// optimize outputs WebP, check for that too
		matches, _ = filepath.Glob(filepath.Join(outputDir, "*"))
	}
	if len(matches) == 0 {
		t.Fatal("No output files produced")
	}

	for _, m := range matches {
		info, err := os.Stat(m)
		if err != nil {
			t.Errorf("Cannot stat output %s: %v", m, err)
			continue
		}
		if info.Size() == 0 {
			t.Errorf("Output file is empty: %s", m)
		}
		t.Logf("  ✓ %s (%d bytes)", filepath.Base(m), info.Size())
	}
}

func TestTier1_ResizeImages(t *testing.T) {
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
		info, err := os.Stat(m)
		if err != nil {
			t.Errorf("Cannot stat output %s: %v", m, err)
			continue
		}
		if info.Size() == 0 {
			t.Errorf("Output file is empty: %s", m)
		}
		t.Logf("  ✓ %s (%d bytes)", filepath.Base(m), info.Size())
	}
}

func TestTier1_ConvertImageFormat(t *testing.T) {
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
		info, err := os.Stat(m)
		if err != nil {
			t.Errorf("Cannot stat output %s: %v", m, err)
			continue
		}
		if info.Size() == 0 {
			t.Errorf("Output file is empty: %s", m)
		}
		t.Logf("  ✓ %s (%d bytes)", filepath.Base(m), info.Size())
	}
}

func TestTier1_RenameFiles(t *testing.T) {
	root := projectRoot(t)

	// Copy test files to a temp input dir (rename is destructive)
	inputDir := t.TempDir()
	outputDir := t.TempDir()

	srcFiles, _ := filepath.Glob(filepath.Join(root, "tests/fixtures/images/*.png"))
	for _, src := range srcFiles {
		data, err := os.ReadFile(src)
		if err != nil {
			t.Fatalf("Failed to read %s: %v", src, err)
		}
		dst := filepath.Join(inputDir, filepath.Base(src))
		if err := os.WriteFile(dst, data, 0644); err != nil {
			t.Fatalf("Failed to write %s: %v", dst, err)
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

	for _, m := range matches {
		base := filepath.Base(m)
		if !strings.HasPrefix(base, "renamed-") {
			t.Errorf("File missing 'renamed-' prefix: %s", base)
		}
		t.Logf("  ✓ %s", base)
	}
}

func TestTier1_CleanCSV(t *testing.T) {
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

	content := string(data)
	if len(content) == 0 {
		t.Fatal("Output CSV is empty")
	}

	// Should contain the test data rows
	if !strings.Contains(content, "Test Item A") {
		t.Error("Output CSV missing 'Test Item A'")
	}
	if !strings.Contains(content, "Test Item B") {
		t.Error("Output CSV missing 'Test Item B'")
	}
	if !strings.Contains(content, "Test Item C") {
		t.Error("Output CSV missing 'Test Item C'")
	}

	t.Logf("  ✓ Clean CSV written (%d bytes)", len(data))
}

func TestTier1_RenameCSVColumns(t *testing.T) {
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

	content := string(data)
	if len(content) == 0 {
		t.Fatal("Output CSV is empty")
	}

	// Should contain the original data (pass-through for now)
	if !strings.Contains(content, "Test Item A") {
		t.Error("Output CSV missing 'Test Item A'")
	}
	if !strings.Contains(content, "Test Item B") {
		t.Error("Output CSV missing 'Test Item B'")
	}

	t.Logf("  ✓ Renamed CSV written (%d bytes)", len(data))
}
