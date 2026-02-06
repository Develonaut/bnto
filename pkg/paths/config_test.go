package paths

import (
	"os"
	"path/filepath"
	"testing"
)

// TestConfigDir verifies config directory path.
func TestConfigDir(t *testing.T) {
	dir, err := configDir()
	if err != nil {
		t.Fatalf("configDir() failed: %v", err)
	}

	home, err := os.UserHomeDir()
	if err != nil {
		t.Fatalf("UserHomeDir() failed: %v", err)
	}

	expected := filepath.Join(home, ".bento", "config")
	if dir != expected {
		t.Errorf("configDir() = %s, want %s", dir, expected)
	}
}

// TestThemeConfigPath verifies theme config file path.
func TestThemeConfigPath(t *testing.T) {
	path, err := themeConfigPath()
	if err != nil {
		t.Fatalf("themeConfigPath() failed: %v", err)
	}

	home, err := os.UserHomeDir()
	if err != nil {
		t.Fatalf("UserHomeDir() failed: %v", err)
	}

	expected := filepath.Join(home, ".bento", "config", "theme")
	if path != expected {
		t.Errorf("themeConfigPath() = %s, want %s", path, expected)
	}
}

// TestSaveAndLoadTheme verifies theme persistence round-trip.
func TestSaveAndLoadTheme(t *testing.T) {
	// Use temp directory to avoid polluting user's actual config
	tmpDir := t.TempDir()
	originalConfigDir := configDir

	// Mock configDir to use temp directory
	configDir = func() (string, error) {
		return tmpDir, nil
	}
	t.Cleanup(func() {
		configDir = originalConfigDir
	})

	// Test themes
	testThemes := []string{"nasu", "wasabi", "toro", "tamago", "tonkotsu", "saba", "ika"}
	for _, theme := range testThemes {
		t.Run(theme, func(t *testing.T) {
			// Save theme
			if err := SaveTheme(theme); err != nil {
				t.Fatalf("SaveTheme(%s) failed: %v", theme, err)
			}

			// Load theme
			loaded := LoadSavedTheme()
			if loaded != theme {
				t.Errorf("LoadSavedTheme() = %s, want %s", loaded, theme)
			}
		})
	}
}

// TestLoadSavedTheme_NoFile verifies default when no theme file exists.
func TestLoadSavedTheme_NoFile(t *testing.T) {
	tmpDir := t.TempDir()
	originalConfigDir := configDir

	configDir = func() (string, error) {
		return tmpDir, nil
	}
	t.Cleanup(func() {
		configDir = originalConfigDir
	})

	// Should return tonkotsu default when no file exists
	theme := LoadSavedTheme()
	if theme != "tonkotsu" {
		t.Errorf("LoadSavedTheme() with no file = %s, want tonkotsu", theme)
	}
}

// TestLoadSavedTheme_InvalidContent verifies that invalid content is returned as-is.
// Note: Validation happens in the miso layer, kombu just reads/writes strings.
func TestLoadSavedTheme_InvalidContent(t *testing.T) {
	tmpDir := t.TempDir()
	originalConfigDir := configDir

	configDir = func() (string, error) {
		return tmpDir, nil
	}
	t.Cleanup(func() {
		configDir = originalConfigDir
	})

	// Write invalid content
	path := filepath.Join(tmpDir, "theme")
	testContent := "InvalidVariant"
	if err := os.WriteFile(path, []byte(testContent), 0644); err != nil {
		t.Fatal(err)
	}

	// Kombu returns raw string - validation happens in miso layer
	theme := LoadSavedTheme()
	if theme != testContent {
		t.Errorf("LoadSavedTheme() = %s, want %s", theme, testContent)
	}
}

// TestSaveTheme_CreatesDirectory verifies directory creation.
func TestSaveTheme_CreatesDirectory(t *testing.T) {
	tmpDir := t.TempDir()
	tmpSubDir := filepath.Join(tmpDir, "nonexistent")
	originalConfigDir := configDir

	configDir = func() (string, error) {
		return tmpSubDir, nil
	}
	t.Cleanup(func() {
		configDir = originalConfigDir
	})

	// Directory should not exist yet
	if _, err := os.Stat(tmpSubDir); err == nil {
		t.Fatal("Directory should not exist yet")
	}

	// SaveTheme should create it
	if err := SaveTheme("nasu"); err != nil {
		t.Fatalf("SaveTheme() failed: %v", err)
	}

	// Directory should now exist
	if _, err := os.Stat(tmpSubDir); err != nil {
		t.Fatalf("Directory was not created: %v", err)
	}
}

// TestVerboseLogging verifies verbose logging persistence.
func TestVerboseLogging(t *testing.T) {
	tmpDir := t.TempDir()
	originalConfigDir := configDir

	configDir = func() (string, error) {
		return tmpDir, nil
	}
	t.Cleanup(func() {
		configDir = originalConfigDir
	})

	// Test saving and loading true
	if err := SaveVerboseLogging(true); err != nil {
		t.Fatalf("SaveVerboseLogging(true) failed: %v", err)
	}
	if loaded := LoadVerboseLogging(); !loaded {
		t.Errorf("LoadVerboseLogging() = false, want true")
	}

	// Test saving and loading false
	if err := SaveVerboseLogging(false); err != nil {
		t.Fatalf("SaveVerboseLogging(false) failed: %v", err)
	}
	if loaded := LoadVerboseLogging(); loaded {
		t.Errorf("LoadVerboseLogging() = true, want false")
	}
}

// TestVerboseLogging_NoFile verifies default when no file exists.
func TestVerboseLogging_NoFile(t *testing.T) {
	tmpDir := t.TempDir()
	originalConfigDir := configDir

	configDir = func() (string, error) {
		return tmpDir, nil
	}
	t.Cleanup(func() {
		configDir = originalConfigDir
	})

	// Should return false default when no file exists
	if loaded := LoadVerboseLogging(); loaded {
		t.Errorf("LoadVerboseLogging() with no file = true, want false")
	}
}
