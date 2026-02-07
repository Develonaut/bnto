package validator

import (
	"fmt"

	"github.com/Develonaut/bnto/pkg/node"
)

// validateHTTPRequest validates http-request node parameters.
func validateHTTPRequest(def *node.Definition) error {
	// Check URL parameter
	url, ok := def.Parameters["url"].(string)
	if !ok || url == "" {
		return fmt.Errorf("http-request node '%s' missing required parameter 'url'", def.ID)
	}

	// Check method parameter
	method, ok := def.Parameters["method"].(string)
	if !ok || method == "" {
		return fmt.Errorf("http-request node '%s' missing required parameter 'method'", def.ID)
	}

	// Validate HTTP method
	if !isValidHTTPMethod(method) {
		return fmt.Errorf("http-request node '%s' has invalid method '%s' (must be GET, POST, PUT, PATCH, DELETE, HEAD, or OPTIONS)",
			def.ID, method)
	}

	return nil
}

// validateLoop validates loop node parameters.
func validateLoop(def *node.Definition) error {
	mode, ok := def.Parameters["mode"].(string)
	if !ok || mode == "" {
		return fmt.Errorf("loop node '%s' missing required parameter 'mode'", def.ID)
	}

	if !isValidLoopMode(mode) {
		return fmt.Errorf("loop node '%s' has invalid mode '%s' (must be forEach, times, or while)",
			def.ID, mode)
	}

	return validateLoopModeParams(def, mode)
}

// validateLoopModeParams validates mode-specific parameters for loop node.
func validateLoopModeParams(def *node.Definition, mode string) error {
	switch mode {
	case "forEach":
		if def.Parameters["items"] == nil {
			return fmt.Errorf("loop node '%s' with mode 'forEach' missing required parameter 'items'", def.ID)
		}
	case "times":
		if def.Parameters["count"] == nil {
			return fmt.Errorf("loop node '%s' with mode 'times' missing required parameter 'count'", def.ID)
		}
	case "while":
		if def.Parameters["condition"] == nil {
			return fmt.Errorf("loop node '%s' with mode 'while' missing required parameter 'condition'", def.ID)
		}
	}
	return nil
}

// validateFileSystem validates file-system node parameters.
func validateFileSystem(def *node.Definition) error {
	// Check operation parameter
	operation, ok := def.Parameters["operation"].(string)
	if !ok || operation == "" {
		return fmt.Errorf("file-system node '%s' missing required parameter 'operation'", def.ID)
	}

	// Validate operation value
	if !isValidFileOperation(operation) {
		return fmt.Errorf("file-system node '%s' has invalid operation '%s' (must be read, write, copy, move, delete, mkdir, exists, or list)",
			def.ID, operation)
	}

	return nil
}

// validateShellCommand validates shell-command node parameters.
func validateShellCommand(def *node.Definition) error {
	// Check command parameter
	command, ok := def.Parameters["command"].(string)
	if !ok || command == "" {
		return fmt.Errorf("shell-command node '%s' missing required parameter 'command'", def.ID)
	}

	return nil
}

// validateEditFields validates edit-fields node parameters.
func validateEditFields(def *node.Definition) error {
	// Edit-fields should have a values parameter
	if def.Parameters["values"] == nil {
		return fmt.Errorf("edit-fields node '%s' missing required parameter 'values'", def.ID)
	}

	return nil
}

// validateParallel validates parallel node parameters.
func validateParallel(def *node.Definition) error {
	// Parallel node are similar to groups
	// No specific required parameters
	return nil
}

// validateSpreadsheet validates spreadsheet node parameters.
func validateSpreadsheet(def *node.Definition) error {
	// Future implementation - no validation yet
	return nil
}

// validateImage validates image node parameters.
func validateImage(def *node.Definition) error {
	// Future implementation - no validation yet
	return nil
}

// validateTransform validates transform node parameters.
func validateTransform(def *node.Definition) error {
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
