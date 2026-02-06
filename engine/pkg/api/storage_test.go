package api

import (
	"context"
	"testing"

	"github.com/Develonaut/bento/pkg/node"
)

func TestSaveAndGetWorkflow(t *testing.T) {
	svc, _ := newTestServiceWithStorage(t)
	ctx := context.Background()

	def := &node.Definition{
		ID:      "wf-1",
		Type:    "group",
		Version: "1.0.0",
		Name:    "My Workflow",
		Nodes: []node.Definition{
			{ID: "n1", Type: "edit-fields", Version: "1.0.0", Name: "Step 1"},
		},
	}

	if err := svc.SaveWorkflow(ctx, "test-workflow", def); err != nil {
		t.Fatalf("save failed: %v", err)
	}

	loaded, err := svc.GetWorkflow(ctx, "test-workflow")
	if err != nil {
		t.Fatalf("get failed: %v", err)
	}
	if loaded.Name != "My Workflow" {
		t.Errorf("expected name 'My Workflow', got %q", loaded.Name)
	}
}

func TestListWorkflows(t *testing.T) {
	svc, _ := newTestServiceWithStorage(t)
	ctx := context.Background()

	// Empty initially
	list, err := svc.ListWorkflows(ctx)
	if err != nil {
		t.Fatalf("list failed: %v", err)
	}
	if len(list) != 0 {
		t.Errorf("expected empty list, got %d", len(list))
	}

	// Save two workflows
	def1 := &node.Definition{
		ID: "wf-1", Type: "group", Version: "1.0.0", Name: "WF One",
		Nodes: []node.Definition{
			{ID: "n1", Type: "edit-fields", Version: "1.0.0"},
			{ID: "n2", Type: "transform", Version: "1.0.0"},
		},
	}
	def2 := &node.Definition{
		ID: "wf-2", Type: "group", Version: "1.0.0", Name: "WF Two",
	}

	if err := svc.SaveWorkflow(ctx, "alpha", def1); err != nil {
		t.Fatalf("save alpha: %v", err)
	}
	if err := svc.SaveWorkflow(ctx, "beta", def2); err != nil {
		t.Fatalf("save beta: %v", err)
	}

	list, err = svc.ListWorkflows(ctx)
	if err != nil {
		t.Fatalf("list failed: %v", err)
	}
	if len(list) != 2 {
		t.Fatalf("expected 2 workflows, got %d", len(list))
	}

	// Find alpha — should have 2 nodes
	found := false
	for _, s := range list {
		if s.Name == "alpha" {
			found = true
			if s.NodeCount != 2 {
				t.Errorf("expected alpha to have 2 nodes, got %d", s.NodeCount)
			}
		}
	}
	if !found {
		t.Error("expected to find workflow 'alpha'")
	}
}

func TestDeleteWorkflow(t *testing.T) {
	svc, _ := newTestServiceWithStorage(t)
	ctx := context.Background()

	def := &node.Definition{
		ID: "wf-1", Type: "group", Version: "1.0.0", Name: "Deletable",
	}
	if err := svc.SaveWorkflow(ctx, "to-delete", def); err != nil {
		t.Fatalf("save failed: %v", err)
	}

	if err := svc.DeleteWorkflow(ctx, "to-delete"); err != nil {
		t.Fatalf("delete failed: %v", err)
	}

	_, err := svc.GetWorkflow(ctx, "to-delete")
	if err == nil {
		t.Error("expected error after deletion")
	}
}

func TestGetWorkflow_NotFound(t *testing.T) {
	svc, _ := newTestServiceWithStorage(t)

	_, err := svc.GetWorkflow(context.Background(), "nonexistent")
	if err == nil {
		t.Error("expected error for nonexistent workflow")
	}
}

func TestDeleteWorkflow_NotFound(t *testing.T) {
	svc, _ := newTestServiceWithStorage(t)

	err := svc.DeleteWorkflow(context.Background(), "nonexistent")
	if err == nil {
		t.Error("expected error for nonexistent workflow")
	}
}
