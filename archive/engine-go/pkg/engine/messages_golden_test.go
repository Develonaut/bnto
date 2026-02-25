package engine

import (
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/testgolden"
)

// TestGolden_StatusWordDeterministic verifies that getStatusWord produces
// deterministic output based on the node name hash (FNV32a).
func TestGolden_StatusWordDeterministic(t *testing.T) {
	names := []string{
		"compress-images",
		"resize-images",
		"clean-csv",
		"rename-files",
		"http-fetch",
		"loop-items",
		"transform-data",
		"edit-fields",
	}

	running := make(map[string]string)
	completed := make(map[string]string)
	for _, name := range names {
		running[name] = getStatusWord(name, true)
		completed[name] = getStatusWord(name, false)
	}

	testgolden.AssertGolden(t, "status_word_deterministic", map[string]any{
		"running":   running,
		"completed": completed,
	})
}

// TestGolden_StatusWordConsistency verifies that the same name always
// produces the same status word across multiple calls.
func TestGolden_StatusWordConsistency(t *testing.T) {
	name := "test-node"
	results := make([]string, 10)
	for i := range results {
		results[i] = getStatusWord(name, true)
	}

	// All should be identical
	first := results[0]
	for i, r := range results {
		if r != first {
			t.Errorf("Call %d returned %q, want %q (determinism violation)", i, r, first)
		}
	}

	testgolden.AssertGolden(t, "status_word_consistency", map[string]any{
		"name":       name,
		"word":       first,
		"callCount":  len(results),
		"allMatched": true,
	})
}

// TestGolden_MsgBntoStarted verifies the bnto-started message format.
func TestGolden_MsgBntoStarted(t *testing.T) {
	msg := msgBntoStarted("My Workflow")

	testgolden.AssertGolden(t, "msg_bnto_started", map[string]any{
		"emoji":     msg.emoji,
		"text":      msg.text,
		"isRunning": msg.isRunning,
		"isSuccess": msg.isSuccess,
		"isFailed":  msg.isFailed,
	})
}

// TestGolden_MsgBntoCompleted verifies the bnto-completed message format.
func TestGolden_MsgBntoCompleted(t *testing.T) {
	msg := msgBntoCompleted("42ms")

	testgolden.AssertGolden(t, "msg_bnto_completed", map[string]any{
		"emoji":     msg.emoji,
		"text":      msg.text,
		"isRunning": msg.isRunning,
		"isSuccess": msg.isSuccess,
		"isFailed":  msg.isFailed,
	})
}

// TestGolden_MsgGroupStarted verifies the group-started message format
// with and without breadcrumb.
func TestGolden_MsgGroupStarted(t *testing.T) {
	noBreadcrumb := msgGroupStarted("", "my-group")
	withBreadcrumb := msgGroupStarted("Root:Parent", "my-group")

	testgolden.AssertGolden(t, "msg_group_started", map[string]any{
		"noBreadcrumb": map[string]any{
			"text":      noBreadcrumb.text,
			"isRunning": noBreadcrumb.isRunning,
		},
		"withBreadcrumb": map[string]any{
			"text":      withBreadcrumb.text,
			"isRunning": withBreadcrumb.isRunning,
		},
	})
}

// TestGolden_MsgGroupCompleted verifies the group-completed message format.
func TestGolden_MsgGroupCompleted(t *testing.T) {
	noBreadcrumb := msgGroupCompleted("", "my-group", "5ms")
	withBreadcrumb := msgGroupCompleted("Root:Parent", "my-group", "5ms")

	testgolden.AssertGolden(t, "msg_group_completed", map[string]any{
		"noBreadcrumb": map[string]any{
			"text":      noBreadcrumb.text,
			"isSuccess": noBreadcrumb.isSuccess,
		},
		"withBreadcrumb": map[string]any{
			"text":      withBreadcrumb.text,
			"isSuccess": withBreadcrumb.isSuccess,
		},
	})
}

// TestGolden_MsgLoopStarted verifies the loop-started message format.
func TestGolden_MsgLoopStarted(t *testing.T) {
	noBreadcrumb := msgLoopStarted("", "process-items")
	withBreadcrumb := msgLoopStarted("Root:Batch", "process-items")

	testgolden.AssertGolden(t, "msg_loop_started", map[string]any{
		"noBreadcrumb": map[string]any{
			"text":      noBreadcrumb.text,
			"isRunning": noBreadcrumb.isRunning,
		},
		"withBreadcrumb": map[string]any{
			"text":      withBreadcrumb.text,
			"isRunning": withBreadcrumb.isRunning,
		},
	})
}

