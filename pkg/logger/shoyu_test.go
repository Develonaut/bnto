package logger_test

import (
	"bytes"
	"context"
	"strings"
	"testing"

	"github.com/Develonaut/bento/pkg/logger"
)

// TestLogger_Output verifies that the logger outputs messages with key-value pairs.
func TestLogger_Output(t *testing.T) {
	var buf bytes.Buffer

	logger := logger.New(logger.Config{
		Level:  logger.LevelInfo,
		Output: &buf,
	})

	logger.Info("Executing HTTP request",
		"neta_type", "http-request",
		"neta_id", "node-1")

	output := buf.String()

	if !strings.Contains(output, "Executing HTTP request") {
		t.Errorf("Output should contain message: %s", output)
	}

	if !strings.Contains(output, "neta_type") || !strings.Contains(output, "http-request") {
		t.Errorf("Output should contain neta_type=http-request: %s", output)
	}

	if !strings.Contains(output, "neta_id") || !strings.Contains(output, "node-1") {
		t.Errorf("Output should contain neta_id=node-1: %s", output)
	}
}

// TestLogger_ConsoleOutput verifies that console mode produces
// human-readable text output, not JSON.
func TestLogger_ConsoleOutput(t *testing.T) {
	var buf bytes.Buffer

	logger := logger.New(logger.Config{
		Level:  logger.LevelInfo,
		Output: &buf,
	})

	logger.Info("Fetching data", "url", "https://api.example.com")

	output := buf.String()

	// Should be human-readable, not JSON
	if strings.Count(output, "{") > 1 {
		t.Error("Console output should not contain JSON objects")
	}

	if !strings.Contains(output, "Fetching data") {
		t.Error("Console output should contain message")
	}

	if !strings.Contains(output, "https://api.example.com") {
		t.Error("Console output should contain URL")
	}
}

// TestLogger_Levels verifies that log level filtering works correctly.
// DEBUG logs should be filtered when level is set to WARN.
func TestLogger_Levels(t *testing.T) {
	var buf bytes.Buffer

	// Set level to WARN - should not see INFO or DEBUG
	logger := logger.New(logger.Config{
		Level:  logger.LevelWarn,
		Output: &buf,
	})

	logger.Debug("This should not appear - debug")
	logger.Info("This should not appear - info")
	logger.Warn("This should appear - warning")
	logger.Error("This should appear - error")

	output := buf.String()

	if strings.Contains(output, "This should not appear") {
		t.Error("Debug and Info messages should be filtered out at WARN level")
	}

	if !strings.Contains(output, "This should appear - warning") {
		t.Error("Warn message should be present")
	}

	if !strings.Contains(output, "This should appear - error") {
		t.Error("Error message should be present")
	}
}

// TestLogger_DebugLevel verifies that DEBUG level shows all logs.
func TestLogger_DebugLevel(t *testing.T) {
	var buf bytes.Buffer

	logger := logger.New(logger.Config{
		Level:  logger.LevelDebug,
		Output: &buf,
	})

	logger.Debug("Debug message")
	logger.Info("Info message")

	output := buf.String()

	if !strings.Contains(output, "Debug message") {
		t.Error("Debug message should appear at DEBUG level")
	}

	if !strings.Contains(output, "Info message") {
		t.Error("Info message should appear at DEBUG level")
	}
}

// TestLogger_WithContext verifies that context values (trace IDs, bento IDs)
// propagate correctly through the logger. This is critical for engine.
func TestLogger_WithContext(t *testing.T) {
	var buf bytes.Buffer

	logger := logger.New(logger.Config{
		Level:  logger.LevelInfo,
		Output: &buf,
	})

	// Create logger with context (like itamae would do)
	contextLogger := logger.With(
		"trace_id", "trace-123",
		"bento_id", "my-workflow")

	contextLogger.Info("Executing neta")

	output := buf.String()

	if !strings.Contains(output, "trace-123") {
		t.Errorf("Output should contain trace_id: %s", output)
	}

	if !strings.Contains(output, "my-workflow") {
		t.Errorf("Output should contain bento_id: %s", output)
	}
}

