package api

import (
	"context"
	"encoding/json"
	"os"
	"sync"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/storage"
)

// loadFixture reads a .bento.json fixture file relative to the engine root.
func loadFixture(t *testing.T, path string) *node.Definition {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("failed to read fixture %s: %v", path, err)
	}
	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		t.Fatalf("failed to parse fixture %s: %v", path, err)
	}
	return &def
}

// chdirToEngineRoot changes to the engine root so fixture relative paths resolve.
func chdirToEngineRoot(t *testing.T) {
	t.Helper()
	orig, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get working directory: %v", err)
	}
	// pkg/api/ → engine root is ../../
	if err := os.Chdir("../../"); err != nil {
		t.Fatalf("failed to change to engine root: %v", err)
	}
	t.Cleanup(func() { _ = os.Chdir(orig) })
}

func TestRunWorkflow_Success(t *testing.T) {
	chdirToEngineRoot(t)
	svc := newTestService(t)
	def := loadFixture(t, "tests/fixtures/workflows/edit-fields-pipeline.bento.json")

	result, err := svc.RunWorkflow(context.Background(), def, RunOptions{
		Timeout: 30 * time.Second,
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result.Status != "success" {
		t.Errorf("expected status success, got %s", result.Status)
	}
	if result.NodesExecuted == 0 {
		t.Error("expected at least one node to execute")
	}
	if result.Duration == 0 {
		t.Error("expected non-zero duration")
	}
}

func TestRunWorkflow_Failure(t *testing.T) {
	svc := newTestService(t)
	def := &node.Definition{
		ID:      "bad-group",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Bad Group",
		Nodes: []node.Definition{
			{
				ID:         "bad-node",
				Type:       "shell-command",
				Version:    "1.0.0",
				Name:       "Missing Command",
				Parameters: map[string]interface{}{"command": "this-command-does-not-exist-xyz"},
			},
		},
		Edges: []node.Edge{},
	}

	result, err := svc.RunWorkflow(context.Background(), def, RunOptions{
		Timeout: 5 * time.Second,
	})
	if err == nil {
		t.Fatal("expected error for bad command")
	}
	if result.Status != "failed" {
		t.Errorf("expected status failed, got %s", result.Status)
	}
	if result.Error == "" {
		t.Error("expected error message")
	}
}

func TestRunWorkflow_Timeout(t *testing.T) {
	svc := newTestService(t)
	def := &node.Definition{
		ID:      "slow-group",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Slow Group",
		Nodes: []node.Definition{
			{
				ID:      "slow-cmd",
				Type:    "shell-command",
				Version: "1.0.0",
				Name:    "Sleep Forever",
				Parameters: map[string]interface{}{
					"command": "sleep",
					"args":    []interface{}{"60"},
				},
			},
		},
		Edges: []node.Edge{},
	}

	result, err := svc.RunWorkflow(context.Background(), def, RunOptions{
		Timeout: 100 * time.Millisecond,
	})
	if err == nil {
		t.Fatal("expected timeout error")
	}
	if result.Status != "failed" {
		t.Errorf("expected status failed, got %s", result.Status)
	}
}

func TestRunWorkflow_Progress(t *testing.T) {
	chdirToEngineRoot(t)
	svc := newTestService(t)
	def := loadFixture(t, "tests/fixtures/workflows/edit-fields-pipeline.bento.json")

	var mu sync.Mutex
	var events []string

	result, err := svc.RunWorkflow(context.Background(), def, RunOptions{
		Timeout: 30 * time.Second,
		OnProgress: func(nodeID, status string) {
			mu.Lock()
			defer mu.Unlock()
			events = append(events, nodeID+":"+status)
		},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result.Status != "success" {
		t.Errorf("expected success, got %s", result.Status)
	}

	mu.Lock()
	count := len(events)
	mu.Unlock()
	if count == 0 {
		t.Error("expected progress events")
	}
}

func TestDryRunWorkflow(t *testing.T) {
	svc := newTestService(t)
	def := &node.Definition{
		ID:      "test-group",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Test Group",
		Nodes: []node.Definition{
			{
				ID:         "step1",
				Type:       "edit-fields",
				Version:    "1.0.0",
				Name:       "Set Fields",
				Parameters: map[string]interface{}{"values": map[string]interface{}{"a": 1}},
			},
			{
				ID:         "step2",
				Type:       "transform",
				Version:    "1.0.0",
				Name:       "Transform",
				Parameters: map[string]interface{}{},
			},
		},
		Edges: []node.Edge{{ID: "e1", Source: "step1", Target: "step2"}},
	}

	result, err := svc.DryRunWorkflow(context.Background(), def)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !result.Valid {
		t.Error("expected valid dry run")
	}
	if result.NodeCount != 2 {
		t.Errorf("expected 2 nodes, got %d", result.NodeCount)
	}
	if result.Nodes[0].ID != "step1" {
		t.Errorf("expected first node ID step1, got %s", result.Nodes[0].ID)
	}
}

func TestRunWorkflow_ValidationError(t *testing.T) {
	svc := newTestService(t)
	def := &node.Definition{
		ID:   "",
		Type: "group",
	}

	_, err := svc.RunWorkflow(context.Background(), def, RunOptions{})
	if err == nil {
		t.Fatal("expected validation error")
	}
}

func newTestServiceWithStorage(t *testing.T) (*BentoService, string) {
	t.Helper()
	dir := t.TempDir()
	return New(DefaultRegistry(), storage.New(dir)), dir
}
