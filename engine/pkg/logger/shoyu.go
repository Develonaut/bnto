package logger

import (
	"context"
	"io"
	"log/slog"
	"os"

	"github.com/charmbracelet/log"
	"github.com/muesli/termenv"
)

// Logger wraps slog.Logger using charm/log as the handler for beautiful output.
// It provides structured logging with charm's beautiful formatting,
// as well as streaming for long-running processes like Blender renders.
type Logger struct {
	sl       *slog.Logger
	handler  *log.Logger
	onStream StreamCallback
}

// New creates a new Logger with the given configuration.
// Uses charm/log as an slog handler for beautiful output.
// Defaults are applied for missing configuration values:
//   - Output: os.Stdout
//   - Level: LevelInfo
func New(cfg Config) *Logger {
	cfg = applyDefaults(cfg)

	// Create charm/log handler with options
	handler := log.NewWithOptions(cfg.Output, log.Options{
		Level:           convertLevel(cfg.Level),
		ReportTimestamp: true,
		ReportCaller:    false,
	})

	// Apply custom styles for colored log levels and breadcrumbs
	handler.SetStyles(createCustomStyles())

	// Create slog logger using charm handler
	sl := slog.New(handler)

	return &Logger{
		sl:       sl,
		handler:  handler,
		onStream: cfg.OnStream,
	}
}

// applyDefaults sets default values for missing config fields.
func applyDefaults(cfg Config) Config {
	if cfg.Output == nil {
		cfg.Output = os.Stdout
	}

	if cfg.Level == "" {
		cfg.Level = LevelInfo
	}

	return cfg
}

// convertLevel converts our Level type to charm/log Level.
func convertLevel(level Level) log.Level {
	switch level {
	case LevelDebug:
		return log.DebugLevel
	case LevelInfo:
		return log.InfoLevel
	case LevelWarn:
		return log.WarnLevel
	case LevelError:
		return log.ErrorLevel
	default:
		return log.InfoLevel
	}
}

// Info logs an informational message with optional key-value pairs.
// Args must be provided as alternating keys and values.
func (l *Logger) Info(msg string, args ...any) {
	l.sl.Info(msg, args...)
}

// Debug logs a debug message with optional key-value pairs.
// Args must be provided as alternating keys and values.
func (l *Logger) Debug(msg string, args ...any) {
	l.sl.Debug(msg, args...)
}

// Warn logs a warning message with optional key-value pairs.
// Args must be provided as alternating keys and values.
func (l *Logger) Warn(msg string, args ...any) {
	l.sl.Warn(msg, args...)
}

// Error logs an error message with optional key-value pairs.
// Args must be provided as alternating keys and values.
func (l *Logger) Error(msg string, args ...any) {
	l.sl.Error(msg, args...)
}

// InfoContext logs an informational message with context.
// This is the preferred method when context is available.
func (l *Logger) InfoContext(ctx context.Context, msg string, args ...any) {
	l.sl.InfoContext(ctx, msg, args...)
}

// DebugContext logs a debug message with context.
func (l *Logger) DebugContext(ctx context.Context, msg string, args ...any) {
	l.sl.DebugContext(ctx, msg, args...)
}

// WarnContext logs a warning message with context.
func (l *Logger) WarnContext(ctx context.Context, msg string, args ...any) {
	l.sl.WarnContext(ctx, msg, args...)
}

// ErrorContext logs an error message with context.
func (l *Logger) ErrorContext(ctx context.Context, msg string, args ...any) {
	l.sl.ErrorContext(ctx, msg, args...)
}

// With creates a child logger with additional context fields.
// This is used by the engine to add trace IDs, bento IDs, node IDs, etc.
//
// Example:
//
//	contextLogger := logger.With(
//	    "bento_id", "my-workflow",
//	    "node_id", "node-1")
func (l *Logger) With(args ...any) *Logger {
	return &Logger{
		sl:       l.sl.With(args...),
		handler:  l.handler,
		onStream: l.onStream,
	}
}

// Stream outputs a line for streaming processes (like Blender renders).
// This bypasses normal log levels and calls the OnStream callback if set.
// Critical for Phase 8: real-time output from shell-command node.
//
// If OnStream callback is set, it handles all output (including file logging).
// Otherwise, log to info level for default visibility.
func (l *Logger) Stream(line string) {
	if l.onStream != nil {
		l.onStream(line)
		return
	}

	// No callback - log at info level to default output
	l.sl.Info("stream", "output", line)
}

// SetOutput changes the output destination.
func (l *Logger) SetOutput(w io.Writer) {
	l.handler.SetOutput(w)
}

// SetColorProfile sets the color profile for the logger.
// Use termenv.ANSI256 or termenv.TrueColor to force colors in non-TTY environments.
func (l *Logger) SetColorProfile(profile termenv.Profile) {
	l.handler.SetColorProfile(profile)
}
