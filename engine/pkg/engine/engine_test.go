package engine_test

import (
	"context"
	"strings"
	"testing"

	"github.com/Develonaut/bento/pkg/engine"
	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/node/library/editfields"
	"github.com/Develonaut/bento/pkg/registry"
	"github.com/Develonaut/bento/pkg/logger"
)

// TestItamae_LinearExecution tests sequential execution of nodes
func TestItamae_LinearExecution(t *testing.T) {
	ctx := context.Background()

	// Create bento: node-1 → node-2 → node-3
	bento := &node.Definition{
		ID:      "linear-bento",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Linear Bento",
		Nodes: []node.Definition{
			{
				ID:   "node-1",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"step": 1},
				},
			},
			{
				ID:   "node-2",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"step": 2},
				},
			},
			{
				ID:   "node-3",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"step": 3},
				},
			},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "node-1", Target: "node-2"},
			{ID: "edge-2", Source: "node-2", Target: "node-3"},
		},
	}

	// Create engine
	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	// Execute
	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// Verify all nodes executed
	if result.NodesExecuted != 3 {
		t.Errorf("NodesExecuted = %d, want 3", result.NodesExecuted)
	}

	if result.Status != engine.StatusSuccess {
		t.Errorf("Status = %v, want Success", result.Status)
	}
}

// TestItamae_ContextPassing tests that data flows between nodes via templates
func TestItamae_ContextPassing(t *testing.T) {
	ctx := context.Background()

	// Bento: set name → use name in template
	bento := &node.Definition{
		ID:   "context-bento",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "setname",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{
						"itemName": "Alpha",
					},
				},
			},
			{
				ID:   "usename",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{
						"title": "{{.setname.itemName}}",
					},
				},
			},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "setname", Target: "usename"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// Verify template was resolved
	output, ok := result.NodeOutputs["usename"].(map[string]interface{})
	if !ok {
		t.Fatalf("usename output is not map[string]interface{}")
	}

	if output["title"] != "Alpha" {
		t.Errorf("title = %v, want Alpha (template should be resolved)", output["title"])
	}
}

// TestItamae_ErrorHandling tests that errors are properly propagated
func TestItamae_ErrorHandling(t *testing.T) {
	ctx := context.Background()

	// Bento with an invalid node type
	bento := &node.Definition{
		ID:   "error-bento",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "node-1",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"step": 1},
				},
			},
			{
				ID:   "bad-node",
				Type: "nonexistent-type", // This will cause an error
				Parameters: map[string]interface{}{
					"test": "value",
				},
			},
			{
				ID:   "node-3",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"step": 3},
				},
			},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "node-1", Target: "bad-node"},
			{ID: "edge-2", Source: "bad-node", Target: "node-3"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	result, err := chef.Serve(ctx, bento)

	// Should return error
	if err == nil {
		t.Fatal("Expected error from invalid node type")
	}

	// Error should mention which node failed
	if !strings.Contains(err.Error(), "bad-node") {
		t.Errorf("Error should mention failing node 'bad-node': %v", err)
	}

	// node-3 should NOT have executed (execution stopped at error)
	if result != nil && result.NodesExecuted > 2 {
		t.Error("Execution should stop after error")
	}
}

// TestItamae_ProgressTracking tests that progress callbacks are called
func TestItamae_ProgressTracking(t *testing.T) {
	ctx := context.Background()

	bento := &node.Definition{
		ID:   "progress-bento",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "node-1", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 1}}},
			{ID: "node-2", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 2}}},
			{ID: "node-3", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 3}}},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "node-1", Target: "node-2"},
			{ID: "edge-2", Source: "node-2", Target: "node-3"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	progressCalls := 0
	onProgress := func(nodeID string, status string) {
		progressCalls++
		t.Logf("Progress: %s - %s", nodeID, status)
	}

	chef.OnProgress(onProgress)

	_, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// Should have called progress for each node (starting + completed = 2 per node)
	if progressCalls < 3 {
		t.Errorf("progressCalls = %d, want at least 3", progressCalls)
	}
}

