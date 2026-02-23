package engine

import (
	"testing"

	"github.com/Develonaut/bnto/pkg/testgolden"
)

// TestGolden_BreadcrumbEmpty verifies breadcrumb for a fresh context.
func TestGolden_BreadcrumbEmpty(t *testing.T) {
	ctx := &executionContext{
		nodeData: make(map[string]any),
		path:     []string{},
	}

	testgolden.AssertGolden(t, "breadcrumb_empty", map[string]any{
		"breadcrumb": ctx.getBreadcrumb(),
		"depth":      ctx.depth,
	})
}

// TestGolden_BreadcrumbSingleNode verifies breadcrumb after one withNode call.
func TestGolden_BreadcrumbSingleNode(t *testing.T) {
	ctx := &executionContext{
		nodeData: make(map[string]any),
		path:     []string{},
	}

	child := ctx.withNode("Root")

	testgolden.AssertGolden(t, "breadcrumb_single_node", map[string]any{
		"breadcrumb": child.getBreadcrumb(),
		"depth":      child.depth,
	})
}

// TestGolden_BreadcrumbNested verifies breadcrumb builds correctly through
// multiple levels of nesting.
func TestGolden_BreadcrumbNested(t *testing.T) {
	ctx := &executionContext{
		nodeData: make(map[string]any),
		path:     []string{},
	}

	level1 := ctx.withNode("Workflow")
	level2 := level1.withNode("BatchGroup")
	level3 := level2.withNode("CompressNode")

	testgolden.AssertGolden(t, "breadcrumb_nested", map[string]any{
		"level0": map[string]any{
			"breadcrumb": ctx.getBreadcrumb(),
			"depth":      ctx.depth,
		},
		"level1": map[string]any{
			"breadcrumb": level1.getBreadcrumb(),
			"depth":      level1.depth,
		},
		"level2": map[string]any{
			"breadcrumb": level2.getBreadcrumb(),
			"depth":      level2.depth,
		},
		"level3": map[string]any{
			"breadcrumb": level3.getBreadcrumb(),
			"depth":      level3.depth,
		},
	})
}

// TestGolden_ContextWithNodeIsolation verifies that withNode creates
// an isolated copy (parent path is not mutated).
func TestGolden_ContextWithNodeIsolation(t *testing.T) {
	parent := &executionContext{
		nodeData: make(map[string]any),
		path:     []string{},
	}

	parent.set("key1", "value1")

	child := parent.withNode("ChildNode")
	child.set("key2", "value2")

	// Parent should not see child's data
	_, parentHasKey2 := parent.nodeData["key2"]

	// Child should see parent's data (shallow copy)
	_, childHasKey1 := child.nodeData["key1"]

	testgolden.AssertGolden(t, "context_with_node_isolation", map[string]any{
		"parentBreadcrumb": parent.getBreadcrumb(),
		"childBreadcrumb":  child.getBreadcrumb(),
		"parentDepth":      parent.depth,
		"childDepth":       child.depth,
		"parentHasKey2":    parentHasKey2,
		"childHasKey1":     childHasKey1,
	})
}

// TestGolden_ContextCopy verifies that copy() creates an independent context.
func TestGolden_ContextCopy(t *testing.T) {
	original := &executionContext{
		nodeData: make(map[string]any),
		path:     []string{"Root", "Group"},
		depth:    2,
	}
	original.set("existing", "data")

	copied := original.copy()
	copied.set("new", "value")

	_, origHasNew := original.nodeData["new"]
	_, copiedHasExisting := copied.nodeData["existing"]

	testgolden.AssertGolden(t, "context_copy", map[string]any{
		"originalBreadcrumb":  original.getBreadcrumb(),
		"copiedBreadcrumb":    copied.getBreadcrumb(),
		"originalDepth":       original.depth,
		"copiedDepth":         copied.depth,
		"origHasNew":          origHasNew,
		"copiedHasExisting":   copiedHasExisting,
	})
}

// TestGolden_ContextSetAndGet verifies data storage and retrieval.
func TestGolden_ContextSetAndGet(t *testing.T) {
	ctx := &executionContext{
		nodeData: make(map[string]any),
		path:     []string{},
	}

	ctx.set("node-1", map[string]any{"output": "hello"})
	ctx.set("node-2", map[string]any{"count": 42})

	result := ctx.toMap()

	// Extract only our test keys (context also picks up env vars when using newExecutionContext)
	filtered := map[string]any{
		"node-1": result["node-1"],
		"node-2": result["node-2"],
	}

	testgolden.AssertGolden(t, "context_set_and_get", filtered)
}

// TestGolden_ContextString verifies the debug string representation.
func TestGolden_ContextString(t *testing.T) {
	ctx := &executionContext{
		nodeData: make(map[string]any),
		path:     []string{},
	}
	ctx.set("a", 1)
	ctx.set("b", 2)
	ctx.set("c", 3)

	testgolden.AssertGolden(t, "context_string", map[string]any{
		"string": ctx.String(),
	})
}