// TestLogger_StreamingCallback verifies that the streaming callback mechanism
// works for real-time output from long-running processes (like Blender).
// This is CRITICAL for Phase 8.
func TestLogger_StreamingCallback(t *testing.T) {
	var buf bytes.Buffer
	var streamLines []string

	logger := logger.New(logger.Config{
		Level:  logger.LevelInfo,
		Output: &buf,
		OnStream: func(line string) {
			streamLines = append(streamLines, line)
		},
	})

	// Simulate Blender output streaming
	logger.Stream("Fra:1 Mem:12.00M (Peak 12.00M) | Rendering 1/100")
	logger.Stream("Fra:2 Mem:12.00M (Peak 12.00M) | Rendering 2/100")
	logger.Stream("Fra:3 Mem:12.00M (Peak 12.00M) | Rendering 3/100")

	if len(streamLines) != 3 {
		t.Errorf("Expected 3 stream lines, got %d", len(streamLines))
	}

	if streamLines[0] != "Fra:1 Mem:12.00M (Peak 12.00M) | Rendering 1/100" {
		t.Errorf("Stream line 0 = %q, want Blender frame 1 output", streamLines[0])
	}

	if streamLines[2] != "Fra:3 Mem:12.00M (Peak 12.00M) | Rendering 3/100" {
		t.Errorf("Stream line 2 = %q, want Blender frame 3 output", streamLines[2])
	}
}

// TestLogger_ContextHelpers verifies that the context helper functions
// (WithBentoID, WithNetaID, etc.) work correctly.
func TestLogger_ContextHelpers(t *testing.T) {
	var buf bytes.Buffer

	log := logger.New(logger.Config{
		Level:  logger.LevelInfo,
		Output: &buf,
	})

	// Use helper functions to add context
	log = logger.WithBentoID(log, "workflow-123")
	log = logger.WithNetaID(log, "neta-456")
	log = logger.WithNetaType(log, "http-request")

	log.Info("Test message")

	output := buf.String()

	if !strings.Contains(output, "workflow-123") {
		t.Errorf("Output should contain bento_id: %s", output)
	}

	if !strings.Contains(output, "neta-456") {
		t.Errorf("Output should contain neta_id: %s", output)
	}

	if !strings.Contains(output, "http-request") {
		t.Errorf("Output should contain neta_type: %s", output)
	}
}

// TestLogger_StreamReader verifies that StreamReader correctly reads
// from an io.Reader and calls the callback for each line.
func TestLogger_StreamReader(t *testing.T) {
	var buf bytes.Buffer
	var streamLines []string

	log := logger.New(logger.Config{
		Level:  logger.LevelInfo,
		Output: &buf,
	})

	// Create a reader with multi-line content
	input := strings.NewReader("Line 1\nLine 2\nLine 3\n")

	// Stream the input
	err := logger.StreamReader(input, log, func(line string) {
		streamLines = append(streamLines, line)
	})

	if err != nil {
		t.Fatalf("StreamReader returned error: %v", err)
	}

	if len(streamLines) != 3 {
		t.Errorf("Expected 3 lines, got %d", len(streamLines))
	}

	if streamLines[0] != "Line 1" {
		t.Errorf("Line 0 = %q, want 'Line 1'", streamLines[0])
	}

	if streamLines[1] != "Line 2" {
		t.Errorf("Line 1 = %q, want 'Line 2'", streamLines[1])
	}

	if streamLines[2] != "Line 3" {
		t.Errorf("Line 2 = %q, want 'Line 3'", streamLines[2])
	}
}

// TestLogger_DefaultConfig verifies that default configuration values
// are applied when not specified.
func TestLogger_DefaultConfig(t *testing.T) {
	// Create logger with minimal config
	logger := logger.New(logger.Config{})

	// Should not panic - defaults should be applied
	logger.Info("Test with defaults")

	// Logger should be created successfully
	if logger == nil {
		t.Error("Logger should not be nil with default config")
	}
}

// TestLogger_InfoContext verifies that context-aware logging works.
func TestLogger_InfoContext(t *testing.T) {
	var buf bytes.Buffer

	logger := logger.New(logger.Config{
		Level:  logger.LevelInfo,
		Output: &buf,
	})

	ctx := context.Background()
	logger.InfoContext(ctx, "Context-aware message", "key", "value")

	output := buf.String()
	if !strings.Contains(output, "Context-aware message") {
		t.Error("Context-aware message should be logged")
	}

	if !strings.Contains(output, "key") || !strings.Contains(output, "value") {
		t.Error("Context-aware logging should include key-value pairs")
	}
}
