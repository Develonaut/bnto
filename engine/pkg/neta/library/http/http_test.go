package http_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	httpneta "github.com/Develonaut/bento/pkg/neta/library/http"
)

// TestHTTPRequest_GET tests basic GET request functionality.
func TestHTTPRequest_GET(t *testing.T) {
	ctx := context.Background()

	// Start test HTTP server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "GET" {
			t.Errorf("Expected GET request, got %s", r.Method)
		}
		w.Header().Set("Content-Type", "application/json")
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"status": "success",
			"data":   []string{"item1", "item2"},
		}); err != nil {
			t.Fatalf("Failed to encode response: %v", err)
		}
	}))
	defer ts.Close()

	httpNeta := httpneta.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Verify response
	response, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	body, ok := response["body"].(map[string]interface{})
	if !ok {
		t.Fatal("Expected body to be map[string]interface{}")
	}

	if body["status"] != "success" {
		t.Errorf("status = %v, want success", body["status"])
	}

	// Verify status code
	if response["statusCode"] != 200 {
		t.Errorf("statusCode = %v, want 200", response["statusCode"])
	}
}

// TestHTTPRequest_POST tests POST request with body.
func TestHTTPRequest_POST(t *testing.T) {
	ctx := context.Background()

	// Start test HTTP server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "POST" {
			t.Errorf("Expected POST request, got %s", r.Method)
		}

		// Read and verify request body
		var reqBody map[string]interface{}
		if err := json.NewDecoder(r.Body).Decode(&reqBody); err != nil {
			t.Fatalf("Failed to decode request body: %v", err)
		}

		if reqBody["name"] != "Test Product" {
			t.Errorf("name = %v, want Test Product", reqBody["name"])
		}

		// Send response
		w.Header().Set("Content-Type", "application/json")
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"id":   123,
			"name": reqBody["name"],
		}); err != nil {
			t.Fatalf("Failed to encode response: %v", err)
		}
	}))
	defer ts.Close()

	httpNeta := httpneta.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "POST",
		"body": map[string]interface{}{
			"name": "Test Product",
			"sku":  "PROD-001",
		},
	}

	result, err := httpNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	body := response["body"].(map[string]interface{})

	if body["name"] != "Test Product" {
		t.Errorf("name = %v, want Test Product", body["name"])
	}
}

// TestHTTPRequest_Headers tests custom headers and authentication.
func TestHTTPRequest_Headers(t *testing.T) {
	ctx := context.Background()

	// Start test HTTP server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Verify custom header
		if r.Header.Get("X-Custom-Header") != "custom-value" {
			t.Errorf("X-Custom-Header = %v, want custom-value", r.Header.Get("X-Custom-Header"))
		}

		// Verify authorization header
		if r.Header.Get("Authorization") != "Bearer test-token-123" {
			t.Errorf("Authorization = %v, want Bearer test-token-123", r.Header.Get("Authorization"))
		}

		w.Header().Set("Content-Type", "application/json")
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"authenticated": true,
		}); err != nil {
			t.Fatalf("Failed to encode response: %v", err)
		}
	}))
	defer ts.Close()

	httpNeta := httpneta.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
		"headers": map[string]interface{}{
			"X-Custom-Header": "custom-value",
			"Authorization":   "Bearer test-token-123",
		},
	}

	result, err := httpNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	if response["statusCode"] != 200 {
		t.Errorf("statusCode = %v, want 200", response["statusCode"])
	}
}

// TestHTTPRequest_Timeout tests request timeout handling.
func TestHTTPRequest_Timeout(t *testing.T) {
	ctx := context.Background()

	// Server that delays 3 seconds
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(3 * time.Second)
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	httpNeta := httpneta.New()

	params := map[string]interface{}{
		"url":     ts.URL,
		"method":  "GET",
		"timeout": 1, // 1 second timeout
	}

	_, err := httpNeta.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected timeout error, got nil")
	}

	if !strings.Contains(err.Error(), "timeout") && !strings.Contains(err.Error(), "deadline") {
		t.Errorf("Expected timeout error, got: %v", err)
	}
}

// TestHTTPRequest_4xxError tests handling of 4xx client errors.
func TestHTTPRequest_4xxError(t *testing.T) {
	ctx := context.Background()

	// Server that returns 404
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNotFound)
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Resource not found",
		}); err != nil {
			t.Fatalf("Failed to encode response: %v", err)
		}
	}))
	defer ts.Close()

	httpNeta := httpneta.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	if response["statusCode"] != 404 {
		t.Errorf("statusCode = %v, want 404", response["statusCode"])
	}

	body := response["body"].(map[string]interface{})
	if body["error"] != "Resource not found" {
		t.Errorf("error = %v, want Resource not found", body["error"])
	}
}

// TestHTTPRequest_5xxError tests handling of 5xx server errors.
func TestHTTPRequest_5xxError(t *testing.T) {
	ctx := context.Background()

	// Server that returns 500
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Internal server error",
		}); err != nil {
			t.Fatalf("Failed to encode response: %v", err)
		}
	}))
	defer ts.Close()

	httpNeta := httpneta.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	if response["statusCode"] != 500 {
		t.Errorf("statusCode = %v, want 500", response["statusCode"])
	}
}

// TestHTTPRequest_DefaultTimeout tests that a default timeout is applied.
func TestHTTPRequest_DefaultTimeout(t *testing.T) {
	ctx := context.Background()

	// Quick server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"status": "ok",
		}); err != nil {
			t.Fatalf("Failed to encode response: %v", err)
		}
	}))
	defer ts.Close()

	httpNeta := httpneta.New()

	// No timeout specified - should use default
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	if response["statusCode"] != 200 {
		t.Errorf("statusCode = %v, want 200", response["statusCode"])
	}
}
