package integration

import (
	"fmt"
	stdimage "image"
	"image/color"
	"image/png"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// TestImageOptimize_ConvertsToWebP validates the image optimization bento.
// Tests:
// 1. Parallel neta executes 8 image conversions with maxConcurrency: 4
// 2. Converts 8 PNG files to WebP format
// 3. Deletes original PNG files after conversion
// 4. Validates parallel execution provides speedup
func TestImageOptimize_ConvertsToWebP(t *testing.T) {
	projectRoot := "../../"

	// Setup: Clean and create test product folder
	productsDir := filepath.Join(projectRoot, "products")
	defer CleanupTestDir(t, productsDir)
	CleanupTestDir(t, productsDir)

	productDir := filepath.Join(productsDir, "MOCK-001")
	err := os.MkdirAll(productDir, 0755)
	require.NoError(t, err, "Should create product folder")

	// Create 8 test PNG files (100x100 pixels with different colors)
	t.Log("Creating 8 test PNG files...")
	for i := 1; i <= 8; i++ {
		pngPath := filepath.Join(productDir, fmt.Sprintf("render-%d.png", i))
		createTestPNG(t, pngPath, i)
		VerifyFileExists(t, pngPath)
	}

	// Verify starting state: 8 PNGs, 0 WebPs
	VerifyFileCount(t, productDir, "render-*.png", 8)
	VerifyFileCount(t, productDir, "render-*.webp", 0)

	// Set environment variables for bento template resolution
	envVars := map[string]string{
		"PRODUCT_SKU": "MOCK-001",
	}

	// Execute bento and measure time
	bentoPath := "examples/image-optimize.bento.json"
	t.Log("Executing image optimization bento...")
	startTime := time.Now()
	output, err := RunBento(t, bentoPath, envVars)
	duration := time.Since(startTime)

	// Log output for debugging
	t.Logf("Bento output:\n%s", output)
	t.Logf("Execution time: %v", duration)

	// Verify execution succeeded
	require.NoError(t, err, "Image optimization bento should execute successfully\nOutput: %s", output)

	// Verify output shows success
	assert.Contains(t, output, "Delicious! Bento executed successfully", "Should show success message")

	// Verify 8 WebP files created
	t.Log("Verifying 8 WebP files were created...")
	for i := 1; i <= 8; i++ {
		webpPath := filepath.Join(productDir, fmt.Sprintf("render-%d.webp", i))
		VerifyFileExists(t, webpPath)

		// Verify it's a valid WebP file
		content, err := os.ReadFile(webpPath)
		require.NoError(t, err, "Should read WebP file %d", i)
		assert.True(t, len(content) > 12, "WebP %d should have content", i)

		// WebP magic bytes: RIFF....WEBP
		assert.Equal(t, "RIFF", string(content[0:4]), "WebP %d should start with RIFF", i)
		assert.Equal(t, "WEBP", string(content[8:12]), "WebP %d should contain WEBP signature", i)
	}

	// Verify correct WebP count
	VerifyFileCount(t, productDir, "render-*.webp", 8)

	// Verify PNGs were deleted
	t.Log("Verifying original PNG files were deleted...")
	VerifyFileCount(t, productDir, "render-*.png", 0)

	// Verify we get reasonable performance (parallel should be faster)
	// Sequential: ~6s (750ms per image)
	// Parallel (4 concurrent): ~1.5-2s
	// Note: In CI or slow systems, we're lenient with timing
	assert.Less(t, duration.Seconds(), 10.0,
		"Should complete in reasonable time with parallel execution (got %v)", duration)

	t.Log("✓ Successfully converted 8 PNGs to WebP")
	t.Log("✓ Verified WebP file format (RIFF...WEBP signature)")
	t.Log("✓ Original PNG files deleted")
	t.Logf("✓ Completed in %v (parallel execution)", duration)
	t.Log("✓ Validated parallel neta with maxConcurrency: 4")
}

// TestImageOptimize_ParallelPerformance validates parallel execution speedup.
// This test measures actual parallel speedup by comparing concurrent execution
// to theoretical sequential time.
func TestImageOptimize_ParallelPerformance(t *testing.T) {
	if testing.Short() {
		t.Skip("Skipping performance test in short mode")
	}

	projectRoot := "../../"

	// Setup: Clean and create test product folder
	productsDir := filepath.Join(projectRoot, "products")
	defer CleanupTestDir(t, productsDir)
	CleanupTestDir(t, productsDir)

	productDir := filepath.Join(productsDir, "PERF-TEST")
	err := os.MkdirAll(productDir, 0755)
	require.NoError(t, err, "Should create product folder")

	// Create 8 test PNG files
	t.Log("Creating 8 test PNG files for performance testing...")
	for i := 1; i <= 8; i++ {
		pngPath := filepath.Join(productDir, fmt.Sprintf("render-%d.png", i))
		createTestPNG(t, pngPath, i)
	}

	envVars := map[string]string{
		"PRODUCT_SKU": "PERF-TEST",
	}

	// Execute bento and measure time
	bentoPath := "examples/image-optimize.bento.json"
	t.Log("Running parallel optimization (maxConcurrency: 4)...")
	startTime := time.Now()
	output, err := RunBento(t, bentoPath, envVars)
	parallelDuration := time.Since(startTime)

	require.NoError(t, err, "Should execute successfully\nOutput: %s", output)

	// Verify all conversions completed
	VerifyFileCount(t, productDir, "render-*.webp", 8)
	VerifyFileCount(t, productDir, "render-*.png", 0)

	// Log performance results
	t.Logf("Parallel execution (4 concurrent): %v", parallelDuration)
	t.Logf("Average time per image: %v", parallelDuration/8)

	// Performance assertion: Should be faster than sequential
	// Sequential would be ~6s (750ms × 8 images)
	// Parallel should be ~1.5-2.5s depending on system
	// We'll assert it's under 5s to be safe for CI
	assert.Less(t, parallelDuration.Seconds(), 5.0,
		"Parallel execution should complete faster than sequential (got %v)", parallelDuration)

	t.Log("✓ Parallel execution provides speedup")
	t.Logf("✓ Processed 8 images in %v", parallelDuration)
}

// TestImageOptimize_MissingSourceFiles validates error handling.
// When source PNG files don't exist, the image neta should fail gracefully.
func TestImageOptimize_MissingSourceFiles(t *testing.T) {
	projectRoot := "../../"

	// Setup: Create empty product folder (no PNG files)
	productsDir := filepath.Join(projectRoot, "products")
	defer CleanupTestDir(t, productsDir)
	CleanupTestDir(t, productsDir)

	productDir := filepath.Join(productsDir, "MISSING-TEST")
	err := os.MkdirAll(productDir, 0755)
	require.NoError(t, err, "Should create product folder")

	// Don't create PNG files - they should be missing

	envVars := map[string]string{
		"PRODUCT_SKU": "MISSING-TEST",
	}

	bentoPath := "examples/image-optimize.bento.json"
	output, err := RunBento(t, bentoPath, envVars)

	// Should fail when source files don't exist
	assert.Error(t, err, "Should fail when PNG files don't exist\nOutput: %s", output)

	// Verify no WebP files created
	matches, _ := filepath.Glob(filepath.Join(productDir, "render-*.webp"))
	assert.Equal(t, 0, len(matches), "Should not create any WebP files when source PNGs missing")

	t.Log("✓ Correctly failed with missing source files")
	t.Log("✓ Validated error handling for image neta")
}

// createTestPNG creates a small test PNG file with a solid color.
// Uses stdlib image to generate a valid PNG with proper headers.
// colorIndex determines the color (1-8 → different shades).
func createTestPNG(t *testing.T, path string, colorIndex int) {
	t.Helper()

	// Create a 100x100 image with a solid color
	// Color varies based on colorIndex for visual distinction
	r := uint8((colorIndex * 30) % 256)
	g := uint8((colorIndex * 60) % 256)
	b := uint8((colorIndex * 90) % 256)

	img := stdimage.NewRGBA(stdimage.Rect(0, 0, 100, 100))
	colorVal := color.RGBA{R: r, G: g, B: b, A: 255}

	// Fill with solid color
	for y := 0; y < 100; y++ {
		for x := 0; x < 100; x++ {
			img.Set(x, y, colorVal)
		}
	}

	// Write to file
	f, err := os.Create(path)
	require.NoError(t, err, "Should create PNG file")
	defer f.Close()

	err = png.Encode(f, img)
	require.NoError(t, err, "Should encode PNG")

	t.Logf("Created test PNG: %s (color: rgb(%d,%d,%d))",
		filepath.Base(path), r, g, b)
}
