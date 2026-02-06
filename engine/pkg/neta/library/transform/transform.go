// Package transform provides data transformation using the expr language.
//
// The transform neta supports:
//   - Expression evaluation (arithmetic, logic, strings)
//   - Field mapping (transform object fields)
//   - Conditional transformations (ternary, if/else)
//   - Array operations (map, filter, reduce)
//
// Uses the expr-lang/expr library for powerful transformations.
//
// Example field mapping:
//
//	params := map[string]interface{}{
//	    "mappings": map[string]interface{}{
//	        "fullName": "firstName + ' ' + lastName",
//	        "age": "years",
//	    },
//	    "_context": map[string]interface{}{
//	        "firstName": "John",
//	        "lastName": "Doe",
//	        "years": 25,
//	    },
//	}
//	result, err := transformNeta.Execute(ctx, params)
//
// Learn more about expr: https://github.com/expr-lang/expr
package transform

import (
	"context"
	"fmt"

	"github.com/expr-lang/expr"
)

// Transform implements the transform neta for data transformations.
type Transform struct{}

// New creates a new transform neta instance.
func New() *Transform {
	return &Transform{}
}

// Execute runs data transformations using expr.
//
// Parameters:
//   - expression: expr expression string (for single transformations)
//   - mappings: map of field names to expressions (for field mapping)
//   - _context: execution context with data
//
// Returns:
//   - result: transformation result (for single expression)
//   - mapped: transformed object (for field mappings)
func (t *Transform) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	// Check if this is a field mapping operation
	if mappings, ok := params["mappings"].(map[string]interface{}); ok {
		return t.executeFieldMapping(ctx, mappings, params)
	}

	// Otherwise, execute single expression
	expression, ok := params["expression"].(string)
	if !ok {
		return nil, fmt.Errorf("either 'expression' or 'mappings' parameter is required")
	}

	return t.executeExpression(ctx, expression, params)
}

// executeExpression evaluates a single expr expression.
func (t *Transform) executeExpression(ctx context.Context, expression string, params map[string]interface{}) (interface{}, error) {
	// Get execution context
	env, _ := params["_context"].(map[string]interface{})
	if env == nil {
		env = make(map[string]interface{})
	}

	// Compile expression
	program, err := expr.Compile(expression, expr.Env(env))
	if err != nil {
		return nil, fmt.Errorf("failed to compile expression: %w", err)
	}

	// Execute expression
	result, err := expr.Run(program, env)
	if err != nil {
		return nil, fmt.Errorf("failed to execute expression: %w", err)
	}

	return map[string]interface{}{
		"result": result,
	}, nil
}

// executeFieldMapping maps multiple fields using expressions.
func (t *Transform) executeFieldMapping(ctx context.Context, mappings map[string]interface{}, params map[string]interface{}) (interface{}, error) {
	// Get execution context
	env, _ := params["_context"].(map[string]interface{})
	if env == nil {
		env = make(map[string]interface{})
	}

	mapped := make(map[string]interface{})

	// Process each mapping
	for fieldName, expressionVal := range mappings {
		expression, ok := expressionVal.(string)
		if !ok {
			return nil, fmt.Errorf("mapping for field '%s' must be a string expression", fieldName)
		}

		// Compile expression
		program, err := expr.Compile(expression, expr.Env(env))
		if err != nil {
			return nil, fmt.Errorf("failed to compile expression for field '%s': %w", fieldName, err)
		}

		// Execute expression
		result, err := expr.Run(program, env)
		if err != nil {
			return nil, fmt.Errorf("failed to execute expression for field '%s': %w", fieldName, err)
		}

		mapped[fieldName] = result
	}

	return map[string]interface{}{
		"mapped": mapped,
	}, nil
}
