package testgolden

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// TempCopy copies a file to a temp directory and returns the new path.
// The temp directory is cleaned up automatically when the test finishes.
func TempCopy(t *testing.T, src string) string {
	t.Helper()

	data, err := os.ReadFile(src)
	if err != nil {
		t.Fatalf("Failed to read source file %s: %v", src, err)
	}

	dir := t.TempDir()
	dst := filepath.Join(dir, filepath.Base(src))

	if err := os.WriteFile(dst, data, 0644); err != nil {
		t.Fatalf("Failed to write temp copy %s: %v", dst, err)
	}

	return dst
}

// ReadJSON reads a JSON file into a generic map.
func ReadJSON(t *testing.T, path string) map[string]any {
	t.Helper()

	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Failed to read JSON file %s: %v", path, err)
	}

	var result map[string]any
	if err := json.Unmarshal(data, &result); err != nil {
		t.Fatalf("Failed to parse JSON file %s: %v", path, err)
	}

	return result
}
