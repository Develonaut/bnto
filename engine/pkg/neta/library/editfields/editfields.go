// Package editfields provides a neta for editing field values using static values or templates.
//
// The edit-fields neta is one of the most commonly used neta types in bento workflows.
// It allows setting field values either as literal values or using Go templates that
// reference data from previous neta in the workflow.
//
// # Common Use Cases
//
//   - Setting metadata fields on products (name, SKU, description)
//   - Constructing file paths dynamically from product IDs
//   - Building URLs using data from API responses
//   - Creating folder names based on workflow data
//
// # Template Syntax
//
// Edit-fields uses Go's text/template package. Templates can reference data from
// the workflow's execution context using dot notation:
//
//	"{{.product.name}}"           // Access nested fields
//	"product-{{.product.id}}.png" // Mix static text and templates
//	"{{index .items 0}}"          // Access array elements
//
// Learn more: https://pkg.go.dev/text/template
//
// # Example
//
//	ef := editfields.New()
//
//	params := map[string]interface{}{
//	    "values": map[string]interface{}{
//	        "name": "Product A",              // Static value
//	        "sku":  "{{.product.sku}}",       // Template value
//	    },
//	    "_context": map[string]interface{}{  // From previous neta
//	        "product": map[string]interface{}{
//	            "sku": "PROD-001",
//	        },
//	    },
//	}
//
//	result, _ := ef.Execute(ctx, params)
//	// result = {"name": "Product A", "sku": "PROD-001"}
package editfields

import (
	"bytes"
	"context"
	"fmt"
	"strings"
	"text/template"

	"github.com/Develonaut/bento/pkg/neta"
)

// EditFields implements the edit-fields neta type.
//
// It processes field values, executing templates against the workflow's
// execution context and returning a map of the processed values.
type EditFields struct{}

// New creates a new edit-fields neta instance.
//
// The returned neta is stateless and can be reused across multiple executions.
func New() *EditFields {
	return &EditFields{}
}

// Execute processes field values and returns the result.
//
// Parameters:
//
//	values (required): Map of field names to values (static or template strings)
//	_context (optional): Execution context from previous neta (for templates)
//
// Returns a map[string]interface{} containing the processed field values.
//
// If a value contains template syntax ({{...}}), it will be executed against
// the _context data. Otherwise, the value is returned as-is.
func (ef *EditFields) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	// Extract values parameter
	values, err := extractValues(params)
	if err != nil {
		return nil, err
	}

	// Extract execution context (may be nil)
	execContext := extractContext(params)

	// Process each field value
	result := make(map[string]interface{})
	for key, value := range values {
		processed, err := processValue(value, execContext)
		if err != nil {
			return nil, fmt.Errorf("failed to process field '%s': %w", key, err)
		}
		result[key] = processed
	}

	return result, nil
}

// Verify EditFields implements neta.Executable at compile time
var _ neta.Executable = (*EditFields)(nil)

// extractValues gets the values map from params.
// Returns an error if values is missing or not a map.
func extractValues(params map[string]interface{}) (map[string]interface{}, error) {
	valuesRaw, exists := params["values"]
	if !exists {
		return nil, fmt.Errorf("values parameter is required")
	}

	values, ok := valuesRaw.(map[string]interface{})
	if !ok {
		return nil, fmt.Errorf("values must be a map, got %T", valuesRaw)
	}

	return values, nil
}

// extractContext gets the execution context from params.
// Returns an empty map if _context is missing.
func extractContext(params map[string]interface{}) map[string]interface{} {
	contextRaw, exists := params["_context"]
	if !exists {
		return make(map[string]interface{})
	}

	execContext, ok := contextRaw.(map[string]interface{})
	if !ok {
		return make(map[string]interface{})
	}

	return execContext
}

// processValue processes a single value, executing templates if present.
//
// If value is a string containing {{...}}, it's treated as a template.
// Otherwise, the value is returned unchanged.
func processValue(value interface{}, context map[string]interface{}) (interface{}, error) {
	// Only process string values as potential templates
	strValue, ok := value.(string)
	if !ok {
		return value, nil
	}

	// Check if string contains template syntax
	if !isTemplate(strValue) {
		return value, nil
	}

	// Execute template
	return executeTemplate(strValue, context)
}

// isTemplate checks if a string contains Go template syntax.
// We check for "{{" which indicates template intent, even if malformed.
func isTemplate(s string) bool {
	return strings.Contains(s, "{{")
}

// executeTemplate executes a Go template string against the given context.
func executeTemplate(tmplStr string, context map[string]interface{}) (string, error) {
	// Parse template
	tmpl, err := template.New("field").Parse(tmplStr)
	if err != nil {
		return "", fmt.Errorf("template parse error: %w", err)
	}

	// Execute template
	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, context); err != nil {
		return "", fmt.Errorf("template execution error: %w", err)
	}

	return buf.String(), nil
}
