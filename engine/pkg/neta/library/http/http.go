// Package http provides HTTP request functionality for the bento workflow system.
//
// The http neta allows you to make HTTP requests (GET, POST, PUT, DELETE, etc.)
// to external APIs and services. It supports:
//   - Custom headers (including authentication)
//   - Request timeouts
//   - JSON request/response bodies
//   - Error handling for 4xx/5xx responses
//
// Example usage:
//
//	params := map[string]interface{}{
//	    "url": "https://api.example.com/users",
//	    "method": "GET",
//	    "headers": map[string]interface{}{
//	        "Authorization": "Bearer token123",
//	    },
//	    "timeout": 30,  // seconds
//	}
//
//	result, err := httpNeta.Execute(ctx, params)
//
// The result contains:
//   - statusCode: HTTP status code (200, 404, etc.)
//   - body: Parsed JSON response body
//   - headers: Response headers
//
// Learn more about Go's net/http package: https://pkg.go.dev/net/http
package http

import (
	"context"
	"fmt"

	"github.com/Develonaut/bento/pkg/neta"
)

const (
	// DefaultTimeout is the default HTTP request timeout in seconds.
	DefaultTimeout = 30
)

// HTTPNeta implements HTTP request functionality.
type HTTPNeta struct{}

// New creates a new HTTP neta instance.
func New() neta.Executable {
	return &HTTPNeta{}
}

// Execute performs an HTTP request based on the provided parameters.
//
// Parameters:
//   - url (string, required): The URL to request
//   - method (string, required): HTTP method (GET, POST, PUT, DELETE, etc.)
//   - headers (map[string]interface{}, optional): Custom headers
//   - body (map[string]interface{}, optional): Request body (will be JSON encoded)
//   - timeout (int, optional): Request timeout in seconds (default: 30)
//   - saveToFile (string, optional): Path to save response body to file (skips JSON parsing)
//   - queryParams (map[string]interface{}, optional): URL query parameters to append
//
// Returns a map with:
//   - statusCode (int): HTTP status code
//   - body (map[string]interface{}): Parsed JSON response (or empty if saveToFile is used)
//   - headers (map[string]string): Response headers
//   - filePath (string): Path where file was saved (only if saveToFile is used)
func (h *HTTPNeta) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	reqParams, err := extractRequestParams(params)
	if err != nil {
		return nil, err
	}

	req, err := h.buildRequest(ctx, reqParams, params)
	if err != nil {
		return nil, err
	}

	resp, err := h.executeRequest(req, reqParams.timeout)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return h.handleResponse(resp, reqParams.saveToFile)
}

// requestParams holds extracted request parameters.
type requestParams struct {
	url        string
	method     string
	timeout    int
	saveToFile string
}

// extractRequestParams extracts and validates request parameters.
func extractRequestParams(params map[string]interface{}) (*requestParams, error) {
	urlStr, ok := params["url"].(string)
	if !ok {
		return nil, fmt.Errorf("url parameter is required and must be a string")
	}

	method, ok := params["method"].(string)
	if !ok {
		return nil, fmt.Errorf("method parameter is required and must be a string")
	}

	timeout := DefaultTimeout
	if t, ok := params["timeout"].(int); ok {
		timeout = t
	}

	saveToFile, _ := params["saveToFile"].(string)

	return &requestParams{
		url:        urlStr,
		method:     method,
		timeout:    timeout,
		saveToFile: saveToFile,
	}, nil
}
