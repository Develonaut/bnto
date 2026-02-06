// Package miso provides terminal output "seasoning" - themed styling and progress display.
//
// Bubbletea progress display using daemon-combo pattern.
package tui

import (
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

// Progress is a Bubbletea model for displaying workflow progress.
// Uses daemon-combo pattern: foreground renders while background works.
type Progress struct {
	sequence *Sequence
	done     bool
}

// StepUpdateMsg is sent to update a step's status.
type StepUpdateMsg struct {
	Name   string
	Status StepStatus
}

// DoneMsg signals the progress display to quit.
type DoneMsg struct{}

// NewProgress creates a new progress display model.
func NewProgress() Progress {
	return Progress{
		sequence: NewSequence(),
		done:     false,
	}
}

// Init implements tea.Model.
func (p Progress) Init() tea.Cmd {
	return nil
}

// Update implements tea.Model.
func (p Progress) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case StepUpdateMsg:
		// Update step status
		p.sequence.UpdateStep(msg.Name, msg.Status)
		return p, nil

	case DoneMsg:
		// Mark as done and quit
		p.done = true
		return p, tea.Quit

	case tea.KeyMsg:
		// Allow Ctrl+C to quit
		if msg.String() == "ctrl+c" {
			return p, tea.Quit
		}
	}

	return p, nil
}

// View implements tea.Model.
func (p Progress) View() string {
	if p.done {
		return ""
	}
	return p.sequence.View()
}

// AddStep adds a new step to the sequence.
func (p *Progress) AddStep(name, nodeType string) {
	p.sequence.AddStep(name, nodeType)
}

// AddStepWithDepth adds a step with nesting depth.
func (p *Progress) AddStepWithDepth(name, nodeType string, depth int) {
	p.sequence.AddStepWithDepth(name, nodeType, depth)
}

// UpdateStep updates a step's status.
func (p *Progress) UpdateStep(name string, status StepStatus) {
	p.sequence.UpdateStep(name, status)
}

// SetDuration sets the duration for a step.
func (p *Progress) SetDuration(name string, duration time.Duration) {
	p.sequence.SetDuration(name, duration)
}

// Done marks the progress as complete.
func (p *Progress) Done() {
	p.done = true
}

// Run starts the Bubbletea program (blocking).
// This is the foreground part of the daemon-combo pattern.
func (p *Progress) Run() error {
	prog := tea.NewProgram(p)
	_, err := prog.Run()
	return err
}

// SendUpdate sends a step update message.
// Called from background goroutine.
func SendUpdate(prog *tea.Program, name string, status StepStatus) {
	prog.Send(StepUpdateMsg{
		Name:   name,
		Status: status,
	})
}

// SendDone sends the done message.
// Called from background goroutine when work completes.
func SendDone(prog *tea.Program) {
	prog.Send(DoneMsg{})
}
