package tui

import (
	"fmt"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/huh"
)

// loadVariablesView loads variables and switches to variables view
func (m Model) loadVariablesView() (tea.Model, tea.Cmd) {
	mgr, err := NewVariablesManager()
	if err != nil {
		m.logs = fmt.Sprintf("Failed to load variables: %v", err)
		m.currentView = executionView
		return m, nil
	}

	vars := mgr.GetAll()

	// Build variables list
	items := make([]list.Item, 0, len(vars))
	for key, value := range vars {
		items = append(items, VariableItem{
			Key:   key,
			Value: value,
		})
	}

	m.variablesList.SetItems(items)
	m.currentView = variablesView
	return m, nil
}

// addVariable prompts for a new variable
func (m Model) addVariable() (tea.Model, tea.Cmd) {
	// Create value holders for form
	key := ""
	value := ""
	m.varHolders = map[string]*string{
		"VAR_KEY":   &key,
		"VAR_VALUE": &value,
	}

	// Create non-blocking form
	m.form = huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Variable Key").
				Description("Uppercase letters, numbers, and underscores (e.g., PRODUCTS_URL)").
				Placeholder("PRODUCTS_URL").
				Value(&key),
			huh.NewInput().
				Title("Variable Value").
				Description("The value to store (supports {{GDRIVE}}, {{DROPBOX}}, {{ONEDRIVE}})").
				Placeholder("{{GDRIVE}}/Products").
				Value(&value),
		),
	).WithTheme(huh.ThemeCharm()).
		WithWidth(m.width).
		WithHeight(m.height)

	m.activeSettingsForm = variableForm
	m.currentView = formView
	return m, m.form.Init()
}

// completeVariableForm handles variable form completion
func (m Model) completeVariableForm() (tea.Model, tea.Cmd) {
	key := getFormValue(m.varHolders, "VAR_KEY")
	value := getFormValue(m.varHolders, "VAR_VALUE")

	// If empty, just cancel
	if key == "" || value == "" {
		m.activeSettingsForm = noSettingsForm
		m.currentView = variablesView
		return m, nil
	}

	// Store variable
	mgr, err := NewVariablesManager()
	if err != nil {
		m.logs = fmt.Sprintf("Failed to initialize variables: %v", err)
		m.activeSettingsForm = noSettingsForm
		m.currentView = variablesView
		return m, nil
	}

	if err := mgr.Set(key, value); err != nil {
		m.logs = fmt.Sprintf("Failed to store variable: %v", err)
		m.activeSettingsForm = noSettingsForm
		m.currentView = variablesView
		return m, nil
	}

	// Successfully added variable - reload variables view
	m.activeSettingsForm = noSettingsForm
	return m.loadVariablesView()
}

// deleteVariable removes a variable
func (m Model) deleteVariable(key string) (tea.Model, tea.Cmd) {
	mgr, err := NewVariablesManager()
	if err != nil {
		m.logs = fmt.Sprintf("Failed to initialize variables: %v", err)
		return m, nil
	}

	if err := mgr.Delete(key); err != nil {
		m.logs = fmt.Sprintf("Failed to delete variable: %v", err)
		return m, nil
	}

	// Reload variables view
	return m.loadVariablesView()
}
