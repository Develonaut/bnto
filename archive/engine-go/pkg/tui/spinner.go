//go:build tui

package tui

import (
	"time"

	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// Spinner wraps bubbles/spinner with theme-aware styling.
type Spinner struct {
	spinner.Model
	palette Palette
}

// NewSpinner creates a themed spinner with custom sushi emoji frames.
func NewSpinner(palette Palette) Spinner {
	s := spinner.New()
	// Use centralized sushi spinner frames
	s.Spinner = spinner.Spinner{
		Frames: SushiSpinner,
		FPS:    time.Second, // 1 second per frame
	}
	s.Style = lipgloss.NewStyle().Foreground(palette.Primary)
	return Spinner{Model: s, palette: palette}
}

// Update handles spinner messages.
func (s Spinner) Update(msg tea.Msg) (Spinner, tea.Cmd) {
	var cmd tea.Cmd
	s.Model, cmd = s.Model.Update(msg)
	return s, cmd
}
