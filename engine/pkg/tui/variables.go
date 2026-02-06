package tui

import (
	"encoding/json"
	"regexp"
	"strings"

	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

// Variable represents a template variable found in a bento.
type Variable struct {
	Name         string   `json:"name"`
	Description  string   `json:"description,omitempty"`
	DefaultValue string   `json:"defaultValue,omitempty"`
	Type         string   `json:"type,omitempty"`
	Options      []string `json:"options,omitempty"`
}

// ExtractVariables finds all {{.VARIABLE}} placeholders in a bento JSON.
// Returns a deduplicated list of variable names.
func ExtractVariables(bentoJSON []byte) []Variable {
	// Regex to match {{.VARIABLE}} or {{.VARIABLE.field}}
	re := regexp.MustCompile(`\{\{\.([A-Z_][A-Z0-9_]*)\}\}`)

	matches := re.FindAllStringSubmatch(string(bentoJSON), -1)

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

// BentoMetadata contains extracted metadata from a bento file.
type BentoMetadata struct {
	Name        string     `json:"name"`
	Description string     `json:"description"`
	Variables   []Variable `json:"-"` // Not part of JSON, computed
}

// ParseBentoMetadata extracts name, description, and variables from bento JSON.
func ParseBentoMetadata(bentoJSON []byte) (*BentoMetadata, error) {
	// First parse the full bento structure to get both metadata and defined variables
	type bentoMeta struct {
		Name     string `json:"name"`
		Metadata struct {
			Description string `json:"description"`
		} `json:"metadata"`
		Variables []Variable `json:"variables"`
	}

	var meta bentoMeta
	if err := json.Unmarshal(bentoJSON, &meta); err != nil {
		return nil, err
	}

	// If bento has defined variables, use those (they may include type and options)
	// Otherwise, extract variables from template placeholders
	vars := meta.Variables
	if len(vars) == 0 {
		vars = ExtractVariables(bentoJSON)
	}

	return &BentoMetadata{
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
