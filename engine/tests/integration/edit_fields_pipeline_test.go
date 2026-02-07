package integration

import (
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/engine"
)

func TestEditFieldsPipeline(t *testing.T) {
	restore := chdirToEngineRoot(t)
	defer restore()

	reg := createRegistry()
	result := executeFixture(t, "tests/fixtures/workflows/edit-fields-pipeline.bnto.json", reg, 10*time.Second)

	if result.Status != engine.StatusSuccess {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	// Verify setFields output
	setFieldsOut, ok := result.NodeOutputs["setFields"].(map[string]interface{})
	if !ok {
		t.Fatal("setFields output missing or wrong type")
	}

	if name, _ := setFieldsOut["productName"].(string); name != "Bnto Widget" {
		t.Errorf("productName = %q, want %q", name, "Bnto Widget")
	}
	if category, _ := setFieldsOut["category"].(string); category != "tools" {
		t.Errorf("category = %q, want %q", category, "tools")
	}
	if inStock, _ := setFieldsOut["inStock"].(bool); !inStock {
		t.Error("inStock should be true")
	}

	// Verify transform output
	transformOut, ok := result.NodeOutputs["transformFields"].(map[string]interface{})
	if !ok {
		t.Fatal("transformFields output missing or wrong type")
	}

	mapped, ok := transformOut["mapped"].(map[string]interface{})
	if !ok {
		t.Fatal("transformFields mapped output missing")
	}

	if label, _ := mapped["label"].(string); label != "Bnto Widget - tools" {
		t.Errorf("label = %q, want %q", label, "Bnto Widget - tools")
	}
	if hasPrice, _ := mapped["hasPrice"].(bool); !hasPrice {
		t.Error("hasPrice should be true")
	}
}
