package tui

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/huh"
)

// RenderConfig represents the structure of render.json
type RenderConfig struct {
	RenderTheme    string  `json:"renderTheme"`
	ZoomMultiplier float64 `json:"zoomMultiplier"`
	OffsetModifier float64 `json:"offsetModifier"`
}

// tryLoadRenderConfig attempts to load render.json from a product path.
// Returns the config if found, nil otherwise.
func tryLoadRenderConfig(productPath string) *RenderConfig {
	configPath := filepath.Join(productPath, "render.json")

	data, err := os.ReadFile(configPath)
	if err != nil {
		return nil // File doesn't exist or can't be read
	}

	var config RenderConfig
	if err := json.Unmarshal(data, &config); err != nil {
		return nil // Invalid JSON
	}

	return &config
}

// applyRenderConfigToVariables updates variable defaults from render.json
func applyRenderConfigToVariables(vars []Variable, productPath string) []Variable {
	config := tryLoadRenderConfig(productPath)
	if config == nil {
		return vars // No config found, return unchanged
	}

	// Apply config values to matching variables
	updated := make([]Variable, len(vars))
	for i, v := range vars {
		updated[i] = v
		switch v.Name {
		case "RENDER_THEME":
			if config.RenderTheme != "" {
				updated[i].DefaultValue = config.RenderTheme
			}
		case "ZOOM_MULTIPLIER":
			if config.ZoomMultiplier != 0 {
				updated[i].DefaultValue = fmt.Sprintf("%.1f", config.ZoomMultiplier)
			}
		case "OFFSET_MODIFIER":
			if config.OffsetModifier != 0 {
				updated[i].DefaultValue = fmt.Sprintf("%.1f", config.OffsetModifier)
			}
		}
	}

	return updated
}

// showForm creates and displays the variable form (potentially multi-step)
func (m Model) showForm() (tea.Model, tea.Cmd) {
	// Sort variables to show path variables first
	sortedVars := sortVariablesByPriority(m.bntoVars)

	// Separate path variables from other variables
	var pathVars, otherVars []Variable
	for _, v := range sortedVars {
		if isPathVariable(v.Name) {
			pathVars = append(pathVars, v)
		} else {
			otherVars = append(otherVars, v)
		}
	}

	// Store variables for later use
	m.pathVariables = pathVars
	m.configVariables = otherVars

	// Determine which form stage to show
	if len(pathVars) > 0 && len(otherVars) > 0 && m.formStage == 0 {
		// Stage 1: Show only path selection
		return m.showPathForm(pathVars)
	}

	// Stage 2 or single-stage form: Show all variables (or remaining variables)
	return m.showConfigForm(sortedVars)
}

// showPathForm creates a form with only path variables (first step)
func (m Model) showPathForm(pathVars []Variable) (tea.Model, tea.Cmd) {
	// Create value holders for path variables
	valueHolders := make(map[string]*string)
	for _, v := range pathVars {
		// If we already have a value from a previous form, use it
		if existingHolder, ok := m.varHolders[v.Name]; ok && existingHolder != nil {
			valueHolders[v.Name] = existingHolder
		} else {
			holder := v.DefaultValue
			valueHolders[v.Name] = &holder
		}
	}

	// Build path fields
	fields := make([]huh.Field, 0, len(pathVars))
	for _, v := range pathVars {
		fields = append(fields, buildFieldWithHeight(v, valueHolders[v.Name], m.height))
	}

	// Create form
	m.form = huh.NewForm(
		huh.NewGroup(fields...),
	).WithTheme(huh.ThemeCharm()).
		WithWidth(m.width).
		WithHeight(m.height)

	// Store value holders
	m.varHolders = valueHolders
	m.currentView = formView
	return m, m.form.Init()
}

