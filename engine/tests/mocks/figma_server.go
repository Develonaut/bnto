package mocks

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
)

// NewFigmaServer creates a mock Figma API server.
// Returns URLs for test images like real Figma API.
//
// Real Figma API response format:
//
//	{
//	  "images": {
//	    "node-id": "https://figma-alpha-api.s3.us-west-2.amazonaws.com/..."
//	  }
//	}
//
// This mock returns the same structure for testing.
// It also serves the actual PNG image at the returned URL.
func NewFigmaServer() *httptest.Server {
	// Create server with nil handler first
	server := httptest.NewUnstartedServer(nil)

	// Now we can create handler that references server.URL
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Serve mock overlay image
		if strings.HasPrefix(r.URL.Path, "/mock-overlay.png") {
			serveMockPNG(w, r)
			return
		}

		// Handle Figma API endpoint
		// server.URL is now safely accessible
		if strings.HasPrefix(r.URL.Path, "/v1/images/") {
			handleFigmaAPI(w, r, server.URL)
			return
		}

		http.NotFound(w, r)
	})

	server.Config.Handler = handler
	server.Start()

	return server
}

// handleFigmaAPI simulates Figma API /v1/images endpoint
func handleFigmaAPI(w http.ResponseWriter, r *http.Request, serverURL string) {
	// Check auth header (real Figma API requires X-Figma-Token)
	if r.Header.Get("X-Figma-Token") == "" {
		w.WriteHeader(http.StatusUnauthorized)
		w.Header().Set("Content-Type", "application/json")
		if err := json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Missing Figma token",
		}); err != nil {
			http.Error(w, "Internal server error", http.StatusInternalServerError)
		}
		return
	}

	// Simulate Figma API response
	// Real API returns: {"images": {"node-id": "https://..."}}
	// Point to our own server's /mock-overlay.png endpoint
	response := map[string]interface{}{
		"images": map[string]interface{}{
			"test-component": serverURL + "/mock-overlay.png",
		},
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(response); err != nil {
		http.Error(w, "Internal server error", http.StatusInternalServerError)
	}
}

// serveMockPNG serves a minimal valid 1x1 PNG image
// PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
// This is a 1x1 transparent PNG (68 bytes)
func serveMockPNG(w http.ResponseWriter, r *http.Request) {
	// Minimal 1x1 transparent PNG (base64 decoded)
	pngBytes := []byte{
		0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
		0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
		0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 dimensions
		0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4, // 8-bit RGBA
		0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, // IDAT chunk
		0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, // compressed data
		0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, // compressed data
		0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, // IEND chunk
		0x42, 0x60, 0x82, // IEND chunk end
	}

	w.Header().Set("Content-Type", "image/png")
	if _, err := w.Write(pngBytes); err != nil {
		http.Error(w, "Failed to write PNG", http.StatusInternalServerError)
	}
}
