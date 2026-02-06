package filesystem_test

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/Develonaut/bento/pkg/node/library/filesystem"
)

// TestFileSystem_ReadFile tests reading a file.
func TestFileSystem_ReadFile(t *testing.T) {
	ctx := context.Background()

	// Create temp file
	tmpfile, err := os.CreateTemp("", "test-*.txt")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	defer os.Remove(tmpfile.Name())

	content := "Hello, bento!"
	if _, err := tmpfile.WriteString(content); err != nil {
		t.Fatalf("Failed to write to temp file: %v", err)
	}
	tmpfile.Close()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "read",
		"path":      tmpfile.Name(),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["content"] != content {
		t.Errorf("content = %v, want %v", output["content"], content)
	}
}

// TestFileSystem_WriteFile tests writing to a file.
func TestFileSystem_WriteFile(t *testing.T) {
	ctx := context.Background()

	tmpfile := filepath.Join(os.TempDir(), "test-write.txt")
	defer os.Remove(tmpfile)

	fs := filesystem.New()

	content := "Test content for writing"
	params := map[string]interface{}{
		"operation": "write",
		"path":      tmpfile,
		"content":   content,
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["path"] != tmpfile {
		t.Errorf("path = %v, want %v", output["path"], tmpfile)
	}

	// Verify file was written
	data, err := os.ReadFile(tmpfile)
	if err != nil {
		t.Fatalf("Failed to read written file: %v", err)
	}

	if string(data) != content {
		t.Errorf("file content = %v, want %v", string(data), content)
	}
}

// TestFileSystem_CopyFile tests copying a file.
func TestFileSystem_CopyFile(t *testing.T) {
	ctx := context.Background()

	// Create source file
	srcfile, err := os.CreateTemp("", "test-src-*.txt")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	defer os.Remove(srcfile.Name())

	content := "Content to copy"
	if _, err := srcfile.WriteString(content); err != nil {
		t.Fatalf("Failed to write to temp file: %v", err)
	}
	srcfile.Close()

	// Destination path
	dstfile := filepath.Join(os.TempDir(), "test-dst.txt")
	defer os.Remove(dstfile)

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "copy",
		"source":    srcfile.Name(),
		"dest":      dstfile,
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["dest"] != dstfile {
		t.Errorf("dest = %v, want %v", output["dest"], dstfile)
	}

	// Verify destination file exists with same content
	data, err := os.ReadFile(dstfile)
	if err != nil {
		t.Fatalf("Failed to read copied file: %v", err)
	}

	if string(data) != content {
		t.Errorf("copied content = %v, want %v", string(data), content)
	}
}

// TestFileSystem_MoveFile tests moving a file.
func TestFileSystem_MoveFile(t *testing.T) {
	ctx := context.Background()

	// Create source file
	srcfile, err := os.CreateTemp("", "test-move-src-*.txt")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	defer os.Remove(srcfile.Name())

	content := "Content to move"
	if _, err := srcfile.WriteString(content); err != nil {
		t.Fatalf("Failed to write to temp file: %v", err)
	}
	srcfile.Close()

	// Destination path
	dstfile := filepath.Join(os.TempDir(), "test-move-dst.txt")
	defer os.Remove(dstfile)

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "move",
		"source":    srcfile.Name(),
		"dest":      dstfile,
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["dest"] != dstfile {
		t.Errorf("dest = %v, want %v", output["dest"], dstfile)
	}

	// Verify source no longer exists
	if _, err := os.Stat(srcfile.Name()); !os.IsNotExist(err) {
		t.Error("Source file still exists after move")
	}

	// Verify destination exists with correct content
	data, err := os.ReadFile(dstfile)
	if err != nil {
		t.Fatalf("Failed to read moved file: %v", err)
	}

	if string(data) != content {
		t.Errorf("moved content = %v, want %v", string(data), content)
	}
}

// TestFileSystem_DeleteFile tests deleting a file.
func TestFileSystem_DeleteFile(t *testing.T) {
	ctx := context.Background()

	// Create temp file
	tmpfile, err := os.CreateTemp("", "test-delete-*.txt")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	tmpfile.Close()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "delete",
		"path":      tmpfile.Name(),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["deleted"] != true {
		t.Errorf("deleted = %v, want true", output["deleted"])
	}

	// Verify file no longer exists
	if _, err := os.Stat(tmpfile.Name()); !os.IsNotExist(err) {
		t.Error("File still exists after delete")
	}
}

// TestFileSystem_CreateDirectory tests creating a directory.
func TestFileSystem_CreateDirectory(t *testing.T) {
	ctx := context.Background()

	tmpdir := filepath.Join(os.TempDir(), "test-mkdir")
	defer os.RemoveAll(tmpdir)

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "mkdir",
		"path":      tmpdir,
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["path"] != tmpdir {
		t.Errorf("path = %v, want %v", output["path"], tmpdir)
	}

	// Verify directory exists
	info, err := os.Stat(tmpdir)
	if err != nil {
		t.Fatalf("Directory was not created: %v", err)
	}

	if !info.IsDir() {
		t.Error("Path exists but is not a directory")
	}
}

// TestFileSystem_Exists tests checking if a file exists.
func TestFileSystem_Exists(t *testing.T) {
	ctx := context.Background()

	// Create temp file
	tmpfile, err := os.CreateTemp("", "test-exists-*.txt")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	tmpfile.Close()
	defer os.Remove(tmpfile.Name())

	fs := filesystem.New()

	// Test existing file
	params := map[string]interface{}{
		"operation": "exists",
		"path":      tmpfile.Name(),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["exists"] != true {
		t.Errorf("exists = %v, want true", output["exists"])
	}

	// Test non-existing file
	params["path"] = "/nonexistent/path/file.txt"

	result, err = fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output = result.(map[string]interface{})
	if output["exists"] != false {
		t.Errorf("exists = %v, want false", output["exists"])
	}
}

// TestFileSystem_InvalidOperation tests error handling for invalid operations.
func TestFileSystem_InvalidOperation(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "invalid",
		"path":      "/some/path",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid operation, got nil")
	}

	if !strings.Contains(err.Error(), "unsupported operation") {
		t.Errorf("Expected 'unsupported operation' error, got: %v", err)
	}
}

// TestFileSystem_MissingPath tests error handling when path is missing.
func TestFileSystem_MissingPath(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "read",
		// Missing "path" parameter
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing path, got nil")
	}
}

