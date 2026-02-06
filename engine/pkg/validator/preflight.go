package validator

import (
	"fmt"
	"os"
	"os/exec"
	"strings"

	"github.com/Develonaut/bento/pkg/node"
)

// preflightShellCommand checks if the command exists in PATH.
func preflightShellCommand(def *node.Definition) error {
	command, ok := def.Parameters["command"].(string)
	if !ok {
		return nil // Already validated by validateShellCommand
	}

	// Check if command exists in PATH
	_, err := exec.LookPath(command)
	if err != nil {
		return fmt.Errorf("shell-command node '%s': command '%s' not found in PATH. Please install it first",
			def.ID, command)
	}

	return nil
}

// preflightHTTPRequest checks for required environment variables in URL/headers.
func preflightHTTPRequest(def *node.Definition) error {
	if err := checkURLEnvVars(def); err != nil {
		return err
	}
	return checkHeaderEnvVars(def)
}

// checkURLEnvVars validates environment variables in URL.
func checkURLEnvVars(def *node.Definition) error {
	url, _ := def.Parameters["url"].(string)
	envVars := extractEnvVars(url)

	for _, envVar := range envVars {
		if os.Getenv(envVar) == "" {
			return fmt.Errorf("http-request node '%s': environment variable '%s' not set (required in URL)",
				def.ID, envVar)
		}
	}
	return nil
}

// checkHeaderEnvVars validates environment variables in headers.
func checkHeaderEnvVars(def *node.Definition) error {
	headers, ok := def.Parameters["headers"].(map[string]string)
	if !ok {
		return nil
	}

	for key, value := range headers {
		if err := checkHeaderValue(def, key, value); err != nil {
			return err
		}
	}
	return nil
}

// checkHeaderValue validates environment variables in a single header value.
func checkHeaderValue(def *node.Definition, key, value string) error {
	envVars := extractEnvVars(value)
	for _, envVar := range envVars {
		if os.Getenv(envVar) == "" {
			return fmt.Errorf("http-request node '%s': environment variable '%s' not set (required in header '%s')",
				def.ID, envVar, key)
		}
	}
	return nil
}

// preflightFileSystem checks if file paths exist for read operations.
func preflightFileSystem(def *node.Definition) error {
	operation, _ := def.Parameters["operation"].(string)
	path, ok := def.Parameters["path"].(string)
	if !ok {
		return nil
	}

	// Check environment variables in path first
	if err := checkPathEnvVars(def, path); err != nil {
		return err
	}

	// For read/copy operations, check source file exists (only if no templates)
	if (operation == "read" || operation == "copy") && !containsTemplates(path) {
		if _, err := os.Stat(path); os.IsNotExist(err) {
			return fmt.Errorf("file-system node '%s': file not found: %s", def.ID, path)
		}
	}

	// For copy operation, also check source path
	if operation == "copy" {
		if source, ok := def.Parameters["source"].(string); ok {
			if err := checkPathEnvVars(def, source); err != nil {
				return err
			}
			if !containsTemplates(source) {
				if _, err := os.Stat(source); os.IsNotExist(err) {
					return fmt.Errorf("file-system node '%s': source file not found: %s", def.ID, source)
				}
			}
		}
	}

	return nil
}

// extractEnvVars finds {{.VAR_NAME}} patterns in a string.
//
// This is a simple string-based approach that doesn't require regex.
// It extracts all environment variable names from Go template syntax.
// Skips context variables like {{.item.*}}, {{.index}}, etc.
func extractEnvVars(s string) []string {
	var vars []string

	for {
		varName, remaining, found := findNextVar(s)
		if !found {
			break
		}
		// Only include if it's an environment variable (not a context variable)
		if isEnvVar(varName) {
			vars = append(vars, varName)
		}
		s = remaining
	}

	return vars
}

// isEnvVar determines if a variable name is an environment variable.
// Returns false for context variables like "item.name", "index", etc.
func isEnvVar(varName string) bool {
	// Skip context variables used in loops and templates
	contextVars := []string{"item.", "index", "output", "result", "context"}

	for _, ctx := range contextVars {
		if strings.HasPrefix(varName, ctx) || varName == ctx {
			return false
		}
	}

	return true
}

// findNextVar finds the next {{.VAR}} pattern and returns (varName, remaining, found).
func findNextVar(s string) (string, string, bool) {
	start := strings.Index(s, "{{.")
	if start == -1 {
		return "", "", false
	}

	end := strings.Index(s[start:], "}}")
	if end == -1 {
		return "", "", false
	}

	varName := s[start+3 : start+end]
	remaining := s[start+end+2:]
	return varName, remaining, true
}

// preflightSpreadsheet checks CSV file exists and environment variables in path.
func preflightSpreadsheet(def *node.Definition) error {
	operation, _ := def.Parameters["operation"].(string)
	if operation != "read" {
		return nil
	}

	path, ok := def.Parameters["path"].(string)
	if !ok {
		return nil
	}

	// Check environment variables in path first
	if err := checkPathEnvVars(def, path); err != nil {
		return err
	}

	// Check file exists (only if no templates like {{.index}})
	if !containsTemplates(path) {
		if _, err := os.Stat(path); os.IsNotExist(err) {
			return fmt.Errorf("spreadsheet node '%s': CSV file not found: %s", def.ID, path)
		}
	}

	return nil
}

// checkPathEnvVars validates environment variables in a file path.
func checkPathEnvVars(def *node.Definition, path string) error {
	envVars := extractEnvVars(path)
	for _, envVar := range envVars {
		if os.Getenv(envVar) == "" {
			return fmt.Errorf("node '%s': environment variable '%s' not set (required in path: %s)",
				def.ID, envVar, path)
		}
	}
	return nil
}

// containsTemplates checks if a string contains any Go template syntax.
// Returns true if it contains {{.VAR}}, {{.item.field}}, {{.index}}, etc.
func containsTemplates(s string) bool {
	return strings.Contains(s, "{{.")
}
