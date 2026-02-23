package transform_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/transform"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_BasicExpression(t *testing.T) {
	tr := transform.New()

	result, err := tr.Execute(context.Background(), map[string]interface{}{
		"expression": "value * 2 + 5",
		"_context":   map[string]interface{}{"value": 10},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "basic_expression", result)
}

func TestGolden_FieldMapping(t *testing.T) {
	tr := transform.New()

	result, err := tr.Execute(context.Background(), map[string]interface{}{
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
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "field_mapping", result)
}

func TestGolden_StringOps(t *testing.T) {
	tr := transform.New()

	result, err := tr.Execute(context.Background(), map[string]interface{}{
		"expression": "upper(trim(name))",
		"_context":   map[string]interface{}{"name": "  hello world  "},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "string_ops", result)
}

func TestGolden_ArrayMap(t *testing.T) {
	tr := transform.New()

	result, err := tr.Execute(context.Background(), map[string]interface{}{
		"expression": "map(items, #.id)",
		"_context": map[string]interface{}{
			"items": []interface{}{
				map[string]interface{}{"id": "REC-001", "title": "Alpha"},
				map[string]interface{}{"id": "REC-002", "title": "Beta"},
				map[string]interface{}{"id": "REC-003", "title": "Gamma"},
			},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "array_map", result)
}

func TestGolden_NestedObjects(t *testing.T) {
	tr := transform.New()

	result, err := tr.Execute(context.Background(), map[string]interface{}{
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
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "nested_objects", result)
}
