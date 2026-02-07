package integration

import (
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/engine"
)

func TestImageComposite(t *testing.T) {
	restore := chdirToEngineRoot(t)
	defer restore()

	tmpDir := t.TempDir()
	t.Setenv("COMPOSITE_OUTPUT", tmpDir)

	reg := createRegistry()
	result := executeFixture(t, "tests/fixtures/workflows/image-composite.bnto.json", reg, 30*time.Second)

	if result.Status != engine.StatusSuccess {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	// Verify composited output exists
	outputPath := filepath.Join(tmpDir, "Product_Render.png")
	info, err := os.Stat(outputPath)
	if err != nil {
		t.Fatalf("Composited image not found at %s: %v", outputPath, err)
	}

	if info.Size() == 0 {
		t.Error("Composited image is empty")
	}

	t.Logf("Composited image size: %d bytes", info.Size())
}
