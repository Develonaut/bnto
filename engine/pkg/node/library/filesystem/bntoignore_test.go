package filesystem

import (
	"os"
	"path/filepath"
	"testing"
)

func TestBntoIgnorePatterns(t *testing.T) {
	tests := []struct {
		name     string
		patterns []string
		path     string
		expected bool
	}{
		{
			name:     "exact filename match",
			patterns: []string{"config.json"},
			path:     "/some/dir/config.json",
			expected: true,
		},
		{
			name:     "glob pattern match",
			patterns: []string{"*.log"},
			path:     "/some/dir/debug.log",
			expected: true,
		},
		{
			name:     "directory pattern",
			patterns: []string{"node_modules"},
			path:     "/project/node_modules/package/file.js",
			expected: true,
		},
		{
			name:     "path pattern",
			patterns: []string{"build/*"},
			path:     "build/output.txt",
			expected: true,
		},
		{
			name:     "no match",
			patterns: []string{"*.log", "temp*"},
			path:     "/project/source.go",
			expected: false,
		},
		{
			name:     "empty patterns",
			patterns: []string{},
			path:     "/any/file.txt",
			expected: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			bi := &BntoIgnore{patterns: tt.patterns}
			result := bi.ShouldIgnore(tt.path)
			if result != tt.expected {
				t.Errorf("ShouldIgnore(%q) = %v, expected %v", tt.path, result, tt.expected)
			}
		})
	}
}

func TestLoadBntoIgnore(t *testing.T) {
	// Create a temporary directory
	tmpDir, err := os.MkdirTemp("", "bntoignore-test-*")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	// Create a .bntoignore file
	ignoreContent := `# Comment line
*.log
temp*

# Another comment
node_modules
`
	ignoreFile := filepath.Join(tmpDir, ".bntoignore")
	if err := os.WriteFile(ignoreFile, []byte(ignoreContent), 0644); err != nil {
		t.Fatalf("Failed to write .bntoignore: %v", err)
	}

	// Load and verify
	bi, err := LoadBntoIgnore(tmpDir)
	if err != nil {
		t.Fatalf("LoadBntoIgnore failed: %v", err)
	}

	expected := 3 // *.log, temp*, node_modules (comments and empty lines should be skipped)
	if len(bi.patterns) != expected {
		t.Errorf("Expected %d patterns, got %d", expected, len(bi.patterns))
	}

	// Verify patterns were parsed correctly
	expectedPatterns := map[string]bool{
		"*.log":        true,
		"temp*":        true,
		"node_modules": true,
	}

	for _, pattern := range bi.patterns {
		if !expectedPatterns[pattern] {
			t.Errorf("Unexpected pattern: %q", pattern)
		}
	}
}

func TestLoadBntoIgnoreNotFound(t *testing.T) {
	// Create a temporary directory without .bntoignore
	tmpDir, err := os.MkdirTemp("", "bntoignore-test-*")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	// Should return empty ignore list without error
	bi, err := LoadBntoIgnore(tmpDir)
	if err != nil {
		t.Fatalf("LoadBntoIgnore should not error when file doesn't exist: %v", err)
	}

	if len(bi.patterns) != 0 {
		t.Errorf("Expected 0 patterns when .bntoignore doesn't exist, got %d", len(bi.patterns))
	}
}
