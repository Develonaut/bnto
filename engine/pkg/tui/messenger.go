//go:build tui

package tui

import (
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

// ProgressMessenger receives execution progress events.
// Used to display real-time progress in TUI or CLI mode.
type ProgressMessenger interface {
	// SendNodeStarted notifies that a node has started execution.
	// path: node path in tree (e.g., "0", "1.2", "0.1.3" or node ID)
	// name: human-readable node name
	// nodeType: node type (e.g., "http-request", "file-system")
	SendNodeStarted(path, name, nodeType string)

	// SendNodeCompleted notifies that a node has finished execution.
	// path: node path in tree
	// duration: how long the node took to execute
	// err: error if node failed, nil if successful
	SendNodeCompleted(path string, duration time.Duration, err error)

	// SendLoopChild notifies which child a loop is currently executing.
	// loopPath: path to the loop node
	// childName: name of the child being executed
	// index: current iteration index (0-based)
	// total: total number of iterations
	SendLoopChild(loopPath, childName string, index, total int)
}

// BubbletMessenger sends progress messages to a Bubbletea program.
// Used for TUI mode with real-time animated display.
type BubbletMessenger struct {
	program *tea.Program
}

// NewBubbletMessenger creates a messenger that sends to a Bubbletea program.
func NewBubbletMessenger(program *tea.Program) *BubbletMessenger {
	return &BubbletMessenger{
		program: program,
	}
}

// SendNodeStarted sends node start message to Bubbletea.
func (m *BubbletMessenger) SendNodeStarted(path, name, nodeType string) {
	if m.program != nil {
		m.program.Send(NodeStartedMsg{
			Path:     path,
			Name:     name,
			NodeType: nodeType,
		})
	}
}

// SendNodeCompleted sends node completion message to Bubbletea.
func (m *BubbletMessenger) SendNodeCompleted(path string, duration time.Duration, err error) {
	if m.program != nil {
		m.program.Send(NodeCompletedMsg{
			Path:     path,
			Duration: duration,
			Error:    err,
		})
	}
}

// SendLoopChild sends loop child execution message to Bubbletea.
func (m *BubbletMessenger) SendLoopChild(loopPath, childName string, index, total int) {
	if m.program != nil {
		m.program.Send(LoopChildMsg{
			LoopPath:  loopPath,
			ChildName: childName,
			Index:     index,
			Total:     total,
		})
	}
}

// SimpleMessenger prints simple progress updates for non-TTY mode.
// Used for CI/CD, pipes, and redirects where Bubbletea cannot run.
type SimpleMessenger struct {
	theme    *Theme
	palette  Palette
	nodeInfo map[string]nodeStartInfo // stores node info from start to completion
}

// nodeStartInfo stores information about a started node.
type nodeStartInfo struct {
	name     string
	nodeType string
}

// NewSimpleMessenger creates a messenger for simple progress output.
func NewSimpleMessenger(theme *Theme, palette Palette) *SimpleMessenger {
	return &SimpleMessenger{
		theme:    theme,
		palette:  palette,
		nodeInfo: make(map[string]nodeStartInfo),
	}
}

// SendNodeStarted stores node start information (doesn't print yet).
func (m *SimpleMessenger) SendNodeStarted(path, name, nodeType string) {
	// Store node info for when it completes
	m.nodeInfo[path] = nodeStartInfo{
		name:     name,
		nodeType: nodeType,
	}
}

// SendNodeCompleted prints the complete node execution line.
func (m *SimpleMessenger) SendNodeCompleted(path string, duration time.Duration, err error) {
	// Get stored node info
	info, ok := m.nodeInfo[path]
	if !ok {
		// Fallback if we don't have the info
		info = nodeStartInfo{
			name: path,
		}
	}

	// Clean up stored info
	delete(m.nodeInfo, path)

	if err != nil {
		// Print: Failed NODE:file-system Create Product Directory... (error message)
		statusWord := getStatusLabel(StepFailed, info.name)
		fmt.Printf("  %s NODE:%s %s... %s\n",
			m.theme.Error.Render(statusWord),
			info.nodeType,
			info.name,
			m.theme.Error.Render(err.Error()))
	} else {
		// Print: Perfected NODE:spreadsheet Read Products CSV... (2ms)
		statusWord := getStatusLabel(StepCompleted, info.name)
		durationStr := formatSimpleDuration(duration)
		fmt.Printf("  %s NODE:%s %s... %s\n",
			m.theme.Success.Render(statusWord),
			info.nodeType,
			info.name,
			m.theme.Subtle.Render(fmt.Sprintf("(%s)", durationStr)))
	}
}

// SendLoopChild is a no-op for SimpleMessenger (no real-time progress).
func (m *SimpleMessenger) SendLoopChild(loopPath, childName string, index, total int) {
	// No-op: Simple mode doesn't show real-time child execution
}

// formatSimpleDuration formats duration for simple display.
func formatSimpleDuration(d time.Duration) string {
	if d < time.Second {
		return fmt.Sprintf("%dms", d.Milliseconds())
	}
	if d < time.Minute {
		return fmt.Sprintf("%.1fs", d.Seconds())
	}
	mins := int(d.Minutes())
	secs := int(d.Seconds()) % 60
	return fmt.Sprintf("%dm %ds", mins, secs)
}

// CallbackMessenger sends log messages via a callback function.
// Used for async TUI execution where we can't access tea.Program directly.
type CallbackMessenger struct {
	theme    *Theme
	palette  Palette
	nodeInfo map[string]nodeStartInfo
	onLog    func(string) // Callback to send log messages
}

// NewCallbackMessenger creates a messenger that sends logs via callback.
func NewCallbackMessenger(theme *Theme, palette Palette, onLog func(string)) *CallbackMessenger {
	return &CallbackMessenger{
		theme:    theme,
		palette:  palette,
		nodeInfo: make(map[string]nodeStartInfo),
		onLog:    onLog,
	}
}

// SendNodeStarted stores node start information.
func (m *CallbackMessenger) SendNodeStarted(path, name, nodeType string) {
	m.nodeInfo[path] = nodeStartInfo{
		name:     name,
		nodeType: nodeType,
	}
}

// SendNodeCompleted formats and sends log via callback.
func (m *CallbackMessenger) SendNodeCompleted(path string, duration time.Duration, err error) {
	info, ok := m.nodeInfo[path]
	if !ok {
		info = nodeStartInfo{name: path}
	}
	delete(m.nodeInfo, path)

	var logLine string
	if err != nil {
		statusWord := getStatusLabel(StepFailed, info.name)
		logLine = fmt.Sprintf("  %s NODE:%s %s... %s\n",
			m.theme.Error.Render(statusWord),
			info.nodeType,
			info.name,
			m.theme.Error.Render(err.Error()))
	} else {
		statusWord := getStatusLabel(StepCompleted, info.name)
		durationStr := formatSimpleDuration(duration)
		logLine = fmt.Sprintf("  %s NODE:%s %s... %s\n",
			m.theme.Success.Render(statusWord),
			info.nodeType,
			info.name,
			m.theme.Subtle.Render(fmt.Sprintf("(%s)", durationStr)))
	}

	if m.onLog != nil {
		m.onLog(logLine)
	}
}

// SendLoopChild is a no-op for CallbackMessenger.
func (m *CallbackMessenger) SendLoopChild(loopPath, childName string, index, total int) {
	// No-op for now
}
