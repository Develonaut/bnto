package miso

import (
	"github.com/charmbracelet/lipgloss"
)

// viewList renders the list view
func (m Model) viewList() string {
	palette := GetPalette(m.theme)
	var helpStr string
	if m.reorderMode {
		// Show reorder mode help with styled keys
		helpStr = helpText(palette, m.listKeys.MoveUp, m.listKeys.SaveReorder, m.listKeys.Quit)
	} else {
		// Show normal mode help
		helpStr = helpText(palette, m.listKeys.Enter, m.listKeys.Settings, m.listKeys.Reorder, m.listKeys.Quit)
	}

	helpStyled := lipgloss.NewStyle().
		Faint(true).
		Padding(1, 0).
		Render(helpStr)

	content := m.list.View() + "\n" + helpStyled
	return lipgloss.NewStyle().
		Padding(1, 2).
		Render(content)
}

// viewSettings renders the settings view
func (m Model) viewSettings() string {
	palette := GetPalette(m.theme)
	helpStr := helpText(palette, m.settingsKeys.Enter, m.settingsKeys.Back, m.settingsKeys.Quit)
	helpStyled := lipgloss.NewStyle().
		Faint(true).
		Padding(1, 0).
		Render(helpStr)

	content := m.settingsList.View() + "\n" + helpStyled
	return lipgloss.NewStyle().
		Padding(1, 2).
		Render(content)
}

// viewSecrets renders the secrets view
func (m Model) viewSecrets() string {
	palette := GetPalette(m.theme)
	helpStr := helpText(palette, m.secretsKeys.Add, m.secretsKeys.Delete, m.secretsKeys.Back, m.secretsKeys.Quit)
	helpStyled := lipgloss.NewStyle().
		Faint(true).
		Padding(1, 0).
		Render(helpStr)

	content := m.secretsList.View() + "\n" + helpStyled
	return lipgloss.NewStyle().
		Padding(1, 2).
		Render(content)
}

// viewVariables renders the variables view
func (m Model) viewVariables() string {
	palette := GetPalette(m.theme)
	helpStr := helpText(palette, m.variablesKeys.Add, m.variablesKeys.Delete, m.variablesKeys.Back, m.variablesKeys.Quit)
	helpStyled := lipgloss.NewStyle().
		Faint(true).
		Padding(1, 0).
		Render(helpStr)

	content := m.variablesList.View() + "\n" + helpStyled
	return lipgloss.NewStyle().
		Padding(1, 2).
		Render(content)
}

// viewForm renders the form view
func (m Model) viewForm() string {
	if m.form == nil {
		return "Loading form..."
	}
	return m.form.View()
}

// viewExecution renders the execution view
func (m Model) viewExecution() string {
	palette := GetPalette(m.theme)

	// Title
	titleStyle := lipgloss.NewStyle().
		Foreground(palette.Primary).
		Bold(true).
		Padding(1, 2)
	title := titleStyle.Render("🍱 Execution")

	// Bordered viewport container
	borderStyle := lipgloss.NewStyle().
		BorderStyle(lipgloss.RoundedBorder()).
		BorderForeground(palette.Primary).
		Padding(1, 2)

	viewportContent := borderStyle.Render(m.logViewport.View())

	// Help text from key bindings
	helpStr := helpText(palette, m.executionKeys.ScrollUp, m.executionKeys.ScrollDown,
		m.executionKeys.PageUp, m.executionKeys.PageDown,
		m.executionKeys.Retry, m.executionKeys.Back, m.executionKeys.Quit)
	help := lipgloss.NewStyle().
		Padding(0, 2).
		Faint(true).
		Render(helpStr)

	return lipgloss.JoinVertical(
		lipgloss.Left,
		title,
		viewportContent,
		help,
	)
}

// viewTheme renders the theme view with dynamic palette preview
func (m Model) viewTheme() string {
	palette := GetPalette(m.theme)

	// Get the selected theme variant for preview
	var selectedVariant Variant
	if selected, ok := m.themeList.SelectedItem().(ThemeItem); ok {
		selectedVariant = selected.Variant
	} else {
		selectedVariant = m.theme
	}
	selectedPalette := GetPalette(selectedVariant)

	// Create the theme list view (left side)
	listView := m.themeList.View()

	// Create the palette preview (right side)
	previewStyle := lipgloss.NewStyle().
		BorderStyle(lipgloss.RoundedBorder()).
		BorderForeground(selectedPalette.Primary).
		Padding(1, 2).
		Width(40)

	// Build color preview content
	primaryStyled := lipgloss.NewStyle().Foreground(selectedPalette.Primary).Render(string(selectedPalette.Primary))
	secondaryStyled := lipgloss.NewStyle().Foreground(selectedPalette.Secondary).Render(string(selectedPalette.Secondary))
	successStyled := lipgloss.NewStyle().Foreground(selectedPalette.Success).Render(string(selectedPalette.Success))
	errorStyled := lipgloss.NewStyle().Foreground(selectedPalette.Error).Render(string(selectedPalette.Error))
	warningStyled := lipgloss.NewStyle().Foreground(selectedPalette.Warning).Render(string(selectedPalette.Warning))
	textStyled := lipgloss.NewStyle().Foreground(selectedPalette.Text).Render(string(selectedPalette.Text))
	mutedStyled := lipgloss.NewStyle().Foreground(selectedPalette.Muted).Render(string(selectedPalette.Muted))

	previewTitle := lipgloss.NewStyle().
		Foreground(selectedPalette.Primary).
		Bold(true).
		Render(string(selectedVariant))

	previewContent := previewStyle.Render(
		previewTitle + "\n\n" +
			"Primary:   " + primaryStyled + "\n" +
			"Secondary: " + secondaryStyled + "\n" +
			"Success:   " + successStyled + "\n" +
			"Error:     " + errorStyled + "\n" +
			"Warning:   " + warningStyled + "\n" +
			"Text:      " + textStyled + "\n" +
			"Muted:     " + mutedStyled,
	)

	// Combine list and preview side by side
	content := lipgloss.JoinHorizontal(
		lipgloss.Top,
		listView,
		"  ", // Spacing between list and preview
		previewContent,
	)

	// Help text
	helpStr := helpText(palette, m.themeKeys.Select, m.themeKeys.Back, m.themeKeys.Quit)
	helpStyled := lipgloss.NewStyle().
		Faint(true).
		Padding(1, 0).
		Render(helpStr)

	return lipgloss.NewStyle().
		Padding(1, 2).
		Render(content + "\n" + helpStyled)
}
