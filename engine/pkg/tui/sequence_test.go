// Package tui provides terminal styling and progress display.
//
// Tests for step sequence rendering.
package tui

import (
	"strings"
	"testing"
	"time"
)

// TestNewSequence verifies sequence initialization.
func TestNewSequence(t *testing.T) {
	seq := NewSequence()

	// Should start with no steps
	output := seq.View()
	if output != "" {
		t.Errorf("NewSequence().View() = %q, want empty", output)
	}
}

// TestAddStep verifies step addition.
func TestAddStep(t *testing.T) {
	seq := NewSequence()
	seq.AddStep("test-step", "test-type")

	steps := seq.GetSteps()
	if len(steps) != 1 {
		t.Fatalf("After AddStep, got %d steps, want 1", len(steps))
	}

	if steps[0].Name != "test-step" {
		t.Errorf("Step name = %q, want %q", steps[0].Name, "test-step")
	}
	if steps[0].Type != "test-type" {
		t.Errorf("Step type = %q, want %q", steps[0].Type, "test-type")
	}
	if steps[0].Status != StepPending {
		t.Errorf("New step status = %v, want StepPending", steps[0].Status)
	}
}

// TestUpdateStep verifies step status updates.
func TestUpdateStep(t *testing.T) {
	seq := NewSequence()
	seq.AddStep("test-step", "test-type")

	// Update to running
	seq.UpdateStep("test-step", StepRunning)
	steps := seq.GetSteps()
	if steps[0].Status != StepRunning {
		t.Errorf("After UpdateStep(Running), status = %v, want StepRunning", steps[0].Status)
	}

	// Update to completed
	seq.UpdateStep("test-step", StepCompleted)
	steps = seq.GetSteps()
	if steps[0].Status != StepCompleted {
		t.Errorf("After UpdateStep(Completed), status = %v, want StepCompleted", steps[0].Status)
	}
}

// TestView_EmptySequence verifies empty sequence rendering.
func TestView_EmptySequence(t *testing.T) {
	seq := NewSequence()
	output := seq.View()

	if output != "" {
		t.Errorf("Empty sequence View() = %q, want empty string", output)
	}
}

// TestView_SingleStep verifies single step rendering.
func TestView_SingleStep(t *testing.T) {
	seq := NewSequence()
	seq.AddStep("test-step", "test-type")
	seq.UpdateStep("test-step", StepRunning)

	output := seq.View()
	if output == "" {
		t.Error("View() returned empty string")
	}

	// Should contain step name
	if !strings.Contains(output, "test-step") {
		t.Errorf("View() output should contain step name: %s", output)
	}
}

// TestView_MultipleSteps verifies multiple step rendering.
func TestView_MultipleSteps(t *testing.T) {
	seq := NewSequence()
	seq.AddStep("step-1", "type-1")
	seq.AddStep("step-2", "type-2")
	seq.AddStep("step-3", "type-3")

	output := seq.View()

	// Should contain all step names
	if !strings.Contains(output, "step-1") {
		t.Error("Output missing step-1")
	}
	if !strings.Contains(output, "step-2") {
		t.Error("Output missing step-2")
	}
	if !strings.Contains(output, "step-3") {
		t.Error("Output missing step-3")
	}
}

// TestStepDuration verifies duration tracking.
func TestStepDuration(t *testing.T) {
	seq := NewSequence()
	seq.AddStep("test-step", "test-type")

	// Set duration
	seq.SetDuration("test-step", 1500*time.Millisecond)

	steps := seq.GetSteps()
	if steps[0].Duration != 1500*time.Millisecond {
		t.Errorf("Step duration = %v, want 1.5s", steps[0].Duration)
	}
}

// TestStepDepth verifies nesting/indentation.
func TestStepDepth(t *testing.T) {
	seq := NewSequence()
	seq.AddStepWithDepth("parent", "group", 0)
	seq.AddStepWithDepth("child-1", "node", 1)
	seq.AddStepWithDepth("child-2", "node", 1)
	seq.AddStepWithDepth("grandchild", "node", 2)

	steps := seq.GetSteps()

	if steps[0].Depth != 0 {
		t.Errorf("Parent depth = %d, want 0", steps[0].Depth)
	}
	if steps[1].Depth != 1 {
		t.Errorf("Child depth = %d, want 1", steps[1].Depth)
	}
	if steps[3].Depth != 2 {
		t.Errorf("Grandchild depth = %d, want 2", steps[3].Depth)
	}
}

// TestStatusWordVariety verifies deterministic status word selection.
func TestStatusWordVariety(t *testing.T) {
	// Same step should get same status word
	word1 := getStatusLabel(StepRunning, "test-step")
	word2 := getStatusLabel(StepRunning, "test-step")

	if word1 != word2 {
		t.Errorf("Status word not deterministic: %s != %s", word1, word2)
	}

	// Different steps should likely get different words (not guaranteed)
	word3 := getStatusLabel(StepRunning, "different-step")
	// Just verify it returns a valid word (don't require uniqueness)
	if word3 == "" {
		t.Error("Status word is empty")
	}
}
