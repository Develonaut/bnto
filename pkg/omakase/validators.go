package omakase

import (
	"fmt"

	"github.com/Develonaut/bento/pkg/neta"
)

// validateHTTPRequest validates http-request neta parameters.
func validateHTTPRequest(def *neta.Definition) error {
	// Check URL parameter
	url, ok := def.Parameters["url"].(string)
	if !ok || url == "" {
		return fmt.Errorf("http-request neta '%s' missing required parameter 'url'", def.ID)
	}

	// Check method parameter
	method, ok := def.Parameters["method"].(string)
	if !ok || method == "" {
		return fmt.Errorf("http-request neta '%s' missing required parameter 'method'", def.ID)
	}

	// Validate HTTP method
	if !isValidHTTPMethod(method) {
		return fmt.Errorf("http-request neta '%s' has invalid method '%s' (must be GET, POST, PUT, PATCH, DELETE, HEAD, or OPTIONS)",
			def.ID, method)
	}

	return nil
}

// validateLoop validates loop neta parameters.
func validateLoop(def *neta.Definition) error {
	mode, ok := def.Parameters["mode"].(string)
	if !ok || mode == "" {
		return fmt.Errorf("loop neta '%s' missing required parameter 'mode'", def.ID)
	}

	if !isValidLoopMode(mode) {
		return fmt.Errorf("loop neta '%s' has invalid mode '%s' (must be forEach, times, or while)",
			def.ID, mode)
	}

	return validateLoopModeParams(def, mode)
}

// validateLoopModeParams validates mode-specific parameters for loop neta.
func validateLoopModeParams(def *neta.Definition, mode string) error {
	switch mode {
	case "forEach":
		if def.Parameters["items"] == nil {
			return fmt.Errorf("loop neta '%s' with mode 'forEach' missing required parameter 'items'", def.ID)
		}
	case "times":
		if def.Parameters["count"] == nil {
			return fmt.Errorf("loop neta '%s' with mode 'times' missing required parameter 'count'", def.ID)
		}
	case "while":
		if def.Parameters["condition"] == nil {
			return fmt.Errorf("loop neta '%s' with mode 'while' missing required parameter 'condition'", def.ID)
		}
	}
	return nil
}

// validateFileSystem validates file-system neta parameters.
func validateFileSystem(def *neta.Definition) error {
	// Check operation parameter
	operation, ok := def.Parameters["operation"].(string)
	if !ok || operation == "" {
		return fmt.Errorf("file-system neta '%s' missing required parameter 'operation'", def.ID)
	}

	// Validate operation value
	if !isValidFileOperation(operation) {
		return fmt.Errorf("file-system neta '%s' has invalid operation '%s' (must be read, write, copy, move, delete, mkdir, exists, or list)",
			def.ID, operation)
	}

	return nil
}

// validateShellCommand validates shell-command neta parameters.
func validateShellCommand(def *neta.Definition) error {
	// Check command parameter
	command, ok := def.Parameters["command"].(string)
	if !ok || command == "" {
		return fmt.Errorf("shell-command neta '%s' missing required parameter 'command'", def.ID)
	}

	return nil
}

// validateEditFields validates edit-fields neta parameters.
func validateEditFields(def *neta.Definition) error {
	// Edit-fields should have a values parameter
	if def.Parameters["values"] == nil {
		return fmt.Errorf("edit-fields neta '%s' missing required parameter 'values'", def.ID)
	}

	return nil
}

// validateParallel validates parallel neta parameters.
func validateParallel(def *neta.Definition) error {
	// Parallel neta are similar to groups
	// No specific required parameters
	return nil
}

// validateSpreadsheet validates spreadsheet neta parameters.
func validateSpreadsheet(def *neta.Definition) error {
	// Future implementation - no validation yet
	return nil
}

// validateImage validates image neta parameters.
func validateImage(def *neta.Definition) error {
	// Future implementation - no validation yet
	return nil
}

// validateTransform validates transform neta parameters.
func validateTransform(def *neta.Definition) error {
	// Future implementation - no validation yet
	return nil
}

// isValidHTTPMethod checks if the HTTP method is valid.
func isValidHTTPMethod(method string) bool {
	validMethods := map[string]bool{
		"GET":     true,
		"POST":    true,
		"PUT":     true,
		"PATCH":   true,
		"DELETE":  true,
		"HEAD":    true,
		"OPTIONS": true,
	}
	return validMethods[method]
}

// isValidLoopMode checks if the loop mode is valid.
func isValidLoopMode(mode string) bool {
	validModes := map[string]bool{
		"forEach": true,
		"times":   true,
		"while":   true,
	}
	return validModes[mode]
}

// isValidFileOperation checks if the file operation is valid.
func isValidFileOperation(operation string) bool {
	validOps := map[string]bool{
		"read":   true,
		"write":  true,
		"copy":   true,
		"move":   true,
		"delete": true,
		"mkdir":  true,
		"exists": true,
		"list":   true,
	}
	return validOps[operation]
}
