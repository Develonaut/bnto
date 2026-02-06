package integration

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// TestFolderSetup_CreateProductFolders tests the folder setup bento.
// This validates that the loop neta correctly iterates over CSV rows
// and that the file-system neta creates directories for each product.
func TestFolderSetup_CreateProductFolders(t *testing.T) {
	projectRoot := "../../"

	// Cleanup products/ folder before and after test (at project root)
	defer CleanupTestDir(t, projectRoot+"products")
	CleanupTestDir(t, projectRoot+"products")

	// Execute bento (runs from project root, CSV path is relative to project root)
	bentoPath := "examples/folder-setup.bento.json"
	output, err := RunBento(t, bentoPath, nil)

	// Verify execution succeeded
	require.NoError(t, err, "Folder setup bento should execute successfully\nOutput: %s", string(output))

	// Verify output shows success
	assert.Contains(t, output, "Delicious! Bento executed successfully", "Should show success message")

	// Verify folders were created (relative to project root)
	// RunBento runs from project root, so products/ is created there
	VerifyFileExists(t, projectRoot+"products/Combat Dog (Supplies)")
	VerifyFileExists(t, projectRoot+"products/Combat Dog (Gas Mask)")
	VerifyFileExists(t, projectRoot+"products/Combat Dog (Attack)")

	// Verify they are directories
	info, err := os.Stat(projectRoot + "products/Combat Dog (Supplies)")
	require.NoError(t, err, "products/Combat Dog (Supplies) should exist")
	assert.True(t, info.IsDir(), "products/Combat Dog (Supplies) should be a directory")

	info, err = os.Stat(projectRoot + "products/Combat Dog (Gas Mask)")
	require.NoError(t, err, "products/Combat Dog (Gas Mask) should exist")
	assert.True(t, info.IsDir(), "products/Combat Dog (Gas Mask) should be a directory")

	info, err = os.Stat(projectRoot + "products/Combat Dog (Attack)")
	require.NoError(t, err, "products/Combat Dog (Attack) should exist")
	assert.True(t, info.IsDir(), "products/Combat Dog (Attack) should be a directory")

	t.Log("✓ Successfully created folders for 3 products")
	t.Log("✓ Validated loop neta with forEach mode")
	t.Log("✓ Validated context passing from CSV to loop")
}

// TestFolderSetup_AlreadyExists tests idempotency.
// Running the bento twice should succeed without errors.
func TestFolderSetup_AlreadyExists(t *testing.T) {
	projectRoot := "../../"

	// Cleanup products/ folder before and after test (at project root)
	defer CleanupTestDir(t, projectRoot+"products")
	CleanupTestDir(t, projectRoot+"products")

	// Pre-create one folder to test idempotency
	err := os.MkdirAll(projectRoot+"products/Combat Dog (Supplies)", 0755)
	require.NoError(t, err, "Should pre-create folder")

	// Execute bento (runs from project root)
	bentoPath := "examples/folder-setup.bento.json"
	output, err := RunBento(t, bentoPath, nil)

	// Should still succeed (idempotent)
	require.NoError(t, err, "Should succeed even if folder exists\nOutput: %s", string(output))

	// All folders should exist
	VerifyFileExists(t, projectRoot+"products/Combat Dog (Supplies)")
	VerifyFileExists(t, projectRoot+"products/Combat Dog (Gas Mask)")
	VerifyFileExists(t, projectRoot+"products/Combat Dog (Attack)")

	t.Log("✓ Idempotent: Successfully ran with pre-existing folder")
}
