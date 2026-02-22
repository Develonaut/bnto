//go:build tui

package tui

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/charmbracelet/huh"
)

// buildField creates a Huh input field for a variable.
// Uses file picker for path variables, text input for others.
func buildField(v Variable, valueHolder *string) huh.Field {
	return buildFieldWithHeight(v, valueHolder, 0)
}

// loadVariableDefault loads the default value for a variable from saved config.
func loadVariableDefault(v Variable) string {
	if v.DefaultValue != "" {
		return v.DefaultValue
	}

	if mgr, err := NewVariablesManager(); err == nil {
		if val, err := mgr.Get(v.Name); err == nil && val != "" {
			return val
		}
	}

	return ""
}

// buildTextInputField creates a text input field for a variable.
func buildTextInputField(title string, defaultValue string, valueHolder *string) huh.Field {
	if defaultValue != "" {
		*valueHolder = defaultValue
	}

	placeholder := defaultValue
	if placeholder == "" {
		placeholder = fmt.Sprintf("Enter %s", title)
	}

	return huh.NewInput().
		Title(title).
		Placeholder(placeholder).
		Value(valueHolder)
}

// buildFieldWithHeight creates a Huh input field with custom height for file pickers.
func buildFieldWithHeight(v Variable, valueHolder *string, terminalHeight int) huh.Field {
	title := FormatVariableName(v.Name)

	// Check if this is a select field
	if v.Type == "select" && len(v.Options) > 0 {
		return buildSelectField(v, title, valueHolder)
	}

	// Check if this is a path/directory variable
	if isPathVariable(v.Name) {
		return buildFilePickerFieldWithHeight(v, title, valueHolder, terminalHeight)
	}

	// Regular text input for non-path variables
	defaultValue := loadVariableDefault(v)
	return buildTextInputField(title, defaultValue, valueHolder)
}

// formatSelectOptions converts option strings to huh.Option format.
// Options can be in format "value|label" or just "value".
func formatSelectOptions(optionStrings []string) []huh.Option[string] {
	options := make([]huh.Option[string], len(optionStrings))
	for i, opt := range optionStrings {
		parts := strings.Split(opt, "|")
		if len(parts) == 2 {
			// Format: "value|label"
			options[i] = huh.NewOption(parts[1], parts[0])
		} else {
			// Format: "value" (use value as label)
			options[i] = huh.NewOption(opt, opt)
		}
	}
	return options
}

// buildSelectField creates a select dropdown field for variables with predefined options.
func buildSelectField(v Variable, title string, valueHolder *string) huh.Field {
	// Load default value
	defaultValue := loadVariableDefault(v)
	if defaultValue != "" {
		*valueHolder = defaultValue
	}

	// Format options
	options := formatSelectOptions(v.Options)

	// Create field
	field := huh.NewSelect[string]().
		Title(title).
		Options(options...).
		Height(10).
		Value(valueHolder)

	// Add description if available
	if v.Description != "" {
		field = field.Description(v.Description)
	}

	return field
}

// loadSavedPathVariable loads a path from saved variables.
// Tries exact match first, then falls back to PRODUCTS_URL for product paths.
func loadSavedPathVariable(varName string, isFile bool) string {
	mgr, err := NewVariablesManager()
	if err != nil {
		return ""
	}

	// First try exact match
	if val, err := mgr.Get(varName); err == nil && val != "" {
		return val
	}

	// For product-related paths or files, try PRODUCTS_URL fallback
	if isProductPath(varName) || isFile {
		if val, err := mgr.Get("PRODUCTS_URL"); err == nil && val != "" {
			return val
		}
	}

	return ""
}

// extractDirectoryPath converts a file path to its directory.
// If path is already a directory, returns it unchanged.
func extractDirectoryPath(path string, isFile bool) string {
	if !isFile || path == "" {
		return path
	}

	// Check if the path looks like a file (has an extension)
	if filepath.Ext(path) != "" {
		return filepath.Dir(path)
	}

	return path
}

// determineStartDirectory finds the best starting directory for file picker.
// Priority: 1) Default value, 2) Saved variable, 3) PRODUCTS_URL, 4) Home dir.
func determineStartDirectory(v Variable, isFile bool) string {
	startDir := v.DefaultValue

	if startDir == "" {
		startDir = loadSavedPathVariable(v.Name, isFile)
	}

	startDir = extractDirectoryPath(startDir, isFile)

	// Fall back to home directory
	if startDir == "" {
		if home, err := os.UserHomeDir(); err == nil {
			return home
		}
		return "."
	}

	return startDir
}

// calculatePickerHeight determines the appropriate height for file picker.
func calculatePickerHeight(terminalHeight int, isFile bool) int {
	if terminalHeight > 0 {
		// Leave room for title, description, help text, and chrome (~8 lines)
		height := terminalHeight - 8
		if height < 5 {
			height = 5 // Minimum usable height
		}

		// For directories, use a bit less space; files need more browsing area
		if !isFile && height > 10 {
			height = height / 2 // Directories get half the available space
			if height < 8 {
				height = 8
			}
		}
		return height
	}

	// Fallback to reasonable defaults when terminal height not available
	if isFile {
		return 15 // Taller for file selection
	}
	return 8 // Default for directories
}

// buildFilePickerFieldWithHeight creates a file picker field for path variables.
func buildFilePickerFieldWithHeight(v Variable, title string, valueHolder *string, terminalHeight int) huh.Field {
	isFile := isFilePath(v.Name)
	startDir := determineStartDirectory(v, isFile)
	height := calculatePickerHeight(terminalHeight, isFile)

	// Set the initial value to the starting directory
	*valueHolder = startDir

	description := "Browse to select a directory"
	if isFile {
		description = "Browse to select a file"
	}

	return huh.NewFilePicker().
		Title(title).
		Description(description).
		CurrentDirectory(startDir).
		DirAllowed(!isFile).
		FileAllowed(isFile).
		ShowHidden(false).
		ShowSize(true).
		Height(height).
		Value(valueHolder)
}

// isPathVariable checks if a variable name indicates it's a file/directory path.
func isPathVariable(name string) bool {
	upperName := strings.ToUpper(name)
	return strings.Contains(upperName, "PATH") ||
		strings.Contains(upperName, "DIR") ||
		strings.Contains(upperName, "DIRECTORY") ||
		strings.Contains(upperName, "FOLDER")
}

// isFilePath checks if a variable name indicates it's a file (not directory) path.
func isFilePath(name string) bool {
	upperName := strings.ToUpper(name)
	// CSV_PATH is treated as directory (will look for products.csv inside)
	// Other file types are actual file selections
	return strings.Contains(upperName, "FILE") ||
		strings.Contains(upperName, "JSON") ||
		strings.Contains(upperName, "YAML") ||
		strings.Contains(upperName, "XML")
}

// isProductPath checks if a variable name indicates it's a product-related path.
func isProductPath(name string) bool {
	upperName := strings.ToUpper(name)
	return strings.Contains(upperName, "PRODUCT") ||
		strings.Contains(upperName, "PARENT") ||
		strings.Contains(upperName, "TARGET") ||
		strings.Contains(upperName, "SOURCE") ||
		strings.Contains(upperName, "CSV")
}
