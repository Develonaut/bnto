package engine

import (
	"fmt"
	"strings"

	"github.com/Develonaut/bento/pkg/node"
)

// splitPreservingURLs splits a space-separated string of URLs.
// This handles the Go stringified array format where items are space-separated.
func splitPreservingURLs(s string) []string {
	// Simple split by space - URLs shouldn't contain spaces
	parts := strings.Fields(s)
	// Trim any whitespace from each part
	result := make([]string, 0, len(parts))
	for _, p := range parts {
		trimmed := strings.TrimSpace(p)
		if trimmed != "" {
			result = append(result, trimmed)
		}
	}
	return result
}

// extractLoopItems extracts and validates items for forEach loop.
func (i *Engine) extractLoopItems(
	def *node.Definition,
	execCtx *executionContext,
) ([]interface{}, error) {
	itemsParam := def.Parameters["items"]

	if i.logger != nil {
		i.logger.Debug("Loop items parameter",
			"loop_id", def.ID,
			"itemsParam", itemsParam,
			"itemsParam_type", fmt.Sprintf("%T", itemsParam))
	}

	resolved := execCtx.resolveValue(itemsParam)

	if i.logger != nil {
		i.logger.Debug("Loop items resolved",
			"loop_id", def.ID,
			"resolved", resolved,
			"resolved_type", fmt.Sprintf("%T", resolved))
	}

	return i.convertToInterfaceArray(def, resolved)
}

// convertToInterfaceArray converts resolved value to []interface{}.
func (i *Engine) convertToInterfaceArray(
	def *node.Definition,
	resolved interface{},
) ([]interface{}, error) {
	switch v := resolved.(type) {
	case []interface{}:
		return v, nil
	case []string:
		items := make([]interface{}, len(v))
		for idx, item := range v {
			items[idx] = item
		}
		return items, nil
	case []map[string]interface{}:
		items := make([]interface{}, len(v))
		for idx, item := range v {
			items[idx] = item
		}
		return items, nil
	case string:
		// Handle stringified array format from template execution: "[item1 item2 item3]"
		if len(v) > 2 && v[0] == '[' && v[len(v)-1] == ']' {
			inner := v[1 : len(v)-1]
			// Split by space (Go's default array stringification)
			parts := splitPreservingURLs(inner)
			items := make([]interface{}, len(parts))
			for idx, item := range parts {
				items[idx] = item
			}
			return items, nil
		}
		// String doesn't match array format
		if i.logger != nil {
			i.logger.Error("Loop items not an array",
				"loop_id", def.ID,
				"resolved_type", fmt.Sprintf("%T", resolved),
				"resolved_value", resolved)
		}
		return nil, newNodeError(def.ID, "loop", "validate",
			fmt.Errorf("'items' must be an array, got %T", resolved))
	default:
		if i.logger != nil {
			i.logger.Error("Loop items not an array",
				"loop_id", def.ID,
				"resolved_type", fmt.Sprintf("%T", resolved),
				"resolved_value", resolved)
		}
		return nil, newNodeError(def.ID, "loop", "validate",
			fmt.Errorf("'items' must be an array, got %T", resolved))
	}
}
