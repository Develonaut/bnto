// Package tui provides terminal styling and progress display.
//
// Step sequence rendering with status words and sushi emojis.
package tui

import (
	"fmt"
	"math/rand"
	"strings"
	"time"
)

// Note: Sushi emojis and status words are centralized in sushi.go

// StepStatus represents the execution status of a step.
type StepStatus int

const (
	StepPending StepStatus = iota
	StepRunning
	StepCompleted
	StepFailed
)

// Step represents a single step in a sequence.
type Step struct {
	Name         string
	Type         string
	Status       StepStatus
	Duration     time.Duration
	Depth        int    // Nesting level for indentation
	CurrentChild string // For loops: name of currently executing child
	ChildIndex   int    // For loops: current iteration index
	ChildTotal   int    // For loops: total iterations
}

// Sequence displays a list of execution steps with status indicators.
type Sequence struct {
	theme   *Theme
	palette Palette
	spinner Spinner
	steps   []Step
}

// NewSequence creates a new sequence display.
func NewSequence() *Sequence {
	// Use default Tonkotsu theme for standalone usage
	manager := NewManager()
	return NewSequenceWithTheme(manager.GetTheme(), manager.GetPalette())
}

// NewSequenceWithTheme creates a new sequence display with custom theme.
func NewSequenceWithTheme(theme *Theme, palette Palette) *Sequence {
	return &Sequence{
		theme:   theme,
		palette: palette,
		spinner: NewSpinner(palette),
		steps:   []Step{},
	}
}

// SetSteps replaces all steps (used for Bubbletea integration).
func (s *Sequence) SetSteps(steps []Step) {
	s.steps = steps
}

// AddStep adds a step with depth 0.
func (s *Sequence) AddStep(name, nodeType string) {
	s.AddStepWithDepth(name, nodeType, 0)
}

// AddStepWithDepth adds a step with specified nesting depth.
func (s *Sequence) AddStepWithDepth(name, nodeType string, depth int) {
	s.steps = append(s.steps, Step{
		Name:     name,
		Type:     nodeType,
		Status:   StepPending,
		Duration: 0,
		Depth:    depth,
	})
}

// UpdateStep updates the status of a step by name.
func (s *Sequence) UpdateStep(name string, status StepStatus) {
	for i := range s.steps {
		if s.steps[i].Name == name {
			s.steps[i].Status = status
			return
		}
	}
}

// SetDuration sets the duration for a step by name.
func (s *Sequence) SetDuration(name string, duration time.Duration) {
	for i := range s.steps {
		if s.steps[i].Name == name {
			s.steps[i].Duration = duration
			return
		}
	}
}

// GetSteps returns all steps in the sequence.
func (s *Sequence) GetSteps() []Step {
	return s.steps
}

// View renders the sequence of steps.
func (s *Sequence) View() string {
	if len(s.steps) == 0 {
		return ""
	}

	lines := []string{}
	for _, step := range s.steps {
		lines = append(lines, s.formatStep(step))

		// Show current child for running loops
		if step.Status == StepRunning && step.CurrentChild != "" {
			childLine := s.formatLoopChild(step)
			lines = append(lines, childLine)
		}
	}

	return strings.Join(lines, "\n")
}

// formatLoopChild renders the current child of a running loop.
func (s *Sequence) formatLoopChild(step Step) string {
	indent := strings.Repeat("  ", step.Depth+1)
	arrow := s.theme.Subtle.Render("→")
	childName := s.theme.Subtle.Render(step.CurrentChild)
	return fmt.Sprintf("%s%s %s", indent, arrow, childName)
}

// formatStep renders a single step with status and timing.
func (s *Sequence) formatStep(step Step) string {
	prefix := s.buildStepPrefix(step)
	emoji := s.getStepEmoji(step)
	status := s.buildStepStatus(step)
	suffix := buildStepSuffix(step)

	parts := []string{prefix}
	if emoji != "" {
		parts = append(parts, emoji)
	}
	parts = append(parts, status, step.Name)
	if suffix != "" {
		parts = append(parts, suffix)
	}

	return strings.Join(parts, " ")
}

// buildStepPrefix creates the indent/icon prefix for a step.
func (s *Sequence) buildStepPrefix(step Step) string {
	indent := strings.Repeat("  ", step.Depth)
	icon := s.getStepIcon(step.Status)

	if icon != "" {
		return indent + icon
	}

	return indent
}

// buildStepStatus creates the colored status word.
func (s *Sequence) buildStepStatus(step Step) string {
	statusWord := getStatusLabel(step.Status, step.Name)
	return s.colorStatusWord(statusWord, step.Status)
}

// buildStepSuffix creates the duration suffix if applicable.
func buildStepSuffix(step Step) string {
	if (step.Status == StepCompleted || step.Status == StepFailed) && step.Duration > 0 {
		return fmt.Sprintf("(%s)", step.Duration.Round(time.Millisecond).String())
	}
	return ""
}

// colorStatusWord colors the status word based on status using the sequence's theme.
func (s *Sequence) colorStatusWord(word string, status StepStatus) string {
	switch status {
	case StepRunning:
		// Use Primary color (bold) for running status
		return s.theme.Title.Render(word)
	case StepCompleted:
		return s.theme.Success.Render(word)
	case StepFailed:
		return s.theme.Error.Render(word)
	default:
		return s.theme.Subtle.Render(word)
	}
}

// getStatusLabel returns a random fun status word.
func getStatusLabel(status StepStatus, stepName string) string {
	switch status {
	case StepPending:
		return StatusWordPending
	case StepRunning:
		return StatusWordsRunning[rand.Intn(len(StatusWordsRunning))]
	case StepCompleted:
		return StatusWordsCompleted[rand.Intn(len(StatusWordsCompleted))]
	case StepFailed:
		return StatusWordsFailed[rand.Intn(len(StatusWordsFailed))]
	default:
		return StatusWordPending
	}
}

// getStepIcon returns the icon for a step status.
func (s *Sequence) getStepIcon(status StepStatus) string {
	switch status {
	case StepRunning:
		return s.spinner.View() // Animated spinner
	case StepCompleted:
		return "" // No icon - rely on emoji and colors
	case StepFailed:
		return "❌" // Red X for failures
	default:
		return "•" // Pending dot
	}
}

// getStepEmoji returns a random sushi emoji for completed steps.
func (s *Sequence) getStepEmoji(step Step) string {
	if step.Status != StepCompleted {
		return ""
	}

	return Sushi[rand.Intn(len(Sushi))]
}

// UpdateSpinner updates the spinner for animation (called from Bubbletea Update).
func (s *Sequence) UpdateSpinner(spinner Spinner) {
	s.spinner = spinner
}