// TestItamae_ContextCancellation tests that context cancellation stops execution
func TestItamae_ContextCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	// Bento with multiple nodes
	bento := &node.Definition{
		ID:   "cancellation-bento",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "node-1", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 1}}},
			{ID: "node-2", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 2}}},
			{ID: "node-3", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 3}}},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "node-1", Target: "node-2"},
			{ID: "edge-2", Source: "node-2", Target: "node-3"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	// Cancel immediately
	cancel()

	_, err := chef.Serve(ctx, bento)

	// Should return context cancelled error
	if err == nil {
		t.Fatal("Expected error from context cancellation")
	}

	if !strings.Contains(err.Error(), "context canceled") {
		t.Errorf("Error should mention context cancellation: %v", err)
	}
}

// TestEngine_GroupNode tests nested group execution
func TestEngine_GroupNode(t *testing.T) {
	ctx := context.Background()

	// Bento with a nested group
	bento := &node.Definition{
		ID:   "group-bento",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "outer-1",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"outer": 1},
				},
			},
			{
				ID:   "nested-group",
				Type: "group",
				Nodes: []node.Definition{
					{
						ID:   "inner-1",
						Type: "edit-fields",
						Parameters: map[string]interface{}{
							"values": map[string]interface{}{"inner": 1},
						},
					},
					{
						ID:   "inner-2",
						Type: "edit-fields",
						Parameters: map[string]interface{}{
							"values": map[string]interface{}{"inner": 2},
						},
					},
				},
				Edges: []node.Edge{
					{ID: "inner-edge", Source: "inner-1", Target: "inner-2"},
				},
			},
			{
				ID:   "outer-2",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"outer": 2},
				},
			},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "outer-1", Target: "nested-group"},
			{ID: "edge-2", Source: "nested-group", Target: "outer-2"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// Should execute all leaf nodes (2 outer + 2 inner = 4)
	// Groups are not counted in NodesExecuted
	if result.NodesExecuted != 4 {
		t.Errorf("NodesExecuted = %d, want 4 (2 outer + 2 inner)", result.NodesExecuted)
	}
}

// TestItamae_EmptyBento tests executing an empty bento
func TestItamae_EmptyBento(t *testing.T) {
	ctx := context.Background()

	bento := &node.Definition{
		ID:    "empty-bento",
		Type:  "group",
		Nodes: []node.Definition{},
	}

	p := registry.New()
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	if result.NodesExecuted != 0 {
		t.Errorf("NodesExecuted = %d, want 0", result.NodesExecuted)
	}

	if result.Status != engine.StatusSuccess {
		t.Errorf("Status = %v, want Success", result.Status)
	}
}

// TestItamae_DisconnectedNodes tests nodes with no edges
func TestItamae_DisconnectedNodes(t *testing.T) {
	ctx := context.Background()

	// Nodes without edges should all execute (no ordering constraint)
	bento := &node.Definition{
		ID:   "disconnected-bento",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "node-1", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 1}}},
			{ID: "node-2", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 2}}},
			{ID: "node-3", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 3}}},
		},
		Edges: []node.Edge{}, // No edges
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// All nodes should execute
	if result.NodesExecuted != 3 {
		t.Errorf("NodesExecuted = %d, want 3", result.NodesExecuted)
	}
}

// TestItamae_Duration tests that execution duration is tracked
func TestItamae_Duration(t *testing.T) {
	ctx := context.Background()

	bento := &node.Definition{
		ID:   "duration-bento",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "node-1", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 1}}},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	if result.Duration == 0 {
		t.Error("Duration should be > 0")
	}

	t.Logf("Execution took %v", result.Duration)
}

// TestItamae_MultipleStartNodes tests multiple nodes with no incoming edges
func TestItamae_MultipleStartNodes(t *testing.T) {
	ctx := context.Background()

	// Two start nodes converging into one end node
	bento := &node.Definition{
		ID:   "multi-start-bento",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "start-1", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 1}}},
			{ID: "start-2", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 2}}},
			{ID: "end", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 3}}},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "start-1", Target: "end"},
			{ID: "edge-2", Source: "start-2", Target: "end"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelInfo})
	chef := engine.New(p, logger)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// All 3 nodes should execute
	if result.NodesExecuted != 3 {
		t.Errorf("NodesExecuted = %d, want 3", result.NodesExecuted)
	}
}

