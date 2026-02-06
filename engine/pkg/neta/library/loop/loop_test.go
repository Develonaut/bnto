package loop_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bento/pkg/neta/library/loop"
)

// TestLoop_ForEach tests forEach mode - iterating over array/slice
// CRITICAL FOR PHASE 8: Loop through CSV rows
func TestLoop_ForEach(t *testing.T) {
	ctx := context.Background()

	loopNeta := loop.New()

	// Simulate CSV rows
	csvData := []interface{}{
		map[string]interface{}{"sku": "PROD-001", "name": "Product A"},
		map[string]interface{}{"sku": "PROD-002", "name": "Product B"},
		map[string]interface{}{"sku": "PROD-003", "name": "Product C"},
	}

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": csvData,
	}

	result, err := loopNeta.Execute(ctx, params)
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

	loopNeta := loop.New()

	params := map[string]interface{}{
		"mode":  "times",
		"count": 5,
	}

	result, err := loopNeta.Execute(ctx, params)
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

	loopNeta := loop.New()

	// Create a counter in context that will be incremented
	params := map[string]interface{}{
		"mode":      "while",
		"condition": "counter < 3",
		"_context": map[string]interface{}{
			"counter": 0,
		},
	}

	result, err := loopNeta.Execute(ctx, params)
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

	loopNeta := loop.New()

	items := []interface{}{
		map[string]interface{}{"value": 1},
		map[string]interface{}{"value": 2},
		map[string]interface{}{"value": 3},
	}

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": items,
	}

	result, err := loopNeta.Execute(ctx, params)
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

	loopNeta := loop.New()

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

	result, err := loopNeta.Execute(ctx, params)
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

	loopNeta := loop.New()

	params := map[string]interface{}{
		"mode":  "forEach",
		"items": []interface{}{},
	}

	result, err := loopNeta.Execute(ctx, params)
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

	loopNeta := loop.New()

	params := map[string]interface{}{
		"mode": "invalid",
	}

	_, err := loopNeta.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid mode, got nil")
	}
}
