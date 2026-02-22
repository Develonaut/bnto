//go:build tui

package tui

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// BntoOrder stores the custom ordering of workflows
type BntoOrder struct {
	Order []string `json:"order"` // List of workflow names in desired order
}

// stripNumberPrefix removes leading numbers and delimiters from workflow names.
// Examples:
//
//	"1. My Workflow" -> "My Workflow"
//	"01 - My Workflow" -> "My Workflow"
//	"2_My_Workflow" -> "My_Workflow"
func stripNumberPrefix(name string) string {
	// Match: optional digits, optional separator (. - _ space), rest of string
	re := regexp.MustCompile(`^\d+[\.\-_\s]*(.+)$`)
	if matches := re.FindStringSubmatch(name); len(matches) > 1 {
		return strings.TrimSpace(matches[1])
	}
	return name
}

// loadBntoOrder loads the custom workflow order from config
func loadBntoOrder() (*BntoOrder, error) {
	bntoHome := LoadBntoHome()
	orderPath := filepath.Join(bntoHome, "bnto_order.json")

	// If file doesn't exist, return empty order
	if _, err := os.Stat(orderPath); os.IsNotExist(err) {
		return &BntoOrder{Order: []string{}}, nil
	}

	data, err := os.ReadFile(orderPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read workflow order: %w", err)
	}

	var order BntoOrder
	if err := json.Unmarshal(data, &order); err != nil {
		return nil, fmt.Errorf("failed to parse workflow order: %w", err)
	}

	return &order, nil
}

// saveBntoOrder saves the custom workflow order to config
func saveBntoOrder(order *BntoOrder) error {
	bntoHome := LoadBntoHome()
	orderPath := filepath.Join(bntoHome, "bnto_order.json")

	data, err := json.MarshalIndent(order, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal workflow order: %w", err)
	}

	if err := os.WriteFile(orderPath, data, 0644); err != nil {
		return fmt.Errorf("failed to write workflow order: %w", err)
	}

	return nil
}

// applyBntoOrder reorders items based on saved order.
// Items not in the saved order are appended at the end.
func applyBntoOrder(items []BntoItem, order *BntoOrder) []BntoItem {
	if len(order.Order) == 0 {
		return items
	}

	// Create map for quick lookup
	itemMap := make(map[string]BntoItem)
	for _, item := range items {
		itemMap[item.Name] = item
	}

	// Build ordered list
	var ordered []BntoItem
	usedNames := make(map[string]bool)

	// Add items in saved order
	for _, name := range order.Order {
		if item, exists := itemMap[name]; exists {
			ordered = append(ordered, item)
			usedNames[name] = true
		}
	}

	// Append any items not in saved order
	for _, item := range items {
		if !usedNames[item.Name] {
			ordered = append(ordered, item)
		}
	}

	return ordered
}

// extractBntoOrder extracts the current order from items
func extractBntoOrder(items []BntoItem) *BntoOrder {
	order := &BntoOrder{
		Order: make([]string, len(items)),
	}

	for i, item := range items {
		order.Order[i] = item.Name
	}

	return order
}
