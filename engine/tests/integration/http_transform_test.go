package integration

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/engine"
)

func TestHTTPTransform(t *testing.T) {
	restore := chdirToEngineRoot(t)
	defer restore()

	// Start test server that returns JSON with items
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/items" {
			http.NotFound(w, r)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"items": []map[string]string{
				{"id": "1", "name": "Alpha"},
				{"id": "2", "name": "Beta"},
				{"id": "3", "name": "Gamma"},
			},
		})
	}))
	defer server.Close()

	t.Setenv("TEST_API_URL", server.URL)

	reg := createRegistry()
	result := executeFixture(t, "tests/fixtures/workflows/http-transform.bnto.json", reg, 10*time.Second)

	if result.Status != engine.StatusSuccess {
		t.Fatalf("Expected success, got %s", result.Status)
	}

	// Verify HTTP response was captured
	fetchOut, ok := result.NodeOutputs["fetchData"].(map[string]interface{})
	if !ok {
		t.Fatal("fetchData output missing or wrong type")
	}

	statusCode, _ := fetchOut["statusCode"].(int)
	if statusCode != 200 {
		t.Errorf("statusCode = %d, want 200", statusCode)
	}

	// Verify transform produced correct derived values
	transformOut, ok := result.NodeOutputs["transformResponse"].(map[string]interface{})
	if !ok {
		t.Fatal("transformResponse output missing or wrong type")
	}

	mapped, ok := transformOut["mapped"].(map[string]interface{})
	if !ok {
		t.Fatal("transformResponse mapped output missing")
	}

	// expr-lang returns int for len()
	itemCount := toInt(mapped["itemCount"])
	if itemCount != 3 {
		t.Errorf("itemCount = %v, want 3", mapped["itemCount"])
	}

	statusOk, _ := mapped["statusOk"].(bool)
	if !statusOk {
		t.Error("statusOk should be true")
	}
}

// toInt converts numeric types to int for comparison.
// expr-lang may return int, int64, or float64 depending on the expression.
func toInt(v interface{}) int {
	switch n := v.(type) {
	case int:
		return n
	case int64:
		return int(n)
	case float64:
		return int(n)
	default:
		return -1
	}
}
