package integration

import (
	"context"
	"encoding/json"
	"os"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/engine"
	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/registry"
)

// loadFixture reads and unmarshals a .bento.json fixture file.
func loadFixture(t *testing.T, path string) *node.Definition {
	t.Helper()

	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Failed to read fixture %s: %v", path, err)
	}

	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		t.Fatalf("Failed to parse fixture %s: %v", path, err)
	}

	return &def
}

// executeFixture loads a fixture and executes it with the given registry.
func executeFixture(t *testing.T, path string, reg *registry.Registry, timeout time.Duration) *engine.Result {
	t.Helper()

	def := loadFixture(t, path)
	eng := engine.New(reg, nil)

	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	result, err := eng.Serve(ctx, def)
	if err != nil {
		t.Fatalf("Fixture execution failed for %s: %v", path, err)
	}

	return result
}

// chdirToEngineRoot changes to the engine root directory and returns a cleanup function.
// Many fixtures use relative paths (e.g., "tests/fixtures/...") that resolve from engine root.
func chdirToEngineRoot(t *testing.T) func() {
	t.Helper()

	originalWd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	if err := os.Chdir("../../"); err != nil {
		t.Fatalf("Failed to change to engine root: %v", err)
	}

	return func() { _ = os.Chdir(originalWd) }
}
