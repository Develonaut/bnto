package engine

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/node/library/editfields"
	"github.com/Develonaut/bnto/pkg/registry"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

// TestGolden_ProgressCallbackEvents verifies the exact sequence of progress
// callback events during execution. This locks in the contract that UI
// consumers depend on.
func TestGolden_ProgressCallbackEvents(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "node-a", Type: "edit-fields", Parameters: map[string]any{"values": map[string]any{"x": 1}}},
			{ID: "node-b", Type: "edit-fields", Parameters: map[string]any{"values": map[string]any{"y": 2}}},
		},
		Edges: []node.Edge{
			{ID: "e1", Source: "node-a", Target: "node-b"},
		},
	}

	reg := registry.New()
	reg.RegisterFactory("edit-fields", func() node.Executable { return editfields.New() })

	eng := New(reg, nil)

	var mu sync.Mutex
	var events []map[string]any

	eng.OnProgress(func(nodeID, status string) {
		mu.Lock()
		defer mu.Unlock()
		events = append(events, map[string]any{
			"nodeID": nodeID,
			"status": status,
		})
	})

	_, err := eng.Serve(context.Background(), def)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	testgolden.AssertGolden(t, "progress_callback_events", map[string]any{
		"eventCount": len(events),
		"events":     events,
	})
}

// TestGolden_ProgressLoopIterationReporting verifies that a loop node
// reports incremental progress as iterations complete. This is critical
// for UI progress bars during long-running loops.
func TestGolden_ProgressLoopIterationReporting(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "test-loop",
				Type: "loop",
				Parameters: map[string]any{
					"mode":  "times",
					"count": float64(5),
				},
				Nodes: []node.Definition{
					{
						ID:   "loop-child",
						Type: "edit-fields",
						Parameters: map[string]any{
							"values": map[string]any{"i": "{{.index}}"},
						},
					},
				},
			},
		},
	}

	reg := registry.New()
	reg.RegisterFactory("edit-fields", func() node.Executable { return editfields.New() })

	eng := New(reg, nil)

	var mu sync.Mutex
	var events []map[string]any

	eng.OnProgress(func(nodeID, status string) {
		mu.Lock()
		defer mu.Unlock()
		events = append(events, map[string]any{
			"nodeID": nodeID,
			"status": status,
		})
	})

	_, err := eng.Serve(context.Background(), def)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	testgolden.AssertGolden(t, "progress_loop_iteration_reporting", map[string]any{
		"eventCount": len(events),
		"events":     events,
	})
}

// TestGolden_ProgressNestedGroupFlattenedWeights verifies that nested
// group children contribute their individual weights to the progress graph,
// not the group's weight.
func TestGolden_ProgressNestedGroupFlattenedWeights(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{ID: "top-node", Type: "edit-fields", Parameters: map[string]any{"values": map[string]any{"x": 1}}},
			{
				ID:   "inner-group",
				Type: "group",
				Nodes: []node.Definition{
					{ID: "inner-1", Type: "shell-command", Parameters: map[string]any{
						"command": "echo", "args": []any{"hello"},
					}},
					{ID: "inner-2", Type: "edit-fields", Parameters: map[string]any{"values": map[string]any{"y": 2}}},
				},
				Edges: []node.Edge{
					{ID: "ie1", Source: "inner-1", Target: "inner-2"},
				},
			},
			{ID: "bottom-node", Type: "edit-fields", Parameters: map[string]any{"values": map[string]any{"z": 3}}},
		},
		Edges: []node.Edge{
			{ID: "e1", Source: "top-node", Target: "inner-group"},
			{ID: "e2", Source: "inner-group", Target: "bottom-node"},
		},
	}

	graph := analyzeGraph(def)

	// Verify the group itself is NOT in the graph (it's transparent)
	_, groupInGraph := graph.Nodes["inner-group"]

	nodes := make(map[string]map[string]any)
	for id, n := range graph.Nodes {
		nodes[id] = map[string]any{
			"type":   n.Type,
			"weight": n.Weight,
		}
	}

	testgolden.AssertGolden(t, "progress_nested_group_flattened_weights", map[string]any{
		"groupInGraph": groupInGraph,
		"totalWeight":  graph.TotalWeight,
		"nodeCount":    len(graph.Nodes),
		"nodes":        nodes,
	})
}

