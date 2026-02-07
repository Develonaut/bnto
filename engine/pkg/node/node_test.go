package node_test

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/Develonaut/bnto/pkg/node"
)

// TestExecutableInterface ensures the Executable interface contract is clear.
//
// Every node type must implement the Execute method to participate in workflows.
// This test uses a simple mock implementation to verify the interface works.
func TestExecutableInterface(t *testing.T) {
	ctx := context.Background()

	// Verify mockNode implements Executable interface at compile time
	var _ node.Executable = &mockNode{}

	mock := &mockNode{
		result: map[string]interface{}{"foo": "bar"},
	}

	result, err := mock.Execute(ctx, map[string]interface{}{})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	if result == nil {
		t.Fatal("Expected result, got nil")
	}

	// Verify result structure
	resultMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map[string]interface{}, got %T", result)
	}

	if resultMap["foo"] != "bar" {
		t.Errorf("foo = %v, want bar", resultMap["foo"])
	}
}

// TestDefinitionJSON verifies that Definition can be serialized to/from JSON.
//
// This is critical because:
// 1. Workflows are stored as JSON files
// 2. The CLI reads/writes workflow definitions
// 3. Round-trip serialization must preserve all data
func TestDefinitionJSON(t *testing.T) {
	def := node.Definition{
		ID:       "test-node-1",
		Type:     "edit-fields",
		Version:  "1.0.0",
		Name:     "Test Node",
		Position: node.Position{X: 100, Y: 200},
		Parameters: map[string]interface{}{
			"values": map[string]interface{}{
				"foo": "bar",
			},
		},
		InputPorts: []node.Port{
			{ID: "input-1", Name: "Input"},
		},
		OutputPorts: []node.Port{
			{ID: "output-1", Name: "Output"},
		},
	}

	// Marshal to JSON
	jsonData, err := json.Marshal(def)
	if err != nil {
		t.Fatalf("Marshal failed: %v", err)
	}

	// Unmarshal back
	var decoded node.Definition
	if err := json.Unmarshal(jsonData, &decoded); err != nil {
		t.Fatalf("Unmarshal failed: %v", err)
	}

	// Verify round-trip preservation
	if decoded.ID != def.ID {
		t.Errorf("ID mismatch: got %s, want %s", decoded.ID, def.ID)
	}

	if decoded.Type != def.Type {
		t.Errorf("Type mismatch: got %s, want %s", decoded.Type, def.Type)
	}

	if decoded.Version != def.Version {
		t.Errorf("Version mismatch: got %s, want %s", decoded.Version, def.Version)
	}

	if decoded.Position.X != def.Position.X {
		t.Errorf("Position.X mismatch: got %f, want %f", decoded.Position.X, def.Position.X)
	}

	if decoded.Position.Y != def.Position.Y {
		t.Errorf("Position.Y mismatch: got %f, want %f", decoded.Position.Y, def.Position.Y)
	}
}

// TestDefinitionWithNestedNodes verifies group nodes can contain child nodes.
//
// Group nodes are containers that execute other nodes in sequence or parallel.
// This test ensures the recursive structure works correctly.
func TestDefinitionWithNestedNodes(t *testing.T) {
	childNode := node.Definition{
		ID:      "child-1",
		Type:    "edit-fields",
		Version: "1.0.0",
		Name:    "Child Node",
	}

	groupNode := node.Definition{
		ID:      "group-1",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Group Node",
		Nodes:   []node.Definition{childNode},
		Edges: []node.Edge{
			{
				ID:     "edge-1",
				Source: "child-1",
				Target: "child-2",
			},
		},
	}

	// Marshal to JSON
	jsonData, err := json.Marshal(groupNode)
	if err != nil {
		t.Fatalf("Marshal failed: %v", err)
	}

	// Unmarshal back
	var decoded node.Definition
	if err := json.Unmarshal(jsonData, &decoded); err != nil {
		t.Fatalf("Unmarshal failed: %v", err)
	}

	// Verify nested nodes
	if len(decoded.Nodes) != 1 {
		t.Errorf("Nodes length = %d, want 1", len(decoded.Nodes))
	}

	if len(decoded.Edges) != 1 {
		t.Errorf("Edges length = %d, want 1", len(decoded.Edges))
	}

	if decoded.Nodes[0].ID != "child-1" {
		t.Errorf("Child node ID = %s, want child-1", decoded.Nodes[0].ID)
	}
}

// TestPortAndEdge verifies Port and Edge structures serialize correctly.
//
// Ports are connection points on nodes, edges connect them together.
// This is how the workflow graph is constructed.
func TestPortAndEdge(t *testing.T) {
	port := node.Port{
		ID:     "port-1",
		Name:   "Input Port",
		Handle: "source",
	}

	edge := node.Edge{
		ID:           "edge-1",
		Source:       "node-1",
		Target:       "node-2",
		SourceHandle: "output",
		TargetHandle: "input",
	}

	// Test Port serialization
	portJSON, err := json.Marshal(port)
	if err != nil {
		t.Fatalf("Port marshal failed: %v", err)
	}

	var decodedPort node.Port
	if err := json.Unmarshal(portJSON, &decodedPort); err != nil {
		t.Fatalf("Port unmarshal failed: %v", err)
	}

	if decodedPort.ID != port.ID {
		t.Errorf("Port ID = %s, want %s", decodedPort.ID, port.ID)
	}

	// Test Edge serialization
	edgeJSON, err := json.Marshal(edge)
	if err != nil {
		t.Fatalf("Edge marshal failed: %v", err)
	}

	var decodedEdge node.Edge
	if err := json.Unmarshal(edgeJSON, &decodedEdge); err != nil {
		t.Fatalf("Edge unmarshal failed: %v", err)
	}

	if decodedEdge.Source != edge.Source {
		t.Errorf("Edge Source = %s, want %s", decodedEdge.Source, edge.Source)
	}

	if decodedEdge.Target != edge.Target {
		t.Errorf("Edge Target = %s, want %s", decodedEdge.Target, edge.Target)
	}
}

// mockNode is a simple implementation of Executable for testing.
// It returns a predefined result without side effects.
type mockNode struct {
	result map[string]interface{}
}

// Execute implements the Executable interface.
func (m *mockNode) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	return m.result, nil
}
