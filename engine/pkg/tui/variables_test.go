package tui

import (
	"testing"
)

// TestExtractVariables tests variable extraction from workflow JSON.
func TestExtractVariables(t *testing.T) {
	tests := []struct {
		name     string
		bntoJSON string
		wantVars []string
	}{
		{
			name:     "no variables",
			bntoJSON: `{"id": "test", "name": "Test"}`,
			wantVars: []string{},
		},
		{
			name:     "single variable",
			bntoJSON: `{"path": "{{.PRODUCT_PATH}}/model.stl"}`,
			wantVars: []string{"PRODUCT_PATH"},
		},
		{
			name:     "multiple variables",
			bntoJSON: `{"path": "{{.PRODUCT_PATH}}/model.stl", "theme": "{{.RENDER_THEME}}"}`,
			wantVars: []string{"PRODUCT_PATH", "RENDER_THEME"},
		},
		{
			name:     "duplicate variables",
			bntoJSON: `{"path1": "{{.PRODUCT_PATH}}", "path2": "{{.PRODUCT_PATH}}"}`,
			wantVars: []string{"PRODUCT_PATH"},
		},
		{
			name:     "mixed case ignored",
			bntoJSON: `{"path": "{{.ProductPath}}"}`,
			wantVars: []string{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			vars := ExtractVariables([]byte(tt.bntoJSON))

			if len(vars) != len(tt.wantVars) {
				t.Errorf("Expected %d variables, got %d", len(tt.wantVars), len(vars))
				return
			}

			// Check each expected variable is found
			varMap := make(map[string]bool)
			for _, v := range vars {
				varMap[v.Name] = true
			}

			for _, want := range tt.wantVars {
				if !varMap[want] {
					t.Errorf("Expected variable %s not found", want)
				}
			}
		})
	}
}

// TestGetDefaultValue tests default value suggestions.
func TestGetDefaultValue(t *testing.T) {
	tests := []struct {
		name    string
		varName string
		want    string
	}{
		{"render theme has default", "RENDER_THEME", "wasteland_blaze"},
		{"product path empty", "PRODUCT_PATH", ""},
		{"zoom multiplier has default", "ZOOM_MULTIPLIER", "1.5"},
		{"unknown var empty", "UNKNOWN_VAR", ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := getDefaultValue(tt.varName)
			if got != tt.want {
				t.Errorf("Expected default '%s', got '%s'", tt.want, got)
			}
		})
	}
}

// TestFormatVariableName tests variable name formatting.
func TestFormatVariableName(t *testing.T) {
	tests := []struct {
		input string
		want  string
	}{
		{"PRODUCT_PATH", "Product Path"},
		{"RENDER_THEME", "Render Theme"},
		{"API_KEY", "Api Key"},
		{"MY_LONG_VARIABLE_NAME", "My Long Variable Name"},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			got := FormatVariableName(tt.input)
			if got != tt.want {
				t.Errorf("Expected '%s', got '%s'", tt.want, got)
			}
		})
	}
}

// TestParseBntoMetadata tests full metadata extraction.
func TestParseBntoMetadata(t *testing.T) {
	bntoJSON := `{
		"name": "Test Workflow",
		"metadata": {
			"description": "A test workflow"
		},
		"parameters": {
			"path": "{{.PRODUCT_PATH}}/model.stl",
			"theme": "{{.RENDER_THEME}}"
		}
	}`

	meta, err := ParseBntoMetadata([]byte(bntoJSON))
	if err != nil {
		t.Fatalf("ParseBntoMetadata failed: %v", err)
	}

	if meta.Name != "Test Workflow" {
		t.Errorf("Expected name 'Test Workflow', got '%s'", meta.Name)
	}

	if meta.Description != "A test workflow" {
		t.Errorf("Expected description 'A test workflow', got '%s'", meta.Description)
	}

	if len(meta.Variables) != 2 {
		t.Errorf("Expected 2 variables, got %d", len(meta.Variables))
	}
}

// TestParseBntoMetadataInvalidJSON tests error handling.
func TestParseBntoMetadataInvalidJSON(t *testing.T) {
	_, err := ParseBntoMetadata([]byte("invalid json"))
	if err == nil {
		t.Error("Expected error for invalid JSON")
	}
}

// TestIsPathVariable tests path variable detection.
func TestIsPathVariable(t *testing.T) {
	tests := []struct {
		name     string
		varName  string
		wantPath bool
	}{
		{"PRODUCT_PATH is path", "PRODUCT_PATH", true},
		{"OUTPUT_DIR is path", "OUTPUT_DIR", true},
		{"BASE_DIRECTORY is path", "BASE_DIRECTORY", true},
		{"WORK_FOLDER is path", "WORK_FOLDER", true},
		{"RENDER_THEME is not path", "RENDER_THEME", false},
		{"API_KEY is not path", "API_KEY", false},
		{"CONFIG_NAME is not path", "CONFIG_NAME", false},
		{"lowercase path works", "product_path", true},
		{"mixed case path works", "Product_Path", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := isPathVariable(tt.varName)
			if got != tt.wantPath {
				t.Errorf("isPathVariable(%s) = %v, want %v", tt.varName, got, tt.wantPath)
			}
		})
	}
}
