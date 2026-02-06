package tui

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"
	"testing"
	"time"
)

// TestSimpleMessenger_Success tests successful node completion.
func TestSimpleMessenger_Success(t *testing.T) {
	// Capture stdout
	old := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	// Create messenger
	manager := NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := NewSimpleMessenger(theme, palette)

	// Send node started
	messenger.SendNodeStarted("test-node", "Test Operation", "edit-fields")

	// Send node completed
	messenger.SendNodeCompleted("test-node", 5*time.Millisecond, nil)

	// Restore stdout and read output
	w.Close()
	os.Stdout = old
	var buf bytes.Buffer
	_, _ = io.Copy(&buf, r)
	output := buf.String()

	// Verify output format
	if !strings.Contains(output, "Test Operation") {
		t.Errorf("Expected output to contain node name 'Test Operation', got: %s", output)
	}

	if !strings.Contains(output, "5ms") {
		t.Errorf("Expected output to contain duration '5ms', got: %s", output)
	}

	// Should have emoji
	hasEmoji := false
	for _, emoji := range Sushi {
		if strings.Contains(output, emoji) {
			hasEmoji = true
			break
		}
	}
	if !hasEmoji {
		t.Errorf("Expected output to contain sushi emoji, got: %s", output)
	}

	// Should have completed status word
	hasStatusWord := false
	for _, word := range StatusWordsCompleted {
		if strings.Contains(output, word) {
			hasStatusWord = true
			break
		}
	}
	if !hasStatusWord {
		t.Errorf("Expected output to contain completed status word, got: %s", output)
	}

	// Should have indentation (2 spaces)
	if !strings.HasPrefix(strings.TrimLeft(output, "\n"), "  ") {
		t.Errorf("Expected output to have 2-space indentation, got: %s", output)
	}

	// Should have ellipsis
	if !strings.Contains(output, "…") {
		t.Errorf("Expected output to contain ellipsis '…', got: %s", output)
	}
}

// TestSimpleMessenger_Failure tests failed node completion.
func TestSimpleMessenger_Failure(t *testing.T) {
	// Capture stdout
	old := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	// Create messenger
	manager := NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := NewSimpleMessenger(theme, palette)

	// Send node started
	messenger.SendNodeStarted("fail-node", "Failing Operation", "file-system")

	// Send node completed with error
	err := fmt.Errorf("file not found")
	messenger.SendNodeCompleted("fail-node", 2*time.Millisecond, err)

	// Restore stdout and read output
	w.Close()
	os.Stdout = old
	var buf bytes.Buffer
	_, _ = io.Copy(&buf, r)
	output := buf.String()

	// Verify output format
	if !strings.Contains(output, "Failing Operation") {
		t.Errorf("Expected output to contain node name 'Failing Operation', got: %s", output)
	}

	if !strings.Contains(output, "file not found") {
		t.Errorf("Expected output to contain error message 'file not found', got: %s", output)
	}

	// Should have error emoji
	errorEmojis := []string{"👹", "👺", "💀", "☠️", "💥", "🔥", "⚠️", "❌", "🚫", "🤢"}
	hasEmoji := false
	for _, emoji := range errorEmojis {
		if strings.Contains(output, emoji) {
			hasEmoji = true
			break
		}
	}
	if !hasEmoji {
		t.Errorf("Expected output to contain error emoji, got: %s", output)
	}

	// Should have failed status word
	hasStatusWord := false
	for _, word := range StatusWordsFailed {
		if strings.Contains(output, word) {
			hasStatusWord = true
			break
		}
	}
	if !hasStatusWord {
		t.Errorf("Expected output to contain failed status word, got: %s", output)
	}

	// Should have indentation (2 spaces)
	if !strings.HasPrefix(strings.TrimLeft(output, "\n"), "  ") {
		t.Errorf("Expected output to have 2-space indentation, got: %s", output)
	}
}

// TestSimpleMessenger_MultipleNodes tests multiple node executions.
func TestSimpleMessenger_MultipleNodes(t *testing.T) {
	// Capture stdout
	old := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	// Create messenger
	manager := NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := NewSimpleMessenger(theme, palette)

	// Execute multiple nodes
	nodes := []struct {
		id   string
		name string
	}{
		{"node-1", "First Operation"},
		{"node-2", "Second Operation"},
		{"node-3", "Third Operation"},
	}

	for _, node := range nodes {
		messenger.SendNodeStarted(node.id, node.name, "test")
		messenger.SendNodeCompleted(node.id, 1*time.Millisecond, nil)
	}

	// Restore stdout and read output
	w.Close()
	os.Stdout = old
	var buf bytes.Buffer
	_, _ = io.Copy(&buf, r)
	output := buf.String()

	// Verify all nodes appear in output
	for _, node := range nodes {
		if !strings.Contains(output, node.name) {
			t.Errorf("Expected output to contain '%s', got: %s", node.name, output)
		}
	}

	// Count lines (should have 3 lines, one per node)
	lines := strings.Split(strings.TrimSpace(output), "\n")
	if len(lines) != 3 {
		t.Errorf("Expected 3 output lines, got %d: %v", len(lines), lines)
	}
}

// TestSimpleMessenger_NoInfo tests fallback when node info is missing.
func TestSimpleMessenger_NoInfo(t *testing.T) {
	// Capture stdout
	old := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	// Create messenger
	manager := NewManager()
	theme := manager.GetTheme()
	palette := manager.GetPalette()
	messenger := NewSimpleMessenger(theme, palette)

	// Send completion without start (edge case)
	messenger.SendNodeCompleted("unknown-node", 10*time.Millisecond, nil)

	// Restore stdout and read output
	w.Close()
	os.Stdout = old
	var buf bytes.Buffer
	_, _ = io.Copy(&buf, r)
	output := buf.String()

	// Should still produce output with fallback
	if !strings.Contains(output, "unknown-node") {
		t.Errorf("Expected output to contain 'unknown-node' as fallback name, got: %s", output)
	}

	if !strings.Contains(output, "10ms") {
		t.Errorf("Expected output to contain duration '10ms', got: %s", output)
	}
}

// TestFormatSimpleDuration tests duration formatting.
func TestFormatSimpleDuration(t *testing.T) {
	tests := []struct {
		duration time.Duration
		expected string
	}{
		{5 * time.Millisecond, "5ms"},
		{500 * time.Millisecond, "500ms"},
		{1 * time.Second, "1.0s"},
		{1500 * time.Millisecond, "1.5s"},
		{90 * time.Second, "1m 30s"},
		{125 * time.Second, "2m 5s"},
	}

	for _, tt := range tests {
		result := formatSimpleDuration(tt.duration)
		if result != tt.expected {
			t.Errorf("formatSimpleDuration(%v) = %s, expected %s", tt.duration, result, tt.expected)
		}
	}
}
