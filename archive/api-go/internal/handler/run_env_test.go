package handler

import (
	"os"
	"path/filepath"
	"testing"
)

func TestSetPerFileEnvVars(t *testing.T) {
	t.Run("sets INPUT and OUTPUT vars for CSV file", func(t *testing.T) {
		inputDir := t.TempDir()
		outputDir := t.TempDir()

		os.WriteFile(filepath.Join(inputDir, "data.csv"), []byte("a,b\n1,2"), 0o644)

		vars := setPerFileEnvVars(inputDir, outputDir)
		defer unsetEnvVars(vars)

		inputCSV := os.Getenv("INPUT_CSV")
		if inputCSV != filepath.Join(inputDir, "data.csv") {
			t.Errorf("INPUT_CSV = %q, want %q", inputCSV, filepath.Join(inputDir, "data.csv"))
		}

		outputCSV := os.Getenv("OUTPUT_CSV")
		if outputCSV != filepath.Join(outputDir, "data.csv") {
			t.Errorf("OUTPUT_CSV = %q, want %q", outputCSV, filepath.Join(outputDir, "data.csv"))
		}
	})

	t.Run("only first file per extension wins", func(t *testing.T) {
		inputDir := t.TempDir()
		outputDir := t.TempDir()

		os.WriteFile(filepath.Join(inputDir, "first.csv"), []byte("a"), 0o644)
		os.WriteFile(filepath.Join(inputDir, "second.csv"), []byte("b"), 0o644)

		vars := setPerFileEnvVars(inputDir, outputDir)
		defer unsetEnvVars(vars)

		inputCSV := os.Getenv("INPUT_CSV")
		if inputCSV == "" {
			t.Fatal("INPUT_CSV not set")
		}
		// Should be one of them (ReadDir returns sorted order)
		if inputCSV != filepath.Join(inputDir, "first.csv") {
			t.Errorf("INPUT_CSV = %q, want first.csv", inputCSV)
		}
	})

	t.Run("multiple extensions set separate vars", func(t *testing.T) {
		inputDir := t.TempDir()
		outputDir := t.TempDir()

		os.WriteFile(filepath.Join(inputDir, "data.csv"), []byte("a"), 0o644)
		os.WriteFile(filepath.Join(inputDir, "image.png"), []byte("b"), 0o644)

		vars := setPerFileEnvVars(inputDir, outputDir)
		defer unsetEnvVars(vars)

		if os.Getenv("INPUT_CSV") == "" {
			t.Error("INPUT_CSV not set")
		}
		if os.Getenv("INPUT_PNG") == "" {
			t.Error("INPUT_PNG not set")
		}
		if len(vars) != 4 {
			t.Errorf("expected 4 vars, got %d: %v", len(vars), vars)
		}
	})

	t.Run("skips directories and extensionless files", func(t *testing.T) {
		inputDir := t.TempDir()
		outputDir := t.TempDir()

		os.MkdirAll(filepath.Join(inputDir, "subdir"), 0o755)
		os.WriteFile(filepath.Join(inputDir, "noext"), []byte("x"), 0o644)

		vars := setPerFileEnvVars(inputDir, outputDir)
		defer unsetEnvVars(vars)

		if len(vars) != 0 {
			t.Errorf("expected 0 vars, got %d: %v", len(vars), vars)
		}
	})

	t.Run("cleans up vars after unset", func(t *testing.T) {
		inputDir := t.TempDir()
		outputDir := t.TempDir()

		os.WriteFile(filepath.Join(inputDir, "test.csv"), []byte("a"), 0o644)

		vars := setPerFileEnvVars(inputDir, outputDir)
		if os.Getenv("INPUT_CSV") == "" {
			t.Fatal("INPUT_CSV should be set")
		}

		unsetEnvVars(vars)

		if os.Getenv("INPUT_CSV") != "" {
			t.Error("INPUT_CSV should be unset after cleanup")
		}
	})

	t.Run("empty directory returns nil", func(t *testing.T) {
		inputDir := t.TempDir()
		outputDir := t.TempDir()

		vars := setPerFileEnvVars(inputDir, outputDir)
		if vars != nil {
			t.Errorf("expected nil, got %v", vars)
		}
	})
}
