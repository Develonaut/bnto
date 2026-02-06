package tui

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/huh"
)

// updateList handles list view updates
func (m Model) updateList(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "enter":
			// Select bento (only if not in reorder mode)
			if !m.reorderMode {
				if selected, ok := m.list.SelectedItem().(BentoItem); ok {
					m.selectedBento = selected.FilePath
					return m.runBento()
				}
			}
		case "s":
			// Go to settings (only if not in reorder mode)
			if !m.reorderMode {
				m.currentView = settingsView
				return m, nil
			}
		case "o":
			// Toggle reorder mode
			m.reorderMode = !m.reorderMode
			return m, nil
		case "esc":
			// Exit reorder mode and save
			if m.reorderMode {
				m.reorderMode = false
				// Save current order
				items := m.list.Items()
				bentoItems := make([]BentoItem, len(items))
				for i, item := range items {
					bentoItems[i] = item.(BentoItem)
				}
				order := extractBentoOrder(bentoItems)
				_ = saveBentoOrder(order) // Ignore errors
				return m, nil
			}
		case "up", "k":
			// Move item up in reorder mode
			if m.reorderMode {
				return m.moveItem(-1)
			}
		case "down", "j":
			// Move item down in reorder mode
			if m.reorderMode {
				return m.moveItem(1)
			}
		}
	}

	var cmd tea.Cmd
	m.list, cmd = m.list.Update(msg)
	return m, cmd
}

// updateSettings handles settings view updates
func (m Model) updateSettings(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "enter":
			// Select setting
			if selected, ok := m.settingsList.SelectedItem().(SettingsItem); ok {
				switch selected.Action {
				case "secrets":
					return m.loadSecretsView()
				case "variables":
					return m.loadVariablesView()
				case "bentohome":
					return m.configureBentoHome()
				case "theme":
					return m.configureTheme()
				case "verbose":
					return m.configureVerbose()
				}
			}
		case "esc":
			// Return to list
			m.currentView = listView
			return m, nil
		}
	}

	var cmd tea.Cmd
	m.settingsList, cmd = m.settingsList.Update(msg)
	return m, cmd
}

// updateSecrets handles secrets view updates
func (m Model) updateSecrets(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "a":
			// Add new secret
			return m.addSecret()
		case "d", "x":
			// Delete selected secret
			if selected, ok := m.secretsList.SelectedItem().(SecretItem); ok {
				return m.deleteSecret(selected.Key)
			}
		case "esc":
			// Return to settings
			m.currentView = settingsView
			return m, nil
		}
	}

	var cmd tea.Cmd
	m.secretsList, cmd = m.secretsList.Update(msg)
	return m, cmd
}

// updateVariables handles variables view updates
func (m Model) updateVariables(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "a":
			// Add new variable
			return m.addVariable()
		case "d", "x":
			// Delete selected variable
			if selected, ok := m.variablesList.SelectedItem().(VariableItem); ok {
				return m.deleteVariable(selected.Key)
			}
		case "esc":
			// Return to settings
			m.currentView = settingsView
			return m, nil
		}
	}

	var cmd tea.Cmd
	m.variablesList, cmd = m.variablesList.Update(msg)
	return m, cmd
}

// updateForm handles form view updates
func (m Model) updateForm(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd

	// Check for ESC to cancel before updating form
	if keyMsg, ok := msg.(tea.KeyMsg); ok {
		if keyMsg.String() == "esc" {
			// If we're in a settings form, go back to settings
			if m.activeSettingsForm != noSettingsForm {
				m.activeSettingsForm = noSettingsForm
				m.currentView = settingsView
				return m, nil
			}

			// If we're in stage 2 of a multi-step form, go back to stage 1
			if m.formStage == 1 {
				m.formStage = 0
				return m.showForm()
			}

			// Otherwise, go back to bento list
			m.formStage = 0
			m.currentView = listView
			return m, nil
		}
	}

	// Update the form
	form, formCmd := m.form.Update(msg)
	if f, ok := form.(*huh.Form); ok {
		m.form = f
	}

	// Check if form is complete
	if m.form.State == huh.StateCompleted {
		// Handle different form types
		switch m.activeSettingsForm {
		case bentoHomeForm:
			return m.completeBentoHomeForm()
		case variableForm:
			return m.completeVariableForm()
		case verboseForm:
			return m.completeVerboseForm()
		default:
			// Bento variable form
			// Check if this is a multi-step form and we're on stage 1
			if len(m.pathVariables) > 0 && len(m.configVariables) > 0 && m.formStage == 0 {
				// Move to stage 2 (config form with loaded render.json values)
				m.formStage = 1
				return m.showConfigForm(m.configVariables)
			}

			// Form complete - reset stage and start execution
			m.formStage = 0
			return m.startExecution()
		}
	}

	return m, tea.Batch(cmd, formCmd)
}

// updateExecution handles execution view updates
func (m Model) updateExecution(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd

	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "esc":
			// Cancel execution if still running
			if m.executing && m.executionCancel != nil {
				m.executionCancel()
				m.logs += "\n\n⚠️  Execution cancelled by user\n\n"
				wrappedLogs := wrapLogContent(m.logs, m.logViewport.Width)
				m.logViewport.SetContent(wrappedLogs)
				m.logViewport.GotoBottom()
				m.executing = false
				m.executionCancel = nil
			}
			// Return to list
			m.currentView = listView
			return m, nil
		case "up", "k":
			m.logViewport.ScrollUp(1)
			return m, nil
		case "down", "j":
			m.logViewport.ScrollDown(1)
			return m, nil
		case "pgup", "b":
			m.logViewport.HalfPageUp()
			return m, nil
		case "pgdown", "f", " ":
			m.logViewport.HalfPageDown()
			return m, nil
		case "r":
			// Cancel current execution if running
			if m.executing && m.executionCancel != nil {
				m.executionCancel()
				m.executing = false
				m.executionCancel = nil
			}
			// Clear logs and restart execution
			m.logs = ""
			m.logViewport.SetContent("")
			return m.startExecution()
		}
	}

	// Update viewport for other events
	m.logViewport, cmd = m.logViewport.Update(msg)
	return m, cmd
}

// updateTheme handles theme view updates
func (m Model) updateTheme(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "enter":
			// Select theme
			if selected, ok := m.themeList.SelectedItem().(ThemeItem); ok {
				newVariant := selected.Variant
				if err := SaveTheme(newVariant); err == nil {
					// Successfully saved theme - update model
					m.theme = newVariant
				}
				// Return to settings view
				m.currentView = settingsView
				return m, nil
			}
		case "esc":
			// Return to settings without changing theme
			m.currentView = settingsView
			return m, nil
		}
	}

	var cmd tea.Cmd
	m.themeList, cmd = m.themeList.Update(msg)
	return m, cmd
}
