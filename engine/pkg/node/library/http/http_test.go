package http_test

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	httpnode "github.com/Develonaut/bento/pkg/node/library/http"
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

	httpNode := httpnode.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNode.Execute(ctx, params)
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

		if reqBody["name"] != "test-item" {
			t.Errorf("name = %v, want test-item", reqBody["name"])
		}

		// Send response
		w.Header().Set("Content-Type", "application/json")
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"created": true,
			"name":    reqBody["name"],
		}); err != nil {
			t.Fatalf("Failed to encode response: %v", err)
		}
	}))
	defer ts.Close()

	httpNode := httpnode.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "POST",
		"body": map[string]interface{}{
			"name":  "test-item",
			"count": 3,
		},
	}

	result, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	body := response["body"].(map[string]interface{})

	if body["name"] != "test-item" {
		t.Errorf("name = %v, want test-item", body["name"])
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

	httpNode := httpnode.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
		"headers": map[string]interface{}{
			"X-Custom-Header": "custom-value",
			"Authorization":   "Bearer test-token-123",
		},
	}

	result, err := httpNode.Execute(ctx, params)
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

	httpNode := httpnode.New()

	params := map[string]interface{}{
		"url":     ts.URL,
		"method":  "GET",
		"timeout": 1, // 1 second timeout
	}

	_, err := httpNode.Execute(ctx, params)
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

	httpNode := httpnode.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNode.Execute(ctx, params)
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

	httpNode := httpnode.New()

	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNode.Execute(ctx, params)
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

	httpNode := httpnode.New()

	// No timeout specified - should use default
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	if response["statusCode"] != 200 {
		t.Errorf("statusCode = %v, want 200", response["statusCode"])
	}
}

// --- Parameter Validation Tests ---

// TestHTTPRequest_MissingURL tests that missing URL returns an error.
func TestHTTPRequest_MissingURL(t *testing.T) {
	ctx := context.Background()
	httpNode := httpnode.New()

	params := map[string]interface{}{
		"method": "GET",
	}

	_, err := httpNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing URL, got nil")
	}
	if !strings.Contains(err.Error(), "url parameter is required") {
		t.Errorf("Expected 'url parameter is required' error, got: %v", err)
	}
}

// TestHTTPRequest_MissingMethod tests that missing method returns an error.
func TestHTTPRequest_MissingMethod(t *testing.T) {
	ctx := context.Background()
	httpNode := httpnode.New()

	params := map[string]interface{}{
		"url": "http://example.com",
	}

	_, err := httpNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing method, got nil")
	}
	if !strings.Contains(err.Error(), "method parameter is required") {
		t.Errorf("Expected 'method parameter is required' error, got: %v", err)
	}
}

// TestHTTPRequest_InvalidParamTypes tests that non-string url/method return errors.
func TestHTTPRequest_InvalidParamTypes(t *testing.T) {
	ctx := context.Background()
	httpNode := httpnode.New()

	// url as int
	_, err := httpNode.Execute(ctx, map[string]interface{}{
		"url":    123,
		"method": "GET",
	})
	if err == nil {
		t.Fatal("Expected error for non-string URL, got nil")
	}
	if !strings.Contains(err.Error(), "url parameter is required") {
		t.Errorf("Expected 'url parameter is required' error, got: %v", err)
	}

	// method as bool
	_, err = httpNode.Execute(ctx, map[string]interface{}{
		"url":    "http://example.com",
		"method": true,
	})
	if err == nil {
		t.Fatal("Expected error for non-string method, got nil")
	}
	if !strings.Contains(err.Error(), "method parameter is required") {
		t.Errorf("Expected 'method parameter is required' error, got: %v", err)
	}
}

// --- Query Parameters Tests ---