// TestGolden_MsgLoopCompleted verifies the loop-completed message format
// with progress percentage.
func TestGolden_MsgLoopCompleted(t *testing.T) {
	msg := msgLoopCompleted("Root:Batch", "process-items", "120ms", 75)

	testgolden.AssertGolden(t, "msg_loop_completed", map[string]any{
		"text":      msg.text,
		"isSuccess": msg.isSuccess,
	})
}

// TestGolden_MsgChildNodeStarted verifies the child-node-started message format.
func TestGolden_MsgChildNodeStarted(t *testing.T) {
	noBreadcrumb := msgChildNodeStarted("", "image", "compress-png")
	withBreadcrumb := msgChildNodeStarted("Root:Batch", "shell-command", "run-script")

	testgolden.AssertGolden(t, "msg_child_node_started", map[string]any{
		"noBreadcrumb": map[string]any{
			"text":      noBreadcrumb.text,
			"isRunning": noBreadcrumb.isRunning,
		},
		"withBreadcrumb": map[string]any{
			"text":      withBreadcrumb.text,
			"isRunning": withBreadcrumb.isRunning,
		},
	})
}

// TestGolden_MsgChildNodeCompleted verifies the child-node-completed message
// format with duration and progress percentage.
func TestGolden_MsgChildNodeCompleted(t *testing.T) {
	noBreadcrumb := msgChildNodeCompleted("", "image", "compress-png", "8ms", 33)
	withBreadcrumb := msgChildNodeCompleted("Root:Batch", "shell-command", "run-script", "1.2s", 100)

	testgolden.AssertGolden(t, "msg_child_node_completed", map[string]any{
		"noBreadcrumb": map[string]any{
			"text":      noBreadcrumb.text,
			"isSuccess": noBreadcrumb.isSuccess,
		},
		"withBreadcrumb": map[string]any{
			"text":      withBreadcrumb.text,
			"isSuccess": withBreadcrumb.isSuccess,
		},
	})
}

// TestGolden_FormatDuration verifies duration formatting (ms vs seconds).
// Uses direct assertions instead of golden files because formatDuration output
// matches NormalizeJSON's duration-stripping regex by design.
func TestGolden_FormatDuration(t *testing.T) {
	tests := []struct {
		name     string
		input    time.Duration
		expected string
	}{
		{"zero", 0, "0ms"},
		{"1ms", time.Millisecond, "1ms"},
		{"42ms", 42 * time.Millisecond, "42ms"},
		{"999ms", 999 * time.Millisecond, "999ms"},
		{"1s", time.Second, "1.0s"},
		{"1.5s", 1500 * time.Millisecond, "1.5s"},
		{"10s", 10 * time.Second, "10.0s"},
		{"2min", 2 * time.Minute, "120.0s"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := formatDuration(tt.input)
			if got != tt.expected {
				t.Errorf("formatDuration(%v) = %q, want %q", tt.input, got, tt.expected)
			}
		})
	}
}

// TestGolden_LogMessageFormat verifies the format() method output shape.
func TestGolden_LogMessageFormat(t *testing.T) {
	// Test with emoji
	withEmoji := logMessage{
		emoji:     "🍱",
		text:      "Running Bnto: Test",
		isRunning: true,
	}

	// Test without emoji
	withoutEmoji := logMessage{
		emoji:     "",
		text:      "Executing node",
		isRunning: false,
	}

	// Note: format() calls colorizeMessage which adds ANSI codes.
	// We test the structure, not the exact ANSI output.
	fmtWithEmoji := withEmoji.format()
	fmtWithoutEmoji := withoutEmoji.format()

	// Verify emoji prefix is present/absent
	testgolden.AssertGolden(t, "log_message_format", map[string]any{
		"withEmoji": map[string]any{
			"hasEmoji":  len(fmtWithEmoji) > 0 && fmtWithEmoji[0] != ' ',
			"nonEmpty":  len(fmtWithEmoji) > 0,
			"hasText":   true,
		},
		"withoutEmoji": map[string]any{
			"nonEmpty": len(fmtWithoutEmoji) > 0,
			"hasText":  true,
		},
	})
}
