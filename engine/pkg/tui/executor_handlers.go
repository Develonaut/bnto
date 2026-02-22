//go:build tui

package tui

import (
	"time"

	"github.com/charmbracelet/bubbles/progress"
	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"
)

// handleKeyMsg handles keyboard input.
func (e Executor) handleKeyMsg(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	if msg.Type == tea.KeyCtrlC {
		return e, tea.Quit
	}
	return e, nil
}

// handleQuitMsg handles final cleanup before quitting.
func (e Executor) handleQuitMsg() (tea.Model, tea.Cmd) {
	e.updateSequence()
	e.updateProgress()
	return e, tea.Quit
}

// handleWindowSizeMsg handles terminal window resize.
func (e Executor) handleWindowSizeMsg(msg tea.WindowSizeMsg) (tea.Model, tea.Cmd) {
	width := msg.Width
	if width > 80 {
		width = 80
	}
	e.progress.Width = width - 4
	return e, nil
}

// handleExecutionInitMsg handles workflow execution initialization.
func (e Executor) handleExecutionInitMsg(msg ExecutionInitMsg) (tea.Model, tea.Cmd) {
	e.nodeStates = flattenDefinition(*msg.Definition, "")
	e.updateSequence()
	e.updateProgress()
	return e, nil
}

// handleNodeStartedMsg handles node execution start.
func (e Executor) handleNodeStartedMsg(msg NodeStartedMsg) (tea.Model, tea.Cmd) {
	e.handleNodeStarted(msg)
	e.updateSequence()
	e.updateProgress()
	return e, nil
}

// handleNodeCompletedMsg handles node execution completion.
func (e Executor) handleNodeCompletedMsg(msg NodeCompletedMsg) (tea.Model, tea.Cmd) {
	e.handleNodeCompleted(msg)
	e.updateSequence()
	e.updateProgress()
	return e, nil
}

// handleLoopChildMsg handles loop child execution updates.
func (e Executor) handleLoopChildMsg(msg LoopChildMsg) (tea.Model, tea.Cmd) {
	e.handleLoopChild(msg)
	e.updateSequence()
	return e, nil
}

// handleExecutionCompleteMsg handles workflow execution completion.
func (e Executor) handleExecutionCompleteMsg(msg ExecutionCompleteMsg) (tea.Model, tea.Cmd) {
	e.complete = true
	e.running = false
	e.success = msg.Success
	if msg.Error != nil {
		e.errorMsg = msg.Error.Error()
	}
	e.updateSequence()
	e.updateProgress()

	// Wait briefly for final node completion messages
	return e, tea.Tick(100*time.Millisecond, func(t time.Time) tea.Msg {
		return quitMsg{}
	})
}

// handleSpinnerTickMsg handles spinner animation updates.
func (e Executor) handleSpinnerTickMsg(msg spinner.TickMsg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd
	e.spinner, cmd = e.spinner.Update(msg)
	e.sequence.UpdateSpinner(e.spinner)
	return e, cmd
}

// handleProgressFrameMsg handles progress bar animation updates.
func (e Executor) handleProgressFrameMsg(msg progress.FrameMsg) (tea.Model, tea.Cmd) {
	progressModel, cmd := e.progress.Update(msg)
	e.progress = progressModel.(progress.Model)
	return e, cmd
}
