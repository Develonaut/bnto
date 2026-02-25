package integration

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/Develonaut/bnto/pkg/api"
	"github.com/Develonaut/bnto/pkg/menu"
)

// TestAllNodeTypesHaveGoldenTests verifies that every registered node type
// has a corresponding golden_test.go file with snapshot assertions.
// This prevents new node types from shipping without test coverage.
func TestAllNodeTypesHaveGoldenTests(t *testing.T) {
	reg := api.DefaultRegistry()
	nodeTypes := reg.List()

	root := projectRoot(t)

	// Map node type names to their package directories
	typeToDir := map[string]string{
		"edit-fields":   "editfields",
		"file-system":   "filesystem",
		"group":         "group",
		"http-request":  "http",
		"image":         "image",
		"loop":          "loop",
		"parallel":      "parallel",
		"shell-command": "shellcommand",
		"spreadsheet":   "spreadsheet",
		"transform":     "transform",
	}

	for _, nodeType := range nodeTypes {
		dirName, ok := typeToDir[nodeType]
		if !ok {
			t.Errorf("Node type %q has no directory mapping — add it to typeToDir in enforcement_test.go", nodeType)
			continue
		}

		goldenTestFile := filepath.Join(root, "pkg", "node", "library", dirName, "golden_test.go")
		if _, err := os.Stat(goldenTestFile); os.IsNotExist(err) {
			t.Errorf("Node type %q has no golden tests — create %s", nodeType, goldenTestFile)
		}
	}
}

// TestAllRecipesHaveAcceptanceTests verifies that every recipe in the menu
// has a corresponding acceptance test. This prevents new recipes from shipping
// without end-to-end test coverage.
func TestAllRecipesHaveAcceptanceTests(t *testing.T) {
	recipes, err := menu.All()
	if err != nil {
		t.Fatalf("Failed to load recipes: %v", err)
	}

	root := projectRoot(t)

	// Check that each recipe has a workflow fixture
	for _, r := range recipes {
		fixturePath := filepath.Join(root, "tests", "fixtures", "workflows", r.Slug+".bnto.json")
		if _, err := os.Stat(fixturePath); os.IsNotExist(err) {
			t.Errorf("Recipe %q has no workflow fixture — create %s", r.Slug, fixturePath)
		}
	}
}
