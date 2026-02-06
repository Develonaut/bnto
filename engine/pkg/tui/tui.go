package tui

import (
	"fmt"
	"strings"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/muesli/reflow/wordwrap"
)

// wrapLogContent wraps log content to fit within viewport width.
// Handles ANSI escape codes and prevents horizontal overflow.
func wrapLogContent(content string, width int) string {
	if width <= 0 {
		return content
	}

	// Split into lines and wrap each line individually
	lines := strings.Split(content, "\n")
	wrappedLines := make([]string, 0, len(lines))

	for _, line := range lines {
		// Use wordwrap to wrap each line to the viewport width
		// This handles ANSI codes properly and prevents overflow
		wrapped := wordwrap.String(line, width)
		wrappedLines = append(wrappedLines, wrapped)
	}

	return strings.Join(wrappedLines, "\n")
}

// Update handles messages
func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		// Global quit
		if k := msg.String(); k == "ctrl+c" || k == "q" {
			m.quitting = true
			return m, tea.Quit
		}

	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		m.list.SetSize(msg.Width, msg.Height-4)
		m.settingsList.SetSize(msg.Width, msg.Height-4)
		m.secretsList.SetSize(msg.Width, msg.Height-4)
		m.variablesList.SetSize(msg.Width, msg.Height-4)
		m.themeList.SetSize(msg.Width/2-4, msg.Height-4)

		// Update viewport dimensions if in execution view
		if m.currentView == executionView && m.logViewport.Width > 0 {
			// Account for: title (3 lines) + help (1 line) + border (2 lines) + border padding (2 lines)
			viewportHeight := m.height - 8
			if viewportHeight < 5 {
				viewportHeight = 5
			}
			// Account for: border (4 chars total) + padding (4 chars total)
			viewportWidth := m.width - 8
			if viewportWidth < 40 {
				viewportWidth = 40
			}
			m.logViewport.Width = viewportWidth
			m.logViewport.Height = viewportHeight
		}

	case executionStartMsg:
		m.executing = true
		m.executionCancel = msg.cancel
		return m, nil

	case executionOutputMsg:
		m.logs += string(msg)
		// Wrap log content to viewport width to prevent horizontal overflow
		wrappedLogs := wrapLogContent(m.logs, m.logViewport.Width)
		m.logViewport.SetContent(wrappedLogs)
		m.logViewport.GotoBottom()
		// Continue listening for more logs
		return m, listenForLogs(m.logChan)

	case executionCompleteMsg:
		m.executing = false
		m.executionCancel = nil // Clear cancel function
		if msg.err != nil {
			m.logs += fmt.Sprintf("\n\n❌ Error: %v\n\n", msg.err)
		} else {
			m.logs += fmt.Sprintf("\n\n✅ Completed successfully in %s\n\n", msg.duration)
		}
		// Wrap log content to viewport width to prevent horizontal overflow
		wrappedLogs := wrapLogContent(m.logs, m.logViewport.Width)
		m.logViewport.SetContent(wrappedLogs)
		m.logViewport.GotoBottom()
		return m, nil
	}

	// Route to view-specific update
	switch m.currentView {
	case listView:
		return m.updateList(msg)
	case settingsView:
		return m.updateSettings(msg)
	case secretsView:
		return m.updateSecrets(msg)
	case variablesView:
		return m.updateVariables(msg)
	case formView:
		return m.updateForm(msg)
	case executionView:
		return m.updateExecution(msg)
	case themeView:
		return m.updateTheme(msg)
	}

	return m, nil
}

// View renders the current view
func (m Model) View() string {
	if m.quitting {
		return "\nSee you later!\n\n"
	}

	switch m.currentView {
	case listView:
		return m.viewList()
	case settingsView:
		return m.viewSettings()
	case secretsView:
		return m.viewSecrets()
	case variablesView:
		return m.viewVariables()
	case formView:
		return m.viewForm()
	case executionView:
		return m.viewExecution()
	case themeView:
		return m.viewTheme()
	}

	return ""
}
