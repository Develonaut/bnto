package execution

import (
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/api"
)

func TestCreateAndGet(t *testing.T) {
	mgr := NewManager()

	exec := mgr.Create("exec-1")
	if exec.ID != "exec-1" {
		t.Fatalf("expected id exec-1, got %s", exec.ID)
	}
	if exec.Status != StatusPending {
		t.Fatalf("expected pending, got %s", exec.Status)
	}

	got := mgr.Get("exec-1")
	if got == nil {
		t.Fatal("expected to find execution")
	}
	if got.ID != "exec-1" {
		t.Fatalf("expected exec-1, got %s", got.ID)
	}
}

func TestGetNotFound(t *testing.T) {
	mgr := NewManager()
	if mgr.Get("nonexistent") != nil {
		t.Fatal("expected nil for nonexistent execution")
	}
}

func TestSetRunning(t *testing.T) {
	mgr := NewManager()
	mgr.Create("exec-1")
	mgr.SetRunning("exec-1")

	exec := mgr.Get("exec-1")
	if exec.Status != StatusRunning {
		t.Fatalf("expected running, got %s", exec.Status)
	}
}

func TestAddProgress(t *testing.T) {
	mgr := NewManager()
	mgr.Create("exec-1")
	mgr.AddProgress("exec-1", "node-1", "started")
	mgr.AddProgress("exec-1", "node-1", "completed")

	exec := mgr.Get("exec-1")
	if len(exec.Progress) != 2 {
		t.Fatalf("expected 2 progress entries, got %d", len(exec.Progress))
	}
	if exec.Progress[0].NodeID != "node-1" {
		t.Errorf("expected node-1, got %s", exec.Progress[0].NodeID)
	}
	if exec.Progress[1].Status != "completed" {
		t.Errorf("expected completed, got %s", exec.Progress[1].Status)
	}
}

func TestComplete(t *testing.T) {
	mgr := NewManager()
	mgr.Create("exec-1")
	mgr.SetRunning("exec-1")

	result := &api.RunResult{Status: "completed", NodesExecuted: 3}
	mgr.Complete("exec-1", result)

	exec := mgr.Get("exec-1")
	if exec.Status != StatusCompleted {
		t.Fatalf("expected completed, got %s", exec.Status)
	}
	if exec.Result == nil {
		t.Fatal("expected result to be set")
	}
	if exec.Result.NodesExecuted != 3 {
		t.Errorf("expected 3 nodes executed, got %d", exec.Result.NodesExecuted)
	}
	if exec.CompletedAt == nil {
		t.Error("expected completedAt to be set")
	}
}

func TestFail(t *testing.T) {
	mgr := NewManager()
	mgr.Create("exec-1")
	mgr.Fail("exec-1", "something broke")

	exec := mgr.Get("exec-1")
	if exec.Status != StatusFailed {
		t.Fatalf("expected failed, got %s", exec.Status)
	}
	if exec.Error != "something broke" {
		t.Errorf("expected error message, got %s", exec.Error)
	}
	if exec.CompletedAt == nil {
		t.Error("expected completedAt to be set")
	}
}

func TestCleanupBefore(t *testing.T) {
	mgr := NewManager()
	mgr.Create("exec-1")
	mgr.Create("exec-2")
	mgr.Create("exec-3")

	// Complete exec-1 and exec-2
	mgr.Complete("exec-1", &api.RunResult{Status: "completed"})
	mgr.Complete("exec-2", &api.RunResult{Status: "completed"})

	// exec-3 is still pending (no completedAt)

	// Cleanup everything completed before 1 second from now
	cutoff := time.Now().Add(time.Second)
	removed := mgr.CleanupBefore(cutoff)

	if removed != 2 {
		t.Fatalf("expected 2 removed, got %d", removed)
	}
	if mgr.Get("exec-1") != nil {
		t.Error("exec-1 should have been cleaned up")
	}
	if mgr.Get("exec-2") != nil {
		t.Error("exec-2 should have been cleaned up")
	}
	if mgr.Get("exec-3") == nil {
		t.Error("exec-3 should still exist (pending)")
	}
}

func TestOperationsOnNonexistent(t *testing.T) {
	mgr := NewManager()

	// These should not panic on nonexistent IDs
	mgr.SetRunning("nope")
	mgr.AddProgress("nope", "node-1", "started")
	mgr.Complete("nope", &api.RunResult{})
	mgr.Fail("nope", "err")
}
