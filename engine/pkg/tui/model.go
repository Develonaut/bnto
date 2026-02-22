//go:build tui

package tui

import (
	"context"
	"fmt"
	"time"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/huh"
)

// View states
const (
	listView = iota
	settingsView
	secretsView
	variablesView
	formView
	executionView
	themeView
)

// Messages for async execution
type executionOutputMsg string
type executionCompleteMsg struct {
	err      error
	duration time.Duration
}
type executionStartMsg struct {
	cancel context.CancelFunc
}

// settingsFormType identifies which settings form is active
type settingsFormType int

const (
	noSettingsForm settingsFormType = iota
	bntoHomeForm
	themeForm
	variableForm
	verboseForm
)

// Model holds the TUI state
type Model struct {
	currentView        int
	list               list.Model
	settingsList       list.Model
	secretsList        list.Model
	variablesList      list.Model
	themeList          list.Model
	form               *huh.Form
	selectedBnto       string
	bntoVars           []Variable
	varHolders         map[string]*string // Pointers to form values
	logs               string
	logViewport        viewport.Model     // Viewport for scrollable log display
	logChan            chan string        // Channel for streaming execution logs
	executionCancel    context.CancelFunc // Cancel function for running execution
	executing          bool
	width              int
	height             int
	theme              Variant
	quitting           bool
	activeSettingsForm settingsFormType // Tracks which settings form is active
	reorderMode        bool             // Tracks if we're in workflow reorder mode
	formStage          int              // Tracks multi-step form progress (0=path, 1=config)
	pathVariables      []Variable       // Path variables for first form stage
	configVariables    []Variable       // Config variables for second form stage

	// Key bindings for each view
	listKeys      listKeyMap
	settingsKeys  settingsKeyMap
	secretsKeys   secretsKeyMap
	variablesKeys variablesKeyMap
	formKeys      formKeyMap
	executionKeys executionKeyMap
	themeKeys     themeKeyMap
}

// BntoItem represents a workflow in the list
type BntoItem struct {
	Name     string
	FilePath string
}

func (i BntoItem) Title() string       { return i.Name }
func (i BntoItem) Description() string { return CompressPath(i.FilePath) }
func (i BntoItem) FilterValue() string { return i.Name }

// SettingsItem represents a settings option
type SettingsItem struct {
	Name   string
	Desc   string
	Action string
}

func (i SettingsItem) Title() string       { return i.Name }
func (i SettingsItem) Description() string { return i.Desc }
func (i SettingsItem) FilterValue() string { return i.Name }

// SecretItem represents a secret in the list
type SecretItem struct {
	Key string
}

func (i SecretItem) Title() string       { return i.Key }
func (i SecretItem) Description() string { return "Use {{SECRETS." + i.Key + "}} in workflows" }
func (i SecretItem) FilterValue() string { return i.Key }

// VariableItem represents a variable in the list
type VariableItem struct {
	Key   string
	Value string
}

func (i VariableItem) Title() string { return i.Key }
func (i VariableItem) Description() string {
	// Show compressed path if it looks like a path
	if len(i.Value) > 0 && (i.Value[0] == '/' || i.Value[0] == '~' || (len(i.Value) > 1 && i.Value[1] == ':')) {
		return CompressPath(i.Value)
	}
	return i.Value
}
func (i VariableItem) FilterValue() string { return i.Key }

// ThemeItem represents a theme in the list
type ThemeItem struct {
	Variant     Variant
	DisplayName string
	Desc        string
}

func (i ThemeItem) Title() string       { return i.DisplayName }
func (i ThemeItem) Description() string { return i.Desc }
func (i ThemeItem) FilterValue() string { return i.DisplayName }

