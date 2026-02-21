package tui

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/Develonaut/bnto/pkg/engine"
	"github.com/Develonaut/bnto/pkg/node"
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
)

// ensureBntoDirectories creates bnto home structure if it doesn't exist
func ensureBntoDirectories() error {
	bntoHome := LoadBntoHome()
	bntosDir := filepath.Join(bntoHome, "bntos")

	// Create bntos directory with parents if needed
	if err := os.MkdirAll(bntosDir, 0755); err != nil {
		return fmt.Errorf("failed to create bntos directory: %w", err)
	}

	return nil
}

// loadBntos scans configured bnto home for workflow files
func loadBntos() ([]list.Item, error) {
	// Ensure directory structure exists
	if err := ensureBntoDirectories(); err != nil {
		return nil, err
	}

	bntoHome := LoadBntoHome()
	bntosDir := filepath.Join(bntoHome, "bntos")

	entries, err := os.ReadDir(bntosDir)
	if err != nil {
		return nil, err
	}

	var bntoItems []BntoItem
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".bnto.json") {
			continue
		}

		name := strings.TrimSuffix(entry.Name(), ".bnto.json")

		// Strip number prefixes for display
		displayName := stripNumberPrefix(name)

		bntoItems = append(bntoItems, BntoItem{
			Name:     displayName,
			FilePath: filepath.Join(bntosDir, entry.Name()),
		})
	}

	// Load saved order and apply it
	order, err := loadBntoOrder()
	if err != nil {
		// If order loading fails, just use default order
		order = &BntoOrder{Order: []string{}}
	}

	bntoItems = applyBntoOrder(bntoItems, order)

	// Convert to list items
	var items []list.Item
	for _, item := range bntoItems {
		items = append(items, item)
	}

	return items, nil
}

// loadBntoDefinition loads a workflow from file
func loadBntoDefinition(path string) (*node.Definition, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read workflow: %w", err)
	}

	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		return nil, fmt.Errorf("failed to parse workflow: %w", err)
	}

	return &def, nil
}

// runBnto executes the selected workflow
func (m Model) runBnto() (tea.Model, tea.Cmd) {
	// Read workflow file
	bntoJSON, err := os.ReadFile(m.selectedBnto)
	if err != nil {
		m.logs = fmt.Sprintf("Failed to read workflow: %v", err)
		m.currentView = executionView
		return m, nil
	}

	// Parse metadata to check for variables
	meta, err := ParseBntoMetadata(bntoJSON)
	if err != nil {
		m.logs = fmt.Sprintf("Failed to parse workflow: %v", err)
		m.currentView = executionView
		return m, nil
	}

	// If variables exist, show form
	if len(meta.Variables) > 0 {
		m.bntoVars = meta.Variables
		return m.showForm()
	}

	// No variables, go straight to execution
	return m.startExecution()
}

// executeBntoAsync runs the workflow in a goroutine and returns two tea.Cmds:
// 1. execCmd - the actual execution command
// 2. startCmd - a command that sends the cancel function to the model
func (m Model) executeBntoAsync(logChan chan string) (tea.Cmd, tea.Cmd) {
	// Create context and cancel function
	ctx, cancel := context.WithTimeout(context.Background(), 6*time.Hour)

	// Return start message with cancel function immediately
	startCmd := func() tea.Msg {
		return executionStartMsg{cancel: cancel}
	}

	// Execution command runs in goroutine
	execCmd := func() tea.Msg {
		defer close(logChan)
		defer cancel() // Ensure context is cancelled when done

		// Load workflow definition
		def, err := loadBntoDefinition(m.selectedBnto)
		if err != nil {
			return executionCompleteMsg{err: err}
		}

		// Create registry
		p := createTUIRegistry()

		// Create logger that writes to both file and TUI
		logFile, logger, err := createTUILogger(logChan)
		if err != nil {
			return executionCompleteMsg{err: err}
		}
		defer logFile.Close()

		// Create engine with logger (no messenger needed)
		eng := engine.NewWithMessenger(p, logger, nil)

		start := time.Now()
		_, err = eng.Serve(ctx, def)
		duration := time.Since(start)

		return executionCompleteMsg{
			err:      err,
			duration: duration,
		}
	}

	return execCmd, startCmd
}
