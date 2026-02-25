package engine

import (
	"testing"

	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

// TestGolden_AnalyzeGraphLeafNodes verifies graph analysis for flat leaf nodes.
func TestGolden_AnalyzeGraphLeafNodes(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "edit-1", Type: "edit-fields"},
			{ID: "shell-1", Type: "shell-command"},
			{ID: "http-1", Type: "http-request"},
		},
	}

	graph := analyzeGraph(def)

	// Build stable output: node details sorted by ID
	nodes := make(map[string]map[string]any)
	for id, n := range graph.Nodes {
		nodes[id] = map[string]any{
			"id":     n.ID,
			"type":   n.Type,
			"weight": n.Weight,
			"level":  n.Level,
		}
	}

	testgolden.AssertGolden(t, "analyze_graph_leaf_nodes", map[string]any{
		"nodeCount":   len(graph.Nodes),
		"totalWeight": graph.TotalWeight,
		"nodes":       nodes,
	})
}

// TestGolden_AnalyzeGraphNestedGroups verifies that groups are transparent
// (flattened) in the execution graph.
func TestGolden_AnalyzeGraphNestedGroups(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "outer-1", Type: "edit-fields"},
			{
				ID:   "inner-group",
				Type: "group",
				Nodes: []node.Definition{
					{ID: "inner-1", Type: "transform"},
					{ID: "inner-2", Type: "file-system"},
				},
			},
			{ID: "outer-2", Type: "image"},
		},
	}

	graph := analyzeGraph(def)

	nodes := make(map[string]map[string]any)
	for id, n := range graph.Nodes {
		nodes[id] = map[string]any{
			"id":     n.ID,
			"type":   n.Type,
			"weight": n.Weight,
			"level":  n.Level,
		}
	}

	testgolden.AssertGolden(t, "analyze_graph_nested_groups", map[string]any{
		"nodeCount":   len(graph.Nodes),
		"totalWeight": graph.TotalWeight,
		"nodes":       nodes,
	})
}

// TestGolden_AnalyzeGraphLoopOpaque verifies that loops are opaque in the
// execution graph (children are hidden, loop counts as single weighted node).
func TestGolden_AnalyzeGraphLoopOpaque(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "before", Type: "edit-fields"},
			{
				ID:   "my-loop",
				Type: "loop",
				Nodes: []node.Definition{
					{ID: "loop-child-1", Type: "shell-command"},
					{ID: "loop-child-2", Type: "http-request"},
				},
			},
			{ID: "after", Type: "transform"},
		},
	}

	graph := analyzeGraph(def)

	nodes := make(map[string]map[string]any)
	for id, n := range graph.Nodes {
		nodes[id] = map[string]any{
			"id":     n.ID,
			"type":   n.Type,
			"weight": n.Weight,
			"level":  n.Level,
		}
	}

	testgolden.AssertGolden(t, "analyze_graph_loop_opaque", map[string]any{
		"nodeCount":   len(graph.Nodes),
		"totalWeight": graph.TotalWeight,
		"nodes":       nodes,
	})
}

// TestGolden_AnalyzeGraphEmpty verifies graph analysis for an empty definition.
func TestGolden_AnalyzeGraphEmpty(t *testing.T) {
	def := &node.Definition{
		ID:    "root",
		Type:  "group",
		Nodes: []node.Definition{},
	}

	graph := analyzeGraph(def)

	testgolden.AssertGolden(t, "analyze_graph_empty", map[string]any{
		"nodeCount":   len(graph.Nodes),
		"totalWeight": graph.TotalWeight,
	})
}

// TestGolden_NodeWeights verifies weight assignment for all known node types.
func TestGolden_NodeWeights(t *testing.T) {
	nodeTypes := []string{
		"shell-command",
		"http-request",
		"image",
		"spreadsheet",
		"loop",
		"file-system",
		"edit-fields",
		"transform",
		"unknown-type",
	}

	weights := make(map[string]int)
	for _, nt := range nodeTypes {
		weights[nt] = getNodeWeight(nt)
	}

	testgolden.AssertGolden(t, "node_weights", weights)
}