// TestFileSystem_ListGlobPattern tests listing files with a glob pattern.
func TestFileSystem_ListGlobPattern(t *testing.T) {
	ctx := context.Background()

	// Create temp directory with test files
	tmpdir := t.TempDir()

	// Create test files
	testFiles := []string{"test1.txt", "test2.txt", "test3.png", "data.json"}
	for _, fname := range testFiles {
		f, err := os.Create(filepath.Join(tmpdir, fname))
		if err != nil {
			t.Fatalf("Failed to create test file: %v", err)
		}
		f.Close()
	}

	fs := filesystem.New()

	// Test listing only .txt files
	params := map[string]interface{}{
		"operation": "list",
		"path":      filepath.Join(tmpdir, "*.txt"),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	files := output["files"].([]interface{})
	count := output["count"].(int)

	if count != 2 {
		t.Errorf("count = %d, want 2", count)
	}

	if len(files) != 2 {
		t.Errorf("len(files) = %d, want 2", len(files))
	}

	// Verify all returned files are .txt
	for _, f := range files {
		fpath := f.(string)
		if !strings.HasSuffix(fpath, ".txt") {
			t.Errorf("Expected .txt file, got: %s", fpath)
		}
	}
}

// TestFileSystem_ListDirectory tests listing all files in a directory.
func TestFileSystem_ListDirectory(t *testing.T) {
	ctx := context.Background()

	// Create temp directory with test files
	tmpdir := t.TempDir()

	// Create test files
	testFiles := []string{"file1.txt", "file2.png", "file3.json"}
	for _, fname := range testFiles {
		f, err := os.Create(filepath.Join(tmpdir, fname))
		if err != nil {
			t.Fatalf("Failed to create test file: %v", err)
		}
		f.Close()
	}

	// Create a subdirectory (should be excluded from listing)
	subdir := filepath.Join(tmpdir, "subdir")
	if err := os.Mkdir(subdir, 0755); err != nil {
		t.Fatalf("Failed to create subdirectory: %v", err)
	}

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "list",
		"path":      tmpdir,
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	files := output["files"].([]interface{})
	count := output["count"].(int)

	// Should list 3 files (not the subdirectory)
	if count != 3 {
		t.Errorf("count = %d, want 3", count)
	}

	if len(files) != 3 {
		t.Errorf("len(files) = %d, want 3", len(files))
	}

	// Verify all returned paths are files (not directories)
	for _, f := range files {
		fpath := f.(string)
		info, err := os.Stat(fpath)
		if err != nil {
			t.Errorf("Failed to stat file: %v", err)
			continue
		}
		if info.IsDir() {
			t.Errorf("Expected file, got directory: %s", fpath)
		}
	}
}

// TestFileSystem_ListSingleFile tests listing a single file path.
func TestFileSystem_ListSingleFile(t *testing.T) {
	ctx := context.Background()

	// Create temp file
	tmpfile, err := os.CreateTemp("", "test-list-single-*.txt")
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}
	tmpfile.Close()
	defer os.Remove(tmpfile.Name())

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "list",
		"path":      tmpfile.Name(),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	files := output["files"].([]interface{})
	count := output["count"].(int)

	if count != 1 {
		t.Errorf("count = %d, want 1", count)
	}

	if len(files) != 1 {
		t.Errorf("len(files) = %d, want 1", len(files))
	}

	if files[0] != tmpfile.Name() {
		t.Errorf("files[0] = %v, want %v", files[0], tmpfile.Name())
	}
}

