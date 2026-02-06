package validator_test

import (
	"context"
	"strings"
	"testing"

	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/validator"
)

// Test: Valid definition should pass validation
func TestValidator_ValidDefinition(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "http-request",
		Version: "1.0.0",
		Name:    "Fetch User Data",
		Parameters: map[string]interface{}{
			"url":    "https://api.example.com/users",
			"method": "GET",
		},
		InputPorts: []node.Port{
			{ID: "in-1", Name: "input"},
		},
		OutputPorts: []node.Port{
			{ID: "out-1", Name: "output"},
		},
	}

	err := validator.Validate(ctx, def)
	if err != nil {
		t.Fatalf("Valid definition should pass validation: %v", err)
	}
}

// Test: Missing required ID field should fail
func TestValidator_MissingID(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		Type:    "http-request",
		Version: "1.0.0",
		Name:    "Fetch Data",
		Parameters: map[string]interface{}{
			"url":    "https://api.example.com",
			"method": "GET",
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for missing ID")
	}

	errMsg := err.Error()
	if !contains(errMsg, "id") || !contains(errMsg, "required") {
		t.Errorf("Error should mention missing 'id': %s", errMsg)
	}
}

// Test: Missing required Type field should fail
func TestValidator_MissingType(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Version: "1.0.0",
		Name:    "Fetch Data",
		Parameters: map[string]interface{}{
			"url":    "https://api.example.com",
			"method": "GET",
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for missing Type")
	}

	errMsg := err.Error()
	if !contains(errMsg, "type") || !contains(errMsg, "node-1") {
		t.Errorf("Error should mention missing 'type' and node ID: %s", errMsg)
	}
}

// Test: Missing required Version field should fail
func TestValidator_MissingVersion(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:   "node-1",
		Type: "http-request",
		Name: "Fetch Data",
		Parameters: map[string]interface{}{
			"url":    "https://api.example.com",
			"method": "GET",
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for missing Version")
	}

	errMsg := err.Error()
	if !contains(errMsg, "version") || !contains(errMsg, "node-1") {
		t.Errorf("Error should mention missing 'version' and node ID: %s", errMsg)
	}
}

// Test: Invalid node type should fail
func TestValidator_InvalidNetaType(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "invalid-type",
		Version: "1.0.0",
		Name:    "Test",
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for invalid node type")
	}

	errMsg := err.Error()
	if !contains(errMsg, "invalid-type") || !contains(errMsg, "unknown") {
		t.Errorf("Error should mention unknown type: %s", errMsg)
	}
}

// Test: HTTP request missing URL should fail
func TestValidator_HTTPMissingURL(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "http-request",
		Version: "1.0.0",
		Name:    "Fetch Data",
		Parameters: map[string]interface{}{
			"method": "GET",
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for missing URL")
	}

	errMsg := err.Error()
	if !contains(errMsg, "url") && !contains(errMsg, "URL") {
		t.Errorf("Error should mention missing 'url': %s", errMsg)
	}
	if !contains(errMsg, "node-1") {
		t.Errorf("Error should mention node ID: %s", errMsg)
	}
}

// Test: HTTP request with invalid method should fail
func TestValidator_HTTPInvalidMethod(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "http-request",
		Version: "1.0.0",
		Name:    "Fetch Data",
		Parameters: map[string]interface{}{
			"url":    "https://api.example.com",
			"method": "INVALID",
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for invalid HTTP method")
	}

	errMsg := err.Error()
	if !contains(errMsg, "method") || !contains(errMsg, "INVALID") {
		t.Errorf("Error should mention invalid method: %s", errMsg)
	}
}

// Test: HTTP request with valid methods should pass
func TestValidator_HTTPValidMethods(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()
	methods := []string{"GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"}

	for _, method := range methods {
		def := &node.Definition{
			ID:      "node-1",
			Type:    "http-request",
			Version: "1.0.0",
			Name:    "Test",
			Parameters: map[string]interface{}{
				"url":    "https://api.example.com",
				"method": method,
			},
		}

		err := validator.Validate(ctx, def)
		if err != nil {
			t.Errorf("Method %s should be valid: %v", method, err)
		}
	}
}

// Test: Validate group node with nested nodes
func TestValidator_GroupWithNestedNodes(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "group-1",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Main Group",
		Nodes: []node.Definition{
			{
				ID:      "node-1",
				Type:    "edit-fields",
				Version: "1.0.0",
				Name:    "Set Fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{
						"foo": "bar",
					},
				},
			},
			{
				ID:      "node-2",
				Type:    "http-request",
				Version: "1.0.0",
				Name:    "Fetch Data",
				Parameters: map[string]interface{}{
					"url":    "https://api.example.com",
					"method": "GET",
				},
			},
		},
		Edges: []node.Edge{
			{
				ID:     "edge-1",
				Source: "node-1",
				Target: "node-2",
			},
		},
	}

	err := validator.Validate(ctx, def)
	if err != nil {
		t.Fatalf("Valid group should pass validation: %v", err)
	}
}

