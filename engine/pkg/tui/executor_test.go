package tui

import (
	"context"
	"fmt"
	"io"
	"strings"
	"testing"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/x/exp/teatest"

	"github.com/Develonaut/bento/pkg/engine"
	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/registry"
)

// TestExecutorSingleNodeSuccess tests successful execution of a single node.
func TestExecutorSingleNodeSuccess(t *testing.T) {
	// Create a simple bento with one node
	def := &node.Definition{
		ID:   "test-bento",
		Name: "Test Bento",
		Type: "test-node",
		Parameters: map[string]interface{}{
			"value": "hello",
		},
	}

	// Create executor
	theme := BuildTheme(GetPalette(VariantTonkotsu))
	palette := GetPalette(VariantTonkotsu)
	model := NewExecutor(def, theme, palette)

	// Create test model
	tm := teatest.NewTestModel(t, model, teatest.WithInitialTermSize(80, 24))
	defer func() { _ = tm.Quit() }()

	// Send initialization
	tm.Send(ExecutionInitMsg{Definition: def})
	waitForRender(tm)

	// Send node started
	tm.Send(NodeStartedMsg{
		Path:     "test-bento",
		Name:     "Test Bento",
		NodeType: "test-node",
	})
	waitForRender(tm)

	// Send node completed
	tm.Send(NodeCompletedMsg{
		Path:     "test-bento",
		Duration: 50 * time.Millisecond,
		Error:    nil,
	})
	waitForRender(tm)

	// Send execution complete
	tm.Send(ExecutionCompleteMsg{
		Success: true,
		Error:   nil,
	})

	// Wait for completion and quit
	tm.WaitFinished(t, teatest.WithFinalTimeout(2*time.Second))

	// Verify final output contains node name
	output := readOutput(t, tm.FinalOutput(t))
	if !strings.Contains(output, "Test Bento") {
		t.Errorf("Expected output to contain 'Test Bento', got: %s", output)
	}
}

// TestExecutorMultipleNodes tests execution of multiple nodes.
func TestExecutorMultipleNodes(t *testing.T) {
	// Create a bento with multiple nodes
	def := &node.Definition{
		ID:   "multi-bento",
		Name: "Multi Node Bento",
		Type: "group",
		Nodes: []node.Definition{
			{
				ID:   "node1",
				Name: "First Node",
				Type: "test-node",
			},
			{
				ID:   "node2",
				Name: "Second Node",
				Type: "test-node",
			},
			{
				ID:   "node3",
				Name: "Third Node",
				Type: "test-node",
			},
		},
	}

	theme := BuildTheme(GetPalette(VariantTonkotsu))
	palette := GetPalette(VariantTonkotsu)
	model := NewExecutor(def, theme, palette)

	tm := teatest.NewTestModel(t, model, teatest.WithInitialTermSize(80, 24))
	defer func() { _ = tm.Quit() }()

	// Initialize
	tm.Send(ExecutionInitMsg{Definition: def})
	waitForRender(tm)

	// Execute each node
	nodes := []struct {
		id   string
		name string
	}{
		{"node1", "First Node"},
		{"node2", "Second Node"},
		{"node3", "Third Node"},
	}

	for _, node := range nodes {
		// Start node
		tm.Send(NodeStartedMsg{
			Path:     node.id,
			Name:     node.name,
			NodeType: "test-node",
		})
		waitForRender(tm)

		// Complete node
		tm.Send(NodeCompletedMsg{
			Path:     node.id,
			Duration: 10 * time.Millisecond,
			Error:    nil,
		})
		waitForRender(tm)
	}

	// Complete execution
	tm.Send(ExecutionCompleteMsg{
		Success: true,
		Error:   nil,
	})

	tm.WaitFinished(t, teatest.WithFinalTimeout(2*time.Second))

	// Verify all nodes appear in output
	output := readOutput(t, tm.FinalOutput(t))
	for _, node := range nodes {
		if !strings.Contains(output, node.name) {
			t.Errorf("Expected output to contain '%s', got: %s", node.name, output)
		}
	}
}

// TestExecutorFailure tests execution failure handling.
func TestExecutorFailure(t *testing.T) {
	def := &node.Definition{
		ID:   "failing-bento",
		Name: "Failing Bento",
		Type: "test-node",
	}

	theme := BuildTheme(GetPalette(VariantTonkotsu))
	palette := GetPalette(VariantTonkotsu)
	model := NewExecutor(def, theme, palette)

	tm := teatest.NewTestModel(t, model, teatest.WithInitialTermSize(80, 24))
	defer func() { _ = tm.Quit() }()

	// Initialize
	tm.Send(ExecutionInitMsg{Definition: def})
	waitForRender(tm)

	// Start node
	tm.Send(NodeStartedMsg{
		Path:     "failing-bento",
		Name:     "Failing Bento",
		NodeType: "test-node",
	})
	waitForRender(tm)

	// Complete with error
	testError := fmt.Errorf("test execution error")
	tm.Send(NodeCompletedMsg{
		Path:     "failing-bento",
		Duration: 5 * time.Millisecond,
		Error:    testError,
	})
	waitForRender(tm)

	// Complete execution with failure
	tm.Send(ExecutionCompleteMsg{
		Success: false,
		Error:   testError,
	})

	tm.WaitFinished(t, teatest.WithFinalTimeout(2*time.Second))

	// Verify error handling - check final model state
	finalModel := tm.FinalModel(t)
	executor, ok := finalModel.(Executor)
	if !ok {
		t.Fatalf("Expected final model to be Executor")
	}
	if executor.Success() {
		t.Error("Expected executor to report failure")
	}
}

