//go:build tui

package tui

import (
	"time"

	"github.com/charmbracelet/bubbles/progress"
	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"

	"github.com/Develonaut/bnto/pkg/node"
)

// NodeStatus represents node execution state.
type NodeStatus int

const (
	NodePending NodeStatus = iota
	NodeRunning
	NodeCompleted
	NodeFailed
)

// NodeState tracks individual node execution.
type NodeState struct {
	path         string
	name         string
	nodeType     string
	status       NodeStatus
	startTime    time.Time
	duration     time.Duration
	depth        int    // Nesting level for indentation
	currentChild string // For loops: name of currently executing child
	childIndex   int    // For loops: current iteration index
	childTotal   int    // For loops: total iterations
}

// Executor displays workflow execution progress using Bubbletea.
// This is a lightweight executor view that shows real-time progress
// and exits automatically when execution completes.
type Executor struct {
	theme      *Theme
	palette    Palette
	sequence   *Sequence
	nodeStates []NodeState
	bntoName   string
	running    bool
	complete   bool
	success    bool
	errorMsg   string
	spinner    Spinner
	progress   progress.Model
}

// Message types for Bubbletea

// NodeStartedMsg signals that a node has started execution.
type NodeStartedMsg struct {
	Path     string
	Name     string
	NodeType string
}

// NodeCompletedMsg signals that a node has finished execution.
type NodeCompletedMsg struct {
	Path     string
	Duration time.Duration
	Error    error
}

// ExecutionInitMsg initializes the executor with workflow definition.
type ExecutionInitMsg struct {
	Definition *node.Definition
}

// ExecutionCompleteMsg signals that workflow execution is complete.
type ExecutionCompleteMsg struct {
	Success bool
	Error   error
}

// LoopChildMsg signals that a loop is executing a specific child.
type LoopChildMsg struct {
	LoopPath  string // Path to the loop node
	ChildName string // Name of the current child being executed
	Index     int    // Current iteration index (0-based)
	Total     int    // Total number of iterations
}

// NewExecutor creates an executor for the given workflow definition.
func NewExecutor(def *node.Definition, theme *Theme, palette Palette) Executor {
	sequence := NewSequenceWithTheme(theme, palette)
	spinner := NewSpinner(palette)

	// Create progress bar with theme colors
	prog := progress.New(progress.WithDefaultGradient())
	prog.ShowPercentage = true
	prog.Width = 76 // Default width, will be updated by WindowSizeMsg

	return Executor{
		theme:      theme,
		palette:    palette,
		sequence:   sequence,
		nodeStates: []NodeState{},
		bntoName:   def.Name,
		running:    true,
		complete:   false,
		success:    false,
		spinner:    spinner,
		progress:   prog,
	}
}

// Init initializes the Bubbletea model.
func (e Executor) Init() tea.Cmd {
	return tea.Batch(
		e.spinner.Model.Tick,
		e.progress.Init(),
	)
}

// quitMsg signals that it's time to quit after waiting for final updates.
type quitMsg struct{}

// Update handles Bubbletea messages.
func (e Executor) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		return e.handleKeyMsg(msg)
	case quitMsg:
		return e.handleQuitMsg()
	case tea.WindowSizeMsg:
		return e.handleWindowSizeMsg(msg)
	case ExecutionInitMsg:
		return e.handleExecutionInitMsg(msg)
	case NodeStartedMsg:
		return e.handleNodeStartedMsg(msg)
	case NodeCompletedMsg:
		return e.handleNodeCompletedMsg(msg)
	case LoopChildMsg:
		return e.handleLoopChildMsg(msg)
	case ExecutionCompleteMsg:
		return e.handleExecutionCompleteMsg(msg)
	case spinner.TickMsg:
		return e.handleSpinnerTickMsg(msg)
	case progress.FrameMsg:
		return e.handleProgressFrameMsg(msg)
	}
	return e, nil
}

// Success returns whether execution was successful.
func (e Executor) Success() bool {
	return e.success
}