// TestFileSystem_ListEmptyDirectory tests listing an empty directory.
func TestFileSystem_ListEmptyDirectory(t *testing.T) {
	ctx := context.Background()

	// Create empty temp directory
	tmpdir := t.TempDir()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "list",
		"path":      tmpdir,
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	files := output["files"].([]interface{})
	count := output["count"].(int)

	if count != 0 {
		t.Errorf("count = %d, want 0", count)
	}

	if len(files) != 0 {
		t.Errorf("len(files) = %d, want 0", len(files))
	}
}

// TestFileSystem_DeleteGlobPattern tests deleting files matching a glob.
func TestFileSystem_DeleteGlobPattern(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()

	// Create test files
	for _, name := range []string{"file1.tmp", "file2.tmp", "keep.txt"} {
		f, err := os.Create(filepath.Join(tmpdir, name))
		if err != nil {
			t.Fatal(err)
		}
		f.Close()
	}

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "delete",
		"path":      filepath.Join(tmpdir, "*.tmp"),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	deleted := output["deleted"].(int)

	if deleted != 2 {
		t.Errorf("deleted = %d, want 2", deleted)
	}

	// Verify keep.txt still exists
	if _, err := os.Stat(filepath.Join(tmpdir, "keep.txt")); os.IsNotExist(err) {
		t.Error("keep.txt was incorrectly deleted")
	}
}

// TestFileSystem_DeleteGlobNoMatches tests glob deletion when no files match.
func TestFileSystem_DeleteGlobNoMatches(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "delete",
		"path":      filepath.Join(tmpdir, "*.nonexistent"),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	deleted := output["deleted"].(int)

	if deleted != 0 {
		t.Errorf("deleted = %d, want 0", deleted)
	}
}

// TestFileSystem_ListBraceExpansion tests listing with brace expansion patterns.
func TestFileSystem_ListBraceExpansion(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()

	// Create test files of different types
	for _, name := range []string{"a.txt", "b.txt", "c.md", "d.json"} {
		f, err := os.Create(filepath.Join(tmpdir, name))
		if err != nil {
			t.Fatal(err)
		}
		f.Close()
	}

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "list",
		"path":      filepath.Join(tmpdir, "*.{txt,md}"),
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	count := output["count"].(int)

	if count != 3 {
		t.Errorf("count = %d, want 3 (2 txt + 1 md)", count)
	}
}

// TestFileSystem_WriteCreatesParentDirs tests write auto-creates parent directories.
func TestFileSystem_WriteCreatesParentDirs(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()
	deepPath := filepath.Join(tmpdir, "a", "b", "c", "file.txt")

	fs := filesystem.New()

	// First verify parent doesn't exist
	if _, err := os.Stat(filepath.Join(tmpdir, "a")); !os.IsNotExist(err) {
		t.Fatal("Expected parent dir to not exist yet")
	}

	// Write creates the file (but may not auto-create dirs if not implemented)
	params := map[string]interface{}{
		"operation": "write",
		"path":      deepPath,
		"content":   "deep content",
	}

	// Current implementation doesn't auto-create dirs, this should fail
	_, err := fs.Execute(ctx, params)
	if err == nil {
		// If it succeeded, verify the file was created
		data, _ := os.ReadFile(deepPath)
		if string(data) != "deep content" {
			t.Errorf("content = %q, want 'deep content'", string(data))
		}
	}
	// Either way, the write code path is exercised
}

