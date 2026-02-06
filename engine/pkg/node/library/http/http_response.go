package http

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
)

// handleResponse processes the HTTP response and returns the result.
func (h *HTTPNeta) handleResponse(resp *http.Response, saveToFile string) (interface{}, error) {
	responseHeaders := collectResponseHeaders(resp)

	if saveToFile != "" {
		return h.handleFileSave(resp, saveToFile, responseHeaders)
	}

	return h.handleJSONResponse(resp, responseHeaders)
}

// collectResponseHeaders extracts response headers into a map.
func collectResponseHeaders(resp *http.Response) map[string]string {
	headers := make(map[string]string)
	for key, values := range resp.Header {
		if len(values) > 0 {
			headers[key] = values[0]
		}
	}
	return headers
}

// handleFileSave saves response body to file and returns metadata.
func (h *HTTPNeta) handleFileSave(
	resp *http.Response,
	filePath string,
	headers map[string]string,
) (interface{}, error) {
	if err := saveResponseToFile(resp.Body, filePath); err != nil {
		return nil, fmt.Errorf("failed to save response to file: %w", err)
	}

	return map[string]interface{}{
		"statusCode": resp.StatusCode,
		"headers":    headers,
		"filePath":   filePath,
		"body":       map[string]interface{}{},
	}, nil
}

// handleJSONResponse parses response body as JSON.
func (h *HTTPNeta) handleJSONResponse(
	resp *http.Response,
	headers map[string]string,
) (interface{}, error) {
	respBodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %w", err)
	}

	responseBody := parseJSONBody(respBodyBytes)

	return map[string]interface{}{
		"statusCode": resp.StatusCode,
		"body":       responseBody,
		"headers":    headers,
	}, nil
}

// parseJSONBody parses JSON bytes into a map, returning raw string on error.
func parseJSONBody(data []byte) map[string]interface{} {
	var body map[string]interface{}
	if len(data) > 0 {
		if err := json.Unmarshal(data, &body); err != nil {
			return map[string]interface{}{
				"raw": string(data),
			}
		}
	}
	return body
}

// saveResponseToFile saves the HTTP response body to a file.
// Creates parent directories if they don't exist.
func saveResponseToFile(body io.Reader, filePath string) error {
	// Create parent directories if needed
	dir := filepath.Dir(filePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory %s: %w", dir, err)
	}

	// Create file
	file, err := os.Create(filePath)
	if err != nil {
		return fmt.Errorf("failed to create file %s: %w", filePath, err)
	}
	defer file.Close()

	// Copy response body to file
	if _, err := io.Copy(file, body); err != nil {
		return fmt.Errorf("failed to write to file %s: %w", filePath, err)
	}

	return nil
}

// addQueryParams adds query parameters to a URL.
// Returns error if URL parsing fails or if param values are unsupported types.
func addQueryParams(baseURL string, params map[string]interface{}) (string, error) {
	if len(params) == 0 {
		return baseURL, nil
	}

	// Parse the URL
	u, err := url.Parse(baseURL)
	if err != nil {
		return "", fmt.Errorf("failed to parse URL %q: %w", baseURL, err)
	}

	// Get existing query params
	q := u.Query()

	// Add new params with type validation
	for key, value := range params {
		strVal := ""
		switch v := value.(type) {
		case string:
			strVal = v
		case int, int64, float64, bool:
			strVal = fmt.Sprintf("%v", v)
		case nil:
			// Skip nil values
			continue
		default:
			return "", fmt.Errorf("unsupported query param type for %q: %T", key, value)
		}
		q.Add(key, strVal)
	}

	// Set updated query params
	u.RawQuery = q.Encode()

	return u.String(), nil
}
