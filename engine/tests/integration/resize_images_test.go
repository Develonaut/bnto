package integration

import (
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/engine"
)

func TestResizeImages(t *testing.T) {
	restore := chdirToEngineRoot(t)
	defer restore()

	tmpDir := t.TempDir()
	t.Setenv("RESIZE_OUTPUT", tmpDir)

	reg := createRegistry()
	result := executeFixture(t, "tests/fixtures/workflows/resize-images.bnto.json", reg, 30*time.Second)

	if result.Status != engine.StatusSuccess {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	// Verify list-images found the fixture image
	listOut, ok := result.NodeOutputs["list-images"].(map[string]interface{})
	if !ok {
		t.Fatal("list-images output missing or wrong type")
	}

	files, ok := listOut["files"].([]interface{})
	if !ok {
		t.Fatal("list-images files missing or wrong type")
	}
	if len(files) == 0 {
		t.Fatal("list-images found no files")
	}

	// Verify resized output exists and is smaller than original
	outputPath := filepath.Join(tmpDir, "Product_Render.png")
	info, err := os.Stat(outputPath)
	if err != nil {
		t.Fatalf("Resized image not found at %s: %v", outputPath, err)
	}

	if info.Size() == 0 {
		t.Error("Resized image is empty")
	}

	// Original is ~440KB; resized to width=200 should be significantly smaller
	if info.Size() >= 440000 {
		t.Errorf("Resized image (%d bytes) should be smaller than original (~440KB)", info.Size())
	}
}