// TestHTTPRequest_QueryParams tests that query parameters are appended to the URL.
func TestHTTPRequest_QueryParams(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		q := r.URL.Query()
		if q.Get("key") != "val" {
			t.Errorf("query key = %q, want %q", q.Get("key"), "val")
		}
		if q.Get("n") != "42" {
			t.Errorf("query n = %q, want %q", q.Get("n"), "42")
		}
		if q.Get("flag") != "true" {
			t.Errorf("query flag = %q, want %q", q.Get("flag"), "true")
		}
		w.WriteHeader(http.StatusOK)
		fmt.Fprint(w, `{"ok":true}`)
	}))
	defer ts.Close()

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
		"queryParams": map[string]interface{}{
			"key":  "val",
			"n":    42,
			"flag": true,
		},
	}

	result, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	if response["statusCode"] != 200 {
		t.Errorf("statusCode = %v, want 200", response["statusCode"])
	}
}

// TestHTTPRequest_QueryParams_NilSkipped tests that nil query params are skipped.
func TestHTTPRequest_QueryParams_NilSkipped(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		q := r.URL.Query()
		if q.Get("a") != "1" {
			t.Errorf("query a = %q, want %q", q.Get("a"), "1")
		}
		if q.Has("b") {
			t.Errorf("query param 'b' should be absent for nil value, got %q", q.Get("b"))
		}
		w.WriteHeader(http.StatusOK)
		fmt.Fprint(w, `{"ok":true}`)
	}))
	defer ts.Close()

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
		"queryParams": map[string]interface{}{
			"a": "1",
			"b": nil,
		},
	}

	_, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
}

// TestHTTPRequest_QueryParams_UnsupportedType tests error on unsupported query param type.
func TestHTTPRequest_QueryParams_UnsupportedType(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
		"queryParams": map[string]interface{}{
			"bad": []string{"not", "supported"},
		},
	}

	_, err := httpNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for unsupported query param type, got nil")
	}
	if !strings.Contains(err.Error(), "unsupported query param type") {
		t.Errorf("Expected 'unsupported query param type' error, got: %v", err)
	}
}

// TestHTTPRequest_QueryParams_Empty tests that empty query params leave URL unchanged.
func TestHTTPRequest_QueryParams_Empty(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.RawQuery != "" {
			t.Errorf("Expected empty query string, got %q", r.URL.RawQuery)
		}
		w.WriteHeader(http.StatusOK)
		fmt.Fprint(w, `{"ok":true}`)
	}))
	defer ts.Close()

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":         ts.URL,
		"method":      "GET",
		"queryParams": map[string]interface{}{},
	}

	_, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
}

// --- Save to File Tests ---

// TestHTTPRequest_SaveToFile tests saving response body to a file.
func TestHTTPRequest_SaveToFile(t *testing.T) {
	ctx := context.Background()
	content := "PNG-binary-content-here"

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "image/png")
		fmt.Fprint(w, content)
	}))
	defer ts.Close()

	outPath := filepath.Join(t.TempDir(), "out.png")

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":        ts.URL,
		"method":     "GET",
		"saveToFile": outPath,
	}

	result, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})

	// File should exist with correct content
	data, err := os.ReadFile(outPath)
	if err != nil {
		t.Fatalf("Failed to read saved file: %v", err)
	}
	if string(data) != content {
		t.Errorf("file content = %q, want %q", string(data), content)
	}

	// Response should have filePath and empty body
	if response["filePath"] != outPath {
		t.Errorf("filePath = %v, want %v", response["filePath"], outPath)
	}

	body, ok := response["body"].(map[string]interface{})
	if !ok {
		t.Fatal("Expected body to be map[string]interface{}")
	}
	if len(body) != 0 {
		t.Errorf("Expected empty body map, got %v", body)
	}
}

// TestHTTPRequest_SaveToFile_NestedDirs tests that parent directories are created.
func TestHTTPRequest_SaveToFile_NestedDirs(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "hello")
	}))
	defer ts.Close()

	outPath := filepath.Join(t.TempDir(), "a", "b", "c", "out.txt")

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":        ts.URL,
		"method":     "GET",
		"saveToFile": outPath,
	}

	_, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	if _, err := os.Stat(outPath); os.IsNotExist(err) {
		t.Error("Expected nested directories and file to be created")
	}
}

