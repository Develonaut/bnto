package integration

import (
	"os"
	"path/filepath"
	"testing"
)

func TestCleanupTestDir(t *testing.T) {
	// Create a temp directory
	dir, err := os.MkdirTemp("", "cleanup-test-")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}

	// Create a test file in it
	testFile := filepath.Join(dir, "test.txt")
	if err := os.WriteFile(testFile, []byte("test"), 0644); err != nil {
		t.Fatalf("Failed to write test file: %v", err)
	}

	// Cleanup should remove the directory
	CleanupTestDir(t, dir)

	// Verify directory no longer exists
	if _, err := os.Stat(dir); !os.IsNotExist(err) {
		t.Error("CleanupTestDir did not remove directory")
	}
}

func TestVerifyFileExists(t *testing.T) {
	// Create a temp file
	tmpfile, err := os.CreateTemp("", "verify-test-")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	defer os.Remove(tmpfile.Name())
	tmpfile.Close()

	// Should not fail for existing file
	VerifyFileExists(t, tmpfile.Name())

	// Note: Testing failure case would require t.Run with separate subtest
	// For now, we verify the happy path works correctly
}

func TestVerifyFileCount(t *testing.T) {
	// Create temp directory with known number of files
	dir, err := os.MkdirTemp("", "count-test-")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	// Create 3 .txt files
	for i := 1; i <= 3; i++ {
		name := filepath.Join(dir, filepath.Base(dir)+"-"+string(rune('0'+i))+".txt")
		if err := os.WriteFile(name, []byte("test"), 0644); err != nil {
			t.Fatalf("Failed to write test file: %v", err)
		}
	}

	// Should pass with correct count
	VerifyFileCount(t, dir, "*.txt", 3)

	// Note: Testing failure case would require t.Run with separate subtest
	// For now, we verify the happy path works correctly
}

func TestCreateTempDir(t *testing.T) {
	dir, cleanup := CreateTempDir(t, "create-test-")
	defer cleanup()

	// Verify directory was created
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		t.Error("CreateTempDir did not create directory")
	}

	// Create a test file
	testFile := filepath.Join(dir, "test.txt")
	if err := os.WriteFile(testFile, []byte("test"), 0644); err != nil {
		t.Fatalf("Failed to write to temp dir: %v", err)
	}

	// Call cleanup
	cleanup()

	// Verify cleanup removed directory
	if _, err := os.Stat(dir); !os.IsNotExist(err) {
		t.Error("Cleanup did not remove directory")
	}
}
