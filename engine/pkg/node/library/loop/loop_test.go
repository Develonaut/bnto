package loop_test

import (
	"context"
	"strings"
	"testing"

	"github.com/Develonaut/bento/pkg/node/library/loop"
)

// TestLoop_ForEach tests forEach mode - iterating over array/slice.
func TestLoop_ForEach(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	items := []interface{}{
		map[string]interface{}{"id": "item-1", "label": "Alpha"},
		map[string]interface{}{"id": "item-2", "label": "Beta"},
		map[string]interface{}{"id": "item-3", "label": "Gamma"},
	}

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": items,
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	iterations, ok := output["iterations"].(int)
	if !ok {
		t.Fatal("Expected iterations to be int")
	}

	if iterations != 3 {
		t.Errorf("iterations = %d, want 3", iterations)
	}

	// Verify results array exists
	results, ok := output["results"].([]interface{})
	if !ok {
		t.Fatal("Expected results to be []interface{}")
	}

	if len(results) != 3 {
		t.Errorf("len(results) = %d, want 3", len(results))
	}
}

// TestLoop_Times tests times mode - repeat N times
func TestLoop_Times(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "times",
		"count": 5,
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	iterations := output["iterations"].(int)

	if iterations != 5 {
		t.Errorf("iterations = %d, want 5", iterations)
	}
}

// TestLoop_While tests while mode - conditional loop
func TestLoop_While(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	// Create a counter in context that will be incremented
	params := map[string]interface{}{
		"mode":      "while",
		"condition": "counter < 3",
		"_context": map[string]interface{}{
			"counter": 0,
		},
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	iterations := output["iterations"].(int)

	// Should loop until counter >= 3
	if iterations < 1 {
		t.Errorf("iterations = %d, want >= 1", iterations)
	}
}

// TestLoop_ContextPassing tests that each iteration gets previous result
func TestLoop_ContextPassing(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	items := []interface{}{
		map[string]interface{}{"value": 1},
		map[string]interface{}{"value": 2},
		map[string]interface{}{"value": 3},
	}

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": items,
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	results := output["results"].([]interface{})

	// Each result should contain the item data
	if len(results) != 3 {
		t.Errorf("len(results) = %d, want 3", len(results))
	}
}

// TestLoop_BreakCondition tests break conditions
func TestLoop_BreakCondition(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	items := []interface{}{
		map[string]interface{}{"value": 1},
		map[string]interface{}{"value": 2},
		map[string]interface{}{"value": 3},
		map[string]interface{}{"value": 4},
		map[string]interface{}{"value": 5},
	}

	params := map[string]interface{}{
		"mode":           "forEach",
		"items":          items,
		"breakCondition": "item.value >= 3",
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	iterations := output["iterations"].(int)

	// Should break when item.value >= 3 (after processing items 1, 2, and 3)
	if iterations != 3 {
		t.Errorf("iterations = %d, want 3 (should break at item.value=3)", iterations)
	}
}

// TestLoop_EmptyItems tests empty array handling
func TestLoop_EmptyItems(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": []interface{}{},
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	iterations := output["iterations"].(int)

	if iterations != 0 {
		t.Errorf("iterations = %d, want 0", iterations)
	}
}

// TestLoop_InvalidMode tests error handling for invalid mode
func TestLoop_InvalidMode(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode": "invalid",
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid mode, got nil")
	}
}

// TestLoop_MissingMode tests error when mode is not a string.
func TestLoop_MissingMode(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing mode, got nil")
	}
}

// TestLoop_TimesZeroCount tests times mode with count=0.
func TestLoop_TimesZeroCount(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "times",
		"count": 0,
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	iterations := output["iterations"].(int)

	if iterations != 0 {
		t.Errorf("iterations = %d, want 0", iterations)
	}
}

// TestLoop_TimesNegativeCount tests times mode with negative count.
func TestLoop_TimesNegativeCount(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "times",
		"count": -1,
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for negative count, got nil")
	}
}

// TestLoop_TimesFloat64Count tests times mode with float64 count (JSON numbers).
func TestLoop_TimesFloat64Count(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "times",
		"count": float64(3),
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	iterations := output["iterations"].(int)

	if iterations != 3 {
		t.Errorf("iterations = %d, want 3", iterations)
	}
}

// TestLoop_TimesNonNumericCount tests times mode with non-numeric count.
func TestLoop_TimesNonNumericCount(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "times",
		"count": "not-a-number",
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for non-numeric count, got nil")
	}
}

// TestLoop_TimesContextCancellation tests times mode respects context cancellation.
func TestLoop_TimesContextCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "times",
		"count": 1000,
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected context cancellation error, got nil")
	}
}

// TestLoop_ForEachContextCancellation tests forEach mode respects context cancellation.
func TestLoop_ForEachContextCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": []interface{}{"a", "b", "c"},
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected context cancellation error, got nil")
	}
}

// TestLoop_ForEachNonIterable tests forEach with non-array items.
func TestLoop_ForEachNonIterable(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": "not-an-array",
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for non-array items, got nil")
	}
}

// TestLoop_WhileNoContext tests while mode with no _context.
func TestLoop_WhileNoContext(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":      "while",
		"condition": "false",
	}

	result, err := loopNode.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	iterations := output["iterations"].(int)

	if iterations != 0 {
		t.Errorf("iterations = %d, want 0", iterations)
	}
}

// TestLoop_WhileMissingCondition tests while mode with missing condition.
func TestLoop_WhileMissingCondition(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode": "while",
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing condition, got nil")
	}
}

// TestLoop_WhileNonBooleanCondition tests while mode with condition that returns non-bool.
func TestLoop_WhileNonBooleanCondition(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":      "while",
		"condition": "42",
		"_context":  map[string]interface{}{},
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for non-boolean condition result, got nil")
	}

	if !strings.Contains(err.Error(), "boolean") {
		t.Errorf("Expected 'boolean' in error, got: %v", err)
	}
}

// TestLoop_WhileContextCancellation tests while mode respects context cancellation.
func TestLoop_WhileContextCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":      "while",
		"condition": "true",
		"_context":  map[string]interface{}{},
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected context cancellation error, got nil")
	}
}

// TestLoop_WhileInvalidConditionExpression tests while with invalid expression.
func TestLoop_WhileInvalidConditionExpression(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":      "while",
		"condition": "invalid syntax {{",
		"_context":  map[string]interface{}{},
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid condition expression, got nil")
	}
}

// TestLoop_BreakConditionError tests forEach with invalid break condition expression.
func TestLoop_BreakConditionError(t *testing.T) {
	ctx := context.Background()

	loopNode := loop.New()

	params := map[string]interface{}{
		"mode":           "forEach",
		"items":          []interface{}{"a"},
		"breakCondition": "invalid {{",
	}

	_, err := loopNode.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid break condition, got nil")
	}
}
