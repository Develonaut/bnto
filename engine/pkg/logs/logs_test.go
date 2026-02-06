package logs

import (
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestGetMostRecentLog_EmptyDirectory(t *testing.T) {
	// Create temp directory
	tmpDir := t.TempDir()

	// Test empty directory
	result, err := GetMostRecentLog(tmpDir)
	if err != nil {
		t.Fatalf("Expected no error for empty directory, got: %v", err)
	}

	if result != "" {
		t.Errorf("Expected empty string for empty directory, got: %s", result)
	}
}

func TestGetMostRecentLog_SingleFile(t *testing.T) {
	tmpDir := t.TempDir()

	// Create a single log file
	logFile := "bento_2025-10-20_13-30-45.log"
	createTestLogFile(t, tmpDir, logFile, "test log content")

	result, err := GetMostRecentLog(tmpDir)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if result != logFile {
		t.Errorf("Expected %s, got: %s", logFile, result)
	}
}

func TestGetMostRecentLog_MultipleFiles(t *testing.T) {
	tmpDir := t.TempDir()

	// Create multiple log files with explicit timestamps
	now := time.Now()

	oldLog := "bento_2025-10-19_10-00-00.log"
	createTestLogFileWithTime(t, tmpDir, oldLog, "old log", now.Add(-2*time.Hour))

	newerLog := "bento_2025-10-20_12-00-00.log"
	createTestLogFileWithTime(t, tmpDir, newerLog, "newer log", now.Add(-1*time.Hour))

	newestLog := "bento_2025-10-20_14-00-00.log"
	createTestLogFileWithTime(t, tmpDir, newestLog, "newest log", now)

	result, err := GetMostRecentLog(tmpDir)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if result != newestLog {
		t.Errorf("Expected most recent file %s, got: %s", newestLog, result)
	}
}

func TestGetMostRecentLog_IgnoresDirectories(t *testing.T) {
	tmpDir := t.TempDir()

	// Create a subdirectory
	subDir := filepath.Join(tmpDir, "subdir")
	if err := os.Mkdir(subDir, 0755); err != nil {
		t.Fatalf("Failed to create subdirectory: %v", err)
	}

	// Create a log file
	logFile := "bento_2025-10-20_13-30-45.log"
	createTestLogFile(t, tmpDir, logFile, "test log")

	result, err := GetMostRecentLog(tmpDir)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if result != logFile {
		t.Errorf("Expected %s, got: %s", logFile, result)
	}
}

func TestGetMostRecentLog_NonExistentDirectory(t *testing.T) {
	result, err := GetMostRecentLog("/nonexistent/path/logs")
	if err == nil {
		t.Fatal("Expected error for nonexistent directory, got nil")
	}

	if result != "" {
		t.Errorf("Expected empty string on error, got: %s", result)
	}
}

func TestListLogFiles_EmptyDirectory(t *testing.T) {
	tmpDir := t.TempDir()

	files, err := ListLogFiles(tmpDir)
	if err != nil {
		t.Fatalf("Expected no error for empty directory, got: %v", err)
	}

	if len(files) != 0 {
		t.Errorf("Expected 0 files, got %d", len(files))
	}
}

func TestListLogFiles_MultipleFiles(t *testing.T) {
	tmpDir := t.TempDir()

	// Create log files with explicit timestamps
	now := time.Now()

	log1 := "bento_2025-10-19_10-00-00.log"
	createTestLogFileWithTime(t, tmpDir, log1, "log 1", now.Add(-2*time.Hour))

	log2 := "bento_2025-10-20_12-00-00.log"
	createTestLogFileWithTime(t, tmpDir, log2, "log 2", now.Add(-1*time.Hour))

	log3 := "bento_2025-10-20_14-00-00.log"
	createTestLogFileWithTime(t, tmpDir, log3, "log 3", now)

	files, err := ListLogFiles(tmpDir)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if len(files) != 3 {
		t.Fatalf("Expected 3 files, got %d", len(files))
	}

	// Files should be sorted by modification time (newest first)
	if files[0].Name != log3 {
		t.Errorf("Expected first file to be %s, got: %s", log3, files[0].Name)
	}

	if files[1].Name != log2 {
		t.Errorf("Expected second file to be %s, got: %s", log2, files[1].Name)
	}

	if files[2].Name != log1 {
		t.Errorf("Expected third file to be %s, got: %s", log1, files[2].Name)
	}
}

func TestListLogFiles_IgnoresDirectories(t *testing.T) {
	tmpDir := t.TempDir()

	// Create subdirectory
	subDir := filepath.Join(tmpDir, "subdir")
	if err := os.Mkdir(subDir, 0755); err != nil {
		t.Fatalf("Failed to create subdirectory: %v", err)
	}

	// Create log file
	logFile := "bento_2025-10-20_13-30-45.log"
	createTestLogFile(t, tmpDir, logFile, "test log")

	files, err := ListLogFiles(tmpDir)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if len(files) != 1 {
		t.Errorf("Expected 1 file (directory should be ignored), got %d", len(files))
	}

	if files[0].Name != logFile {
		t.Errorf("Expected file %s, got: %s", logFile, files[0].Name)
	}
}

func TestGetLogsDirectory(t *testing.T) {
	path, err := GetLogsDirectory("")
	if err != nil {
		t.Fatalf("Expected no error getting logs directory, got: %v", err)
	}

	// Should contain .bento/logs in path
	if !contains(path, ".bento") || !contains(path, "logs") {
		t.Errorf("Expected path to contain .bento/logs, got: %s", path)
	}
}

func TestEnsureLogsDirectory(t *testing.T) {
	// Use temp dir for testing
	tmpHome := t.TempDir()
	t.Setenv("HOME", tmpHome)

	logsDir, err := GetLogsDirectory("")
	if err != nil {
		t.Fatalf("Failed to get logs directory: %v", err)
	}

	// Directory shouldn't exist yet
	if _, err := os.Stat(logsDir); err == nil {
		t.Fatalf("Directory should not exist yet")
	}

	// Ensure it's created
	if err := EnsureLogsDirectory(""); err != nil {
		t.Fatalf("Failed to ensure logs directory: %v", err)
	}

	// Now it should exist
	if _, err := os.Stat(logsDir); err != nil {
		t.Errorf("Directory should exist after EnsureLogsDirectory, got error: %v", err)
	}
}

func TestGenerateLogFileName(t *testing.T) {
	fileName := GenerateLogFileName()

	expected := "bento.log"
	if fileName != expected {
		t.Errorf("Expected file name %s, got: %s", expected, fileName)
	}
}

func TestTrimLogFile_UnderThreshold(t *testing.T) {
	tmpDir := t.TempDir()
	logPath := filepath.Join(tmpDir, "test.log")

	// Create file with 100 lines (under 10,000 threshold)
	lines := generateLines(100)
	writeLines(t, logPath, lines)

	// Trim should not change the file
	err := TrimLogFile(logPath, 10000, 5000)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Verify all lines still present
	content := readFile(t, logPath)
	lineCount := countLines(content)
	if lineCount != 100 {
		t.Errorf("Expected 100 lines, got: %d", lineCount)
	}
}

func TestTrimLogFile_OverThreshold(t *testing.T) {
	tmpDir := t.TempDir()
	logPath := filepath.Join(tmpDir, "test.log")

	// Create file with 15,000 lines (over 10,000 threshold)
	lines := generateLines(15000)
	writeLines(t, logPath, lines)

	// Trim should keep most recent 5,000 lines
	err := TrimLogFile(logPath, 10000, 5000)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Verify only 5,000 lines remain
	content := readFile(t, logPath)
	lineCount := countLines(content)
	if lineCount != 5000 {
		t.Errorf("Expected 5,000 lines after trim, got: %d", lineCount)
	}

	// Verify it kept the MOST RECENT lines (lines 10,001-15,000)
	firstLine := getFirstLine(content)
	expectedFirstLine := "line 10001"
	if firstLine != expectedFirstLine {
		t.Errorf("Expected first line to be %s, got: %s", expectedFirstLine, firstLine)
	}
}

func TestTrimLogFile_ExactlyAtThreshold(t *testing.T) {
	tmpDir := t.TempDir()
	logPath := filepath.Join(tmpDir, "test.log")

	// Create file with exactly 10,000 lines
	lines := generateLines(10000)
	writeLines(t, logPath, lines)

	// Should not trim (need to exceed threshold)
	err := TrimLogFile(logPath, 10000, 5000)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	content := readFile(t, logPath)
	lineCount := countLines(content)
	if lineCount != 10000 {
		t.Errorf("Expected 10,000 lines (no trim at threshold), got: %d", lineCount)
	}
}

func TestTrimLogFile_NonExistentFile(t *testing.T) {
	// Should not error on non-existent file
	err := TrimLogFile("/nonexistent/path/test.log", 10000, 5000)
	if err == nil {
		t.Error("Expected error for non-existent file")
	}
}

// Test helpers

func generateLines(count int) []string {
	lines := make([]string, count)
	for i := 0; i < count; i++ {
		lines[i] = fmt.Sprintf("line %d\n", i+1)
	}
	return lines
}

func writeLines(t *testing.T, path string, lines []string) {
	t.Helper()
	content := ""
	for _, line := range lines {
		content += line
	}
	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to write test file: %v", err)
	}
}

func readFile(t *testing.T, path string) string {
	t.Helper()
	content, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	return string(content)
}

func countLines(content string) int {
	if content == "" {
		return 0
	}
	count := 0
	for i := 0; i < len(content); i++ {
		if content[i] == '\n' {
			count++
		}
	}
	return count
}

func getFirstLine(content string) string {
	for i := 0; i < len(content); i++ {
		if content[i] == '\n' {
			return content[:i]
		}
	}
	return content
}

// Helper functions

func createTestLogFile(t *testing.T, dir, name, content string) {
	t.Helper()
	path := filepath.Join(dir, name)
	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test log file: %v", err)
	}
}

func createTestLogFileWithTime(t *testing.T, dir, name, content string, modTime time.Time) {
	t.Helper()
	path := filepath.Join(dir, name)
	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test log file: %v", err)
	}
	// Set explicit modification time
	if err := os.Chtimes(path, modTime, modTime); err != nil {
		t.Fatalf("Failed to set file modification time: %v", err)
	}
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || len(s) > len(substr) && findSubstring(s, substr))
}

func findSubstring(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