// Test: Invalid edge (source doesn't exist) should fail
func TestValidator_InvalidEdgeSource(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "group-1",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Main Group",
		Nodes: []node.Definition{
			{
				ID:      "node-1",
				Type:    "edit-fields",
				Version: "1.0.0",
				Name:    "Set Fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{
						"test": "value",
					},
				},
			},
		},
		Edges: []node.Edge{
			{
				ID:     "edge-1",
				Source: "nonexistent-node",
				Target: "node-1",
			},
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for invalid edge source")
	}

	errMsg := err.Error()
	if !contains(errMsg, "nonexistent-node") {
		t.Errorf("Error should mention missing source node: %s", errMsg)
	}
}

// Test: Invalid edge (target doesn't exist) should fail
func TestValidator_InvalidEdgeTarget(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "group-1",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Main Group",
		Nodes: []node.Definition{
			{
				ID:      "node-1",
				Type:    "edit-fields",
				Version: "1.0.0",
				Name:    "Set Fields",
				Parameters: map[string]interface{}{
					"values": map[string]interface{}{
						"test": "value",
					},
				},
			},
		},
		Edges: []node.Edge{
			{
				ID:     "edge-1",
				Source: "node-1",
				Target: "nonexistent-target",
			},
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for invalid edge target")
	}

	errMsg := err.Error()
	if !contains(errMsg, "nonexistent-target") {
		t.Errorf("Error should mention missing target node: %s", errMsg)
	}
}

// Test: Loop node with invalid mode should fail
func TestValidator_LoopInvalidMode(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "loop",
		Version: "1.0.0",
		Name:    "Loop",
		Parameters: map[string]interface{}{
			"mode": "invalid-mode",
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for invalid loop mode")
	}
}

// Test: Loop node with valid modes should pass
func TestValidator_LoopValidModes(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	testCases := []struct {
		mode       string
		extraParam string
		value      interface{}
	}{
		{"forEach", "items", []string{"a", "b"}},
		{"times", "count", 5},
		{"while", "condition", "x < 10"},
	}

	for _, tc := range testCases {
		def := &node.Definition{
			ID:      "node-1",
			Type:    "loop",
			Version: "1.0.0",
			Name:    "Loop",
			Parameters: map[string]interface{}{
				"mode":        tc.mode,
				tc.extraParam: tc.value,
			},
		}

		err := validator.Validate(ctx, def)
		if err != nil {
			t.Errorf("Loop mode %s should be valid: %v", tc.mode, err)
		}
	}
}

// Test: File-system node with invalid operation should fail
func TestValidator_FileSystemInvalidOperation(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "file-system",
		Version: "1.0.0",
		Name:    "File Op",
		Parameters: map[string]interface{}{
			"operation": "invalid-op",
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for invalid file operation")
	}
}

// Test: Shell command missing command parameter should fail
func TestValidator_ShellCommandMissingCommand(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "shell-command",
		Version: "1.0.0",
		Name:    "Run Command",
		Parameters: map[string]interface{}{
			"args": []string{"--version"},
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for missing command")
	}

	errMsg := err.Error()
	if !contains(errMsg, "command") {
		t.Errorf("Error should mention missing 'command': %s", errMsg)
	}
}

// Test: Edit-fields node should validate
func TestValidator_EditFieldsValid(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "node-1",
		Type:    "edit-fields",
		Version: "1.0.0",
		Name:    "Set Fields",
		Parameters: map[string]interface{}{
			"values": map[string]interface{}{
				"foo": "bar",
				"num": 42,
			},
		},
	}

	err := validator.Validate(ctx, def)
	if err != nil {
		t.Fatalf("Valid edit-fields should pass: %v", err)
	}
}

// Test: Nested group validation (group within group)
func TestValidator_NestedGroups(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "group-1",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Outer Group",
		Nodes: []node.Definition{
			{
				ID:      "inner-group",
				Type:    "group",
				Version: "1.0.0",
				Name:    "Inner Group",
				Nodes: []node.Definition{
					{
						ID:      "node-1",
						Type:    "edit-fields",
						Version: "1.0.0",
						Name:    "Set Fields",
						Parameters: map[string]interface{}{
							"values": map[string]interface{}{
								"test": "value",
							},
						},
					},
				},
			},
		},
	}

	err := validator.Validate(ctx, def)
	if err != nil {
		t.Fatalf("Valid nested groups should pass: %v", err)
	}
}

// Test: Invalid child node in group should fail
func TestValidator_InvalidChildNode(t *testing.T) {
	validator := validator.New()
	ctx := context.Background()

	def := &node.Definition{
		ID:      "group-1",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Main Group",
		Nodes: []node.Definition{
			{
				ID:   "node-1",
				Type: "http-request",
				// Missing Version!
				Name: "Fetch Data",
			},
		},
	}

	err := validator.Validate(ctx, def)
	if err == nil {
		t.Fatal("Expected validation error for invalid child node")
	}

	errMsg := err.Error()
	if !contains(errMsg, "group-1") {
		t.Errorf("Error should mention parent group: %s", errMsg)
	}
}

// Helper function
func contains(s, substr string) bool {
	return strings.Contains(strings.ToLower(s), strings.ToLower(substr))
}
