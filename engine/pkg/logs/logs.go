// Package logs provides utilities for managing bento execution logs.
//
// Logs are stored in {bento-home}/logs/ with timestamped filenames.
package logs

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"time"
)

// LogFile represents a log file with metadata.
type LogFile struct {
	Name    string
	ModTime time.Time
}

// GetLogsDirectory returns the path to the logs directory.
// If bentoHome is empty, defaults to ~/.bento
func GetLogsDirectory(bentoHome string) (string, error) {
	if bentoHome == "" {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			return "", fmt.Errorf("failed to get home directory: %w", err)
		}
		bentoHome = filepath.Join(homeDir, ".bento")
	}
	return filepath.Join(bentoHome, "logs"), nil
}

// EnsureLogsDirectory creates the logs directory if it doesn't exist.
// Accepts optional bentoHome parameter.
func EnsureLogsDirectory(bentoHome string) error {
	logsDir, err := GetLogsDirectory(bentoHome)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(logsDir, 0755); err != nil {
		return fmt.Errorf("failed to create logs directory: %w", err)
	}

	return nil
}

// GetMostRecentLog finds the most recently modified log file.
// Returns empty string if no log files exist.
func GetMostRecentLog(logsDir string) (string, error) {
	files, err := os.ReadDir(logsDir)
	if err != nil {
		return "", fmt.Errorf("failed to read logs directory: %w", err)
	}

	var mostRecent string
	var mostRecentTime int64

	for _, file := range files {
		if file.IsDir() {
			continue
		}

		info, err := file.Info()
		if err != nil {
			continue
		}

		if info.ModTime().Unix() > mostRecentTime {
			mostRecent = file.Name()
			mostRecentTime = info.ModTime().Unix()
		}
	}

	return mostRecent, nil
}

// ListLogFiles returns all log files sorted by modification time (newest first).
func ListLogFiles(logsDir string) ([]LogFile, error) {
	files, err := os.ReadDir(logsDir)
	if err != nil {
		return nil, fmt.Errorf("failed to read logs directory: %w", err)
	}

	logFiles := []LogFile{}

	for _, file := range files {
		if file.IsDir() {
			continue
		}

		info, err := file.Info()
		if err != nil {
			continue
		}

		logFiles = append(logFiles, LogFile{
			Name:    file.Name(),
			ModTime: info.ModTime(),
		})
	}

	// Sort by modification time (newest first)
	sort.Slice(logFiles, func(i, j int) bool {
		return logFiles[i].ModTime.After(logFiles[j].ModTime)
	})

	return logFiles, nil
}

// GenerateLogFileName generates the log file name.
// Always returns "bento.log" (single log file with rotation).
func GenerateLogFileName() string {
	return "bento.log"
}

// TrimLogFile trims a log file to keep only the most recent lines.
// If the file has more than maxLines, it keeps only keepLines most recent lines.
func TrimLogFile(logPath string, maxLines, keepLines int) error {
	// Read the file
	content, err := os.ReadFile(logPath)
	if err != nil {
		return fmt.Errorf("failed to read log file: %w", err)
	}

	// Count lines
	lineCount := countLogLines(string(content))

	// If under threshold, no trimming needed
	if lineCount <= maxLines {
		return nil
	}

	// Keep only the most recent keepLines
	lines := splitLogLines(string(content))
	if len(lines) > keepLines {
		lines = lines[len(lines)-keepLines:]
	}

	// Write back the trimmed content
	newContent := joinLogLines(lines)
	if err := os.WriteFile(logPath, []byte(newContent), 0644); err != nil {
		return fmt.Errorf("failed to write trimmed log: %w", err)
	}

	return nil
}

// countLogLines counts the number of lines in a string.
func countLogLines(content string) int {
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

// splitLogLines splits content into lines (keeping newline characters).
func splitLogLines(content string) []string {
	if content == "" {
		return []string{}
	}
	lines := []string{}
	start := 0
	for i := 0; i < len(content); i++ {
		if content[i] == '\n' {
			lines = append(lines, content[start:i+1])
			start = i + 1
		}
	}
	// Add remaining content if any
	if start < len(content) {
		lines = append(lines, content[start:])
	}
	return lines
}

// joinLogLines joins lines back into a single string.
func joinLogLines(lines []string) string {
	result := ""
	for _, line := range lines {
		result += line
	}
	return result
}
