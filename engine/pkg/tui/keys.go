package tui

import (
	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/lipgloss"
)

// listKeyMap defines key bindings for the workflow list view
type listKeyMap struct {
	Enter       key.Binding
	Settings    key.Binding
	Reorder     key.Binding
	Quit        key.Binding
	MoveUp      key.Binding
	MoveDown    key.Binding
	SaveReorder key.Binding
}

// settingsKeyMap defines key bindings for the settings view
type settingsKeyMap struct {
	Enter key.Binding
	Back  key.Binding
	Quit  key.Binding
}

// secretsKeyMap defines key bindings for secrets view
type secretsKeyMap struct {
	Add    key.Binding
	Delete key.Binding
	Back   key.Binding
	Quit   key.Binding
}

// variablesKeyMap defines key bindings for variables view
type variablesKeyMap struct {
	Add    key.Binding
	Delete key.Binding
	Back   key.Binding
	Quit   key.Binding
}

// formKeyMap defines key bindings for form view
type formKeyMap struct {
	Cancel key.Binding
}

// executionKeyMap defines key bindings for execution view
type executionKeyMap struct {
	ScrollUp   key.Binding
	ScrollDown key.Binding
	PageUp     key.Binding
	PageDown   key.Binding
	Retry      key.Binding
	Back       key.Binding
	Quit       key.Binding
}

// themeKeyMap defines key bindings for theme view
type themeKeyMap struct {
	Select key.Binding
	Back   key.Binding
	Quit   key.Binding
}

// newListKeyMap creates the default key bindings for list view
func newListKeyMap() listKeyMap {
	return listKeyMap{
		Enter: key.NewBinding(
			key.WithKeys("enter"),
			key.WithHelp("enter", "select workflow"),
		),
		Settings: key.NewBinding(
			key.WithKeys("s"),
			key.WithHelp("s", "settings"),
		),
		Reorder: key.NewBinding(
			key.WithKeys("o"),
			key.WithHelp("o", "reorder"),
		),
		Quit: key.NewBinding(
			key.WithKeys("q", "ctrl+c"),
			key.WithHelp("q", "quit"),
		),
		MoveUp: key.NewBinding(
			key.WithKeys("up"),
			key.WithHelp("↑/↓", "move item"),
		),
		MoveDown: key.NewBinding(
			key.WithKeys("down"),
			key.WithHelp("", ""),
		),
		SaveReorder: key.NewBinding(
			key.WithKeys("esc"),
			key.WithHelp("esc", "save & exit"),
		),
	}
}

// newSettingsKeyMap creates the default key bindings for settings view
func newSettingsKeyMap() settingsKeyMap {
	return settingsKeyMap{
		Enter: key.NewBinding(
			key.WithKeys("enter"),
			key.WithHelp("enter", "select"),
		),
		Back: key.NewBinding(
			key.WithKeys("esc"),
			key.WithHelp("esc", "back to list"),
		),
		Quit: key.NewBinding(
			key.WithKeys("q", "ctrl+c"),
			key.WithHelp("q", "quit"),
		),
	}
}

// newSecretsKeyMap creates the default key bindings for secrets view
func newSecretsKeyMap() secretsKeyMap {
	return secretsKeyMap{
		Add: key.NewBinding(
			key.WithKeys("a"),
			key.WithHelp("a", "add secret"),
		),
		Delete: key.NewBinding(
			key.WithKeys("d", "x"),
			key.WithHelp("d/x", "delete"),
		),
		Back: key.NewBinding(
			key.WithKeys("esc"),
			key.WithHelp("esc", "back"),
		),
		Quit: key.NewBinding(
			key.WithKeys("q", "ctrl+c"),
			key.WithHelp("q", "quit"),
		),
	}
}

// newVariablesKeyMap creates the default key bindings for variables view
func newVariablesKeyMap() variablesKeyMap {
	return variablesKeyMap{
		Add: key.NewBinding(
			key.WithKeys("a"),
			key.WithHelp("a", "add variable"),
		),
		Delete: key.NewBinding(
			key.WithKeys("d", "x"),
			key.WithHelp("d/x", "delete"),
		),
		Back: key.NewBinding(
			key.WithKeys("esc"),
			key.WithHelp("esc", "back"),
		),
		Quit: key.NewBinding(
			key.WithKeys("q", "ctrl+c"),
			key.WithHelp("q", "quit"),
		),
	}
}

// newFormKeyMap creates the default key bindings for form view
func newFormKeyMap() formKeyMap {
	return formKeyMap{
		Cancel: key.NewBinding(
			key.WithKeys("esc"),
			key.WithHelp("esc", "cancel"),
		),
	}
}

// newExecutionKeyMap creates the default key bindings for execution view
func newExecutionKeyMap() executionKeyMap {
	return executionKeyMap{
		ScrollUp: key.NewBinding(
			key.WithKeys("up", "k"),
			key.WithHelp("↑/k", "scroll up"),
		),
		ScrollDown: key.NewBinding(
			key.WithKeys("down", "j"),
			key.WithHelp("↓/j", "scroll down"),
		),
		PageUp: key.NewBinding(
			key.WithKeys("pgup", "b"),
			key.WithHelp("pgup/b", "page up"),
		),
		PageDown: key.NewBinding(
			key.WithKeys("pgdown", "f", " "),
			key.WithHelp("pgdn/f", "page down"),
		),
		Retry: key.NewBinding(
			key.WithKeys("r"),
			key.WithHelp("r", "retry"),
		),
		Back: key.NewBinding(
			key.WithKeys("esc"),
			key.WithHelp("esc", "back to list"),
		),
		Quit: key.NewBinding(
			key.WithKeys("q", "ctrl+c"),
			key.WithHelp("q", "quit"),
		),
	}
}

// newThemeKeyMap creates the default key bindings for theme view
func newThemeKeyMap() themeKeyMap {
	return themeKeyMap{
		Select: key.NewBinding(
			key.WithKeys("enter"),
			key.WithHelp("enter", "select theme"),
		),
		Back: key.NewBinding(
			key.WithKeys("esc"),
			key.WithHelp("esc", "back to settings"),
		),
		Quit: key.NewBinding(
			key.WithKeys("q", "ctrl+c"),
			key.WithHelp("q", "quit"),
		),
	}
}

// helpText generates a help string from key bindings with theme colors
func helpText(palette Palette, keys ...key.Binding) string {
	keyStyle := lipgloss.NewStyle().Foreground(palette.Text)
	descStyle := lipgloss.NewStyle().Foreground(palette.Muted)

	var helpParts []string
	for _, k := range keys {
		if k.Enabled() {
			h := k.Help()
			// Style key with Text color and description with Muted color
			styledPart := keyStyle.Render(h.Key) + " " + descStyle.Render(h.Desc)
			helpParts = append(helpParts, styledPart)
		}
	}

	result := ""
	for i, part := range helpParts {
		if i > 0 {
			result += " • "
		}
		result += part
	}
	return result
}
