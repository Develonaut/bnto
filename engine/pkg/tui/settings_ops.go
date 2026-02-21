package tui

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/huh"
)

// expandHomePath expands ~ to full path
func expandHomePath(path string) string {
	if strings.HasPrefix(path, "~") {
		if homeDir, err := os.UserHomeDir(); err == nil {
			return filepath.Join(homeDir, path[1:])
		}
	}
	return path
}

// configureBntoHome prompts for bnto home directory configuration
func (m Model) configureBntoHome() (tea.Model, tea.Cmd) {
	currentHome := LoadBntoHome()
	// Resolve the path for the file picker to start in the right place
	resolvedHome, err := ResolvePath(currentHome)
	if err != nil {
		resolvedHome = expandHomePath(currentHome)
	}

	newHome := resolvedHome
	m.varHolders = map[string]*string{"BNTO_HOME": &newHome}

	m.form = huh.NewForm(
		huh.NewGroup(
			huh.NewFilePicker().
				Title("Bnto Home Directory").
				Description(fmt.Sprintf("Current: %s (Tip: Use {{GDRIVE}} for cross-platform paths)", CompressPath(resolvedHome))).
				CurrentDirectory(resolvedHome).
				DirAllowed(true).
				FileAllowed(false).
				ShowHidden(true).
				Value(&newHome),
		),
	).WithTheme(huh.ThemeCharm()).
		WithWidth(m.width).
		WithHeight(m.height)

	m.activeSettingsForm = bntoHomeForm
	m.currentView = formView
	return m, m.form.Init()
}

// configureTheme switches to the theme view for theme selection
func (m Model) configureTheme() (tea.Model, tea.Cmd) {
	m.currentView = themeView
	return m, nil
}

// getFormValue retrieves a value from form holders
func getFormValue(holders map[string]*string, key string) string {
	if holder, ok := holders[key]; ok && holder != nil {
		return *holder
	}
	return ""
}

// completeSettingsForm finishes a settings form and returns to settings view
func (m Model) completeSettingsForm() (tea.Model, tea.Cmd) {
	m.activeSettingsForm = noSettingsForm
	m.currentView = settingsView
	return m, nil
}

// completeBntoHomeForm handles bnto home form completion
func (m Model) completeBntoHomeForm() (tea.Model, tea.Cmd) {
	currentHome := LoadBntoHome()
	newHome := getFormValue(m.varHolders, "BNTO_HOME")

	// Resolve current home for comparison
	resolvedCurrentHome, err := ResolvePath(currentHome)
	if err != nil {
		resolvedCurrentHome = expandHomePath(currentHome)
	}

	if newHome == "" || newHome == resolvedCurrentHome {
		return m.completeSettingsForm()
	}

	// Compress the path to use {{GDRIVE}} markers for portability
	compressedPath := CompressPath(newHome)

	if err := SaveBntoHome(compressedPath); err != nil {
		// Return to settings on error
		m.activeSettingsForm = noSettingsForm
		m.currentView = settingsView
		return m, nil
	}

	// Successfully changed bnto home - return to settings
	m.activeSettingsForm = noSettingsForm
	m.currentView = settingsView
	return m, nil
}

// configureVerbose prompts for verbose logging toggle
func (m Model) configureVerbose() (tea.Model, tea.Cmd) {
	currentVerbose := LoadVerboseLogging()
	verboseChoice := "false"
	if currentVerbose {
		verboseChoice = "true"
	}

	m.varHolders = map[string]*string{"VERBOSE": &verboseChoice}

	m.form = huh.NewForm(
		huh.NewGroup(
			huh.NewSelect[string]().
				Title("Verbose Logging").
				Description("Enable detailed output for debugging").
				Options(
					huh.NewOption("Disabled - Show only progress and errors", "false"),
					huh.NewOption("Enabled - Show all output", "true"),
				).
				Value(&verboseChoice),
		),
	).WithTheme(huh.ThemeCharm()).
		WithWidth(m.width).
		WithHeight(m.height)

	m.activeSettingsForm = verboseForm
	m.currentView = formView
	return m, m.form.Init()
}

// completeVerboseForm handles verbose logging form completion
func (m Model) completeVerboseForm() (tea.Model, tea.Cmd) {
	verboseChoice := getFormValue(m.varHolders, "VERBOSE")
	enabled := verboseChoice == "true"

	if err := SaveVerboseLogging(enabled); err != nil {
		// Return to settings on error
		m.activeSettingsForm = noSettingsForm
		m.currentView = settingsView
		return m, nil
	}

	// Successfully changed verbose logging - return to settings
	m.activeSettingsForm = noSettingsForm
	m.currentView = settingsView
	return m, nil
}
