package tui

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// BentoOrder stores the custom ordering of bentos
type BentoOrder struct {
	Order []string `json:"order"` // List of bento names in desired order
}

// stripNumberPrefix removes leading numbers and delimiters from bento names.
// Examples:
//
//	"1. My Bento" -> "My Bento"
//	"01 - My Bento" -> "My Bento"
//	"2_My_Bento" -> "My_Bento"
func stripNumberPrefix(name string) string {
	// Match: optional digits, optional separator (. - _ space), rest of string
	re := regexp.MustCompile(`^\d+[\.\-_\s]*(.+)$`)
	if matches := re.FindStringSubmatch(name); len(matches) > 1 {
		return strings.TrimSpace(matches[1])
	}
	return name
}

// loadBentoOrder loads the custom bento order from config
func loadBentoOrder() (*BentoOrder, error) {
	bentoHome := LoadBentoHome()
	orderPath := filepath.Join(bentoHome, "bento_order.json")

	// If file doesn't exist, return empty order
	if _, err := os.Stat(orderPath); os.IsNotExist(err) {
		return &BentoOrder{Order: []string{}}, nil
	}

	data, err := os.ReadFile(orderPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read bento order: %w", err)
	}

	var order BentoOrder
	if err := json.Unmarshal(data, &order); err != nil {
		return nil, fmt.Errorf("failed to parse bento order: %w", err)
	}

	return &order, nil
}

// saveBentoOrder saves the custom bento order to config
func saveBentoOrder(order *BentoOrder) error {
	bentoHome := LoadBentoHome()
	orderPath := filepath.Join(bentoHome, "bento_order.json")

	data, err := json.MarshalIndent(order, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal bento order: %w", err)
	}

	if err := os.WriteFile(orderPath, data, 0644); err != nil {
		return fmt.Errorf("failed to write bento order: %w", err)
	}

	return nil
}

// applyBentoOrder reorders items based on saved order.
// Items not in the saved order are appended at the end.
func applyBentoOrder(items []BentoItem, order *BentoOrder) []BentoItem {
	if len(order.Order) == 0 {
		return items
	}

	// Create map for quick lookup
	itemMap := make(map[string]BentoItem)
	for _, item := range items {
		itemMap[item.Name] = item
	}

	// Build ordered list
	var ordered []BentoItem
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

// extractBentoOrder extracts the current order from items
func extractBentoOrder(items []BentoItem) *BentoOrder {
	order := &BentoOrder{
		Order: make([]string, len(items)),
	}

	for i, item := range items {
		order.Order[i] = item.Name
	}

	return order
}