// NewTUI creates a new TUI model
func NewTUI() (*Model, error) {
	l, err := createBntoList()
	if err != nil {
		return nil, err
	}

	return &Model{
		currentView:   listView,
		list:          l,
		settingsList:  createSettingsList(),
		secretsList:   createSecretsList(),
		variablesList: createVariablesList(),
		themeList:     createThemeList(),
		theme:         VariantNasu,
		listKeys:      newListKeyMap(),
		settingsKeys:  newSettingsKeyMap(),
		secretsKeys:   newSecretsKeyMap(),
		variablesKeys: newVariablesKeyMap(),
		formKeys:      newFormKeyMap(),
		executionKeys: newExecutionKeyMap(),
		themeKeys:     newThemeKeyMap(),
	}, nil
}

// createBntoList loads workflows and creates the workflow list
func createBntoList() (list.Model, error) {
	items, err := loadBntos()
	if err != nil {
		return list.Model{}, fmt.Errorf("failed to load workflows: %w", err)
	}

	l := list.New(items, list.NewDefaultDelegate(), 0, 0)
	l.Title = "🍱 Workflows"
	l.SetShowStatusBar(false)
	l.SetShowHelp(false)
	return l, nil
}

// createSettingsList creates the settings list with current configuration values
func createSettingsList() list.Model {
	currentHome := LoadBntoHome()
	currentTheme := LoadSavedTheme()
	verboseEnabled := LoadVerboseLogging()
	verboseStatus := "Disabled"
	if verboseEnabled {
		verboseStatus = "Enabled"
	}

	settingsItems := []list.Item{
		SettingsItem{Name: "Bnto Home", Desc: fmt.Sprintf("Current: %s", CompressPath(currentHome)), Action: "bntohome"},
		SettingsItem{Name: "Manage Secrets", Desc: "Add, view, or delete secrets", Action: "secrets"},
		SettingsItem{Name: "Manage Variables", Desc: "Add, view, or delete configuration variables", Action: "variables"},
		SettingsItem{Name: "Change Theme", Desc: fmt.Sprintf("Current: %s", currentTheme), Action: "theme"},
		SettingsItem{Name: "Verbose Logging", Desc: fmt.Sprintf("Current: %s", verboseStatus), Action: "verbose"},
	}

	sl := list.New(settingsItems, list.NewDefaultDelegate(), 0, 0)
	sl.Title = "⚙️  Settings"
	sl.SetShowStatusBar(false)
	sl.SetShowHelp(false)
	return sl
}

// createSecretsList creates an empty secrets list (loaded on demand)
func createSecretsList() list.Model {
	l := list.New([]list.Item{}, list.NewDefaultDelegate(), 0, 0)
	l.Title = "🔐 Secrets"
	l.SetShowStatusBar(false)
	l.SetShowHelp(false)
	return l
}

// createVariablesList creates an empty variables list (loaded on demand)
func createVariablesList() list.Model {
	l := list.New([]list.Item{}, list.NewDefaultDelegate(), 0, 0)
	l.Title = "📝 Variables"
	l.SetShowStatusBar(false)
	l.SetShowHelp(false)
	return l
}

// createThemeList creates the theme list with all available themes
func createThemeList() list.Model {
	themeItems := []list.Item{
		ThemeItem{Variant: VariantNasu, DisplayName: "Nasu", Desc: "Purple - eggplant sushi"},
		ThemeItem{Variant: VariantWasabi, DisplayName: "Wasabi", Desc: "Green - wasabi"},
		ThemeItem{Variant: VariantToro, DisplayName: "Toro", Desc: "Pink - fatty tuna"},
		ThemeItem{Variant: VariantTamago, DisplayName: "Tamago", Desc: "Yellow - egg sushi"},
		ThemeItem{Variant: VariantTonkotsu, DisplayName: "Tonkotsu", Desc: "Red - pork bone broth"},
		ThemeItem{Variant: VariantSaba, DisplayName: "Saba", Desc: "Cyan - mackerel"},
		ThemeItem{Variant: VariantIka, DisplayName: "Ika", Desc: "White - squid"},
	}

	l := list.New(themeItems, list.NewDefaultDelegate(), 0, 0)
	l.Title = "🎨 Themes"
	l.SetShowStatusBar(false)
	l.SetShowHelp(false)
	return l
}

// Init initializes the TUI
func (m Model) Init() tea.Cmd {
	return nil
}
