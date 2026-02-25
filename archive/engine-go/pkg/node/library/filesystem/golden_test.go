package filesystem_test

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/filesystem"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_ReadFile(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "hello.txt")
	if err := os.WriteFile(path, []byte("Hello, bnto!"), 0644); err != nil {
		t.Fatal(err)
	}

	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "read",
		"path":      path,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "read_file", result)
}

func TestGolden_WriteFile(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "output.txt")

	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "write",
		"path":      path,
		"content":   "Written by golden test",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "write_file", result)
}

func TestGolden_CopyFile(t *testing.T) {
	dir := t.TempDir()
	src := filepath.Join(dir, "source.txt")
	dst := filepath.Join(dir, "dest.txt")
	if err := os.WriteFile(src, []byte("copy me"), 0644); err != nil {
		t.Fatal(err)
	}

	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "copy",
		"source":    src,
		"dest":      dst,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "copy_file", result)
}

func TestGolden_ListFiles(t *testing.T) {
	dir := t.TempDir()
	for _, name := range []string{"alpha.txt", "beta.txt", "gamma.png"} {
		if err := os.WriteFile(filepath.Join(dir, name), []byte("x"), 0644); err != nil {
			t.Fatal(err)
		}
	}

	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "list",
		"path":      filepath.Join(dir, "*.txt"),
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "list_files", result)
}

func TestGolden_ListDirectory(t *testing.T) {
	dir := t.TempDir()
	for _, name := range []string{"a.txt", "b.png", "c.json"} {
		if err := os.WriteFile(filepath.Join(dir, name), []byte("x"), 0644); err != nil {
			t.Fatal(err)
		}
	}

	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "list",
		"path":      dir,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "list_directory", result)
}

func TestGolden_ExistsTrue(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "exists.txt")
	if err := os.WriteFile(path, []byte("hi"), 0644); err != nil {
		t.Fatal(err)
	}

	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "exists",
		"path":      path,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "exists_true", result)
}

func TestGolden_ExistsFalse(t *testing.T) {
	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "exists",
		"path":      "/nonexistent/path/file.txt",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "exists_false", result)
}

func TestGolden_DeleteFile(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "to-delete.txt")
	if err := os.WriteFile(path, []byte("bye"), 0644); err != nil {
		t.Fatal(err)
	}

	fs := filesystem.New()
	result, err := fs.Execute(context.Background(), map[string]any{
		"operation": "delete",
		"path":      path,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "delete_file", result)
}
