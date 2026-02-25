package r2_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bnto-api/internal/r2"
)

func TestCleanupSession(t *testing.T) {
	store := newMockStore()
	store.Put("uploads/sess-abc/file1.png", []byte("data1"))
	store.Put("uploads/sess-abc/file2.jpg", []byte("data2"))
	store.Put("uploads/other-sess/keep.txt", []byte("keep"))

	err := r2.CleanupSession(context.Background(), store, "sess-abc")
	if err != nil {
		t.Fatalf("CleanupSession: %v", err)
	}

	// Session files should be deleted.
	keys, _ := store.ListObjects(context.Background(), "uploads/sess-abc/")
	if len(keys) != 0 {
		t.Errorf("expected 0 remaining objects, got %d", len(keys))
	}

	// Other sessions should be untouched.
	keys, _ = store.ListObjects(context.Background(), "uploads/other-sess/")
	if len(keys) != 1 {
		t.Errorf("expected 1 remaining object in other session, got %d", len(keys))
	}
}

func TestCleanupSessionEmpty(t *testing.T) {
	store := newMockStore()

	err := r2.CleanupSession(context.Background(), store, "nonexistent")
	if err != nil {
		t.Fatalf("CleanupSession on empty prefix should not error: %v", err)
	}
}

func TestCleanupOutputs(t *testing.T) {
	store := newMockStore()
	store.Put("executions/exec-123/output/result.png", []byte("output1"))
	store.Put("executions/exec-123/output/result.csv", []byte("output2"))
	store.Put("executions/exec-456/output/other.txt", []byte("other"))

	err := r2.CleanupOutputs(context.Background(), store, "exec-123")
	if err != nil {
		t.Fatalf("CleanupOutputs: %v", err)
	}

	keys, _ := store.ListObjects(context.Background(), "executions/exec-123/output/")
	if len(keys) != 0 {
		t.Errorf("expected 0 remaining objects, got %d", len(keys))
	}

	// Other executions should be untouched.
	keys, _ = store.ListObjects(context.Background(), "executions/exec-456/output/")
	if len(keys) != 1 {
		t.Errorf("expected 1 remaining object in other execution, got %d", len(keys))
	}
}

func TestCleanupCancelled(t *testing.T) {
	store := newMockStore()
	store.Put("uploads/sess/file.txt", []byte("data"))

	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	err := r2.CleanupSession(ctx, store, "sess")
	if err == nil {
		t.Fatal("expected error for cancelled context")
	}
}

func TestCleanupSessionBestEffort(t *testing.T) {
	store := newMockStore()
	store.Put("uploads/sess-ok/file.txt", []byte("data"))

	// Should not panic or error — just logs.
	r2.CleanupSessionBestEffort(context.Background(), store, "sess-ok")

	keys, _ := store.ListObjects(context.Background(), "uploads/sess-ok/")
	if len(keys) != 0 {
		t.Errorf("expected cleanup to delete files, got %d remaining", len(keys))
	}
}
