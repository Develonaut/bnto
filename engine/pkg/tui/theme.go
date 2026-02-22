//go:build tui

// Package tui provides terminal styling and progress display.
//
// Lipgloss styles for CLI output.
package tui

import "github.com/charmbracelet/lipgloss"

// Theme contains all lipgloss styles for CLI output.
type Theme struct {
	Title   lipgloss.Style
	Success lipgloss.Style
	Error   lipgloss.Style
	Warning lipgloss.Style
	Info    lipgloss.Style
	Subtle  lipgloss.Style
}

// BuildTheme creates a Theme from a Palette.
// This function is pure - no global state.
func BuildTheme(p Palette) *Theme {
	return &Theme{
		Title: lipgloss.NewStyle().
			Bold(true).
			Foreground(p.Primary),

		Success: lipgloss.NewStyle().
			Foreground(p.Success).
			Bold(true),

		Error: lipgloss.NewStyle().
			Foreground(p.Error).
			Bold(true),

		Warning: lipgloss.NewStyle().
			Foreground(p.Warning).
			Bold(true),

		Info: lipgloss.NewStyle().
			Foreground(p.Text),

		Subtle: lipgloss.NewStyle().
			Foreground(p.Muted),
	}
}