// TestGolden_ProgressLoopOpaqueWeight verifies that a loop with many
// heavy children still counts as a single node with loop weight.
// This documents the current limitation: a loop of 100 shell-commands
// gets the same weight as a loop of 2 edit-fields operations.
func TestGolden_ProgressLoopOpaqueWeight(t *testing.T) {
	// Heavy loop: 10 shell-command children
	heavyLoop := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "heavy-loop",
				Type: "loop",
				Nodes: []node.Definition{
					{ID: "child-1", Type: "shell-command"},
					{ID: "child-2", Type: "shell-command"},
					{ID: "child-3", Type: "shell-command"},
					{ID: "child-4", Type: "shell-command"},
					{ID: "child-5", Type: "shell-command"},
				},
			},
		},
	}

	// Light loop: 1 edit-fields child
	lightLoop := &node.Definition{
		ID:   "root",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "light-loop",
				Type: "loop",
				Nodes: []node.Definition{
					{ID: "child-1", Type: "edit-fields"},
				},
			},
		},
	}

	heavyGraph := analyzeGraph(heavyLoop)
	lightGraph := analyzeGraph(lightLoop)

	testgolden.AssertGolden(t, "progress_loop_opaque_weight", map[string]any{
		"heavyLoop": map[string]any{
			"totalWeight":    heavyGraph.TotalWeight,
			"loopWeight":     heavyGraph.Nodes["heavy-loop"].Weight,
			"childrenHidden": len(heavyGraph.Nodes) == 1,
		},
		"lightLoop": map[string]any{
			"totalWeight":    lightGraph.TotalWeight,
			"loopWeight":     lightGraph.Nodes["light-loop"].Weight,
			"childrenHidden": len(lightGraph.Nodes) == 1,
		},
		"sameWeight": heavyGraph.TotalWeight == lightGraph.TotalWeight,
	})
}

// TestGolden_MessengerEventSequence verifies the exact sequence of
// ProgressMessenger events during a mixed workflow execution.
// This is the contract between the engine and any TUI/web progress UI.
func TestGolden_MessengerEventSequence(t *testing.T) {
	def := &node.Definition{
		ID:   "root",
		Type: "group",
		Name: "Test Workflow",
		Nodes: []node.Definition{
			{
				ID:   "step-1",
				Type: "edit-fields",
				Name: "Set Data",
				Parameters: map[string]any{
					"values": map[string]any{"key": "value"},
				},
			},
			{
				ID:   "step-2",
				Type: "edit-fields",
				Name: "Transform",
				Parameters: map[string]any{
					"values": map[string]any{"result": "done"},
				},
			},
		},
		Edges: []node.Edge{
			{ID: "e1", Source: "step-1", Target: "step-2"},
		},
	}

	reg := registry.New()
	reg.RegisterFactory("edit-fields", func() node.Executable { return editfields.New() })

	var mu sync.Mutex
	var messengerEvents []map[string]any

	messenger := &testMessenger{
		onNodeStarted: func(path, name, nodeType string) {
			mu.Lock()
			defer mu.Unlock()
			messengerEvents = append(messengerEvents, map[string]any{
				"event":    "nodeStarted",
				"path":     path,
				"name":     name,
				"nodeType": nodeType,
			})
		},
		onNodeCompleted: func(path string, duration time.Duration, err error) {
			mu.Lock()
			defer mu.Unlock()
			errStr := ""
			if err != nil {
				errStr = err.Error()
			}
			messengerEvents = append(messengerEvents, map[string]any{
				"event": "nodeCompleted",
				"path":  path,
				"error": errStr,
			})
		},
		onLoopChild: func(loopPath, childName string, index, total int) {
			mu.Lock()
			defer mu.Unlock()
			messengerEvents = append(messengerEvents, map[string]any{
				"event":     "loopChild",
				"loopPath":  loopPath,
				"childName": childName,
				"index":     index,
				"total":     total,
			})
		},
	}

	eng := NewWithMessenger(reg, nil, messenger)

	_, err := eng.Serve(context.Background(), def)
	if err != nil {
		t.Fatalf("Serve failed: %v", err)
	}

	testgolden.AssertGolden(t, "messenger_event_sequence", map[string]any{
		"eventCount": len(messengerEvents),
		"events":     messengerEvents,
	})
}

// testMessenger is a mock ProgressMessenger that records events.
type testMessenger struct {
	onNodeStarted   func(path, name, nodeType string)
	onNodeCompleted func(path string, duration time.Duration, err error)
	onLoopChild     func(loopPath, childName string, index, total int)
}

func (m *testMessenger) SendNodeStarted(path, name, nodeType string) {
	if m.onNodeStarted != nil {
		m.onNodeStarted(path, name, nodeType)
	}
}

func (m *testMessenger) SendNodeCompleted(path string, duration time.Duration, err error) {
	if m.onNodeCompleted != nil {
		m.onNodeCompleted(path, duration, err)
	}
}

func (m *testMessenger) SendLoopChild(loopPath, childName string, index, total int) {
	if m.onLoopChild != nil {
		m.onLoopChild(loopPath, childName, index, total)
	}
}
