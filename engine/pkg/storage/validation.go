package storage

import (
	"fmt"
	"path/filepath"
	"strings"
)

// validateName ensures the workflow name is safe for filesystem use.
//
// This prevents directory traversal attacks and reserved names.
func validateName(name string) error {
	if name == "" {
		return fmt.Errorf("workflow name cannot be empty")
	}

	if err := checkTraversal(name); err != nil {
		return err
	}

	if err := checkSeparators(name); err != nil {
		return err
	}

	if err := checkReserved(name); err != nil {
		return err
	}

	if err := checkClean(name); err != nil {
		return err
	}

	return nil
}

// checkTraversal checks for directory traversal attempts.
func checkTraversal(name string) error {
	if strings.Contains(name, "..") {
		return fmt.Errorf("workflow name cannot contain '..'")
	}
	return nil
}

// checkSeparators checks for path separators.
func checkSeparators(name string) error {
	if strings.Contains(name, "/") || strings.Contains(name, "\\") {
		return fmt.Errorf("workflow name cannot contain path separators")
	}
	return nil
}

// checkReserved checks for reserved Windows names.
func checkReserved(name string) error {
	reserved := []string{"CON", "PRN", "AUX", "NUL", "COM1", "LPT1"}
	upper := strings.ToUpper(name)
	for _, r := range reserved {
		if upper == r {
			return fmt.Errorf("workflow name '%s' is reserved", name)
		}
	}
	return nil
}

// checkClean ensures the path is clean (no sneaky characters).
func checkClean(name string) error {
	if filepath.Clean(name) != name {
		return fmt.Errorf("workflow name contains invalid characters")
	}
	return nil
}
