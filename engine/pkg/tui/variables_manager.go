//go:build tui

package tui

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
)

// VariablesManager handles storage and retrieval of user-defined variables.
// Unlike secrets (which use OS keychain), variables are stored in JSON
// for easy editing and non-sensitive configuration values.
type VariablesManager struct {
	filePath  string
	variables map[string]string
}

// NewVariablesManager creates a new variables manager.
func NewVariablesManager() (*VariablesManager, error) {
	bntoDir := LoadBntoHome()
	filePath := filepath.Join(bntoDir, "variables.json")

	// Ensure bnto directory exists
	if err := os.MkdirAll(bntoDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create bnto directory: %w", err)
	}

	mgr := &VariablesManager{
		filePath:  filePath,
		variables: make(map[string]string),
	}

	// Load existing variables if file exists
	if err := mgr.load(); err != nil && !os.IsNotExist(err) {
		return nil, err
	}

	return mgr, nil
}

// load reads variables from JSON file and platform-specific overrides
func (m *VariablesManager) load() error {
	// Load base variables.json
	data, err := os.ReadFile(m.filePath)
	if err != nil {
		return err
	}

	if err := json.Unmarshal(data, &m.variables); err != nil {
		return err
	}

	// Try to load platform-specific overrides (e.g., variables.darwin.json)
	m.loadPlatformOverrides()

	return nil
}

// loadPlatformOverrides loads platform-specific variable overrides
func (m *VariablesManager) loadPlatformOverrides() {
	// Construct platform-specific filename
	baseDir := filepath.Dir(m.filePath)
	platformFile := filepath.Join(baseDir, fmt.Sprintf("variables.%s.json", runtime.GOOS))

	// Try to read platform-specific file
	data, err := os.ReadFile(platformFile)
	if err != nil {
		return // Platform file doesn't exist or can't be read - that's OK
	}

	// Parse platform overrides
	var overrides map[string]string
	if err := json.Unmarshal(data, &overrides); err != nil {
		return // Invalid JSON - ignore silently
	}

	// Merge overrides into variables (overrides take precedence)
	for k, v := range overrides {
		m.variables[k] = v
	}
}

// save writes variables to JSON file
func (m *VariablesManager) save() error {
	data, err := json.MarshalIndent(m.variables, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal variables: %w", err)
	}

	if err := os.WriteFile(m.filePath, data, 0644); err != nil {
		return fmt.Errorf("failed to write variables file: %w", err)
	}

	return nil
}

// Set stores a variable
func (m *VariablesManager) Set(key, value string) error {
	if key == "" {
		return fmt.Errorf("variable key cannot be empty")
	}

	m.variables[key] = value
	return m.save()
}

// Get retrieves a variable with path resolution
func (m *VariablesManager) Get(key string) (string, error) {
	value, ok := m.variables[key]
	if !ok {
		return "", fmt.Errorf("variable %s not found", key)
	}

	// Resolve any special markers or environment variables
	resolved, err := ResolvePath(value)
	if err != nil {
		return value, nil // Return original if resolution fails
	}

	return resolved, nil
}

// Delete removes a variable
func (m *VariablesManager) Delete(key string) error {
	if _, ok := m.variables[key]; !ok {
		return fmt.Errorf("variable %s not found", key)
	}

	delete(m.variables, key)
	return m.save()
}

// List returns all variable keys
func (m *VariablesManager) List() []string {
	keys := make([]string, 0, len(m.variables))
	for key := range m.variables {
		keys = append(keys, key)
	}
	return keys
}

// GetAll returns all variables with path resolution
func (m *VariablesManager) GetAll() map[string]string {
	result := make(map[string]string, len(m.variables))
	for k, v := range m.variables {
		// Resolve paths for each variable
		resolved, err := ResolvePath(v)
		if err != nil {
			result[k] = v // Use original if resolution fails
		} else {
			result[k] = resolved
		}
	}
	return result
}
