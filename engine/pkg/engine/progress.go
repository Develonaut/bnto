package engine

import (
	"sync"

	"github.com/Develonaut/bento/pkg/node"
)

// graphNode represents a node in the flattened execution graph for progress tracking.
type graphNode struct {
	ID     string
	Type   string
	Weight int
	Level  int
}

// executionGraph is the analyzed graph structure (immutable after creation).
type executionGraph struct {
	Nodes       map[string]*graphNode
	TotalWeight int
}

// nodeState tracks runtime state for a single node.
type nodeState struct {
	State    string // "pending" | "executing" | "completed" | "error"
	Progress int    // 0-100
	Message  string
}

// executionState tracks runtime progress (separate from graph analysis).
type executionState struct {
	graph          *executionGraph
	nodes          map[string]*nodeState
	cachedProgress int
	mu             sync.RWMutex
}

// analyzeGraph flattens the bento definition into an execution graph.
// Groups are transparent (children are flattened).
// Loops are opaque (count as single node, children hidden).
func analyzeGraph(def *node.Definition) *executionGraph {
	graph := &executionGraph{
		Nodes: make(map[string]*graphNode),
	}
	analyzeNode(def, graph, 0)
	return graph
}

// analyzeNode recursively analyzes a node and adds it to the graph.
func analyzeNode(def *node.Definition, graph *executionGraph, level int) {
	switch def.Type {
	case "group", "parallel":
		analyzeGroupNode(def, graph, level)
	case "loop":
		analyzeLoopNode(def, graph, level)
	default:
		analyzeLeafNode(def, graph, level)
	}
}

// analyzeGroupNode flattens group children (groups are transparent in progress graph).
func analyzeGroupNode(def *node.Definition, graph *executionGraph, level int) {
	for i := range def.Nodes {
		analyzeNode(&def.Nodes[i], graph, level)
	}
}

// analyzeLoopNode adds loop as single node (loop children execute internally).
func analyzeLoopNode(def *node.Definition, graph *executionGraph, level int) {
	addNodeToGraph(def, graph, level)
}

// analyzeLeafNode adds a leaf node to the graph.
func analyzeLeafNode(def *node.Definition, graph *executionGraph, level int) {
	addNodeToGraph(def, graph, level)
}

// addNodeToGraph creates a graph node and adds it with its weight.
func addNodeToGraph(def *node.Definition, graph *executionGraph, level int) {
	weight := getNodeWeight(def.Type)
	graph.Nodes[def.ID] = &graphNode{
		ID:     def.ID,
		Type:   def.Type,
		Weight: weight,
		Level:  level,
	}
	graph.TotalWeight += weight
}

// Node weight constants for progress calculation.
// Weights reflect typical execution time to provide realistic progress indication.
const (
	// WeightShellCommand is highest because shell commands involve I/O,
	// external process execution, and unpredictable runtime.
	WeightShellCommand = 300

	// WeightHTTPRequest is high due to network latency and remote server response time.
	WeightHTTPRequest = 200

	// WeightImage is medium-high for image processing operations (decode, encode, transform).
	WeightImage = 150

	// WeightSpreadsheet is medium for CSV/Excel read/write operations.
	WeightSpreadsheet = 100

	// WeightLoop is medium as loops aggregate multiple child executions.
	WeightLoop = 100

	// WeightDefault is the fallback for unknown node types.
	WeightDefault = 100

	// WeightFileSystem is low for fast file operations (read, write, mkdir).
	WeightFileSystem = 50

	// WeightEditFields is low for in-memory field manipulation.
	WeightEditFields = 50

	// WeightTransform is low for in-memory data transformation.
	WeightTransform = 50
)

// getNodeWeight returns the weight for a node type based on typical execution duration.
// Weights provide realistic progress indication: shell commands (slow) vs transforms (fast).
func getNodeWeight(nodeType string) int {
	weights := map[string]int{
		"shell-command": WeightShellCommand,
		"http-request":  WeightHTTPRequest,
		"image":         WeightImage,
		"spreadsheet":   WeightSpreadsheet,
		"loop":          WeightLoop,
		"file-system":   WeightFileSystem,
		"edit-fields":   WeightEditFields,
		"transform":     WeightTransform,
	}
	if w, ok := weights[nodeType]; ok {
		return w
	}
	return WeightDefault
}

// newExecutionState creates a new execution state from a graph.
func newExecutionState(graph *executionGraph) *executionState {
	state := &executionState{
		graph:          graph,
		nodes:          make(map[string]*nodeState),
		cachedProgress: 0,
	}

	// Initialize all nodes to pending
	for nodeID := range graph.Nodes {
		state.nodes[nodeID] = &nodeState{
			State:    "pending",
			Progress: 0,
		}
	}

	return state
}

// setNodeProgress updates a node's progress and recalculates overall progress.
func (s *executionState) setNodeProgress(nodeID string, progress int, message string) {
	s.mu.Lock()
	defer s.mu.Unlock()

	node, ok := s.nodes[nodeID]
	if !ok {
		return
	}

	// Clamp progress 0-100
	if progress < 0 {
		progress = 0
	}
	if progress > 100 {
		progress = 100
	}

	node.Progress = progress
	node.Message = message

	// Recalculate overall progress
	s.cachedProgress = s.calculateProgressUnsafe()
}

// setNodeState updates a node's execution state.
func (s *executionState) setNodeState(nodeID string, state string) {
	s.mu.Lock()
	defer s.mu.Unlock()

	node, ok := s.nodes[nodeID]
	if !ok {
		return
	}

	node.State = state

	// Recalculate overall progress
	s.cachedProgress = s.calculateProgressUnsafe()
}

// calculateProgressUnsafe computes overall graph progress (caller must hold lock).
func (s *executionState) calculateProgressUnsafe() int {
	if s.graph.TotalWeight == 0 {
		return 0
	}

	completedWeight := 0

	for nodeID, graphNode := range s.graph.Nodes {
		state := s.nodes[nodeID]

		switch state.State {
		case "completed":
			completedWeight += graphNode.Weight

		case "executing":
			// Include partial progress from executing nodes
			completedWeight += (graphNode.Weight * state.Progress) / 100
		}
		// pending and error nodes contribute 0
	}

	return (completedWeight * 100) / s.graph.TotalWeight
}

// getProgress returns the cached overall progress (thread-safe).
func (s *executionState) getProgress() int {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.cachedProgress
}
