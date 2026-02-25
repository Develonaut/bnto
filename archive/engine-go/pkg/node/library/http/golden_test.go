package http_test

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	httpnode "github.com/Develonaut/bnto/pkg/node/library/http"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_GetJSON(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]any{
			"status": "success",
			"items":  []string{"alpha", "beta", "gamma"},
			"count":  3,
		})
	}))
	defer ts.Close()

	node := httpnode.New()
	result, err := node.Execute(context.Background(), map[string]any{
		"url":    ts.URL,
		"method": "GET",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Normalize: strip dynamic headers (Date, Content-Length vary)
	resp := result.(map[string]any)
	delete(resp, "headers")
	testgolden.AssertGolden(t, "get_json", resp)
}

func TestGolden_PostJSON(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var body map[string]any
		json.NewDecoder(r.Body).Decode(&body)
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]any{
			"created": true,
			"name":    body["name"],
			"id":      "new-123",
		})
	}))
	defer ts.Close()

	node := httpnode.New()
	result, err := node.Execute(context.Background(), map[string]any{
		"url":    ts.URL,
		"method": "POST",
		"body":   map[string]any{"name": "test-item", "count": 3},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	resp := result.(map[string]any)
	delete(resp, "headers")
	testgolden.AssertGolden(t, "post_json", resp)
}

func TestGolden_NotFoundError(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNotFound)
		json.NewEncoder(w).Encode(map[string]any{
			"error": "Resource not found",
		})
	}))
	defer ts.Close()

	node := httpnode.New()
	result, err := node.Execute(context.Background(), map[string]any{
		"url":    ts.URL,
		"method": "GET",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	resp := result.(map[string]any)
	delete(resp, "headers")
	testgolden.AssertGolden(t, "not_found_error", resp)
}

func TestGolden_NonJSONResponse(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/plain")
		fmt.Fprint(w, "hello world")
	}))
	defer ts.Close()

	node := httpnode.New()
	result, err := node.Execute(context.Background(), map[string]any{
		"url":    ts.URL,
		"method": "GET",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	resp := result.(map[string]any)
	delete(resp, "headers")
	testgolden.AssertGolden(t, "non_json_response", resp)
}
