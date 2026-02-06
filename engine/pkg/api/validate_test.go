package api

import (
	"context"
	"testing"

	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/storage"
)

func newTestService(t *testing.T) *BentoService {
	t.Helper()
	return New(DefaultRegistry(), storage.New(t.TempDir()))
}

func TestValidateWorkflow_Valid(t *testing.T) {
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
				Parameters: map[string]interface{}{"values": map[string]interface{}{"key": "val"}},
			},
		},
		Edges: []node.Edge{},
	}

	result, err := svc.ValidateWorkflow(context.Background(), def)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !result.Valid {
		t.Errorf("expected valid, got errors: %v", result.Errors)
	}
}

func TestValidateWorkflow_Invalid(t *testing.T) {
	svc := newTestService(t)
	def := &node.Definition{
		ID:      "",
		Type:    "group",
		Version: "1.0.0",
	}

	result, err := svc.ValidateWorkflow(context.Background(), def)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result.Valid {
		t.Error("expected invalid result")
	}
	if len(result.Errors) == 0 {
		t.Error("expected at least one validation error")
	}
}
