package tui

import (
	"fmt"

	"github.com/Develonaut/bnto/pkg/secrets"
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/huh"
)

// loadSecretsView loads secrets and switches to secrets view
func (m Model) loadSecretsView() (tea.Model, tea.Cmd) {
	mgr, err := secrets.NewManager()
	if err != nil {
		m.logs = fmt.Sprintf("Failed to load secrets: %v", err)
		m.currentView = executionView
		return m, nil
	}

	keys, err := mgr.List()
	if err != nil {
		m.logs = fmt.Sprintf("Failed to list secrets: %v", err)
		m.currentView = executionView
		return m, nil
	}

	// Build secrets list
	items := make([]list.Item, len(keys))
	for i, key := range keys {
		items[i] = SecretItem{Key: key}
	}

	m.secretsList.SetItems(items)
	m.currentView = secretsView
	return m, nil
}

// addSecret prompts for a new secret
func (m Model) addSecret() (tea.Model, tea.Cmd) {
	// Use Huh form to get secret key and value
	var key, value string

	form := huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Secret Key").
				Description("Uppercase letters, numbers, and underscores only").
				Placeholder("FIGMA_TOKEN").
				Value(&key),
			huh.NewInput().
				Title("Secret Value").
				Description("The secret value to store").
				EchoMode(huh.EchoModePassword).
				Value(&value),
		),
	).WithTheme(huh.ThemeCharm())

	if err := form.Run(); err != nil {
		// User cancelled
		return m, nil
	}

	// Validate and store secret
	mgr, err := secrets.NewManager()
	if err != nil {
		m.logs = fmt.Sprintf("Failed to initialize secrets: %v", err)
		return m, nil
	}

	if err := mgr.Set(key, value); err != nil {
		m.logs = fmt.Sprintf("Failed to store secret: %v", err)
		return m, nil
	}

	// Reload secrets view
	return m.loadSecretsView()
}

// deleteSecret removes a secret
func (m Model) deleteSecret(key string) (tea.Model, tea.Cmd) {
	mgr, err := secrets.NewManager()
	if err != nil {
		m.logs = fmt.Sprintf("Failed to initialize secrets: %v", err)
		return m, nil
	}

	if err := mgr.Delete(key); err != nil {
		m.logs = fmt.Sprintf("Failed to delete secret: %v", err)
		return m, nil
	}

	// Reload secrets view
	return m.loadSecretsView()
}
