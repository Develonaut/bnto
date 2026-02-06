//go:build integration
// +build integration

package main

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/logs"
)

// TestLogTailingWithExecution tests that logs are properly written during bento execution.
// This integration test verifies:
//  1. A bento execution writes to the log file
//  2. The log file contains expected start and completion messages
func TestLogTailingWithExecution(t *testing.T) {
	// Skip if not running integration tests
	if testing.Short() {
		t.Skip("Skipping integration test in short mode")
	}

	bentoHome := tui.LoadBentoHome()

	// Ensure logs directory exists
	if err := logs.EnsureLogsDirectory(bentoHome); err != nil {
		t.Fatalf("Failed to ensure logs directory: %v", err)
	}

	logsDir, err := logs.GetLogsDirectory(bentoHome)
	if err != nil {
		t.Fatalf("Failed to get logs directory: %v", err)
	}

	logPath := filepath.Join(logsDir, "bento.log")

	// Get initial line count (or 0 if file doesn't exist)
	initialLines := 0
	if content, err := os.ReadFile(logPath); err == nil {
		initialLines = countLinesInContent(string(content))
	}

	// Create a simple test bento file
	testBentoPath := createTestBento(t)
	defer os.Remove(testBentoPath)

	// Run bento
	bentoCmd := exec.Command("bento", "run", testBentoPath)
	bentoOutput, err := bentoCmd.CombinedOutput()
	if err != nil {
		t.Fatalf("Failed to run bento: %v\nOutput: %s", err, string(bentoOutput))
	}

	// Give a moment for log file to be flushed
	time.Sleep(100 * time.Millisecond)

	// Read log file
	content, err := os.ReadFile(logPath)
	if err != nil {
		t.Fatalf("Failed to read log file: %v", err)
	}

	logContent := string(content)

	// Verify new content was added
	finalLines := countLinesInContent(logContent)
	if finalLines <= initialLines {
		t.Fatalf("Expected log file to have more than %d lines, got %d", initialLines, finalLines)
	}

	// Verify expected log messages are present
	if !strings.Contains(logContent, "Bento execution started") {
		t.Error("Log file missing 'Bento execution started' message")
	}

	if !strings.Contains(logContent, "Bento execution completed") {
		t.Error("Log file missing 'Bento execution completed' message")
	}

	t.Logf("✓ Log file properly written: %d lines added", finalLines-initialLines)
}

// TestLogRotation tests that log files are properly trimmed when exceeding threshold.
func TestLogRotation(t *testing.T) {
	// Skip if not running integration tests
	if testing.Short() {
		t.Skip("Skipping integration test in short mode")
	}

	// Create temp log file
	tmpDir := t.TempDir()
	logPath := filepath.Join(tmpDir, "test.log")

	// Generate 12,000 lines (exceeds 10,000 threshold)
	content := ""
	for i := 0; i < 12000; i++ {
		content += "2025/10/20 14:00:00 INFO Test log line\n"
	}
	if err := os.WriteFile(logPath, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test log: %v", err)
	}

	// Verify initial line count
	initialCount := countLines(t, logPath)
	if initialCount != 12000 {
		t.Fatalf("Expected 12,000 lines, got %d", initialCount)
	}

	// Trim the log file (using same thresholds as production)
	if err := logs.TrimLogFile(logPath, 10000, 5000); err != nil {
		t.Fatalf("Failed to trim log file: %v", err)
	}

	// Verify trimmed line count
	trimmedCount := countLines(t, logPath)
	if trimmedCount != 5000 {
		t.Fatalf("Expected 5,000 lines after trim, got %d", trimmedCount)
	}

	t.Logf("✓ Log rotation working: 12,000 lines → 5,000 lines")
}

// createTestBento creates a minimal test bento definition.
func createTestBento(t *testing.T) string {
	t.Helper()

	bentoJSON := `{
		"id": "test-bento",
		"type": "group",
		"version": "1.0.0",
		"name": "Test Bento",
		"position": {"x": 0, "y": 0},
		"metadata": {
			"description": "Test bento for integration testing"
		},
		"parameters": {},
		"inputPorts": [],
		"outputPorts": [],
		"nodes": [
			{
				"id": "test-node",
				"type": "shell-command",
				"version": "1.0.0",
				"name": "Test Node",
				"position": {"x": 100, "y": 100},
				"metadata": {
					"description": "Test shell command"
				},
				"parameters": {
					"command": "/bin/echo",
					"args": ["test"]
				},
				"inputPorts": [],
				"outputPorts": []
			}
		],
		"edges": []
	}`

	tmpFile := filepath.Join(t.TempDir(), "test.bento.json")
	if err := os.WriteFile(tmpFile, []byte(bentoJSON), 0644); err != nil {
		t.Fatalf("Failed to create test bento: %v", err)
	}

	return tmpFile
}

// countLines counts the number of lines in a file.
func countLines(t *testing.T, path string) int {
	t.Helper()

	content, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}

	return countLinesInContent(string(content))
}

// countLinesInContent counts the number of lines in a string.
func countLinesInContent(content string) int {
	count := 0
	for i := 0; i < len(content); i++ {
		if content[i] == '\n' {
			count++
		}
	}
	return count
}
