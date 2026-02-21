package tui

import (
	"encoding/json"
	"regexp"
	"strings"

	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

// Variable represents a template variable found in a workflow.
type Variable struct {
	Name         string   `json:"name"`
	Description  string   `json:"description,omitempty"`
	DefaultValue string   `json:"defaultValue,omitempty"`
	Type         string   `json:"type,omitempty"`
	Options      []string `json:"options,omitempty"`
}

// ExtractVariables finds all {{.VARIABLE}} placeholders in a workflow JSON.
// Returns a deduplicated list of variable names.
func ExtractVariables(bntoJSON []byte) []Variable {
	// Regex to match {{.VARIABLE}} or {{.VARIABLE.field}}
	re := regexp.MustCompile(`\{\{\.([A-Z_][A-Z0-9_]*)\}\}`)

	matches := re.FindAllStringSubmatch(string(bntoJSON), -1)

	// Use map to deduplicate
	varMap := make(map[string]bool)
	for _, match := range matches {
		if len(match) > 1 {
			varMap[match[1]] = true
		}
	}

	// Convert to sorted list
	vars := make([]Variable, 0, len(varMap))
	for name := range varMap {
		vars = append(vars, Variable{
			Name:         name,
			DefaultValue: getDefaultValue(name),
		})
	}

	return vars
}

// getDefaultValue returns a sensible default for common variable names.
func getDefaultValue(name string) string {
	defaults := map[string]string{
		"RENDER_THEME":    "wasteland_blaze",
		"PRODUCT_PATH":    "",
		"ZOOM_MULTIPLIER": "1.5",
		"OFFSET_MODIFIER": "0.0",
	}

	if val, ok := defaults[name]; ok {
		return val
	}
	return ""
}

// BntoMetadata contains extracted metadata from a workflow file.
type BntoMetadata struct {
	Name        string     `json:"name"`
	Description string     `json:"description"`
	Variables   []Variable `json:"-"` // Not part of JSON, computed
}

// ParseBntoMetadata extracts name, description, and variables from workflow JSON.
func ParseBntoMetadata(bntoJSON []byte) (*BntoMetadata, error) {
	// First parse the full workflow structure to get both metadata and defined variables
	type bntoMeta struct {
		Name     string `json:"name"`
		Metadata struct {
			Description string `json:"description"`
		} `json:"metadata"`
		Variables []Variable `json:"variables"`
	}

	var meta bntoMeta
	if err := json.Unmarshal(bntoJSON, &meta); err != nil {
		return nil, err
	}

	// If workflow has defined variables, use those (they may include type and options)
	// Otherwise, extract variables from template placeholders
	vars := meta.Variables
	if len(vars) == 0 {
		vars = ExtractVariables(bntoJSON)
	}

	return &BntoMetadata{
		Name:        meta.Name,
		Description: meta.Metadata.Description,
		Variables:   vars,
	}, nil
}

// FormatVariableName formats a variable name for display.
// PRODUCT_PATH -> Product Path
func FormatVariableName(name string) string {
	// Replace underscores with spaces
	formatted := strings.ReplaceAll(name, "_", " ")

	// Title case using golang.org/x/text/cases
	caser := cases.Title(language.English)
	return caser.String(strings.ToLower(formatted))
}
