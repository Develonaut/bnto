package image

import (
	"context"
	"image/color"
	"os"
	"path/filepath"
	"testing"

	"github.com/disintegration/imaging"
)

func TestComposite(t *testing.T) {
	// Create temp directory for test files
	tmpDir := t.TempDir()

	// Create a simple base image (200x200 blue square)
	baseImg := imaging.New(200, 200, color.RGBA{0, 0, 255, 255})
	basePath := filepath.Join(tmpDir, "base.png")
	if err := imaging.Save(baseImg, basePath); err != nil {
		t.Fatalf("failed to create base image: %v", err)
	}

	// Create a simple overlay image (100x100 red square)
	overlayImg := imaging.New(100, 100, color.RGBA{255, 0, 0, 255})
	overlayPath := filepath.Join(tmpDir, "overlay.png")
	if err := imaging.Save(overlayImg, overlayPath); err != nil {
		t.Fatalf("failed to create overlay image: %v", err)
	}

	// Test composite
	img := New()
	outputPath := filepath.Join(tmpDir, "output.png")

	params := map[string]interface{}{
		"operation": "composite",
		"base":      basePath,
		"overlay":   overlayPath,
		"output":    outputPath,
		"position":  "center",
		"format":    "png",
		"quality":   95,
	}

	result, err := img.Execute(context.Background(), params)
	if err != nil {
		t.Fatalf("composite failed: %v", err)
	}

	// Verify result
	resultMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map result, got %T", result)
	}

	// Check output file exists
	if _, err := os.Stat(outputPath); err != nil {
		t.Fatalf("output file not created: %v", err)
	}

	// Verify path in result
	if path, ok := resultMap["path"].(string); !ok || path != outputPath {
		t.Errorf("expected path %s, got %v", outputPath, resultMap["path"])
	}

	// Verify dimensions match base image
	if dims, ok := resultMap["dimensions"].(map[string]int); ok {
		if dims["width"] != 200 || dims["height"] != 200 {
			t.Errorf("expected dimensions 200x200, got %dx%d", dims["width"], dims["height"])
		}
	} else {
		t.Error("dimensions not in result")
	}

	t.Logf("Composite test passed: %+v", resultMap)
}
