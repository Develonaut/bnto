package group_test

import (
	"context"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/group"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_SequentialGroup(t *testing.T) {
	grp := group.New()

	result, err := grp.Execute(context.Background(), map[string]interface{}{
		"mode": "sequential",
		"nodes": []interface{}{
			map[string]interface{}{"id": "node-1", "type": "edit-fields"},
			map[string]interface{}{"id": "node-2", "type": "transform"},
			map[string]interface{}{"id": "node-3", "type": "edit-fields"},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "sequential_group", result)
}

func TestGolden_ParallelGroup(t *testing.T) {
	grp := group.New()

	result, err := grp.Execute(context.Background(), map[string]interface{}{
		"mode": "parallel",
		"nodes": []interface{}{
			map[string]interface{}{"id": "fetch-1", "type": "http-request"},
			map[string]interface{}{"id": "fetch-2", "type": "http-request"},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "parallel_group", result)
}

func TestGolden_EmptyGroup(t *testing.T) {
	grp := group.New()

	result, err := grp.Execute(context.Background(), map[string]interface{}{
		"mode":  "sequential",
		"nodes": []interface{}{},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "empty_group", result)
}

func TestGolden_NestedGroups(t *testing.T) {
	grp := group.New()

	result, err := grp.Execute(context.Background(), map[string]interface{}{
		"mode": "sequential",
		"nodes": []interface{}{
			map[string]interface{}{"id": "step-1", "type": "edit-fields"},
			map[string]interface{}{
				"id":   "inner-group",
				"type": "group",
				"parameters": map[string]interface{}{
					"mode": "parallel",
					"nodes": []interface{}{
						map[string]interface{}{"id": "inner-1", "type": "http-request"},
						map[string]interface{}{"id": "inner-2", "type": "http-request"},
					},
				},
			},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "nested_groups", result)
}
