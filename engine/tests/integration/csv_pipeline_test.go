package integration

import (
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/engine"
)

func TestCSVDataPipeline(t *testing.T) {
	restore := chdirToEngineRoot(t)
	defer restore()

	reg := createRegistry()
	result := executeFixture(t, "tests/fixtures/workflows/csv-data-pipeline.bento.json", reg, 10*time.Second)

	if result.Status != engine.StatusSuccess {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	// Verify CSV was read
	csvOut, ok := result.NodeOutputs["readCSV"].(map[string]interface{})
	if !ok {
		t.Fatal("readCSV output missing or wrong type")
	}

	// Spreadsheet returns []map[string]interface{}, not []interface{}
	var rowCount int
	switch r := csvOut["rows"].(type) {
	case []map[string]interface{}:
		rowCount = len(r)
	case []interface{}:
		rowCount = len(r)
	default:
		t.Fatalf("readCSV rows unexpected type: %T", csvOut["rows"])
	}
	if rowCount != 3 {
		t.Fatalf("Expected 3 CSV rows, got %d", rowCount)
	}

	// Verify loop ran 3 iterations
	loopOut, ok := result.NodeOutputs["processLoop"].([]interface{})
	if !ok {
		t.Fatal("processLoop output missing or wrong type (expected []interface{})")
	}
	if len(loopOut) != 3 {
		t.Fatalf("Expected 3 loop iterations, got %d", len(loopOut))
	}

	// Verify first iteration's transform output
	iter0, ok := loopOut[0].(map[string]interface{})
	if !ok {
		t.Fatal("First iteration output missing or wrong type")
	}

	transformOut, ok := iter0["transformRow"].(map[string]interface{})
	if !ok {
		t.Fatal("transformRow output missing from first iteration")
	}

	mapped, ok := transformOut["mapped"].(map[string]interface{})
	if !ok {
		t.Fatal("transformRow mapped output missing")
	}

	displayName, _ := mapped["displayName"].(string)
	if displayName != "Test Item A - Standard" {
		t.Errorf("displayName = %q, want %q", displayName, "Test Item A - Standard")
	}

	hasDesc, _ := mapped["hasDescription"].(bool)
	if !hasDesc {
		t.Error("hasDescription should be true")
	}
}
