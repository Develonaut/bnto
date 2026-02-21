package tui

import "github.com/charmbracelet/lipgloss"

// View renders the current state of the executor.
// Switches between running and completion views based on execution state.
func (e Executor) View() string {
	if e.complete {
		return e.completionView()
	}
	return e.runningView()
}

// runningView shows execution in progress.
func (e Executor) runningView() string {
	lines := []string{}

	// Show workflow name
	lines = append(lines, e.theme.Title.Render("🍱 Running: "+e.bntoName))
	lines = append(lines, "")

	// Show sequence
	if e.sequence != nil {
		lines = append(lines, e.sequence.View())
	}

	// Show progress bar at bottom
	lines = append(lines, "")
	lines = append(lines, e.progress.View())

	return lipgloss.JoinVertical(lipgloss.Left, lines...)
}

// completionView shows execution complete.
func (e Executor) completionView() string {
	lines := []string{}

	// Show sequence
	if e.sequence != nil {
		lines = append(lines, e.sequence.View())
	}

	// Note: Progress bar is printed manually after TUI exit
	// We don't include it here to avoid blank lines in the output

	// Note: We don't show completion message in TUI mode anymore
	// The final success/error message is printed after TUI exits
	// This prevents duplicate messages and ensures consistent formatting

	return lipgloss.JoinVertical(lipgloss.Left, lines...)
}
