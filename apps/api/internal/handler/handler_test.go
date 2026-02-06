package handler_test

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/api"
	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/storage"

	"github.com/Develonaut/bento-api/internal/execution"
	"github.com/Develonaut/bento-api/internal/server"
)

// newTestServer creates a test server with a real BentoService and temp storage.
func newTestServer(t *testing.T) http.Handler {
	t.Helper()
	reg := api.DefaultRegistry()
	store := storage.New(t.TempDir())
	svc := api.New(reg, store)
	mgr := execution.NewManager()
	return server.New(svc, mgr)
}

func TestValidateWorkflow(t *testing.T) {
	handler := newTestServer(t)

	def := node.Definition{
		ID:      "root",
		Type:    "group",
		Version: "1.0.0",
		Name:    "test",
		Nodes: []node.Definition{
			{
				ID:         "n1",
				Type:       "edit-fields",
				Version:    "1.0.0",
				Name:       "Set Fields",
				Parameters: map[string]any{"values": map[string]any{"a": 1}},
			},
		},
	}
	body, _ := json.Marshal(def)

	req := httptest.NewRequest("POST", "/api/validate", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d: %s", w.Code, w.Body.String())
	}

	var result api.ValidationResult
	if err := json.NewDecoder(w.Body).Decode(&result); err != nil {
		t.Fatalf("decoding response: %v", err)
	}
	if !result.Valid {
		t.Errorf("expected valid, got errors: %v", result.Errors)
	}
}

func TestValidateWorkflowBadJSON(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("POST", "/api/validate", bytes.NewBufferString("not json"))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Fatalf("expected 400, got %d", w.Code)
	}
}

func TestWorkflowCRUD(t *testing.T) {
	handler := newTestServer(t)

	// Save a workflow
	saveBody, _ := json.Marshal(map[string]any{
		"name": "test-flow",
		"definition": node.Definition{
			ID:      "root",
			Type:    "group",
			Version: "1.0.0",
			Name:    "Test Flow",
		},
	})
	req := httptest.NewRequest("POST", "/api/workflows", bytes.NewReader(saveBody))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusCreated {
		t.Fatalf("save: expected 201, got %d: %s", w.Code, w.Body.String())
	}

	// List workflows
	req = httptest.NewRequest("GET", "/api/workflows", nil)
	w = httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("list: expected 200, got %d", w.Code)
	}
	var summaries []api.WorkflowSummary
	if err := json.NewDecoder(w.Body).Decode(&summaries); err != nil {
		t.Fatalf("list decode: %v", err)
	}
	if len(summaries) != 1 {
		t.Fatalf("expected 1 workflow, got %d", len(summaries))
	}
	if summaries[0].Name != "test-flow" {
		t.Errorf("expected test-flow, got %s", summaries[0].Name)
	}

	// Get workflow
	req = httptest.NewRequest("GET", "/api/workflows/test-flow", nil)
	w = httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("get: expected 200, got %d: %s", w.Code, w.Body.String())
	}
	var def node.Definition
	if err := json.NewDecoder(w.Body).Decode(&def); err != nil {
		t.Fatalf("get decode: %v", err)
	}
	if def.Name != "Test Flow" {
		t.Errorf("expected 'Test Flow', got %q", def.Name)
	}

	// Delete workflow
	req = httptest.NewRequest("DELETE", "/api/workflows/test-flow", nil)
	w = httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusNoContent {
		t.Fatalf("delete: expected 204, got %d", w.Code)
	}

	// Verify deleted
	req = httptest.NewRequest("GET", "/api/workflows/test-flow", nil)
	w = httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusNotFound {
		t.Fatalf("get after delete: expected 404, got %d", w.Code)
	}
}

func TestSaveWorkflowMissingName(t *testing.T) {
	handler := newTestServer(t)

	body, _ := json.Marshal(map[string]any{
		"definition": node.Definition{
			ID:      "root",
			Type:    "group",
			Version: "1.0.0",
			Name:    "x",
		},
	})
	req := httptest.NewRequest("POST", "/api/workflows", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Fatalf("expected 400, got %d", w.Code)
	}
}

