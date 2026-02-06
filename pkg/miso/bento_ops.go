package miso

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/Develonaut/bento/pkg/itamae"
	"github.com/Develonaut/bento/pkg/neta"
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
)

// ensureBentoDirectories creates bento home structure if it doesn't exist
func ensureBentoDirectories() error {
	bentoHome := LoadBentoHome()
	bentosDir := filepath.Join(bentoHome, "bentos")

	// Create bentos directory with parents if needed
	if err := os.MkdirAll(bentosDir, 0755); err != nil {
		return fmt.Errorf("failed to create bentos directory: %w", err)
	}

	return nil
}

// loadBentos scans configured bento home for bento files
func loadBentos() ([]list.Item, error) {
	// Ensure directory structure exists
	if err := ensureBentoDirectories(); err != nil {
		return nil, err
	}

	bentoHome := LoadBentoHome()
	bentosDir := filepath.Join(bentoHome, "bentos")

	entries, err := os.ReadDir(bentosDir)
	if err != nil {
		return nil, err
	}

	var bentoItems []BentoItem
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".bento.json") {
			continue
		}

		name := strings.TrimSuffix(entry.Name(), ".bento.json")

		// Strip number prefixes for display
		displayName := stripNumberPrefix(name)

		bentoItems = append(bentoItems, BentoItem{
			Name:     displayName,
			FilePath: filepath.Join(bentosDir, entry.Name()),
		})
	}

	// Load saved order and apply it
	order, err := loadBentoOrder()
	if err != nil {
		// If order loading fails, just use default order
		order = &BentoOrder{Order: []string{}}
	}

	bentoItems = applyBentoOrder(bentoItems, order)

	// Convert to list items
	var items []list.Item
	for _, item := range bentoItems {
		items = append(items, item)
	}

	return items, nil
}

// loadBentoDefinition loads a bento from file
func loadBentoDefinition(path string) (*neta.Definition, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read bento: %w", err)
	}

	var def neta.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		return nil, fmt.Errorf("failed to parse bento: %w", err)
	}

	return &def, nil
}

// runBento executes the selected bento
func (m Model) runBento() (tea.Model, tea.Cmd) {
	// Read bento file
	bentoJSON, err := os.ReadFile(m.selectedBento)
	if err != nil {
		m.logs = fmt.Sprintf("Failed to read bento: %v", err)
		m.currentView = executionView
		return m, nil
	}

	// Parse metadata to check for variables
	meta, err := ParseBentoMetadata(bentoJSON)
	if err != nil {
		m.logs = fmt.Sprintf("Failed to parse bento: %v", err)
		m.currentView = executionView
		return m, nil
	}

	// If variables exist, show form
	if len(meta.Variables) > 0 {
		m.bentoVars = meta.Variables
		return m.showForm()
	}

	// No variables, go straight to execution
	return m.startExecution()
}

// executeBentoAsync runs the bento in a goroutine and returns two tea.Cmds:
// 1. execCmd - the actual execution command
// 2. startCmd - a command that sends the cancel function to the model
func (m Model) executeBentoAsync(logChan chan string) (tea.Cmd, tea.Cmd) {
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

		// Load bento definition
		def, err := loadBentoDefinition(m.selectedBento)
		if err != nil {
			return executionCompleteMsg{err: err}
		}

		// Create pantry
		p := createTUIPantry()

		// Create logger that writes to both file and TUI
		logFile, logger, err := createTUILogger(logChan)
		if err != nil {
			return executionCompleteMsg{err: err}
		}
		defer logFile.Close()

		// Create chef with logger (no messenger needed)
		chef := itamae.NewWithMessenger(p, logger, nil)

		start := time.Now()
		_, err = chef.Serve(ctx, def)
		duration := time.Since(start)

		return executionCompleteMsg{
			err:      err,
			duration: duration,
		}
	}

	return execCmd, startCmd
}
