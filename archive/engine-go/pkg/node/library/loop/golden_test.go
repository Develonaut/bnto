package loop_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/loop"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_ForEachArray(t *testing.T) {
	loopNode := loop.New()

	result, err := loopNode.Execute(context.Background(), map[string]interface{}{
		"mode": "forEach",
		"items": []interface{}{
			map[string]interface{}{"id": "item-1", "label": "Alpha"},
			map[string]interface{}{"id": "item-2", "label": "Beta"},
			map[string]interface{}{"id": "item-3", "label": "Gamma"},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "foreach_array", result)
}

func TestGolden_ForEachEmpty(t *testing.T) {
	loopNode := loop.New()

	result, err := loopNode.Execute(context.Background(), map[string]interface{}{
		"mode":  "forEach",
		"items": []interface{}{},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "foreach_empty", result)
}

func TestGolden_Times(t *testing.T) {
	loopNode := loop.New()

	result, err := loopNode.Execute(context.Background(), map[string]interface{}{
		"mode":  "times",
		"count": 4,
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "times", result)
}

func TestGolden_ForEachBreak(t *testing.T) {
	loopNode := loop.New()

	result, err := loopNode.Execute(context.Background(), map[string]interface{}{
		"mode": "forEach",
		"items": []interface{}{
			map[string]interface{}{"value": 1},
			map[string]interface{}{"value": 2},
			map[string]interface{}{"value": 3},
			map[string]interface{}{"value": 4},
			map[string]interface{}{"value": 5},
		},
		"breakCondition": "item.value >= 3",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "foreach_break", result)
}
