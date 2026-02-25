//go:build tui

// Package tui provides terminal styling and progress display.
//
// Tests for Bubbletea progress display model.
package tui

import (
	"strings"
	"testing"
	"time"

	"github.com/charmbracelet/x/exp/teatest"
)

// TestNewProgress verifies progress model initialization.
func TestNewProgress(t *testing.T) {
	prog := NewProgress()

	// Verify sequence is initialized
	if prog.sequence == nil {
		t.Fatal("Progress sequence is nil")
	}

	// Should start with empty sequence
	if len(prog.sequence.GetSteps()) != 0 {
		t.Errorf("New progress should have 0 steps, got %d", len(prog.sequence.GetSteps()))
	}

	// Should not be done initially
	if prog.done {
		t.Error("New progress should not be done")
	}
}

// TestProgress_AddStep verifies step addition.
func TestProgress_AddStep(t *testing.T) {
	prog := NewProgress()
	prog.AddStep("test-step", "test-type")

	steps := prog.sequence.GetSteps()
	if len(steps) != 1 {
		t.Fatalf("After AddStep, got %d steps, want 1", len(steps))
	}

	if steps[0].Name != "test-step" {
		t.Errorf("Step name = %q, want %q", steps[0].Name, "test-step")
	}
}

// TestProgress_UpdateStep verifies step updates.
func TestProgress_UpdateStep(t *testing.T) {
	prog := NewProgress()
	prog.AddStep("test-step", "test-type")

	prog.UpdateStep("test-step", StepRunning)
	steps := prog.sequence.GetSteps()
	if steps[0].Status != StepRunning {
		t.Errorf("Status = %v, want StepRunning", steps[0].Status)
	}

	prog.UpdateStep("test-step", StepCompleted)
	steps = prog.sequence.GetSteps()
	if steps[0].Status != StepCompleted {
		t.Errorf("Status = %v, want StepCompleted", steps[0].Status)
	}
}

// TestProgress_Done verifies done signal.
func TestProgress_Done(t *testing.T) {
	prog := NewProgress()

	if prog.done {
		t.Error("Progress should not start done")
	}

	prog.Done()

	if !prog.done {
		t.Error("Progress should be done after Done() call")
	}
}

// TestProgress_View verifies rendering.
func TestProgress_View(t *testing.T) {
	prog := NewProgress()
	prog.AddStep("step-1", "type-1")
	prog.UpdateStep("step-1", StepRunning)

	view := prog.View()

	if view == "" {
		t.Error("View() should not be empty")
	}

	// Should contain the step name
	if !strings.Contains(view, "step-1") {
		t.Errorf("View should contain step name: %s", view)
	}
}

// TestProgress_BubbletaModel verifies Bubbletea integration using teatest.
func TestProgress_BubbletaModel(t *testing.T) {
	prog := NewProgress()
	prog.AddStep("test-step", "test-type")

	// Create teatest model
	tm := teatest.NewTestModel(t, prog, teatest.WithInitialTermSize(80, 24))

	// Send an update message
	updateMsg := StepUpdateMsg{
		Name:   "test-step",
		Status: StepRunning,
	}
	tm.Send(updateMsg)

	// Give time for update to process
	time.Sleep(time.Millisecond * 50)

	// Send quit
	tm.Send(DoneMsg{})

	// Wait for quit to process
	time.Sleep(time.Millisecond * 50)

	// Quit the model
	_ = tm.Quit()

	// Verify the final model state rather than terminal output
	finalModel := tm.FinalModel(t)
	finalProg, ok := finalModel.(Progress)
	if !ok {
		t.Fatalf("Final model is not Progress type")
	}

	// Verify model state
	if !finalProg.done {
		t.Error("Progress should be marked as done")
	}

	steps := finalProg.sequence.GetSteps()
	if len(steps) != 1 {
		t.Fatalf("Expected 1 step, got %d", len(steps))
	}

	if steps[0].Name != "test-step" {
		t.Errorf("Step name = %q, want %q", steps[0].Name, "test-step")
	}

	if steps[0].Status != StepRunning {
		t.Errorf("Step status = %v, want StepRunning", steps[0].Status)
	}
}

// TestProgress_DoneMessage verifies done message handling.
func TestProgress_DoneMessage(t *testing.T) {
	prog := NewProgress()

	// Send done message via Update
	updatedModel, cmd := prog.Update(DoneMsg{})

	// Model should be marked as done
	updatedProg := updatedModel.(Progress)
	if !updatedProg.done {
		t.Error("Progress should be done after DoneMsg")
	}

	// Should return quit command
	if cmd == nil {
		t.Error("Update should return quit command")
	}
}

// TestProgress_Init verifies initialization.
func TestProgress_Init(t *testing.T) {
	prog := NewProgress()
	cmd := prog.Init()

	// Init should return nil (no initial command)
	if cmd != nil {
		t.Error("Init() should return nil command")
	}
}

// TestProgress_MultipleSteps verifies multiple step handling.
func TestProgress_MultipleSteps(t *testing.T) {
	prog := NewProgress()

	// Add multiple steps
	prog.AddStep("step-1", "type-1")
	prog.AddStep("step-2", "type-2")
	prog.AddStep("step-3", "type-3")

	steps := prog.sequence.GetSteps()
	if len(steps) != 3 {
		t.Fatalf("Expected 3 steps, got %d", len(steps))
	}

	// Update each step
	prog.UpdateStep("step-1", StepCompleted)
	prog.UpdateStep("step-2", StepRunning)
	prog.UpdateStep("step-3", StepPending)

	// Verify states
	steps = prog.sequence.GetSteps()
	if steps[0].Status != StepCompleted {
		t.Errorf("step-1 status = %v, want StepCompleted", steps[0].Status)
	}
	if steps[1].Status != StepRunning {
		t.Errorf("step-2 status = %v, want StepRunning", steps[1].Status)
	}
	if steps[2].Status != StepPending {
		t.Errorf("step-3 status = %v, want StepPending", steps[2].Status)
	}
}
