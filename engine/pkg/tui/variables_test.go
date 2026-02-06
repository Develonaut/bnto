package tui

import (
	"testing"
)

// TestExtractVariables tests variable extraction from bento JSON.
func TestExtractVariables(t *testing.T) {
	tests := []struct {
		name      string
		bentoJSON string
		wantVars  []string
	}{
		{
			name:      "no variables",
			bentoJSON: `{"id": "test", "name": "Test"}`,
			wantVars:  []string{},
		},
		{
			name:      "single variable",
			bentoJSON: `{"path": "{{.PRODUCT_PATH}}/model.stl"}`,
			wantVars:  []string{"PRODUCT_PATH"},
		},
		{
			name:      "multiple variables",
			bentoJSON: `{"path": "{{.PRODUCT_PATH}}/model.stl", "theme": "{{.RENDER_THEME}}"}`,
			wantVars:  []string{"PRODUCT_PATH", "RENDER_THEME"},
		},
		{
			name:      "duplicate variables",
			bentoJSON: `{"path1": "{{.PRODUCT_PATH}}", "path2": "{{.PRODUCT_PATH}}"}`,
			wantVars:  []string{"PRODUCT_PATH"},
		},
		{
			name:      "mixed case ignored",
			bentoJSON: `{"path": "{{.ProductPath}}"}`,
			wantVars:  []string{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			vars := ExtractVariables([]byte(tt.bentoJSON))

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

// TestParseBentoMetadata tests full metadata extraction.
func TestParseBentoMetadata(t *testing.T) {
	bentoJSON := `{
		"name": "Test Bento",
		"metadata": {
			"description": "A test bento"
		},
		"parameters": {
			"path": "{{.PRODUCT_PATH}}/model.stl",
			"theme": "{{.RENDER_THEME}}"
		}
	}`

	meta, err := ParseBentoMetadata([]byte(bentoJSON))
	if err != nil {
		t.Fatalf("ParseBentoMetadata failed: %v", err)
	}

	if meta.Name != "Test Bento" {
		t.Errorf("Expected name 'Test Bento', got '%s'", meta.Name)
	}

	if meta.Description != "A test bento" {
		t.Errorf("Expected description 'A test bento', got '%s'", meta.Description)
	}

	if len(meta.Variables) != 2 {
		t.Errorf("Expected 2 variables, got %d", len(meta.Variables))
	}
}

// TestParseBentoMetadataInvalidJSON tests error handling.
func TestParseBentoMetadataInvalidJSON(t *testing.T) {
	_, err := ParseBentoMetadata([]byte("invalid json"))
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
