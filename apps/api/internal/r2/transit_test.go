package r2_test

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"testing"

	"github.com/Develonaut/bnto-api/internal/r2"
)

// mockStore implements ObjectStore for testing.
type mockStore struct {
	objects map[string][]byte
}

func newMockStore() *mockStore {
	return &mockStore{objects: make(map[string][]byte)}
}

func (m *mockStore) Put(key string, data []byte) {
	m.objects[key] = data
}

func (m *mockStore) ListObjects(_ context.Context, prefix string) ([]string, error) {
	var keys []string
	for k := range m.objects {
		if strings.HasPrefix(k, prefix) {
			keys = append(keys, k)
		}
	}
	sort.Strings(keys)
	return keys, nil
}

func (m *mockStore) Download(_ context.Context, key string) (*r2.Object, error) {
	data, ok := m.objects[key]
	if !ok {
		return nil, fmt.Errorf("object not found: %s", key)
	}
	return &r2.Object{
		Key:         key,
		Body:        io.NopCloser(bytes.NewReader(data)),
		ContentType: "application/octet-stream",
		Size:        int64(len(data)),
	}, nil
}

func (m *mockStore) Upload(_ context.Context, key string, body io.Reader, _ string) error {
	data, err := io.ReadAll(body)
	if err != nil {
		return err
	}
	m.objects[key] = data
	return nil
}

func (m *mockStore) DeleteObject(_ context.Context, key string) error {
	delete(m.objects, key)
	return nil
}

func TestDownloadSession(t *testing.T) {
	store := newMockStore()
	store.Put("uploads/session-123/image1.png", []byte("png-data-1"))
	store.Put("uploads/session-123/image2.jpg", []byte("jpg-data-2"))

	destDir := t.TempDir()
	err := r2.DownloadSession(context.Background(), store, "session-123", destDir)
	if err != nil {
		t.Fatalf("DownloadSession: %v", err)
	}

	assertFileContent(t, filepath.Join(destDir, "image1.png"), "png-data-1")
	assertFileContent(t, filepath.Join(destDir, "image2.jpg"), "jpg-data-2")
}

func TestDownloadSessionEmpty(t *testing.T) {
	store := newMockStore()
	destDir := t.TempDir()

	err := r2.DownloadSession(context.Background(), store, "no-such-session", destDir)
	if err == nil {
		t.Fatal("expected error for empty session")
	}
	if !strings.Contains(err.Error(), "no files found") {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestDownloadSessionCancelled(t *testing.T) {
	store := newMockStore()
	store.Put("uploads/sess/file.txt", []byte("data"))

	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	err := r2.DownloadSession(ctx, store, "sess", t.TempDir())
	if err == nil {
		t.Fatal("expected error for cancelled context")
	}
}

func TestUploadOutputs(t *testing.T) {
	store := newMockStore()
	outputDir := t.TempDir()

	writeFile(t, filepath.Join(outputDir, "result.png"), "compressed-png")
	writeFile(t, filepath.Join(outputDir, "result.csv"), "col1,col2\na,b")

	files, err := r2.UploadOutputs(context.Background(), store, "exec-456", outputDir)
	if err != nil {
		t.Fatalf("UploadOutputs: %v", err)
	}

	if len(files) != 2 {
		t.Fatalf("expected 2 files, got %d", len(files))
	}

	sort.Slice(files, func(i, j int) bool { return files[i].Name < files[j].Name })

	if files[0].Name != "result.csv" {
		t.Errorf("expected result.csv, got %s", files[0].Name)
	}
	if files[0].Key != "executions/exec-456/output/result.csv" {
		t.Errorf("unexpected key: %s", files[0].Key)
	}
	if !strings.HasPrefix(files[0].ContentType, "text/csv") {
		t.Errorf("expected text/csv prefix, got %s", files[0].ContentType)
	}
	if files[0].SizeBytes != 13 {
		t.Errorf("expected 13 bytes, got %d", files[0].SizeBytes)
	}

	if files[1].Name != "result.png" {
		t.Errorf("expected result.png, got %s", files[1].Name)
	}
	if files[1].ContentType != "image/png" {
		t.Errorf("expected image/png, got %s", files[1].ContentType)
	}

	// Verify data was uploaded to the store.
	uploaded, ok := store.objects["executions/exec-456/output/result.png"]
	if !ok {
		t.Fatal("result.png not found in store")
	}
	if string(uploaded) != "compressed-png" {
		t.Errorf("unexpected uploaded content: %s", uploaded)
	}
}

func TestUploadOutputsEmptyDir(t *testing.T) {
	store := newMockStore()
	outputDir := t.TempDir()

	files, err := r2.UploadOutputs(context.Background(), store, "exec-empty", outputDir)
	if err != nil {
		t.Fatalf("UploadOutputs: %v", err)
	}
	if files != nil {
		t.Errorf("expected nil files for empty dir, got %d", len(files))
	}
}

func TestUploadOutputsSkipsDirectories(t *testing.T) {
	store := newMockStore()
	outputDir := t.TempDir()

	writeFile(t, filepath.Join(outputDir, "file.txt"), "data")
	if err := os.MkdirAll(filepath.Join(outputDir, "subdir"), 0o755); err != nil {
		t.Fatal(err)
	}

	files, err := r2.UploadOutputs(context.Background(), store, "exec-sub", outputDir)
	if err != nil {
		t.Fatalf("UploadOutputs: %v", err)
	}
	if len(files) != 1 {
		t.Fatalf("expected 1 file (subdirs skipped), got %d", len(files))
	}
}

func TestUploadOutputsCancelled(t *testing.T) {
	store := newMockStore()
	outputDir := t.TempDir()
	writeFile(t, filepath.Join(outputDir, "file.txt"), "data")

	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	_, err := r2.UploadOutputs(ctx, store, "exec-cancel", outputDir)
	if err == nil {
		t.Fatal("expected error for cancelled context")
	}
}

// --- helpers ---

func assertFileContent(t *testing.T, path, expected string) {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("reading %s: %v", path, err)
	}
	if string(data) != expected {
		t.Errorf("%s: got %q, want %q", path, string(data), expected)
	}
}

func writeFile(t *testing.T, path, content string) {
	t.Helper()
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
}
