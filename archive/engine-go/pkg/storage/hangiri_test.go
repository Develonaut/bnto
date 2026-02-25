package storage_test

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"

	"github.com/Develonaut/bnto/pkg/storage"
	"github.com/Develonaut/bnto/pkg/node"
)

// Test: Save and load a bnto definition
func TestStorage_SaveAndLoad(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir() // Go creates temp dir, cleans up after test
	storage := storage.New(tempDir)

	// Create test definition
	def := &node.Definition{
		ID:      "node-1",
		Type:    "http-request",
		Version: "1.0.0",
		Name:    "Test Workflow",
		Parameters: map[string]interface{}{
			"url":    "https://api.example.com",
			"method": "GET",
		},
	}

	// Save
	err := storage.Save(ctx, "test-workflow", def)
	if err != nil {
		t.Fatalf("Save failed: %v", err)
	}

	// Load
	loaded, err := storage.Load(ctx, "test-workflow")
	if err != nil {
		t.Fatalf("Load failed: %v", err)
	}

	// Verify
	if loaded.ID != def.ID {
		t.Errorf("Expected ID %s, got %s", def.ID, loaded.ID)
	}
	if loaded.Type != def.Type {
		t.Errorf("Expected Type %s, got %s", def.Type, loaded.Type)
	}
}

// Test: List saved bntos
func TestStorage_List(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	// Save multiple bntos
	def1 := &node.Definition{ID: "1", Type: "edit-fields", Version: "1.0.0"}
	def2 := &node.Definition{ID: "2", Type: "http-request", Version: "1.0.0"}

	if err := storage.Save(ctx, "workflow-1", def1); err != nil {
		t.Fatalf("Failed to save workflow-1: %v", err)
	}
	if err := storage.Save(ctx, "workflow-2", def2); err != nil {
		t.Fatalf("Failed to save workflow-2: %v", err)
	}

	// List
	names, err := storage.List(ctx)
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}

	// Verify
	if len(names) != 2 {
		t.Fatalf("Expected 2 workflows, got %d", len(names))
	}
}

// Test: Delete a bnto
func TestStorage_Delete(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	// Save
	def := &node.Definition{ID: "1", Type: "edit-fields", Version: "1.0.0"}
	if err := storage.Save(ctx, "test", def); err != nil {
		t.Fatalf("Failed to save test workflow: %v", err)
	}

	// Delete
	err := storage.Delete(ctx, "test")
	if err != nil {
		t.Fatalf("Delete failed: %v", err)
	}

	// Verify deleted
	_, err = storage.Load(ctx, "test")
	if err == nil {
		t.Fatal("Expected error loading deleted workflow")
	}
}

// Test: Load non-existent bnto fails gracefully
func TestStorage_LoadNotFound(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	_, err := storage.Load(ctx, "nonexistent")
	if err == nil {
		t.Fatal("Expected error for non-existent workflow")
	}

	// Error should be clear
	if !contains(err.Error(), "not found") && !contains(err.Error(), "does not exist") {
		t.Errorf("Error should mention file not found: %s", err.Error())
	}
}

// Test: Invalid name sanitization (security)
func TestStorage_InvalidName(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	def := &node.Definition{ID: "1", Type: "edit-fields", Version: "1.0.0"}

	// Try directory traversal attack
	invalidNames := []string{
		"../etc/passwd",
		"../../secret",
		"foo/../bar",
		"con/aux", // Windows reserved
	}

	for _, name := range invalidNames {
		err := storage.Save(ctx, name, def)
		if err == nil {
			t.Errorf("Should reject invalid name: %s", name)
		}
	}
}

// Test: Save with nested group structures
func TestStorage_NestedGroups(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	// Create nested structure
	def := &node.Definition{
		ID:      "main-group",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Main Group",
		Nodes: []node.Definition{
			{
				ID:      "sub-group",
				Type:    "group",
				Version: "1.0.0",
				Name:    "Sub Group",
				Nodes: []node.Definition{
					{
						ID:      "nested-node",
						Type:    "edit-fields",
						Version: "1.0.0",
						Name:    "Nested Node",
						Parameters: map[string]interface{}{
							"values": map[string]interface{}{
								"foo": "bar",
							},
						},
					},
				},
				Edges: []node.Edge{},
			},
		},
		Edges: []node.Edge{},
	}

	// Save
	err := storage.Save(ctx, "nested-workflow", def)
	if err != nil {
		t.Fatalf("Save failed: %v", err)
	}

	// Load
	loaded, err := storage.Load(ctx, "nested-workflow")
	if err != nil {
		t.Fatalf("Load failed: %v", err)
	}

	// Verify nested structure
	if len(loaded.Nodes) != 1 {
		t.Fatalf("Expected 1 top-level node, got %d", len(loaded.Nodes))
	}

	subGroup := loaded.Nodes[0]
	if subGroup.Type != "group" {
		t.Errorf("Sub-node should be group, got %s", subGroup.Type)
	}

	if len(subGroup.Nodes) != 1 {
		t.Fatalf("Expected 1 nested node, got %d", len(subGroup.Nodes))
	}

	if subGroup.Nodes[0].ID != "nested-node" {
		t.Errorf("Nested node ID = %s, want nested-node", subGroup.Nodes[0].ID)
	}
}

