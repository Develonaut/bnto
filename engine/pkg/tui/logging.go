package tui

import (
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/Develonaut/bnto/pkg/logs"
	"github.com/Develonaut/bnto/pkg/logger"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/muesli/termenv"
)

// channelWriter implements io.Writer and sends each write to a channel
type channelWriter struct {
	ch chan string
}

func (w *channelWriter) Write(p []byte) (n int, err error) {
	w.ch <- string(p)
	return len(p), nil
}

// createTUILogger creates a logger that writes to both file and TUI channel
func createTUILogger(logChan chan string) (*os.File, *logger.Logger, error) {
	bntoHome := LoadBntoHome()

	// Ensure logs directory exists
	if err := logs.EnsureLogsDirectory(bntoHome); err != nil {
		return nil, nil, err
	}

	// Get logs directory path
	logsDir, err := logs.GetLogsDirectory(bntoHome)
	if err != nil {
		return nil, nil, err
	}

	// Generate log file name
	logFileName := logs.GenerateLogFileName()
	logPath := filepath.Join(logsDir, logFileName)

	// Trim log file if it exceeds threshold
	logs.TrimLogFile(logPath, 10000, 5000) //nolint:errcheck

	// Open in append mode
	logFile, err := os.OpenFile(logPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to open log file: %w", err)
	}

	// Create channel writer for TUI
	chanWriter := &channelWriter{ch: logChan}

	// Use MultiWriter to write to both file and TUI
	multiWriter := io.MultiWriter(logFile, chanWriter)

	// Create logger with multi-writer
	log := logger.New(logger.Config{
		Output: multiWriter,
		Level:  logger.LevelInfo,
	})

	// Force ANSI256 color profile for proper syntax highlighting in TUI
	// This ensures colors show up even though we're not writing to a TTY
	log.SetColorProfile(termenv.ANSI256)

	// Add blank line for separation
	log.Info("")

	return logFile, log, nil
}

// listenForLogs listens to the log channel and sends log messages to the TUI
func listenForLogs(logChan chan string) tea.Cmd {
	return func() tea.Msg {
		log, ok := <-logChan
		if !ok {
			// Channel closed, no more logs
			return nil
		}
		return executionOutputMsg(log)
	}
}