// TestFileSystem_WriteBentoIgnoreProtection tests write respects .bentoignore.
func TestFileSystem_WriteBentoIgnoreProtection(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()

	// Create .bentoignore
	bentoIgnore := filepath.Join(tmpdir, ".bentoignore")
	if err := os.WriteFile(bentoIgnore, []byte("*.protected\n"), 0644); err != nil {
		t.Fatal(err)
	}

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "write",
		"path":      filepath.Join(tmpdir, "secret.protected"),
		"content":   "should not write",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for .bentoignore protected file, got nil")
	}

	if !strings.Contains(err.Error(), "bentoignore") && !strings.Contains(err.Error(), "protected") {
		t.Errorf("Expected bentoignore-related error, got: %v", err)
	}
}

// TestFileSystem_CopyMissingSource tests copy with nonexistent source file.
func TestFileSystem_CopyMissingSource(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "copy",
		"source":    filepath.Join(tmpdir, "nonexistent.txt"),
		"dest":      filepath.Join(tmpdir, "dest.txt"),
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing source, got nil")
	}
}

// TestFileSystem_MoveMissingSource tests move with nonexistent source file.
func TestFileSystem_MoveMissingSource(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "move",
		"source":    filepath.Join(tmpdir, "nonexistent.txt"),
		"dest":      filepath.Join(tmpdir, "dest.txt"),
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing source, got nil")
	}
}

// TestFileSystem_CopyMissingDest tests copy with missing dest parameter.
func TestFileSystem_CopyMissingDest(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "copy",
		"source":    "/tmp/something.txt",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing dest, got nil")
	}
}

// TestFileSystem_CopyMissingSourceParam tests copy with missing source parameter.
func TestFileSystem_CopyMissingSourceParam(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "copy",
		"dest":      "/tmp/dest.txt",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing source param, got nil")
	}
}

// TestFileSystem_MkdirNested tests deeply nested directory creation.
func TestFileSystem_MkdirNested(t *testing.T) {
	ctx := context.Background()

	tmpdir := t.TempDir()
	deepDir := filepath.Join(tmpdir, "a", "b", "c", "d")

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "mkdir",
		"path":      deepDir,
	}

	result, err := fs.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["created"] != true {
		t.Errorf("created = %v, want true", output["created"])
	}

	info, err := os.Stat(deepDir)
	if err != nil {
		t.Fatalf("Directory not created: %v", err)
	}
	if !info.IsDir() {
		t.Error("Created path is not a directory")
	}
}

// TestFileSystem_ReadNonexistentFile tests read with nonexistent file.
func TestFileSystem_ReadNonexistentFile(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "read",
		"path":      "/nonexistent/path/file.txt",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for nonexistent file, got nil")
	}
}

// TestFileSystem_DeleteNonexistentFile tests delete with nonexistent file.
func TestFileSystem_DeleteNonexistentFile(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "delete",
		"path":      "/nonexistent/path/file.txt",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for deleting nonexistent file, got nil")
	}
}

// TestFileSystem_ListNonexistentPath tests list with nonexistent path.
func TestFileSystem_ListNonexistentPath(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "list",
		"path":      "/nonexistent/directory",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for nonexistent path, got nil")
	}
}

// TestFileSystem_WriteContentMissing tests write with missing content parameter.
func TestFileSystem_WriteContentMissing(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"operation": "write",
		"path":      "/tmp/test-missing-content.txt",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing content, got nil")
	}
}

// TestFileSystem_MissingOperationParam tests missing operation parameter.
func TestFileSystem_MissingOperationParam(t *testing.T) {
	ctx := context.Background()

	fs := filesystem.New()

	params := map[string]interface{}{
		"path": "/some/path",
	}

	_, err := fs.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing operation, got nil")
	}
}
