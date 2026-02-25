package integration

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// TestWorkflowAutomation_EndToEnd validates the complete workflow automation.
// Tests CSV reading → folder creation → copy overlays → external tool execution → WebP optimization
func TestWorkflowAutomation_EndToEnd(t *testing.T) {
	t.Skip("Skipping test: tests/fixtures/workflows/workflow-automation.bnto.json does not exist yet")

	projectRoot := "../../"
	defer CleanupTestDir(t, projectRoot+"output")
	CleanupTestDir(t, projectRoot+"output")

	envVars := setupTestEnvironment(t, projectRoot)
	output, err := RunBnto(t, "tests/fixtures/workflows/workflow-automation.bnto.json", envVars)

	require.NoError(t, err, "Workflow automation should complete successfully\nOutput: %s", output)
	verifyBntoSuccess(t, output)
	verifyAllItemsProcessed(t, output, []string{"Test Item A", "Test Item B", "Test Item C"})
	verifyItemOutputs(t, projectRoot, []string{"Test Item A", "Test Item B", "Test Item C"})
	verifyStreamingOutput(t, output)
}

// setupTestEnvironment creates environment variables for the test
func setupTestEnvironment(t *testing.T, projectRoot string) map[string]string {
	t.Helper()

	testCSV := "tests/fixtures/test-data.csv"
	mockScript := "tests/mocks/external-tool-mock.sh"

	return map[string]string{
		"INPUT_CSV":        testCSV,
		"MOCK_TOOL_SCRIPT": mockScript,
	}
}

// verifyBntoSuccess checks that bnto execution succeeded
func verifyBntoSuccess(t *testing.T, output string) {
	t.Helper()
	assert.Contains(t, output, "Delicious! Bnto executed successfully", "Should show success message")
}

// verifyAllItemsProcessed checks that all expected items appear in output
func verifyAllItemsProcessed(t *testing.T, output string, items []string) {
	t.Helper()
	for _, item := range items {
		assert.Contains(t, output, item, "Should process item %s", item)
	}
}

// verifyItemOutputs validates folder structure and files for all items
func verifyItemOutputs(t *testing.T, projectRoot string, items []string) {
	t.Helper()
	for _, item := range items {
		verifyItemOutput(t, filepath.Join(projectRoot, "output", item))
	}
}

// verifyItemOutput validates a single item's folder and files
func verifyItemOutput(t *testing.T, itemDir string) {
	t.Helper()

	VerifyFileExists(t, itemDir)
	verifyItemDirectory(t, itemDir)
	verifyOverlayDownloaded(t, itemDir)
	verifyWebPFilesCreated(t, itemDir)
	verifyPNGsCleanedUp(t, itemDir)
}

// verifyItemDirectory checks item folder is a directory
func verifyItemDirectory(t *testing.T, itemDir string) {
	t.Helper()
	info, err := os.Stat(itemDir)
	require.NoError(t, err, "Item directory should exist: %s", itemDir)
	assert.True(t, info.IsDir(), "Should be a directory: %s", itemDir)
}

// verifyOverlayDownloaded checks overlay.png exists and has content
func verifyOverlayDownloaded(t *testing.T, itemDir string) {
	t.Helper()
	overlayPath := filepath.Join(itemDir, "overlay.png")
	VerifyFileExists(t, overlayPath)

	overlayInfo, err := os.Stat(overlayPath)
	require.NoError(t, err, "Overlay should exist: %s", overlayPath)
	assert.Greater(t, overlayInfo.Size(), int64(0), "Overlay should have content")
}

// verifyWebPFilesCreated checks all 8 WebP files exist with reasonable sizes
func verifyWebPFilesCreated(t *testing.T, itemDir string) {
	t.Helper()
	VerifyFileCount(t, itemDir, "*.webp", 8)

	for i := 1; i <= 8; i++ {
		verifyWebPFile(t, itemDir, i)
	}
}

// verifyWebPFile checks a single WebP file is valid
func verifyWebPFile(t *testing.T, itemDir string, index int) {
	t.Helper()
	matches, _ := filepath.Glob(filepath.Join(itemDir, "output-*.webp"))
	if index <= len(matches) {
		webpPath := matches[index-1]
		webpInfo, err := os.Stat(webpPath)
		require.NoError(t, err, "WebP file should exist: %s", webpPath)
		assert.Greater(t, webpInfo.Size(), int64(1000), "WebP should have content")
		assert.Less(t, webpInfo.Size(), int64(1*1024*1024), "WebP should be compressed")
	}
}

// verifyPNGsCleanedUp checks original PNGs were deleted (only overlay.png remains)
func verifyPNGsCleanedUp(t *testing.T, itemDir string) {
	t.Helper()
	VerifyFileCount(t, itemDir, "*.png", 1)
}

// verifyStreamingOutput checks external tool progress appeared in output
func verifyStreamingOutput(t *testing.T, output string) {
	t.Helper()
	assert.Contains(t, output, "Processing 1/8", "Should show external tool progress for step 1")
	assert.Contains(t, output, "Processing 8/8", "Should show external tool progress for step 8")
}