// showConfigForm creates a form with config variables (second step or single-stage)
func (m Model) showConfigForm(vars []Variable) (tea.Model, tea.Cmd) {
	// Load render.json if we have a path variable value
	configVars := vars
	if m.formStage == 1 && len(m.pathVariables) > 0 {
		// Get the path value from the first path variable
		if pathHolder, ok := m.varHolders[m.pathVariables[0].Name]; ok && pathHolder != nil && *pathHolder != "" {
			configVars = applyRenderConfigToVariables(m.configVariables, *pathHolder)
		}
	}

	// Create value holders for all remaining variables
	valueHolders := make(map[string]*string)

	// Copy over path variable holders if they exist
	if m.formStage == 1 {
		for name, holder := range m.varHolders {
			// Only copy path variable holders, we'll create new ones for config vars
			if isPathVariable(name) {
				valueHolders[name] = holder
			}
		}
	}

	// Add/update config variable holders with loaded defaults from render.json
	for _, v := range configVars {
		// Always create new holders with the (potentially loaded) default values
		holder := v.DefaultValue
		valueHolders[v.Name] = &holder
	}

	// Build form fields
	fields := make([]huh.Field, 0, len(configVars))
	for _, v := range configVars {
		fields = append(fields, buildFieldWithHeight(v, valueHolders[v.Name], m.height))
	}

	// Create form
	m.form = huh.NewForm(
		huh.NewGroup(fields...),
	).WithTheme(huh.ThemeCharm()).
		WithWidth(m.width).
		WithHeight(m.height)

	// Store value holders
	m.varHolders = valueHolders
	m.currentView = formView
	return m, m.form.Init()
}

// sortVariablesByPriority sorts variables to show path variables first
func sortVariablesByPriority(vars []Variable) []Variable {
	// Create a copy to avoid modifying original
	sorted := make([]Variable, len(vars))
	copy(sorted, vars)

	// Sort with custom comparator:
	// 1. Path variables (containing PATH, DIR, FOLDER) first
	// 2. Within each group, alphabetically
	var pathVars, otherVars []Variable

	for _, v := range sorted {
		upperName := strings.ToUpper(v.Name)
		if strings.Contains(upperName, "PATH") ||
			strings.Contains(upperName, "DIR") ||
			strings.Contains(upperName, "DIRECTORY") ||
			strings.Contains(upperName, "FOLDER") {
			pathVars = append(pathVars, v)
		} else {
			otherVars = append(otherVars, v)
		}
	}

	// Combine: path vars first, then others
	result := make([]Variable, 0, len(sorted))
	result = append(result, pathVars...)
	result = append(result, otherVars...)

	return result
}

// startExecution runs the workflow with collected variables
func (m Model) startExecution() (tea.Model, tea.Cmd) {
	// Clear logs from previous execution
	m.logs = fmt.Sprintf("🍱 Executing: %s\n\n", filepath.Base(m.selectedBnto))

	// Set environment variables from form value holders
	if m.varHolders != nil {
		for name, valuePtr := range m.varHolders {
			if valuePtr != nil && *valuePtr != "" {
				os.Setenv(name, *valuePtr)
				// Log the variable being set for debugging
				m.logs += fmt.Sprintf("Set %s = %s\n", name, *valuePtr)
			}
		}
		m.logs += "\n"
	}

	// Set VERBOSE from config (both as env var and in varHolders for template resolution)
	verboseEnabled := LoadVerboseLogging()
	verboseValue := "false"
	if verboseEnabled {
		verboseValue = "true"
	}
	os.Setenv("VERBOSE", verboseValue)

	// Add to varHolders so it can be resolved in workflow templates
	if m.varHolders == nil {
		m.varHolders = make(map[string]*string)
	}
	m.varHolders["VERBOSE"] = &verboseValue

	m.logs += fmt.Sprintf("Set VERBOSE = %s (from config)\n\n", verboseValue)

	m.currentView = executionView
	m.executing = true

	// Initialize viewport for log display
	// Account for: title (3 lines) + help (1 line) + border (2 lines) + border padding (2 lines)
	viewportHeight := m.height - 8
	if viewportHeight < 5 {
		viewportHeight = 5
	}
	// Account for: border (1 char each side) + border padding (2 chars each side)
	// Add small buffer for safety margin
	viewportWidth := m.width - 6 - 2
	if viewportWidth < 40 {
		viewportWidth = 40
	}
	m.logViewport = viewport.New(viewportWidth, viewportHeight)
	// Wrap initial log content to viewport width
	wrappedLogs := wrapLogContent(m.logs, viewportWidth)
	m.logViewport.SetContent(wrappedLogs)

	// Create log channel for streaming logs
	m.logChan = make(chan string, 100)

	// Start async execution and log listener
	execCmd, startCmd := m.executeBntoAsync(m.logChan)
	return m, tea.Batch(
		startCmd, // Send start message with cancel function
		execCmd,  // Start execution
		listenForLogs(m.logChan),
	)
}
