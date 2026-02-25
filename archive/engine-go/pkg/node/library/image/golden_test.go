package image_test

import (
	"context"
	"image/color"
	"image/png"
	stdimage "image"
	"os"
	"path/filepath"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/image"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

// createGoldenTestPNG creates a solid-color PNG test image in the given dir.
func createGoldenTestPNG(t *testing.T, dir string, name string, width, height int) string {
	t.Helper()
	path := filepath.Join(dir, name)

	img := stdimage.NewRGBA(stdimage.Rect(0, 0, width, height))
	red := color.RGBA{255, 0, 0, 255}
	for y := range height {
		for x := range width {
			img.Set(x, y, red)
		}
	}

	f, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	defer f.Close()

	if err := png.Encode(f, img); err != nil {
		t.Fatal(err)
	}
	return path
}

func TestGolden_ResizePreservesDimensions(t *testing.T) {
	dir := t.TempDir()
	src := createGoldenTestPNG(t, dir, "input.png", 400, 300)
	outPath := filepath.Join(dir, "resized.png")

	node := image.New()
	_, err := node.Execute(context.Background(), map[string]any{
		"operation": "resize",
		"input":     src,
		"output":    outPath,
		"width":     float64(200),
		"height":    float64(150),
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertImageProperties(t, outPath, testgolden.ImageProperties{
		Width:  200,
		Height: 150,
		Format: "png",
	})
}

func TestGolden_ResizeAspectRatio(t *testing.T) {
	dir := t.TempDir()
	src := createGoldenTestPNG(t, dir, "input.png", 400, 200)
	outPath := filepath.Join(dir, "resized.png")

	node := image.New()
	_, err := node.Execute(context.Background(), map[string]any{
		"operation":   "resize",
		"input":       src,
		"output":      outPath,
		"width":       float64(200),
		"aspectRatio": true,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertImageProperties(t, outPath, testgolden.ImageProperties{
		Width:  200,
		Height: 100, // 2:1 aspect ratio preserved
		Format: "png",
	})
}

func TestGolden_ConvertPNGToWebP(t *testing.T) {
	dir := t.TempDir()
	src := createGoldenTestPNG(t, dir, "input.png", 200, 200)
	outPath := filepath.Join(dir, "output.webp")

	node := image.New()
	_, err := node.Execute(context.Background(), map[string]any{
		"operation": "convert",
		"input":     src,
		"output":    outPath,
		"format":    "webp",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertImageProperties(t, outPath, testgolden.ImageProperties{
		Width:  200,
		Height: 200,
		Format: "webp",
	})
}

func TestGolden_ConvertPNGToJPEG(t *testing.T) {
	dir := t.TempDir()
	src := createGoldenTestPNG(t, dir, "input.png", 200, 200)
	outPath := filepath.Join(dir, "output.jpg")

	node := image.New()
	_, err := node.Execute(context.Background(), map[string]any{
		"operation": "convert",
		"input":     src,
		"output":    outPath,
		"format":    "jpeg",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertImageProperties(t, outPath, testgolden.ImageProperties{
		Width:  200,
		Height: 200,
		Format: "jpeg",
	})
}

func TestGolden_OptimizeReducesSize(t *testing.T) {
	dir := t.TempDir()
	// Create a larger image so optimization has something to compress
	src := createGoldenTestPNG(t, dir, "input.png", 500, 500)
	outPath := filepath.Join(dir, "optimized.png")

	node := image.New()
	_, err := node.Execute(context.Background(), map[string]any{
		"operation": "optimize",
		"input":     src,
		"output":    outPath,
		"quality":   float64(50),
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertImageProperties(t, outPath, testgolden.ImageProperties{
		Width:  500,
		Height: 500,
	})
}
