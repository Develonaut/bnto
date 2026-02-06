package http

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

// buildRequest creates an HTTP request with headers and body.
func (h *HTTPNeta) buildRequest(
	ctx context.Context,
	reqParams *requestParams,
	params map[string]interface{},
) (*http.Request, error) {
	urlStr, err := h.prepareURL(reqParams.url, params)
	if err != nil {
		return nil, err
	}

	reqBody, err := h.prepareRequestBody(params)
	if err != nil {
		return nil, err
	}

	req, err := http.NewRequestWithContext(ctx, reqParams.method, urlStr, reqBody)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	h.setRequestHeaders(req, reqBody, params)
	return req, nil
}

// prepareURL adds query parameters to the base URL.
func (h *HTTPNeta) prepareURL(baseURL string, params map[string]interface{}) (string, error) {
	if queryParams, ok := params["queryParams"].(map[string]interface{}); ok {
		urlStr, err := addQueryParams(baseURL, queryParams)
		if err != nil {
			return "", fmt.Errorf("failed to add query parameters: %w", err)
		}
		return urlStr, nil
	}
	return baseURL, nil
}

// prepareRequestBody marshals the request body to JSON.
func (h *HTTPNeta) prepareRequestBody(params map[string]interface{}) (io.Reader, error) {
	body, ok := params["body"].(map[string]interface{})
	if !ok {
		return nil, nil
	}

	jsonBody, err := json.Marshal(body)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request body: %w", err)
	}
	return bytes.NewReader(jsonBody), nil
}

// setRequestHeaders sets Content-Type and custom headers on the request.
func (h *HTTPNeta) setRequestHeaders(req *http.Request, reqBody io.Reader, params map[string]interface{}) {
	if reqBody != nil {
		req.Header.Set("Content-Type", "application/json")
	}

	if headers, ok := params["headers"].(map[string]interface{}); ok {
		for key, value := range headers {
			if strValue, ok := value.(string); ok {
				req.Header.Set(key, strValue)
			}
		}
	}
}

// executeRequest executes the HTTP request with timeout.
func (h *HTTPNeta) executeRequest(req *http.Request, timeout int) (*http.Response, error) {
	client := &http.Client{
		Timeout: time.Duration(timeout) * time.Second,
	}

	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("request failed: %w", err)
	}
	return resp, nil
}
