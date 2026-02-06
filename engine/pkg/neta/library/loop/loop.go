// Package loop provides iteration neta (forEach, times, while modes).
//
// The loop neta enables repeating operations over collections or conditions:
//   - forEach: Iterate over an array/slice, passing each item to nested neta
//   - times: Repeat N times with index counter
//   - while: Loop while a condition evaluates to true
//
// CRITICAL FOR PHASE 8: Used to process CSV rows (array of maps).
//
// Example forEach usage:
//
//	params := map[string]interface{}{
//	    "mode": "forEach",
//	    "items": []interface{}{
//	        map[string]interface{}{"sku": "PROD-001"},
//	        map[string]interface{}{"sku": "PROD-002"},
//	    },
//	}
//	result, err := loopNeta.Execute(ctx, params)
//
// Learn more about Go slices: https://go.dev/tour/moretypes/7
package loop

import (
	"context"
	"fmt"

	"github.com/expr-lang/expr"
)

// Loop implements the loop neta for iteration operations.
type Loop struct{}

// New creates a new loop neta instance.
func New() *Loop {
	return &Loop{}
}

// Execute runs the loop neta with the specified mode.
//
// Parameters:
//   - mode: "forEach", "times", or "while"
//   - items: array/slice for forEach mode
//   - count: number of iterations for times mode
//   - condition: expression string for while mode
//   - breakCondition: optional expression to break early
//   - _context: execution context from previous neta
//
// Returns a map with:
//   - iterations: number of iterations completed
//   - results: array of results from each iteration
func (l *Loop) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	mode, ok := params["mode"].(string)
	if !ok {
		return nil, fmt.Errorf("mode parameter is required and must be a string")
	}

	switch mode {
	case "forEach":
		return l.executeForEach(ctx, params)
	case "times":
		return l.executeTimes(ctx, params)
	case "while":
		return l.executeWhile(ctx, params)
	default:
		return nil, fmt.Errorf("invalid mode: %s (must be forEach, times, or while)", mode)
	}
}

// executeForEach iterates over an array/slice.
func (l *Loop) executeForEach(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	items, ok := params["items"].([]interface{})
	if !ok {
		return nil, fmt.Errorf("items parameter is required for forEach mode and must be an array")
	}

	results := make([]interface{}, 0, len(items))
	breakCondition, hasBreak := params["breakCondition"].(string)

	for i, item := range items {
		// Check context cancellation
		if err := ctx.Err(); err != nil {
			return nil, err
		}

		// Check break condition
		if hasBreak {
			shouldBreak, err := l.evaluateCondition(breakCondition, map[string]interface{}{
				"item":  item,
				"index": i,
			})
			if err != nil {
				return nil, fmt.Errorf("break condition evaluation failed: %w", err)
			}
			if shouldBreak {
				// Include current item before breaking
				results = append(results, item)
				return map[string]interface{}{
					"iterations": len(results),
					"results":    results,
					"broken":     true,
				}, nil
			}
		}

		// Add item to results
		results = append(results, item)
	}

	return map[string]interface{}{
		"iterations": len(results),
		"results":    results,
	}, nil
}

// executeTimes repeats N times.
func (l *Loop) executeTimes(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	var count int

	// Handle both int and float64 (from JSON)
	switch v := params["count"].(type) {
	case int:
		count = v
	case float64:
		count = int(v)
	default:
		return nil, fmt.Errorf("count parameter is required for times mode and must be a number")
	}

	if count < 0 {
		return nil, fmt.Errorf("count must be non-negative, got %d", count)
	}

	results := make([]interface{}, 0, count)

	for i := 0; i < count; i++ {
		// Check context cancellation
		if err := ctx.Err(); err != nil {
			return nil, err
		}

		// Store iteration index
		results = append(results, map[string]interface{}{
			"index": i,
		})
	}

	return map[string]interface{}{
		"iterations": count,
		"results":    results,
	}, nil
}

// executeWhile loops while a condition is true.
func (l *Loop) executeWhile(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	condition, ok := params["condition"].(string)
	if !ok {
		return nil, fmt.Errorf("condition parameter is required for while mode and must be a string")
	}

	// Get initial context
	loopContext, _ := params["_context"].(map[string]interface{})
	if loopContext == nil {
		loopContext = make(map[string]interface{})
	}

	results := make([]interface{}, 0)
	maxIterations := 1000 // Safety limit to prevent infinite loops

	for i := 0; i < maxIterations; i++ {
		// Check context cancellation
		if err := ctx.Err(); err != nil {
			return nil, err
		}

		// Evaluate condition
		shouldContinue, err := l.evaluateCondition(condition, loopContext)
		if err != nil {
			return nil, fmt.Errorf("condition evaluation failed: %w", err)
		}

		if !shouldContinue {
			break
		}

		// Store iteration
		results = append(results, map[string]interface{}{
			"index": i,
		})

		// Increment counter if exists (for testing)
		if counter, ok := loopContext["counter"].(int); ok {
			loopContext["counter"] = counter + 1
		}
	}

	return map[string]interface{}{
		"iterations": len(results),
		"results":    results,
	}, nil
}

// evaluateCondition evaluates an expr expression and returns boolean result.
func (l *Loop) evaluateCondition(condition string, env map[string]interface{}) (bool, error) {
	program, err := expr.Compile(condition, expr.Env(env))
	if err != nil {
		return false, fmt.Errorf("failed to compile condition: %w", err)
	}

	output, err := expr.Run(program, env)
	if err != nil {
		return false, fmt.Errorf("failed to evaluate condition: %w", err)
	}

	result, ok := output.(bool)
	if !ok {
		return false, fmt.Errorf("condition must evaluate to boolean, got %T", output)
	}

	return result, nil
}