// Test: Round-trip (save -> load -> save should be identical)
func TestStorage_RoundTrip(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	original := &node.Definition{
		ID:      "round-trip-test",
		Type:    "group",
		Version: "1.0.0",
		Name:    "Round Trip Test",
		Parameters: map[string]interface{}{
			"nested": map[string]interface{}{
				"foo": "bar",
				"num": float64(42), // JSON unmarshals numbers as float64
			},
		},
	}

	// Save original
	if err := storage.Save(ctx, "roundtrip", original); err != nil {
		t.Fatalf("Save failed: %v", err)
	}

	// Load it back
	loaded, err := storage.Load(ctx, "roundtrip")
	if err != nil {
		t.Fatalf("Load failed: %v", err)
	}

	// Verify equality
	if !reflect.DeepEqual(original.Parameters, loaded.Parameters) {
		t.Errorf("Round-trip resulted in different parameters")
	}
}

// Test: Pretty-printed JSON (2-space indentation)
func TestStorage_PrettyPrint(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	def := &node.Definition{
		ID:      "my-bnto",
		Type:    "group",
		Version: "1.0.0",
		Name:    "My Bnto",
	}

	err := storage.Save(ctx, "pretty-test", def)
	if err != nil {
		t.Fatalf("Save failed: %v", err)
	}

	// Read file directly - now in bntos/ subdirectory with .bnto.json extension
	path := filepath.Join(tempDir, "bntos", "pretty-test.bnto.json")
	content, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}

	contentStr := string(content)

	// Should have 2-space indentation
	if !strings.Contains(contentStr, "  \"id\": \"my-bnto\"") {
		t.Error("JSON should be pretty-printed with 2-space indentation")
	}

	// Should be valid JSON
	var parsed map[string]interface{}
	if err := json.Unmarshal(content, &parsed); err != nil {
		t.Errorf("Saved JSON is invalid: %v", err)
	}
}

// Test: Empty name should fail
func TestStorage_EmptyName(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	def := &node.Definition{ID: "1", Type: "edit-fields", Version: "1.0.0"}

	err := storage.Save(ctx, "", def)
	if err == nil {
		t.Fatal("Should reject empty workflow name")
	}
}

// Test: Reserved Windows names should fail
func TestStorage_ReservedNames(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	def := &node.Definition{ID: "1", Type: "edit-fields", Version: "1.0.0"}

	reservedNames := []string{"CON", "PRN", "AUX", "NUL", "COM1", "LPT1"}

	for _, name := range reservedNames {
		err := storage.Save(ctx, name, def)
		if err == nil {
			t.Errorf("Should reject reserved name: %s", name)
		}
	}
}

// Test: Context cancellation should be respected
func TestStorage_ContextCancellation(t *testing.T) {
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	// Create cancelled context
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	def := &node.Definition{ID: "1", Type: "edit-fields", Version: "1.0.0"}

	// Should fail immediately
	err := storage.Save(ctx, "test", def)
	if err == nil {
		t.Fatal("Expected error for cancelled context")
	}

	if err != context.Canceled {
		t.Errorf("Expected context.Canceled, got %v", err)
	}
}

// Test: List returns empty array when no workflows exist
func TestStorage_ListEmpty(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	names, err := storage.List(ctx)
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}

	if len(names) != 0 {
		t.Errorf("Expected 0 workflows, got %d", len(names))
	}
}

// Test: Delete non-existent workflow fails gracefully
func TestStorage_DeleteNotFound(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	storage := storage.New(tempDir)

	err := storage.Delete(ctx, "nonexistent")
	if err == nil {
		t.Fatal("Expected error deleting non-existent workflow")
	}

	if !contains(err.Error(), "not found") {
		t.Errorf("Error should mention 'not found': %s", err.Error())
	}
}

// Helper: Check if string contains substring (case-insensitive)
func contains(s, substr string) bool {
	return strings.Contains(strings.ToLower(s), strings.ToLower(substr))
}
