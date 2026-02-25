package parallel_test

import (
	"context"
	"encoding/json"
	"sort"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/parallel"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_ParallelBasic(t *testing.T) {
	p := parallel.New()

	result, err := p.Execute(context.Background(), map[string]any{
		"tasks": []any{
			map[string]any{"id": "a", "value": 1},
			map[string]any{"id": "b", "value": 2},
			map[string]any{"id": "c", "value": 3},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Sort results by index to ensure deterministic comparison
	normalized := sortParallelResults(t, result)
	testgolden.AssertGolden(t, "parallel_basic", normalized)
}

func TestGolden_ParallelSingleTask(t *testing.T) {
	p := parallel.New()

	result, err := p.Execute(context.Background(), map[string]any{
		"tasks": []any{
			map[string]any{"value": "only-one"},
		},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	normalized := sortParallelResults(t, result)
	testgolden.AssertGolden(t, "parallel_single_task", normalized)
}

func TestGolden_ParallelSequential(t *testing.T) {
	p := parallel.New()

	result, err := p.Execute(context.Background(), map[string]any{
		"tasks": []any{
			map[string]any{"step": "first"},
			map[string]any{"step": "second"},
			map[string]any{"step": "third"},
		},
		"maxWorkers": 1, // Forces sequential execution
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	normalized := sortParallelResults(t, result)
	testgolden.AssertGolden(t, "parallel_sequential", normalized)
}

func TestGolden_ParallelEmpty(t *testing.T) {
	p := parallel.New()

	result, err := p.Execute(context.Background(), map[string]any{
		"tasks": []any{},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "parallel_empty", result)
}

// sortParallelResults normalizes result ordering for deterministic comparison.
// Parallel execution doesn't guarantee order, so we sort by JSON representation.
func sortParallelResults(t *testing.T, result any) any {
	t.Helper()

	output, ok := result.(map[string]any)
	if !ok {
		return result
	}

	results, ok := output["results"].([]any)
	if !ok || len(results) == 0 {
		return result
	}

	// Sort results by their JSON representation for deterministic comparison
	sort.Slice(results, func(i, j int) bool {
		a, _ := json.Marshal(results[i])
		b, _ := json.Marshal(results[j])
		return string(a) < string(b)
	})

	output["results"] = results
	return output
}
