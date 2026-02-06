package integration

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// TestProductAutomation_EndToEnd validates the complete product automation workflow.
// Tests CSV reading → folder creation → copy overlays → Blender render → WebP optimization
func TestProductAutomation_EndToEnd(t *testing.T) {
	t.Skip("Skipping test: examples/product-automation.bento.json does not exist yet")

	projectRoot := "../../"
	defer CleanupTestDir(t, projectRoot+"products")
	CleanupTestDir(t, projectRoot+"products")

	envVars := setupTestEnvironment(t, projectRoot)
	output, err := RunBento(t, "examples/product-automation.bento.json", envVars)

	require.NoError(t, err, "Product automation should complete successfully\nOutput: %s", output)
	verifyBentoSuccess(t, output)
	verifyAllProductsProcessed(t, output, []string{"Combat Dog (Supplies)", "Combat Dog (Gas Mask)", "Combat Dog (Attack)"})
	verifyProductOutputs(t, projectRoot, []string{"Combat Dog (Supplies)", "Combat Dog (Gas Mask)", "Combat Dog (Attack)"})
	verifyStreamingOutput(t, output)
}

// setupTestEnvironment creates environment variables for the test
func setupTestEnvironment(t *testing.T, projectRoot string) map[string]string {
	t.Helper()

	testCSV := "tests/fixtures/products-test.csv"
	blenderScript := "tests/mocks/blender-mock.sh"

	return map[string]string{
		"INPUT_CSV":           testCSV,
		"BLENDER_MOCK_SCRIPT": blenderScript,
	}
}

// verifyBentoSuccess checks that bento execution succeeded
func verifyBentoSuccess(t *testing.T, output string) {
	t.Helper()
	assert.Contains(t, output, "Delicious! Bento executed successfully", "Should show success message")
}

// verifyAllProductsProcessed checks that all expected products appear in output
func verifyAllProductsProcessed(t *testing.T, output string, skus []string) {
	t.Helper()
	for _, sku := range skus {
		assert.Contains(t, output, sku, "Should process product %s", sku)
	}
}

// verifyProductOutputs validates folder structure and files for all products
func verifyProductOutputs(t *testing.T, projectRoot string, skus []string) {
	t.Helper()
	for _, sku := range skus {
		verifyProductOutput(t, filepath.Join(projectRoot, "products", sku))
	}
}

// verifyProductOutput validates a single product's folder and files
func verifyProductOutput(t *testing.T, productDir string) {
	t.Helper()

	VerifyFileExists(t, productDir)
	verifyProductDirectory(t, productDir)
	verifyOverlayDownloaded(t, productDir)
	verifyWebPFilesCreated(t, productDir)
	verifyPNGsCleanedUp(t, productDir)
}

// verifyProductDirectory checks product folder is a directory
func verifyProductDirectory(t *testing.T, productDir string) {
	t.Helper()
	info, err := os.Stat(productDir)
	require.NoError(t, err, "Product directory should exist: %s", productDir)
	assert.True(t, info.IsDir(), "Should be a directory: %s", productDir)
}

// verifyOverlayDownloaded checks overlay.png exists and has content
func verifyOverlayDownloaded(t *testing.T, productDir string) {
	t.Helper()
	overlayPath := filepath.Join(productDir, "overlay.png")
	VerifyFileExists(t, overlayPath)

	overlayInfo, err := os.Stat(overlayPath)
	require.NoError(t, err, "Overlay should exist: %s", overlayPath)
	assert.Greater(t, overlayInfo.Size(), int64(0), "Overlay should have content")
}

// verifyWebPFilesCreated checks all 8 WebP files exist with reasonable sizes
func verifyWebPFilesCreated(t *testing.T, productDir string) {
	t.Helper()
	VerifyFileCount(t, productDir, "*.webp", 8)

	for i := 1; i <= 8; i++ {
		verifyWebPFile(t, productDir, i)
	}
}

// verifyWebPFile checks a single WebP file is valid
func verifyWebPFile(t *testing.T, productDir string, index int) {
	t.Helper()
	matches, _ := filepath.Glob(filepath.Join(productDir, "render-*.webp"))
	if index <= len(matches) {
		webpPath := matches[index-1]
		webpInfo, err := os.Stat(webpPath)
		require.NoError(t, err, "WebP file should exist: %s", webpPath)
		assert.Greater(t, webpInfo.Size(), int64(1000), "WebP should have content")
		assert.Less(t, webpInfo.Size(), int64(1*1024*1024), "WebP should be compressed")
	}
}

// verifyPNGsCleanedUp checks original PNGs were deleted (only overlay.png remains)
func verifyPNGsCleanedUp(t *testing.T, productDir string) {
	t.Helper()
	VerifyFileCount(t, productDir, "*.png", 1)
}

// verifyStreamingOutput checks Blender progress appeared in output
func verifyStreamingOutput(t *testing.T, output string) {
	t.Helper()
	assert.Contains(t, output, "Rendering 1/8", "Should show Blender progress for angle 1")
	assert.Contains(t, output, "Rendering 8/8", "Should show Blender progress for angle 8")
}

// TODO: Add error handling tests in future phase
// These tests require http-request neta to fail on 4xx/5xx responses
// Currently it just returns the response without treating it as an error
// See Phase 8.8 or 9.x for proper error handling implementation