// TestGolden_ExecutionStateProgressSequence verifies the full lifecycle of
// progress tracking as nodes transition through states.
func TestGolden_ExecutionStateProgressSequence(t *testing.T) {
	// Build a graph with 3 nodes of different weights
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "fast-node", Type: "edit-fields"},     // weight 50
			{ID: "medium-node", Type: "spreadsheet"},   // weight 100
			{ID: "slow-node", Type: "shell-command"},    // weight 300
		},
	}

	graph := analyzeGraph(def)
	state := newExecutionState(graph)

	// Capture progress at each transition
	steps := make([]map[string]any, 0)

	// Step 0: Initial state (all pending)
	steps = append(steps, map[string]any{
		"step":     "initial",
		"progress": state.getProgress(),
	})

	// Step 1: fast-node starts executing
	state.setNodeState("fast-node", "executing")
	steps = append(steps, map[string]any{
		"step":     "fast-node executing",
		"progress": state.getProgress(),
	})

	// Step 2: fast-node 50% done
	state.setNodeProgress("fast-node", 50, "halfway")
	steps = append(steps, map[string]any{
		"step":     "fast-node 50%",
		"progress": state.getProgress(),
	})

	// Step 3: fast-node completed
	state.setNodeState("fast-node", "completed")
	steps = append(steps, map[string]any{
		"step":     "fast-node completed",
		"progress": state.getProgress(),
	})

	// Step 4: medium-node starts and completes
	state.setNodeState("medium-node", "executing")
	state.setNodeState("medium-node", "completed")
	steps = append(steps, map[string]any{
		"step":     "medium-node completed",
		"progress": state.getProgress(),
	})

	// Step 5: slow-node partial progress
	state.setNodeState("slow-node", "executing")
	state.setNodeProgress("slow-node", 33, "one third")
	steps = append(steps, map[string]any{
		"step":     "slow-node 33%",
		"progress": state.getProgress(),
	})

	// Step 6: All done
	state.setNodeState("slow-node", "completed")
	steps = append(steps, map[string]any{
		"step":     "all completed",
		"progress": state.getProgress(),
	})

	testgolden.AssertGolden(t, "execution_state_progress_sequence", map[string]any{
		"totalWeight": graph.TotalWeight,
		"steps":       steps,
	})
}

// TestGolden_ExecutionStateProgressClamp verifies that progress values
// are clamped to 0-100 range.
func TestGolden_ExecutionStateProgressClamp(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "node-1", Type: "edit-fields"},
		},
	}

	graph := analyzeGraph(def)
	state := newExecutionState(graph)

	state.setNodeState("node-1", "executing")

	// Try setting progress beyond bounds
	state.setNodeProgress("node-1", -50, "below zero")
	belowZero := state.getProgress()

	state.setNodeProgress("node-1", 150, "above hundred")
	aboveHundred := state.getProgress()

	state.setNodeProgress("node-1", 50, "normal")
	normal := state.getProgress()

	testgolden.AssertGolden(t, "execution_state_progress_clamp", map[string]any{
		"belowZero":    belowZero,
		"aboveHundred": aboveHundred,
		"normal":       normal,
	})
}

// TestGolden_ExecutionStateUnknownNode verifies that setting state on an
// unknown node ID is a no-op (doesn't panic or corrupt state).
func TestGolden_ExecutionStateUnknownNode(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "real-node", Type: "edit-fields"},
		},
	}

	graph := analyzeGraph(def)
	state := newExecutionState(graph)

	// These should be no-ops
	state.setNodeState("nonexistent", "executing")
	state.setNodeProgress("nonexistent", 50, "ghost")

	// Real node should still be at 0 progress
	testgolden.AssertGolden(t, "execution_state_unknown_node", map[string]any{
		"progress": state.getProgress(),
	})
}

// TestGolden_ExecutionStateEmptyGraph verifies progress for a graph with no nodes.
func TestGolden_ExecutionStateEmptyGraph(t *testing.T) {
	def := &node.Definition{
		ID:    "root",
		Type:  "group",
		Nodes: []node.Definition{},
	}

	graph := analyzeGraph(def)
	state := newExecutionState(graph)

	testgolden.AssertGolden(t, "execution_state_empty_graph", map[string]any{
		"progress":    state.getProgress(),
		"totalWeight": graph.TotalWeight,
	})
}
