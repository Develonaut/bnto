package transform_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bento/pkg/neta/library/transform"
)

// TestTransform_BasicExpression tests simple expr execution
func TestTransform_BasicExpression(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "value * 2",
		"_context": map[string]interface{}{
			"value": 10,
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	resultVal, ok := output["result"].(int)
	if !ok {
		t.Fatal("Expected result to be int")
	}

	if resultVal != 20 {
		t.Errorf("result = %d, want 20", resultVal)
	}
}

// TestTransform_FieldMapping tests mapping fields
func TestTransform_FieldMapping(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"mappings": map[string]interface{}{
			"fullName": "firstName + ' ' + lastName",
			"age":      "years",
			"isAdult":  "years >= 18",
		},
		"_context": map[string]interface{}{
			"firstName": "John",
			"lastName":  "Doe",
			"years":     25,
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	mapped, ok := output["mapped"].(map[string]interface{})
	if !ok {
		t.Fatal("Expected mapped to be map[string]interface{}")
	}

	if mapped["fullName"] != "John Doe" {
		t.Errorf("fullName = %v, want 'John Doe'", mapped["fullName"])
	}

	if mapped["age"] != 25 {
		t.Errorf("age = %v, want 25", mapped["age"])
	}

	if mapped["isAdult"] != true {
		t.Errorf("isAdult = %v, want true", mapped["isAdult"])
	}
}

// TestTransform_ConditionalTransformations tests conditional logic
func TestTransform_ConditionalTransformations(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "price > 100 ? price * 0.9 : price",
		"_context": map[string]interface{}{
			"price": 150.0,
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	resultVal := output["result"].(float64)

	if resultVal != 135.0 {
		t.Errorf("result = %v, want 135.0", resultVal)
	}
}

// TestTransform_ArrayMap tests mapping field values from array
func TestTransform_ArrayMap(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "map(items, #.id)",
		"_context": map[string]interface{}{
			"items": []interface{}{
				map[string]interface{}{"id": "PROD-001", "title": "Product A"},
				map[string]interface{}{"id": "PROD-002", "title": "Product B"},
			},
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	resultVal, ok := output["result"].([]interface{})
	if !ok {
		t.Fatal("Expected result to be []interface{}")
	}

	if len(resultVal) != 2 {
		t.Errorf("len(result) = %d, want 2", len(resultVal))
	}

	if resultVal[0] != "PROD-001" {
		t.Errorf("result[0] = %v, want PROD-001", resultVal[0])
	}

	if resultVal[1] != "PROD-002" {
		t.Errorf("result[1] = %v, want PROD-002", resultVal[1])
	}
}

// TestTransform_ArrayFilter tests filtering arrays
func TestTransform_ArrayFilter(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "filter(items, # > 10)",
		"_context": map[string]interface{}{
			"items": []interface{}{5, 15, 8, 20, 12},
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	resultVal := output["result"].([]interface{})

	if len(resultVal) != 3 {
		t.Errorf("len(result) = %d, want 3", len(resultVal))
	}

	// Should contain 15, 20, 12
	expected := []int{15, 20, 12}
	for i, val := range resultVal {
		if val != expected[i] {
			t.Errorf("result[%d] = %v, want %v", i, val, expected[i])
		}
	}
}

// TestTransform_ArrayReduce tests reducing arrays
func TestTransform_ArrayReduce(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "sum(items)",
		"_context": map[string]interface{}{
			"items": []interface{}{1, 2, 3, 4, 5},
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	resultVal := output["result"].(int)

	if resultVal != 15 {
		t.Errorf("result = %d, want 15", resultVal)
	}
}

// TestTransform_StringOperations tests string manipulation
func TestTransform_StringOperations(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "upper(trim(name))",
		"_context": map[string]interface{}{
			"name": "  hello world  ",
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	resultVal := output["result"].(string)

	if resultVal != "HELLO WORLD" {
		t.Errorf("result = %v, want 'HELLO WORLD'", resultVal)
	}
}

// TestTransform_InvalidExpression tests error handling
func TestTransform_InvalidExpression(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "invalid syntax {{",
	}

	_, err := tr.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid expression, got nil")
	}
}

// TestTransform_MissingExpression tests error handling for missing expression
func TestTransform_MissingExpression(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		// Missing expression parameter
	}

	_, err := tr.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing expression, got nil")
	}
}

// TestTransform_NestedObjects tests working with nested data
func TestTransform_NestedObjects(t *testing.T) {
	ctx := context.Background()

	tr := transform.New()

	params := map[string]interface{}{
		"expression": "user.address.city + ', ' + user.address.country",
		"_context": map[string]interface{}{
			"user": map[string]interface{}{
				"name": "John",
				"address": map[string]interface{}{
					"city":    "New York",
					"country": "USA",
				},
			},
		},
	}

	result, err := tr.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	resultVal := output["result"].(string)

	if resultVal != "New York, USA" {
		t.Errorf("result = %v, want 'New York, USA'", resultVal)
	}
}
