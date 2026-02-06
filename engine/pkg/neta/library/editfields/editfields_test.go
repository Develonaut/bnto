package editfields_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bento/pkg/neta/library/editfields"
)

// TestEditFields_SetStaticValues verifies setting static field values.
//
// This is the most basic use case: setting fields to literal values
// without any template processing.
//
// Example workflow usage:
//
//	User wants to set product.name = "Product A" and product.sku = "PROD-001"
func TestEditFields_SetStaticValues(t *testing.T) {
	ctx := context.Background()

	ef := editfields.New()

	params := map[string]interface{}{
		"values": map[string]interface{}{
			"name": "Product A",
			"sku":  "PROD-001",
		},
	}

	result, err := ef.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	if output["name"] != "Product A" {
		t.Errorf("name = %v, want Product A", output["name"])
	}

	if output["sku"] != "PROD-001" {
		t.Errorf("sku = %v, want PROD-001", output["sku"])
	}
}

// TestEditFields_TemplateVariables verifies Go template processing.
//
// This tests the core template engine integration using text/template.
// Templates can reference data from previous neta in the workflow.
//
// Example workflow usage:
//
//	HTTP neta fetches {"product": {"name": "Widget", "id": 123}}
//	Edit-fields neta uses templates to create filename: "product-123.png"
func TestEditFields_TemplateVariables(t *testing.T) {
	ctx := context.Background()

	ef := editfields.New()

	// Context from previous neta (passed by itamae)
	prevContext := map[string]interface{}{
		"product": map[string]interface{}{
			"name": "Widget",
			"id":   123,
		},
	}

	params := map[string]interface{}{
		"values": map[string]interface{}{
			"title":    "{{.product.name}}",
			"filename": "product-{{.product.id}}.png",
		},
		"_context": prevContext,
	}

	result, err := ef.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	if output["title"] != "Widget" {
		t.Errorf("title = %v, want Widget", output["title"])
	}

	if output["filename"] != "product-123.png" {
		t.Errorf("filename = %v, want product-123.png", output["filename"])
	}
}

// TestEditFields_NestedFieldAccess tests template access to nested data.
//
// Templates should be able to access deeply nested structures using
// Go template dot notation.
func TestEditFields_NestedFieldAccess(t *testing.T) {
	ctx := context.Background()

	ef := editfields.New()

	prevContext := map[string]interface{}{
		"api": map[string]interface{}{
			"response": map[string]interface{}{
				"data": map[string]interface{}{
					"items": []interface{}{
						map[string]interface{}{"name": "First Item"},
					},
				},
			},
		},
	}

	params := map[string]interface{}{
		"values": map[string]interface{}{
			"firstItem": "{{index .api.response.data.items 0}}",
		},
		"_context": prevContext,
	}

	result, err := ef.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	// The template should output the map as a string representation
	if output["firstItem"] == nil {
		t.Error("firstItem should not be nil")
	}
}

// TestEditFields_MixedStaticAndTemplate tests combining static and templated values.
//
// Real workflows often mix literal values and computed values from templates.
func TestEditFields_MixedStaticAndTemplate(t *testing.T) {
	ctx := context.Background()

	ef := editfields.New()

	prevContext := map[string]interface{}{
		"user": map[string]interface{}{
			"id": 456,
		},
	}

	params := map[string]interface{}{
		"values": map[string]interface{}{
			"type":   "user",         // Static
			"userId": "{{.user.id}}", // Template
			"status": "active",       // Static
		},
		"_context": prevContext,
	}

	result, err := ef.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	if output["type"] != "user" {
		t.Errorf("type = %v, want user", output["type"])
	}

	if output["userId"] != "456" {
		t.Errorf("userId = %v, want 456", output["userId"])
	}

	if output["status"] != "active" {
		t.Errorf("status = %v, want active", output["status"])
	}
}

// TestEditFields_EmptyValues tests behavior with empty values map.
//
// Should return an empty map without error.
func TestEditFields_EmptyValues(t *testing.T) {
	ctx := context.Background()

	ef := editfields.New()

	params := map[string]interface{}{
		"values": map[string]interface{}{},
	}

	result, err := ef.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})

	if len(output) != 0 {
		t.Errorf("Expected empty map, got %d entries", len(output))
	}
}

// TestEditFields_MissingValues tests error handling when values param is missing.
//
// Should return an error indicating values parameter is required.
func TestEditFields_MissingValues(t *testing.T) {
	ctx := context.Background()

	ef := editfields.New()

	params := map[string]interface{}{}

	_, err := ef.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error when values parameter is missing")
	}
}

// TestEditFields_InvalidTemplateString tests error handling for invalid templates.
//
// Malformed template syntax should return a descriptive error.
func TestEditFields_InvalidTemplateString(t *testing.T) {
	ctx := context.Background()

	ef := editfields.New()

	params := map[string]interface{}{
		"values": map[string]interface{}{
			"bad": "{{.unclosed",
		},
		"_context": map[string]interface{}{},
	}

	_, err := ef.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for invalid template syntax")
	}
}
