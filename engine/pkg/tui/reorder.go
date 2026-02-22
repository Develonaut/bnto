//go:build tui

package tui

import (
	tea "github.com/charmbracelet/bubbletea"
)

// moveItem moves the selected item up or down in the list.
// direction: -1 for up, +1 for down
func (m Model) moveItem(direction int) (tea.Model, tea.Cmd) {
	items := m.list.Items()
	if len(items) == 0 {
		return m, nil
	}

	currentIdx := m.list.Index()
	newIdx := currentIdx + direction

	// Check bounds
	if newIdx < 0 || newIdx >= len(items) {
		return m, nil
	}

	// Swap items
	items[currentIdx], items[newIdx] = items[newIdx], items[currentIdx]

	// Update list with new order
	m.list.SetItems(items)

	// Move cursor to follow the item
	m.list.Select(newIdx)

	return m, nil
}