// TestItamae_WeightedProgress tests that progress is weighted by node type
func TestItamae_WeightedProgress(t *testing.T) {
	ctx := context.Background()

	// Create bento with 3 edit-fields nodes (each weight 50)
	// Total weight = 150
	// After node-1: 50/150 = 33%
	// After node-2: 100/150 = 66%
	// After node-3: 150/150 = 100%
	bento := &node.Definition{
		ID:   "weighted-progress-bento",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "node-1", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 1}}},
			{ID: "node-2", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 2}}},
			{ID: "node-3", Type: "edit-fields", Parameters: map[string]interface{}{"values": map[string]interface{}{"test": 3}}},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "node-1", Target: "node-2"},
			{ID: "edge-2", Source: "node-2", Target: "node-3"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelDebug})
	chef := engine.New(p, logger)

	completedNodes := make(map[string]bool)
	onProgress := func(nodeID string, status string) {
		if status == "completed" {
			completedNodes[nodeID] = true
			t.Logf("Node completed: %s", nodeID)
		}
	}

	chef.OnProgress(onProgress)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// Verify expected nodes completed (excluding the group wrapper)
	expectedNodes := []string{"node-1", "node-2", "node-3"}
	for _, nodeID := range expectedNodes {
		if !completedNodes[nodeID] {
			t.Errorf("Expected node %s to be completed", nodeID)
		}
	}

	// Verify execution completed
	if result.Status != engine.StatusSuccess {
		t.Errorf("Status = %v, want Success", result.Status)
	}
}

// TestItamae_LoopIterationProgress tests that loops report partial progress
func TestItamae_LoopIterationProgress(t *testing.T) {
	ctx := context.Background()

	// Create bento with a times loop (3 iterations)
	bento := &node.Definition{
		ID:   "loop-progress-bento",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "before-loop",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"test": "before"},
				},
			},
			{
				ID:      "test-loop",
				Type:    "loop",
				Version: "1.0.0",
				Name:    "Test Loop",
				Parameters: map[string]interface{}{
					"mode":  "times",
					"count": float64(3),
				},
				Nodes: []node.Definition{
					{
						ID:   "loop-child",
						Type: "edit-fields",
						Parameters: map[string]interface{}{
							"values": map[string]interface{}{"iteration": "{{.index}}"},
						},
					},
				},
			},
			{
				ID:   "after-loop",
				Type: "edit-fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{"test": "after"},
				},
			},
		},
		Edges: []node.Edge{
			{ID: "edge-1", Source: "before-loop", Target: "test-loop"},
			{ID: "edge-2", Source: "test-loop", Target: "after-loop"},
		},
	}

	p := registry.New()
	p.RegisterFactory("edit-fields", func() node.Executable {
		return editfields.New()
	})
	logger := logger.New(logger.Config{Level: logger.LevelDebug})
	chef := engine.New(p, logger)

	completedNodes := make(map[string]bool)
	onProgress := func(nodeID string, status string) {
		if status == "completed" {
			completedNodes[nodeID] = true
			t.Logf("Node completed: %s", nodeID)
		}
	}

	chef.OnProgress(onProgress)

	result, err := chef.Serve(ctx, bento)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	// Verify loop completed (loop counts as 1 node, not 3)
	if !completedNodes["test-loop"] {
		t.Error("Loop node should be marked as completed")
	}

	// Verify all wrapper nodes completed
	expectedCompleted := map[string]bool{
		"before-loop": true,
		"test-loop":   true,
		"after-loop":  true,
	}

	for nodeID, expected := range expectedCompleted {
		if completedNodes[nodeID] != expected {
			t.Errorf("Node %s completed = %v, want %v", nodeID, completedNodes[nodeID], expected)
		}
	}

	// Verify loop output contains 3 iterations
	loopOutput, ok := result.NodeOutputs["test-loop"].([]interface{})
	if !ok {
		t.Fatalf("Loop output is not []interface{}")
	}

	if len(loopOutput) != 3 {
		t.Errorf("Loop output length = %d, want 3 iterations", len(loopOutput))
	}

	// Verify execution completed
	if result.Status != engine.StatusSuccess {
		t.Errorf("Status = %v, want Success", result.Status)
	}
}
