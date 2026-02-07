// Package paths provides configuration management and persistence for bnto.
//
// This package manages:
//   - Bnto home directory configuration
//   - Theme/variant persistence
//   - Verbose logging settings
//   - Slow-motion execution delay
//   - Save directory preferences
//
// All configuration is stored in ~/.bnto/config/ with individual files
// per setting for simplicity and atomicity.
package paths

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// configDir returns the bnto config directory path.
// Mutable var allows mocking in tests.
var configDir = func() (string, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	// Store config in ~/.bnto/config/ subdirectory for consistency with storage structure
	return filepath.Join(home, ".bnto", "config"), nil
}

// bntoHomeConfigPath returns the path to the bnto home config file.
func bntoHomeConfigPath() (string, error) {
	dir, err := configDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(dir, "bntohome"), nil
}

// LoadBntoHome loads the configured bnto home directory from disk.
// Returns the default ~/.bnto if no custom home is configured.
// Automatically resolves {{GDRIVE}} and other special markers.
func LoadBntoHome() string {
	path, err := bntoHomeConfigPath()
	if err != nil {
		return defaultBntoHome()
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return defaultBntoHome()
	}

	dir := strings.TrimSpace(string(data))
	if dir == "" {
		return defaultBntoHome()
	}

	return resolveCloudMarkers(dir)
}

// resolveCloudMarkers resolves {{GDRIVE}}, {{DROPBOX}}, {{ONEDRIVE}} in path.
// Note: Can't call ResolvePath here due to circular dependency
// (ResolvePath → LoadBntoHome → ResolvePath).
// We safely resolve cloud markers without recursion.
func resolveCloudMarkers(path string) string {
	resolved := path
	resolved = replaceMarkerIfDetected(resolved, "{{GDRIVE}}", DetectGoogleDrive())
	resolved = replaceMarkerIfDetected(resolved, "{{DROPBOX}}", DetectDropbox())
	resolved = replaceMarkerIfDetected(resolved, "{{ONEDRIVE}}", DetectOneDrive())
	return filepath.Clean(resolved)
}

// replaceMarkerIfDetected replaces marker in path if cloudPath is detected
func replaceMarkerIfDetected(path, marker, cloudPath string) string {
	if cloudPath != "" && strings.Contains(path, marker) {
		return strings.ReplaceAll(path, marker, cloudPath)
	}
	return path
}

// SaveBntoHome saves the bnto home directory to disk.
// Creates ~/.bnto directory if it doesn't exist.
func SaveBntoHome(dir string) error {
	confDir, err := configDir()
	if err != nil {
		return err
	}

	// Create config directory if needed
	if err := os.MkdirAll(confDir, 0755); err != nil {
		return err
	}

	path, err := bntoHomeConfigPath()
	if err != nil {
		return err
	}

	return os.WriteFile(path, []byte(dir), 0644)
}

// defaultBntoHome returns the default bnto home directory.
func defaultBntoHome() string {
	home, err := os.UserHomeDir()
	if err != nil {
		return "./.bnto"
	}
	return filepath.Join(home, ".bnto")
}

// themeConfigPath returns the path to the theme config file.
func themeConfigPath() (string, error) {
	dir, err := configDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(dir, "theme"), nil
}

// LoadSavedTheme loads the saved theme variant from disk.
// Returns "tonkotsu" (creamy white) as default if no saved theme or on error.
func LoadSavedTheme() string {
	path, err := themeConfigPath()
	if err != nil {
		return "tonkotsu"
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return "tonkotsu"
	}

	return strings.TrimSpace(string(data))
}

// SaveTheme saves the theme variant to disk.
// Creates ~/.bnto directory if it doesn't exist.
func SaveTheme(variant string) error {
	dir, err := configDir()
	if err != nil {
		return err
	}

	// Create config directory if needed
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	path, err := themeConfigPath()
	if err != nil {
		return err
	}

	return os.WriteFile(path, []byte(variant), 0644)
}

// slowMoConfigPath returns the path to the slowMo config file.
func slowMoConfigPath() (string, error) {
	dir, err := configDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(dir, "slowmo"), nil
}

// LoadSlowMoDelay loads the saved slowMo delay from disk.
// Returns 5000ms as default if no saved value or on error.
// SlowMo adds artificial delays between node executions to make animations visible.
func LoadSlowMoDelay() int {
	path, err := slowMoConfigPath()
	if err != nil {
		return 5000
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return 5000
	}

	value := strings.TrimSpace(string(data))

	// Parse as milliseconds
	var ms int
	_, err = fmt.Sscanf(value, "%d", &ms)
	if err != nil || ms < 0 {
		return 5000
	}

	return ms
}

// SaveSlowMoDelay saves the slowMo delay (in milliseconds) to disk.
// Creates ~/.bnto directory if it doesn't exist.
func SaveSlowMoDelay(ms int) error {
	dir, err := configDir()
	if err != nil {
		return err
	}

	// Create config directory if needed
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	path, err := slowMoConfigPath()
	if err != nil {
		return err
	}

	return os.WriteFile(path, []byte(fmt.Sprintf("%d", ms)), 0644)
}

// saveDirConfigPath returns the path to the save directory config file.
func saveDirConfigPath() (string, error) {
	dir, err := configDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(dir, "savedir"), nil
}

// LoadSaveDirectory loads the saved bntos directory from disk.
// Returns ~/.bnto as default if no saved value or on error.
func LoadSaveDirectory() string {
	path, err := saveDirConfigPath()
	if err != nil {
		return defaultSaveDir()
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return defaultSaveDir()
	}

	dir := strings.TrimSpace(string(data))
	if dir == "" {
		return defaultSaveDir()
	}

	return dir
}

// SaveSaveDirectory saves the bntos directory to disk.
// Creates ~/.bnto directory if it doesn't exist.
func SaveSaveDirectory(dir string) error {
	confDir, err := configDir()
	if err != nil {
		return err
	}

	// Create config directory if needed
	if err := os.MkdirAll(confDir, 0755); err != nil {
		return err
	}

	path, err := saveDirConfigPath()
	if err != nil {
		return err
	}

	return os.WriteFile(path, []byte(dir), 0644)
}

// defaultSaveDir returns the default save directory.
func defaultSaveDir() string {
	home, err := os.UserHomeDir()
	if err != nil {
		return "./.bnto"
	}
	return filepath.Join(home, ".bnto")
}

// verboseConfigPath returns the path to the verbose config file.
func verboseConfigPath() (string, error) {
	dir, err := configDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(dir, "verbose"), nil
}

// LoadVerboseLogging loads the verbose logging setting from disk.
// Returns false as default if no saved value or on error.
func LoadVerboseLogging() bool {
	path, err := verboseConfigPath()
	if err != nil {
		return false
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return false
	}

	value := strings.TrimSpace(string(data))
	return value == "true"
}

// SaveVerboseLogging saves the verbose logging setting to disk.
// Creates ~/.bnto directory if it doesn't exist.
func SaveVerboseLogging(enabled bool) error {
	dir, err := configDir()
	if err != nil {
		return err
	}

	// Create config directory if needed
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	path, err := verboseConfigPath()
	if err != nil {
		return err
	}

	value := "false"
	if enabled {
		value = "true"
	}

	return os.WriteFile(path, []byte(value), 0644)
}
