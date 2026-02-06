// Package kombu provides path resolution utilities.
//
// Kombu (昆布 - kelp/seaweed) is a foundational ingredient in Japanese cooking,
// used to make dashi stock. Similarly, this package provides foundational path
// resolution that other packages depend on.
//
// Features:
//   - Cross-platform path resolution
//   - Special marker expansion ({{GDRIVE}}, {{DROPBOX}}, etc.)
//   - Environment variable expansion
//   - Auto-detection of cloud storage paths
package paths

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
)

// ResolvePath expands special markers and environment variables in a path.
// Supports:
//   - {{GDRIVE}} - Google Drive root
//   - {{DROPBOX}} - Dropbox root
//   - {{ONEDRIVE}} - OneDrive root
//   - {{BENTO_HOME}} - Configured bento home
//   - ${VAR} or $VAR - Environment variables
func ResolvePath(path string) (string, error) {
	if path == "" {
		return "", nil
	}

	// Step 1: Expand special markers
	resolved := path
	resolved = expandSpecialMarkers(resolved)

	// Step 2: Expand environment variables
	resolved = os.ExpandEnv(resolved)

	// Step 3: Clean the path
	resolved = filepath.Clean(resolved)

	return resolved, nil
}

// expandSpecialMarkers replaces special markers with platform-specific paths
func expandSpecialMarkers(path string) string {
	// Expand markers in order: BENTO_HOME first, then cloud storage
	if strings.Contains(path, "{{BENTO_HOME}}") {
		path = strings.ReplaceAll(path, "{{BENTO_HOME}}", LoadBentoHome())
	}

	path = replaceMarkerIfDetected(path, "{{GDRIVE}}", DetectGoogleDrive())
	path = replaceMarkerIfDetected(path, "{{DROPBOX}}", DetectDropbox())
	path = replaceMarkerIfDetected(path, "{{ONEDRIVE}}", DetectOneDrive())

	return path
}

// CompressPath converts absolute paths to use special markers for portability.
// This is the inverse of ResolvePath - useful for displaying paths in a platform-independent way.
// Example: "/Users/Ryan/Library/CloudStorage/GoogleDrive-email/My Drive/foo" -> "{{GDRIVE}}/foo"
func CompressPath(path string) string {
	if path == "" {
		return ""
	}

	cleaned := filepath.Clean(path)

	// Try to compress with each marker in order of specificity
	if compressed, ok := tryCompressWithMarker(cleaned, DetectGoogleDrive(), "{{GDRIVE}}"); ok {
		return compressed
	}
	if compressed, ok := tryCompressWithMarker(cleaned, DetectDropbox(), "{{DROPBOX}}"); ok {
		return compressed
	}
	if compressed, ok := tryCompressWithMarker(cleaned, DetectOneDrive(), "{{ONEDRIVE}}"); ok {
		return compressed
	}
	if compressed, ok := tryCompressWithMarker(cleaned, LoadBentoHome(), "{{BENTO_HOME}}"); ok {
		return compressed
	}

	return cleaned
}

// tryCompressWithMarker attempts to compress a path using a marker if it has the base path as prefix
func tryCompressWithMarker(cleaned, basePath, marker string) (string, bool) {
	if basePath == "" {
		return "", false
	}

	cleanedBase := filepath.Clean(basePath)
	if !strings.HasPrefix(cleaned, cleanedBase) {
		return "", false
	}

	rel := strings.TrimPrefix(cleaned, cleanedBase)
	rel = strings.TrimPrefix(rel, string(filepath.Separator))

	if rel == "" {
		return marker, true
	}
	return marker + string(filepath.Separator) + rel, true
}

// ResolvePathsInMap resolves all paths in a string map (useful for variables)
func ResolvePathsInMap(m map[string]string) (map[string]string, error) {
	if m == nil {
		return nil, nil
	}

	resolved := make(map[string]string, len(m))
	for key, value := range m {
		resolvedValue, err := ResolvePath(value)
		if err != nil {
			return nil, fmt.Errorf("failed to resolve path for key %s: %w", key, err)
		}
		resolved[key] = resolvedValue
	}

	return resolved, nil
}

