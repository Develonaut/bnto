package mocks

import (
	"encoding/json"
	"net/http"
	"strings"
	"testing"
)

func TestFigmaServer_ValidRequest(t *testing.T) {
	server := NewFigmaServer()
	defer server.Close()

	req, err := http.NewRequest("GET", server.URL+"/v1/images/test-file", nil)
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}
	req.Header.Set("X-Figma-Token", "test-token")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		t.Errorf("Expected status 200, got %d", resp.StatusCode)
	}

	var body map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	// Verify response structure matches real Figma API
	images, ok := body["images"].(map[string]interface{})
	if !ok {
		t.Fatal("Response should contain 'images' object")
	}

	url, ok := images["test-component"].(string)
	if !ok || url == "" {
		t.Error("Should return URL for test-component")
	}
}

func TestFigmaServer_MissingToken(t *testing.T) {
	server := NewFigmaServer()
	defer server.Close()

	resp, err := http.Get(server.URL + "/v1/images/test-file")
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("Expected status 401, got %d", resp.StatusCode)
	}

	var body map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	// Verify error message
	if body["error"] == nil || body["error"] == "" {
		t.Error("Should return error message")
	}
}

func TestFigmaServer_ResponseFormat(t *testing.T) {
	server := NewFigmaServer()
	defer server.Close()

	req, err := http.NewRequest("GET", server.URL+"/v1/images/test-file", nil)
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}
	req.Header.Set("X-Figma-Token", "test-token")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	// Verify Content-Type header
	contentType := resp.Header.Get("Content-Type")
	if !strings.Contains(contentType, "application/json") {
		t.Errorf("Expected Content-Type to contain 'application/json', got %s", contentType)
	}

	var body map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	// Verify the images object contains a valid URL
	images, ok := body["images"].(map[string]interface{})
	if !ok {
		t.Fatal("Response should contain 'images' object")
	}

	imageURL, ok := images["test-component"].(string)
	if !ok {
		t.Fatal("Image URL should be a string")
	}
	if imageURL == "" {
		t.Error("Image URL should not be empty")
	}
}