// TestExecutorWithRealItamae tests executor with actual engine execution.
func TestExecutorWithRealItamae(t *testing.T) {
	// Create registry and register test node
	p := registry.New()
	p.RegisterFactory("test-node", func() node.Executable {
		return &testExecutable{output: map[string]interface{}{"result": "success"}}
	})

	// Create bento definition
	def := &node.Definition{
		ID:   "real-test",
		Name: "Real Execution Test",
		Type: "test-node",
	}

	// Create executor with theme
	theme := BuildTheme(GetPalette(VariantTonkotsu))
	palette := GetPalette(VariantTonkotsu)
	model := NewExecutor(def, theme, palette)

	// Create Bubbletea program
	program := tea.NewProgram(model, tea.WithoutRenderer())

	// Create messenger
	messenger := NewBubbletMessenger(program)

	// Create engine with messenger (no logger for clean test)
	chef := engine.NewWithMessenger(p, nil, messenger)

	// Send init message
	program.Send(ExecutionInitMsg{Definition: def})

	// Execute in background
	go func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()

		_, err := chef.Serve(ctx, def)

		// Send completion
		program.Send(ExecutionCompleteMsg{
			Success: err == nil,
			Error:   err,
		})
	}()

	// Run program
	finalModel, err := program.Run()
	if err != nil {
		t.Fatalf("Program error: %v", err)
	}

	// Verify success
	executor := finalModel.(Executor)
	if !executor.Success() {
		t.Error("Expected execution to succeed")
	}
}

// TestExecutorNodeStateTransitions tests that nodes transition through states correctly.
func TestExecutorNodeStateTransitions(t *testing.T) {
	def := &node.Definition{
		ID:   "transition-test",
		Name: "State Transition Test",
		Type: "test-node",
	}

	theme := BuildTheme(GetPalette(VariantTonkotsu))
	palette := GetPalette(VariantTonkotsu)
	model := NewExecutor(def, theme, palette)

	tm := teatest.NewTestModel(t, model, teatest.WithInitialTermSize(80, 24))
	defer func() { _ = tm.Quit() }()

	// Initialize - should be pending
	tm.Send(ExecutionInitMsg{Definition: def})
	waitForRender(tm)

	output := readOutput(t, tm.FinalOutput(t))
	if !strings.Contains(output, "State Transition Test") {
		t.Errorf("Expected output to show node, got: %s", output)
	}

	// Start - should show running indicator
	tm.Send(NodeStartedMsg{
		Path:     "transition-test",
		Name:     "State Transition Test",
		NodeType: "test-node",
	})
	waitForRender(tm)

	output = readOutput(t, tm.FinalOutput(t))
	// Output should show some status word (deterministic)
	hasStatusWord := false
	statusWords := []string{"Tasting", "Sampling", "Trying", "Enjoying", "Devouring", "Nibbling", "Savoring", "Testing"}
	for _, word := range statusWords {
		if strings.Contains(output, word) {
			hasStatusWord = true
			break
		}
	}
	if !hasStatusWord {
		t.Errorf("Expected output to show status word, got: %s", output)
	}

	// Complete - should show completion
	tm.Send(NodeCompletedMsg{
		Path:     "transition-test",
		Duration: 100 * time.Millisecond,
		Error:    nil,
	})
	waitForRender(tm)

	tm.Send(ExecutionCompleteMsg{
		Success: true,
		Error:   nil,
	})

	tm.WaitFinished(t, teatest.WithFinalTimeout(2*time.Second))

	output = readOutput(t, tm.FinalOutput(t))
	// Should show duration
	if !strings.Contains(output, "ms") && !strings.Contains(output, "µs") {
		t.Errorf("Expected output to show duration, got: %s", output)
	}
}

// testExecutable is a mock node for testing.
type testExecutable struct {
	output map[string]interface{}
	err    error
}

func (t *testExecutable) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	// Simulate brief execution
	time.Sleep(10 * time.Millisecond)
	return t.output, t.err
}

// waitForRender waits for the next render cycle.
func waitForRender(tm *teatest.TestModel) {
	time.Sleep(50 * time.Millisecond)
}

// readOutput reads the output from an io.Reader and returns it as a string.
func readOutput(t *testing.T, reader io.Reader) string {
	t.Helper()
	data, err := io.ReadAll(reader)
	if err != nil {
		t.Fatalf("Failed to read output: %v", err)
	}
	return string(data)
}