func TestGetWorkflowNotFound(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("GET", "/api/workflows/nonexistent", nil)
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusNotFound {
		t.Fatalf("expected 404, got %d", w.Code)
	}
}

func TestDeleteWorkflowNotFound(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("DELETE", "/api/workflows/nonexistent", nil)
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusNotFound {
		t.Fatalf("expected 404, got %d", w.Code)
	}
}

func TestRunAndPollExecution(t *testing.T) {
	handler := newTestServer(t)

	def := node.Definition{
		ID:      "root",
		Type:    "group",
		Version: "1.0.0",
		Name:    "run-test",
		Nodes: []node.Definition{
			{
				ID:         "n1",
				Type:       "edit-fields",
				Version:    "1.0.0",
				Name:       "Set Fields",
				Parameters: map[string]any{"values": map[string]any{"key": "value"}},
				Fields: &node.FieldsConfig{
					Values:      map[string]any{"key": "value"},
					KeepOnlySet: true,
				},
			},
		},
	}
	runBody, _ := json.Marshal(map[string]any{
		"definition": def,
		"timeout":    "30s",
	})
	req := httptest.NewRequest("POST", "/api/run", bytes.NewReader(runBody))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusAccepted {
		t.Fatalf("run: expected 202, got %d: %s", w.Code, w.Body.String())
	}

	var runResp map[string]string
	if err := json.NewDecoder(w.Body).Decode(&runResp); err != nil {
		t.Fatalf("run decode: %v", err)
	}
	execID := runResp["id"]
	if execID == "" {
		t.Fatal("expected non-empty execution id")
	}

	// Poll until completed or timeout
	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		req = httptest.NewRequest("GET", "/api/executions/"+execID, nil)
		w = httptest.NewRecorder()
		handler.ServeHTTP(w, req)

		if w.Code != http.StatusOK {
			t.Fatalf("poll: expected 200, got %d", w.Code)
		}

		var exec execution.Execution
		if err := json.NewDecoder(w.Body).Decode(&exec); err != nil {
			t.Fatalf("poll decode: %v", err)
		}

		if exec.Status == execution.StatusCompleted || exec.Status == execution.StatusFailed {
			if exec.Status == execution.StatusFailed {
				t.Fatalf("execution failed: %s", exec.Error)
			}
			return // Success
		}

		time.Sleep(50 * time.Millisecond)
	}
	t.Fatal("execution did not complete within timeout")
}

func TestGetExecutionNotFound(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("GET", "/api/executions/nonexistent", nil)
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusNotFound {
		t.Fatalf("expected 404, got %d", w.Code)
	}
}

func TestCORSPreflight(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("OPTIONS", "/api/workflows", nil)
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusNoContent {
		t.Fatalf("expected 204, got %d", w.Code)
	}
	if got := w.Header().Get("Access-Control-Allow-Origin"); got != "*" {
		t.Errorf("expected CORS origin *, got %q", got)
	}
}

func TestCORSHeaders(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("GET", "/api/workflows", nil)
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if got := w.Header().Get("Access-Control-Allow-Origin"); got != "*" {
		t.Errorf("expected CORS origin *, got %q", got)
	}
}

func TestRunBadJSON(t *testing.T) {
	handler := newTestServer(t)

	req := httptest.NewRequest("POST", "/api/run", bytes.NewBufferString("{bad"))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Fatalf("expected 400, got %d", w.Code)
	}
}

func TestRunInvalidTimeout(t *testing.T) {
	handler := newTestServer(t)

	body, _ := json.Marshal(map[string]any{
		"definition": node.Definition{
			ID:      "root",
			Type:    "group",
			Version: "1.0.0",
			Name:    "x",
		},
		"timeout": "not-a-duration",
	})
	req := httptest.NewRequest("POST", "/api/run", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Fatalf("expected 400, got %d", w.Code)
	}
}
