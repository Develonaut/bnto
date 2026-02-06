package handler_test

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/node"

	"github.com/Develonaut/bento-api/internal/execution"
)

// Contract tests verify that Go API JSON responses match the TypeScript
// types defined in @bento/core. If these tests break, the TS types
// must be updated to match (or vice versa).

// TestContract_ValidationResult verifies POST /api/validate returns
// {valid: boolean, errors?: string[]}.
func TestContract_ValidationResult(t *testing.T) {
	handler := newTestServer(t)

	def := node.Definition{
		ID: "root", Type: "group", Version: "1.0.0", Name: "test",
		Nodes: []node.Definition{
			{ID: "n1", Type: "edit-fields", Version: "1.0.0", Name: "x",
				Parameters: map[string]any{"values": map[string]any{"a": 1}}},
		},
	}
	body, _ := json.Marshal(def)

	req := httptest.NewRequest("POST", "/api/validate", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	m := decodeJSON(t, w)
	requireKey(t, m, "valid", "boolean")

	// errors is optional (omitempty) — when valid, may be absent
	if _, ok := m["errors"]; ok {
		requireKey(t, m, "errors", "array")
	}
}

// TestContract_WorkflowSummary verifies GET /api/workflows returns
// [{name: string, nodeCount: number}].
func TestContract_WorkflowSummary(t *testing.T) {
	handler := newTestServer(t)

	// Save a workflow first
	saveBody, _ := json.Marshal(map[string]any{
		"name":       "contract-test",
		"definition": node.Definition{ID: "r", Type: "group", Version: "1.0.0", Name: "x"},
	})
	req := httptest.NewRequest("POST", "/api/workflows", bytes.NewReader(saveBody))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	// List
	req = httptest.NewRequest("GET", "/api/workflows", nil)
	w = httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	var items []map[string]any
	if err := json.NewDecoder(w.Body).Decode(&items); err != nil {
		t.Fatalf("decode: %v", err)
	}
	if len(items) == 0 {
		t.Fatal("expected at least one workflow")
	}

	item := items[0]
	requireKey(t, item, "name", "string")
	requireKey(t, item, "nodeCount", "number")
}

// TestContract_WorkflowDefinition verifies GET /api/workflows/{name} returns
// a full WorkflowDefinition matching the node.Definition JSON shape.
func TestContract_WorkflowDefinition(t *testing.T) {
	handler := newTestServer(t)

	saveBody, _ := json.Marshal(map[string]any{
		"name": "contract-def",
		"definition": node.Definition{
			ID: "root", Type: "group", Version: "1.0.0", Name: "Test",
			Parameters:  map[string]any{},
			InputPorts:  []node.Port{},
			OutputPorts: []node.Port{},
			Nodes: []node.Definition{
				{ID: "n1", Type: "edit-fields", Version: "1.0.0", Name: "x",
					Parameters: map[string]any{"values": map[string]any{"a": 1}}},
			},
		},
	})
	req := httptest.NewRequest("POST", "/api/workflows", bytes.NewReader(saveBody))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	req = httptest.NewRequest("GET", "/api/workflows/contract-def", nil)
	w = httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	m := decodeJSON(t, w)
	requireKey(t, m, "id", "string")
	requireKey(t, m, "type", "string")
	requireKey(t, m, "version", "string")
	requireKey(t, m, "name", "string")
	requireKey(t, m, "position", "object")
	requireKey(t, m, "metadata", "object")
	requireKey(t, m, "parameters", "object")
	requireKey(t, m, "inputPorts", "array")
	requireKey(t, m, "outputPorts", "array")
}

// TestContract_RunResponse verifies POST /api/run returns {id: string}.
func TestContract_RunResponse(t *testing.T) {
	handler := newTestServer(t)

	def := node.Definition{
		ID: "root", Type: "group", Version: "1.0.0", Name: "run",
		Nodes: []node.Definition{
			{ID: "n1", Type: "edit-fields", Version: "1.0.0", Name: "x",
				Parameters: map[string]any{"values": map[string]any{"a": 1}},
				Fields:     &node.FieldsConfig{Values: map[string]any{"a": 1}}},
		},
	}
	body, _ := json.Marshal(map[string]any{"definition": def})

	req := httptest.NewRequest("POST", "/api/run", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusAccepted {
		t.Fatalf("expected 202, got %d: %s", w.Code, w.Body.String())
	}

	m := decodeJSON(t, w)
	requireKey(t, m, "id", "string")
}

// TestContract_Execution verifies GET /api/executions/{id} returns
// {id, status, progress: [{nodeId, status}], startedAt, ...}.
func TestContract_Execution(t *testing.T) {
	handler := newTestServer(t)

	// Start a run
	def := node.Definition{
		ID: "root", Type: "group", Version: "1.0.0", Name: "run",
		Nodes: []node.Definition{
			{ID: "n1", Type: "edit-fields", Version: "1.0.0", Name: "x",
				Parameters: map[string]any{"values": map[string]any{"a": 1}},
				Fields:     &node.FieldsConfig{Values: map[string]any{"a": 1}}},
		},
	}
	body, _ := json.Marshal(map[string]any{"definition": def})
	req := httptest.NewRequest("POST", "/api/run", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	var runResp map[string]string
	json.NewDecoder(w.Body).Decode(&runResp)
	execID := runResp["id"]

	// Wait for completion
	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		req = httptest.NewRequest("GET", "/api/executions/"+execID, nil)
		w = httptest.NewRecorder()
		handler.ServeHTTP(w, req)

		var exec execution.Execution
		json.NewDecoder(w.Body).Decode(&exec)
		if exec.Status == "completed" || exec.Status == "failed" {
			break
		}
		time.Sleep(50 * time.Millisecond)
	}

	// Now verify the completed execution's JSON shape
	req = httptest.NewRequest("GET", "/api/executions/"+execID, nil)
	w = httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	m := decodeJSON(t, w)
	requireKey(t, m, "id", "string")
	requireKey(t, m, "status", "string")
	requireKey(t, m, "progress", "array")
	requireKey(t, m, "startedAt", "string")

	// completedAt present when done
	requireKey(t, m, "completedAt", "string")

	// Verify progress items have correct shape
	progress, _ := m["progress"].([]any)
	if len(progress) > 0 {
		item, ok := progress[0].(map[string]any)
		if !ok {
			t.Fatal("progress item should be an object")
		}
		requireKey(t, item, "nodeId", "string")
		requireKey(t, item, "status", "string")
	}
}

// TestContract_ErrorResponse verifies error responses return {error: string}.
func TestContract_ErrorResponse(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("GET", "/api/workflows/nonexistent", nil)
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	m := decodeJSON(t, w)
	requireKey(t, m, "error", "string")
}

// --- helpers ---

func decodeJSON(t *testing.T, w *httptest.ResponseRecorder) map[string]any {
	t.Helper()
	var m map[string]any
	if err := json.NewDecoder(w.Body).Decode(&m); err != nil {
		t.Fatalf("failed to decode JSON response: %v", err)
	}
	return m
}

func requireKey(t *testing.T, m map[string]any, key, expectedType string) {
	t.Helper()
	v, ok := m[key]
	if !ok {
		t.Errorf("missing required key %q", key)
		return
	}
	switch expectedType {
	case "string":
		if _, ok := v.(string); !ok {
			t.Errorf("key %q: expected string, got %T", key, v)
		}
	case "boolean":
		if _, ok := v.(bool); !ok {
			t.Errorf("key %q: expected boolean, got %T", key, v)
		}
	case "number":
		if _, ok := v.(float64); !ok {
			t.Errorf("key %q: expected number, got %T", key, v)
		}
	case "array":
		if _, ok := v.([]any); !ok {
			t.Errorf("key %q: expected array, got %T", key, v)
		}
	case "object":
		if _, ok := v.(map[string]any); !ok {
			t.Errorf("key %q: expected object, got %T", key, v)
		}
	}
}