// DetectGoogleDrive attempts to detect the Google Drive root path.
func DetectGoogleDrive() string {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return ""
	}

	switch runtime.GOOS {
	case "darwin":
		return detectGoogleDriveMac(homeDir)
	case "windows":
		return detectGoogleDriveWindows(homeDir)
	case "linux":
		return detectGoogleDriveLinux(homeDir)
	}

	return ""
}

// detectGoogleDriveMac detects Google Drive on macOS
func detectGoogleDriveMac(homeDir string) string {
	// Mac: ~/Library/CloudStorage/GoogleDrive-{email}/My Drive
	cloudStorageDir := filepath.Join(homeDir, "Library", "CloudStorage")
	entries, err := os.ReadDir(cloudStorageDir)
	if err != nil {
		return ""
	}

	for _, entry := range entries {
		if entry.IsDir() && strings.HasPrefix(entry.Name(), "GoogleDrive-") {
			myDrive := filepath.Join(cloudStorageDir, entry.Name(), "My Drive")
			if stat, err := os.Stat(myDrive); err == nil && stat.IsDir() {
				return myDrive
			}
		}
	}
	return ""
}

// detectGoogleDriveWindows detects Google Drive on Windows
func detectGoogleDriveWindows(homeDir string) string {
	// Check common drive letters for "My Drive"
	driveLetters := []string{"G", "H", "I", "J", "K", "L", "M", "N", "O"}
	for _, letter := range driveLetters {
		myDrive := filepath.Join(letter+":\\", "My Drive")
		if stat, err := os.Stat(myDrive); err == nil && stat.IsDir() {
			return myDrive
		}
	}

	// Also check %USERPROFILE%\Google Drive
	googleDrive := filepath.Join(homeDir, "Google Drive")
	if stat, err := os.Stat(googleDrive); err == nil && stat.IsDir() {
		return googleDrive
	}

	return ""
}

// detectGoogleDriveLinux detects Google Drive on Linux
func detectGoogleDriveLinux(homeDir string) string {
	// Linux: ~/Google Drive (older Drive File Stream)
	googleDrive := filepath.Join(homeDir, "Google Drive")
	if stat, err := os.Stat(googleDrive); err == nil && stat.IsDir() {
		return googleDrive
	}
	return ""
}

// DetectDropbox attempts to detect the Dropbox root path.
func DetectDropbox() string {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return ""
	}

	dropboxPath := filepath.Join(homeDir, "Dropbox")
	if stat, err := os.Stat(dropboxPath); err == nil && stat.IsDir() {
		return dropboxPath
	}

	return ""
}

// DetectOneDrive attempts to detect the OneDrive root path.
func DetectOneDrive() string {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return ""
	}

	switch runtime.GOOS {
	case "darwin":
		return detectOneDriveMac(homeDir)
	case "windows", "linux":
		return detectOneDriveDefault(homeDir)
	}

	return ""
}

// detectOneDriveMac detects OneDrive on macOS
func detectOneDriveMac(homeDir string) string {
	// Mac: ~/Library/CloudStorage/OneDrive-{org}
	cloudStorageDir := filepath.Join(homeDir, "Library", "CloudStorage")
	entries, err := os.ReadDir(cloudStorageDir)
	if err != nil {
		return ""
	}

	for _, entry := range entries {
		if entry.IsDir() && strings.HasPrefix(entry.Name(), "OneDrive") {
			return filepath.Join(cloudStorageDir, entry.Name())
		}
	}
	return ""
}

// detectOneDriveDefault detects OneDrive on Windows and Linux
func detectOneDriveDefault(homeDir string) string {
	// Windows and Linux: ~/OneDrive
	oneDrivePath := filepath.Join(homeDir, "OneDrive")
	if stat, err := os.Stat(oneDrivePath); err == nil && stat.IsDir() {
		return oneDrivePath
	}
	return ""
}
