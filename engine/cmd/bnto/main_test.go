// Package main provides integration tests for the bnto CLI.
//
// These tests verify the CLI commands work correctly end-to-end by:
//   - Creating test bnto files on disk
//   - Executing the compiled bnto binary
//   - Verifying output and exit codes
//
// Tests use the compiled binary to ensure we're testing the actual user experience.
package main_test

import (
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// Test helpers

// createTestBnto creates a test bnto file with the given content.
func createTestBnto(t *testing.T, name, content string) string {
	t.Helper()

	tmpfile, err := os.CreateTemp("", name)
	if err != nil {
		t.Fatalf("Failed to create temp file: %v", err)
	}

	if _, err := tmpfile.Write([]byte(content)); err != nil {
		t.Fatalf("Failed to write temp file: %v", err)
	}

	if err := tmpfile.Close(); err != nil {
		t.Fatalf("Failed to close temp file: %v", err)
	}

	return tmpfile.Name()
}

// createTestBntoInDir creates a test bnto in a specific directory.
func createTestBntoInDir(t *testing.T, dir, name, title string) {
	t.Helper()

	content := simpleValidBnto(title)
	path := filepath.Join(dir, name)

	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to write bnto file: %v", err)
	}
}

// bntoParts returns the parts of a bnto definition.
func bntoParts() (header, node, footer string) {
	header = `{
		"id": "test-bnto",
		"type": "group",
		"version": "1.0.0",
		"name": "`

	node = `",
		"position": {"x": 0, "y": 0},
		"metadata": {},
		"parameters": {},
		"inputPorts": [],
		"outputPorts": [],
		"nodes": [
			{
				"id": "node-1",
				"type": "edit-fields",
				"version": "1.0.0",
				"name": "Set Field",
				"position": {"x": 0, "y": 0},
				"metadata": {},
				"parameters": {
					"values": {"foo": "bar"}
				},
				"inputPorts": [],
				"outputPorts": []
			}
		],
		"edges": []
	}`

	return
}

// simpleValidBnto returns a valid minimal bnto definition.
func simpleValidBnto(title string) string {
	if title == "" {
		title = "Test Bnto"
	}
	h, n, _ := bntoParts()
	return h + title + n
}

// verifyCommandSuccess checks if command succeeded with expected output.
func verifyCommandSuccess(t *testing.T, cmd *exec.Cmd, expectedText string) string {
	t.Helper()
	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("command failed: %v\nOutput: %s", err, string(output))
	}
	outputStr := string(output)
	if !strings.Contains(outputStr, expectedText) {
		t.Errorf("Output should contain '%s': %s", expectedText, outputStr)
	}
	return outputStr
}

// verifyCommandError checks if command failed with expected error.
func verifyCommandError(t *testing.T, cmd *exec.Cmd) {
	t.Helper()
	output, err := cmd.CombinedOutput()
	if err == nil {
		t.Fatal("command should fail")
	}
	if exitError, ok := err.(*exec.ExitError); ok {
		if exitError.ExitCode() != 1 {
			t.Errorf("Exit code = %d, want 1", exitError.ExitCode())
		}
	}
	outputStr := strings.ToLower(string(output))
	if !strings.Contains(outputStr, "error") &&
		!strings.Contains(outputStr, "failed") &&
		!strings.Contains(outputStr, "missing") {
		t.Errorf("Output should mention error: %s", string(output))
	}
}

// invalidHTTPBnto returns a bnto with missing URL parameter.
func invalidHTTPBnto() string {
	return `{
		"id": "invalid",
		"type": "group",
		"version": "1.0.0",
		"name": "Invalid",
		"position": {"x": 0, "y": 0},
		"metadata": {},
		"parameters": {},
		"inputPorts": [],
		"outputPorts": [],
		"nodes": [{
			"id": "http-1",
			"type": "http-request",
			"version": "1.0.0",
			"name": "HTTP",
			"position": {"x": 0, "y": 0},
			"metadata": {},
			"parameters": {"method": "GET"},
			"inputPorts": [],
			"outputPorts": []
		}],
		"edges": []
	}`
}

// verifyBntosListed checks if bntos are listed in output.
func verifyBntosListed(t *testing.T, output string, files ...string) {
	t.Helper()
	for _, file := range files {
		if !strings.Contains(output, file) {
			t.Errorf("Output should list %s", file)
		}
	}
}

// createExistingFile creates a file in the directory.
func createExistingFile(t *testing.T, dir, name string) {
	t.Helper()
	path := filepath.Join(dir, name)
	if err := os.WriteFile(path, []byte("existing content"), 0644); err != nil {
		t.Fatal(err)
	}
}

// changeToDir changes to directory and sets up cleanup.
func changeToDir(t *testing.T, dir string) {
	t.Helper()
	oldDir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	if err := os.Chdir(dir); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() {
		if err := os.Chdir(oldDir); err != nil {
			t.Logf("failed to restore directory: %v", err)
		}
	})
}

// verifyBntoJSONValid checks if bnto file is valid JSON with correct ID.
func verifyBntoJSONValid(t *testing.T, path, expectedID string) {
	t.Helper()
	if _, err := os.Stat(path); os.IsNotExist(err) {
		t.Fatal("Bnto file was not created")
	}

	content, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}

	var def map[string]interface{}
	if err := json.Unmarshal(content, &def); err != nil {
		t.Errorf("Created bnto is not valid JSON: %v", err)
	}

	if def["id"] != expectedID {
		t.Errorf("Bnto ID = %v, want %s", def["id"], expectedID)
	}

	if def["type"] != "group" {
		t.Errorf("Bnto type = %v, want group", def["type"])
	}
}

// Test: bnto version command

// TestVersionCommand_ShowsVersion verifies version command displays version.
func TestVersionCommand_ShowsVersion(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "version"), "version")
	// Should show either "dev" or a semantic version like "v1.0.0"
	if !strings.Contains(output, "dev") && !strings.Contains(output, "v") {
		t.Errorf("Should display version info: %s", output)
	}
}

// TestVersionCommand_NoArguments verifies version works without arguments.
func TestVersionCommand_NoArguments(t *testing.T) {
	cmd := exec.Command("bnto", "version")
	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("version command should succeed: %v\nOutput: %s", err, string(output))
	}
	if len(output) == 0 {
		t.Error("version output should not be empty")
	}
}

// TestVersionCommand_Shorthand verifies -v alias works.
func TestVersionCommand_Shorthand(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "v"), "version")
	// Both 'bnto version' and 'bnto v' should produce identical output
	if !strings.Contains(output, "dev") && !strings.Contains(output, "v") {
		t.Errorf("Should display version info: %s", output)
	}
}
