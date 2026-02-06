package image_test

import (
	"context"
	stdimage "image"
	"image/color"
	"image/png"
	"os"
	"strings"
	"testing"

	imageneta "github.com/Develonaut/bento/pkg/neta/library/image"
)

// createTestImage creates a simple test PNG image
func createTestImage(t *testing.T, width, height int) string {
	t.Helper()

	img := stdimage.NewRGBA(stdimage.Rect(0, 0, width, height))
	red := color.RGBA{255, 0, 0, 255}

	// Fill with red
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			img.Set(x, y, red)
		}
	}

	tmpfile, err := os.CreateTemp("", "test-*.png")
	if err != nil {
		t.Fatal(err)
	}

	if err := png.Encode(tmpfile, img); err != nil {
		t.Fatal(err)
	}
	tmpfile.Close()

	return tmpfile.Name()
}

// TestImage_Resize tests image resizing with aspect ratio
func TestImage_Resize(t *testing.T) {
	ctx := context.Background()

	inputPath := createTestImage(t, 800, 600)
	defer os.Remove(inputPath)

	outputPath := strings.Replace(inputPath, ".png", "-resized.png", 1)
	defer os.Remove(outputPath)

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "resize",
		"input":     inputPath,
		"output":    outputPath,
		"width":     400,
		"height":    300,
	}

	result, err := imgNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	if output["path"] != outputPath {
		t.Errorf("path = %v, want %v", output["path"], outputPath)
	}

	// Verify output file exists
	if _, err := os.Stat(outputPath); os.IsNotExist(err) {
		t.Error("Output file not created")
	}
}

// TestImage_ConvertToWebP tests format conversion to WebP
// CRITICAL FOR PHASE 8: Blender outputs PNG, optimize to WebP
func TestImage_ConvertToWebP(t *testing.T) {
	ctx := context.Background()

	inputPath := createTestImage(t, 100, 100)
	defer os.Remove(inputPath)

	outputPath := strings.Replace(inputPath, ".png", ".webp", 1)
	defer os.Remove(outputPath)

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "convert",
		"input":     inputPath,
		"output":    outputPath,
		"format":    "webp",
		"quality":   80,
	}

	result, err := imgNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	// Verify output file exists
	if _, err := os.Stat(output["path"].(string)); os.IsNotExist(err) {
		t.Errorf("Output file not created: %v", output["path"])
	}

	// Check file size was reported
	if _, ok := output["size"].(int64); !ok {
		t.Error("Expected size in output")
	}
}

// TestImage_Optimize tests image optimization with quality settings
func TestImage_Optimize(t *testing.T) {
	ctx := context.Background()

	inputPath := createTestImage(t, 200, 200)
	defer os.Remove(inputPath)

	outputPath := strings.Replace(inputPath, ".png", "-optimized.webp", 1)
	defer os.Remove(outputPath)

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "optimize",
		"input":     inputPath,
		"output":    outputPath,
		"quality":   60, // Lower quality for smaller file
	}

	result, err := imgNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	// Verify output file exists
	if _, err := os.Stat(output["path"].(string)); os.IsNotExist(err) {
		t.Error("Output file not created")
	}
}

// TestImage_BatchOperations tests processing multiple images
func TestImage_BatchOperations(t *testing.T) {
	ctx := context.Background()

	// Create 3 test images
	images := []string{
		createTestImage(t, 100, 100),
		createTestImage(t, 100, 100),
		createTestImage(t, 100, 100),
	}
	defer func() {
		for _, img := range images {
			os.Remove(img)
		}
	}()

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "batch",
		"inputs":    images,
		"format":    "webp",
		"quality":   80,
	}

	result, err := imgNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	processed, ok := output["processed"].(int)
	if !ok {
		t.Fatal("Expected processed count")
	}

	if processed != 3 {
		t.Errorf("processed = %d, want 3", processed)
	}

	// Clean up batch outputs
	if outputs, ok := output["outputs"].([]string); ok {
		for _, out := range outputs {
			os.Remove(out)
		}
	}
}

// TestImage_MaintainAspectRatio tests aspect ratio preservation
func TestImage_MaintainAspectRatio(t *testing.T) {
	ctx := context.Background()

	inputPath := createTestImage(t, 800, 600) // 4:3 ratio
	defer os.Remove(inputPath)

	outputPath := strings.Replace(inputPath, ".png", "-resized.png", 1)
	defer os.Remove(outputPath)

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation":      "resize",
		"input":          inputPath,
		"output":         outputPath,
		"width":          400,
		"maintainAspect": true, // Should calculate height automatically
	}

	result, err := imgNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	// Verify output dimensions maintain aspect ratio
	if dims, ok := output["dimensions"].(map[string]int); ok {
		expectedHeight := 300 // 4:3 ratio of 400 width
		if dims["height"] != expectedHeight {
			t.Errorf("height = %d, want %d (aspect ratio not maintained)", dims["height"], expectedHeight)
		}
	}
}

// TestImage_InvalidInput tests error handling for missing input
func TestImage_InvalidInput(t *testing.T) {
	ctx := context.Background()

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "resize",
		"input":     "/nonexistent/file.png",
		"output":    "/tmp/output.png",
	}

	_, err := imgNeta.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for nonexistent input, got nil")
	}
}

// TestImage_InvalidOperation tests error handling for invalid operation
func TestImage_InvalidOperation(t *testing.T) {
	ctx := context.Background()

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "invalid",
	}

	_, err := imgNeta.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid operation, got nil")
	}
}

// TestImage_DefaultQuality tests default quality parameter
func TestImage_DefaultQuality(t *testing.T) {
	ctx := context.Background()

	inputPath := createTestImage(t, 100, 100)
	defer os.Remove(inputPath)

	outputPath := strings.Replace(inputPath, ".png", ".webp", 1)
	defer os.Remove(outputPath)

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "convert",
		"input":     inputPath,
		"output":    outputPath,
		"format":    "webp",
		// No quality specified - should use default (80)
	}

	result, err := imgNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	// Verify file was created
	if _, err := os.Stat(output["path"].(string)); os.IsNotExist(err) {
		t.Error("Output file not created")
	}
}

// TestImage_JPEGConversion tests JPEG format conversion
func TestImage_JPEGConversion(t *testing.T) {
	ctx := context.Background()

	inputPath := createTestImage(t, 100, 100)
	defer os.Remove(inputPath)

	outputPath := strings.Replace(inputPath, ".png", ".jpg", 1)
	defer os.Remove(outputPath)

	imgNeta := imageneta.New()

	params := map[string]interface{}{
		"operation": "convert",
		"input":     inputPath,
		"output":    outputPath,
		"format":    "jpeg",
		"quality":   90,
	}

	result, err := imgNeta.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	// Verify output file exists
	if _, err := os.Stat(output["path"].(string)); os.IsNotExist(err) {
		t.Error("Output file not created")
	}
}
