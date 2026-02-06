// Package integration provides end-to-end integration tests for bento execution.
// Tests in this package execute the full bento binary against real fixture files
// to validate complete workflows and integrations.
package integration

import (
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

// RunBento executes a bento file and returns output.
// envVars is a map of environment variable key-value pairs to set.
func RunBento(t *testing.T, bentoPath string, envVars map[string]string) (string, error) {
	t.Helper()

	cmd := exec.Command("bento", "run", bentoPath)

	// Set working directory to project root (two levels up from tests/integration)
	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}
	projectRoot := filepath.Join(wd, "../..")
	cmd.Dir = projectRoot

	// Set environment variables
	cmd.Env = os.Environ()
	for k, v := range envVars {
		cmd.Env = append(cmd.Env, k+"="+v)
	}

	output, err := cmd.CombinedOutput()
	return string(output), err
}

// CleanupTestDir removes test output directory.
// Safe to call even if directory doesn't exist.
func CleanupTestDir(t *testing.T, dir string) {
	t.Helper()
	if err := os.RemoveAll(dir); err != nil {
		t.Logf("Warning: failed to cleanup test dir %s: %v", dir, err)
	}
}

// VerifyFileExists checks if a file exists.
// Fails the test if the file does not exist.
func VerifyFileExists(t *testing.T, path string) {
	t.Helper()
	if _, err := os.Stat(path); os.IsNotExist(err) {
		t.Errorf("File does not exist: %s", path)
	}
}

// VerifyFileCount checks number of files matching pattern.
// pattern should be a glob pattern like "*.png" or "render-*.webp"
// Fails the test if the count doesn't match expected.
func VerifyFileCount(t *testing.T, dir, pattern string, expected int) {
	t.Helper()

	matches, err := filepath.Glob(filepath.Join(dir, pattern))
	if err != nil {
		t.Fatalf("Glob failed for pattern %s in %s: %v", pattern, dir, err)
	}

	if len(matches) != expected {
		t.Errorf("Expected %d files matching %s in %s, got %d", expected, pattern, dir, len(matches))
		if len(matches) > 0 {
			t.Logf("Found files: %v", matches)
		}
	}
}

// CreateTempDir creates a temporary directory for test output.
// Returns the directory path and cleanup function.
// Cleanup function should be deferred by the caller.
func CreateTempDir(t *testing.T, prefix string) (string, func()) {
	t.Helper()

	dir, err := os.MkdirTemp("", prefix)
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}

	cleanup := func() {
		CleanupTestDir(t, dir)
	}

	return dir, cleanup
}
