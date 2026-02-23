package handler_test

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"sort"
	"strings"
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/api"
	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/storage"

	"github.com/Develonaut/bnto-api/internal/execution"
	"github.com/Develonaut/bnto-api/internal/r2"
	"github.com/Develonaut/bnto-api/internal/server"
)

// mockStore implements r2.ObjectStore for testing.
type mockStore struct {
	objects map[string][]byte
}

func newMockStore() *mockStore {
	return &mockStore{objects: make(map[string][]byte)}
}

func (m *mockStore) ListObjects(_ context.Context, prefix string) ([]string, error) {
	var keys []string
	for k := range m.objects {
		if strings.HasPrefix(k, prefix) {
			keys = append(keys, k)
		}
	}
	sort.Strings(keys)
	return keys, nil
}

func (m *mockStore) Download(_ context.Context, key string) (*r2.Object, error) {
	data, ok := m.objects[key]
	if !ok {
		return nil, fmt.Errorf("object not found: %s", key)
	}
	return &r2.Object{
		Key:         key,
		Body:        io.NopCloser(bytes.NewReader(data)),
		ContentType: "application/octet-stream",
		Size:        int64(len(data)),
	}, nil
}

func (m *mockStore) Upload(_ context.Context, key string, body io.Reader, _ string) error {
	data, err := io.ReadAll(body)
	if err != nil {
		return err
	}
	m.objects[key] = data
	return nil
}

func newTestServerWithStore(t *testing.T, store r2.ObjectStore) http.Handler {
	t.Helper()
	reg := api.DefaultRegistry()
	s := storage.New(t.TempDir())
	svc := api.New(reg, s)
	mgr := execution.NewManager()
	return server.New(svc, mgr, store)
}

func TestRunWithSessionID(t *testing.T) {
	store := newMockStore()
	store.objects["uploads/test-session/data.json"] = []byte(`{"key":"value"}`)

	handler := newTestServerWithStore(t, store)

	// Use edit-fields node which doesn't need files — just verify the session
	// download/upload pipeline works. The node writes to OUTPUT_DIR via the
	// env var set by the transit layer.
	def := node.Definition{
		ID:      "root",
		Type:    "group",
		Version: "1.0.0",
		Name:    "transit-test",
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

	body, _ := json.Marshal(map[string]any{
		"definition": def,
		"sessionId":  "test-session",
	})

	req := httptest.NewRequest("POST", "/api/run", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusAccepted {
		t.Fatalf("expected 202, got %d: %s", w.Code, w.Body.String())
	}

	var resp map[string]string
	json.NewDecoder(w.Body).Decode(&resp)
	execID := resp["id"]

	// Poll until completed.
	deadline := time.Now().Add(10 * time.Second)
	for time.Now().Before(deadline) {
		req = httptest.NewRequest("GET", "/api/executions/"+execID, nil)
		w = httptest.NewRecorder()
		handler.ServeHTTP(w, req)

		var exec execution.Execution
		json.NewDecoder(w.Body).Decode(&exec)

		if exec.Status == execution.StatusCompleted {
			return
		}
		if exec.Status == execution.StatusFailed {
			t.Fatalf("execution failed: %s", exec.Error)
		}

		time.Sleep(100 * time.Millisecond)
	}
	t.Fatal("execution did not complete within timeout")
}

func TestRunWithSessionIDNoStore(t *testing.T) {
	handler := newTestServerWithStore(t, nil)

	def := node.Definition{
		ID: "root", Type: "group", Version: "1.0.0", Name: "test",
	}
	body, _ := json.Marshal(map[string]any{
		"definition": def,
		"sessionId":  "some-session",
	})

	req := httptest.NewRequest("POST", "/api/run", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusServiceUnavailable {
		t.Fatalf("expected 503, got %d: %s", w.Code, w.Body.String())
	}
}

func TestRunWithEmptySession(t *testing.T) {
	store := newMockStore()
	handler := newTestServerWithStore(t, store)

	def := node.Definition{
		ID:      "root",
		Type:    "group",
		Version: "1.0.0",
		Name:    "empty-session-test",
		Nodes: []node.Definition{
			{
				ID:         "n1",
				Type:       "edit-fields",
				Version:    "1.0.0",
				Name:       "Set Fields",
				Parameters: map[string]any{"values": map[string]any{"a": 1}},
				Fields:     &node.FieldsConfig{Values: map[string]any{"a": 1}},
			},
		},
	}

	body, _ := json.Marshal(map[string]any{
		"definition": def,
		"sessionId":  "nonexistent-session",
	})

	req := httptest.NewRequest("POST", "/api/run", bytes.NewReader(body))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusAccepted {
		t.Fatalf("expected 202, got %d", w.Code)
	}

	var resp map[string]string
	json.NewDecoder(w.Body).Decode(&resp)
	execID := resp["id"]

	// Poll until failed (no files for this session).
	deadline := time.Now().Add(10 * time.Second)
	for time.Now().Before(deadline) {
		req = httptest.NewRequest("GET", "/api/executions/"+execID, nil)
		w = httptest.NewRecorder()
		handler.ServeHTTP(w, req)

		var exec execution.Execution
		json.NewDecoder(w.Body).Decode(&exec)

		if exec.Status == execution.StatusFailed {
			if !strings.Contains(exec.Error, "no files found") {
				t.Fatalf("expected 'no files found' error, got: %s", exec.Error)
			}
			return
		}
		if exec.Status == execution.StatusCompleted {
			t.Fatal("expected failure, got completed")
		}

		time.Sleep(100 * time.Millisecond)
	}
	t.Fatal("execution did not fail within timeout")
}

func TestContract_ExecutionOutputFiles(t *testing.T) {
	handler := newTestServerWithStore(t, nil)

	// Run without sessionId (no transit) — outputFiles should be absent.
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

	var resp map[string]string
	json.NewDecoder(w.Body).Decode(&resp)
	execID := resp["id"]

	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		req = httptest.NewRequest("GET", "/api/executions/"+execID, nil)
		w = httptest.NewRecorder()
		handler.ServeHTTP(w, req)

		var exec map[string]any
		json.NewDecoder(w.Body).Decode(&exec)
		if exec["status"] == "completed" {
			// outputFiles should be omitted (nil) when no transit was used.
			if _, ok := exec["outputFiles"]; ok {
				t.Error("outputFiles should be omitted when no transit used")
			}
			return
		}
		time.Sleep(50 * time.Millisecond)
	}
	t.Fatal("execution did not complete")
}
