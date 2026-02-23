package editfields_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/editfields"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_SetStaticValues(t *testing.T) {
	ef := editfields.New()

	result, err := ef.Execute(context.Background(), map[string]interface{}{
		"values": map[string]interface{}{
			"name":   "Item A",
			"label":  "REC-001",
			"active": true,
			"count":  42,
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "set_static_values", result)
}

func TestGolden_TemplateValues(t *testing.T) {
	ef := editfields.New()

	result, err := ef.Execute(context.Background(), map[string]interface{}{
		"values": map[string]interface{}{
			"title":    "{{.record.name}}",
			"filename": "output-{{.record.id}}.png",
			"static":   "unchanged",
		},
		"_context": map[string]interface{}{
			"record": map[string]interface{}{
				"name": "Alpha",
				"id":   123,
			},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "template_values", result)
}

func TestGolden_EmptyValues(t *testing.T) {
	ef := editfields.New()

	result, err := ef.Execute(context.Background(), map[string]interface{}{
		"values": map[string]interface{}{},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "empty_values", result)
}

func TestGolden_MixedStaticAndTemplate(t *testing.T) {
	ef := editfields.New()

	result, err := ef.Execute(context.Background(), map[string]interface{}{
		"values": map[string]interface{}{
			"type":   "user",
			"userId": "{{.user.id}}",
			"status": "active",
		},
		"_context": map[string]interface{}{
			"user": map[string]interface{}{"id": 456},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "mixed_static_template", result)
}