// TestHTTPRequest_SaveToFile_ResponseStructure verifies the full result shape for saveToFile.
func TestHTTPRequest_SaveToFile_ResponseStructure(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("X-Custom", "test")
		fmt.Fprint(w, "data")
	}))
	defer ts.Close()

	outPath := filepath.Join(t.TempDir(), "out.bin")

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":        ts.URL,
		"method":     "GET",
		"saveToFile": outPath,
	}

	result, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})

	// Verify all expected fields
	if response["statusCode"] != 200 {
		t.Errorf("statusCode = %v, want 200", response["statusCode"])
	}
	if response["filePath"] != outPath {
		t.Errorf("filePath = %v, want %v", response["filePath"], outPath)
	}
	if _, ok := response["headers"].(map[string]string); !ok {
		t.Error("Expected headers to be map[string]string")
	}
	if _, ok := response["body"].(map[string]interface{}); !ok {
		t.Error("Expected body to be map[string]interface{}")
	}
}

// --- Response Handling Tests ---

// TestHTTPRequest_NonJSONResponse tests that non-JSON responses return raw body.
func TestHTTPRequest_NonJSONResponse(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/plain")
		fmt.Fprint(w, "hello world")
	}))
	defer ts.Close()

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	result, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	body := response["body"].(map[string]interface{})

	if body["raw"] != "hello world" {
		t.Errorf("body[raw] = %v, want %q", body["raw"], "hello world")
	}
}

// TestHTTPRequest_EmptyBody tests handling of empty response body (204 No Content).
func TestHTTPRequest_EmptyBody(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNoContent)
	}))
	defer ts.Close()

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "DELETE",
	}

	result, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	response := result.(map[string]interface{})
	if response["statusCode"] != 204 {
		t.Errorf("statusCode = %v, want 204", response["statusCode"])
	}

	// Empty body should parse to nil map
	body := response["body"].(map[string]interface{})
	if body != nil && len(body) != 0 {
		t.Errorf("Expected nil or empty body, got %v", body)
	}
}

// --- HTTP Methods Test ---

// TestHTTPRequest_Methods tests PUT, DELETE, and PATCH methods via table-driven test.
func TestHTTPRequest_Methods(t *testing.T) {
	methods := []string{"PUT", "DELETE", "PATCH"}

	for _, method := range methods {
		t.Run(method, func(t *testing.T) {
			ctx := context.Background()

			ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.Method != method {
					t.Errorf("Method = %s, want %s", r.Method, method)
				}
				w.WriteHeader(http.StatusOK)
				fmt.Fprint(w, `{"ok":true}`)
			}))
			defer ts.Close()

			httpNode := httpnode.New()
			params := map[string]interface{}{
				"url":    ts.URL,
				"method": method,
			}

			result, err := httpNode.Execute(ctx, params)
			if err != nil {
				t.Fatalf("Execute failed: %v", err)
			}

			response := result.(map[string]interface{})
			if response["statusCode"] != 200 {
				t.Errorf("statusCode = %v, want 200", response["statusCode"])
			}
		})
	}
}

// --- Context Cancellation Test ---

// TestHTTPRequest_ContextCancellation tests that a cancelled context returns an error.
func TestHTTPRequest_ContextCancellation(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(5 * time.Second)
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "GET",
	}

	_, err := httpNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for cancelled context, got nil")
	}
	if !strings.Contains(err.Error(), "canceled") {
		t.Errorf("Expected 'canceled' in error, got: %v", err)
	}
}

// --- Content-Type Auto-Set Test ---

// TestHTTPRequest_ContentTypeAutoSet tests that Content-Type is auto-set when body is present.
func TestHTTPRequest_ContentTypeAutoSet(t *testing.T) {
	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ct := r.Header.Get("Content-Type")
		if ct != "application/json" {
			t.Errorf("Content-Type = %q, want %q", ct, "application/json")
		}
		w.WriteHeader(http.StatusOK)
		fmt.Fprint(w, `{"ok":true}`)
	}))
	defer ts.Close()

	httpNode := httpnode.New()
	params := map[string]interface{}{
		"url":    ts.URL,
		"method": "POST",
		"body": map[string]interface{}{
			"key": "value",
		},
		// No explicit headers — Content-Type should be auto-set
	}

	_, err := httpNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
}
