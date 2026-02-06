// Package main implements logger creation for the run command.
//
// This file contains functions for creating file loggers and dual loggers
// (file + stdout) for bento execution.
package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/Develonaut/bento/pkg/logs"
	"github.com/Develonaut/bento/pkg/tui"
	"github.com/Develonaut/bento/pkg/logger"
)

// createFileLogger creates a logger that writes to {bento-home}/logs/
// Returns the logger, the log file (for cleanup), and any error.
func createFileLogger() (*logger.Logger, *os.File, error) {
	bentoHome := tui.LoadBentoHome()

	// Ensure logs directory exists
	if err := logs.EnsureLogsDirectory(bentoHome); err != nil {
		return nil, nil, err
	}

	// Get logs directory path
	logsDir, err := logs.GetLogsDirectory(bentoHome)
	if err != nil {
		return nil, nil, err
	}

	// Generate log file name
	logFileName := logs.GenerateLogFileName()
	logPath := filepath.Join(logsDir, logFileName)

	// Trim log file if it exceeds threshold (10,000 lines → 5,000 lines)
	// This keeps the log file bounded and prevents unlimited growth
	if err := logs.TrimLogFile(logPath, 10000, 5000); err != nil { //nolint:staticcheck // Intentionally ignoring trim errors - file might not exist yet
		// Don't fail on trim errors - just continue and try to log
		// (file might not exist yet, which is fine)
	}

	// Open in append mode (create if doesn't exist)
	// This allows multiple bento runs to append to the same log
	logFile, err := os.OpenFile(logPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to open log file: %w", err)
	}

	level := logger.LevelInfo
	if verboseFlag {
		level = logger.LevelDebug
	}

	logger := logger.New(logger.Config{
		Level:  level,
		Output: logFile,
	})

	// Add blank line before execution for separation
	logger.Info("")

	return logger, logFile, nil
}

// createDualLogger creates a logger that writes to both file and stdout.
func createDualLogger(fileLogger *logger.Logger) *logger.Logger {
	level := logger.LevelInfo
	if verboseFlag {
		level = logger.LevelDebug
	}

	return logger.New(logger.Config{
		Level: level,
		// Enable streaming output for long-running processes
		// This outputs lines from shell-command neta in real-time
		OnStream: func(line string) {
			// Always write to file for record-keeping
			if fileLogger != nil {
				fileLogger.Stream(line)
			}

			// Filter output to stdout unless verbose
			if verboseFlag || shouldShowStreamLine(line) {
				fmt.Println(line)
			}
		},
	})
}

// shouldShowStreamLine determines if a stream line should be shown.
// Returns false for noisy Blender startup messages, true for important output.
func shouldShowStreamLine(line string) bool {
	// Skip common Blender noise
	noisePatterns := []string{
		"WARN (bgl):",
		"Photographer preferences",
		"Blender 4.",
		"Read blend:",
		"Info: Read library:",
		"Warning: Unable to open",
		"Info: Cannot find lib",
		"Info: LIB:",
		"Warning: 1 libraries",
		"blenderkit:",
		"Deleting object:",
		"Imported object name:",
		"Import finished in",
		"Selected imported object:",
		"Camera '",
		"Removed image:",
	}

	for _, pattern := range noisePatterns {
		if strings.Contains(line, pattern) {
			return false
		}
	}

	return true
}
