package tui

import (
	"fmt"
	"strings"

	"github.com/Develonaut/bento/pkg/neta"
)

// flattenDefinition converts a bento definition tree into a flat list of node states.
// Groups and parallel nodes are transparent containers (children are flattened).
// Loops are opaque leaf nodes (shown as single unit, children execute internally).
func flattenDefinition(def neta.Definition, basePath string) []NodeState {
	// Loops are opaque leaf nodes (even though they have children)
	if def.Type == "loop" {
		return flattenSingleNode(def, basePath)
	}

	// Other leaf nodes (not groups or parallel)
	if def.Type != "group" && def.Type != "parallel" {
		return flattenSingleNode(def, basePath)
	}

	// Groups and parallel nodes are transparent containers
	return flattenGroupNodes(def, basePath)
}

// flattenSingleNode creates state for a single node bento.
func flattenSingleNode(def neta.Definition, basePath string) []NodeState {
	// Use node ID if present, otherwise use basePath (or "0" as fallback)
	path := basePath
	if def.ID != "" {
		path = def.ID
	} else if path == "" {
		path = "0" // Fallback for single nodes without ID
	}

	return []NodeState{{
		path:     path,
		name:     def.Name,
		nodeType: def.Type,
		status:   NodePending,
		depth:    0,
	}}
}

// flattenGroupNodes flattens all nodes in a group recursively.
func flattenGroupNodes(def neta.Definition, basePath string) []NodeState {
	states := []NodeState{}

	for idx, child := range def.Nodes {
		// Use node ID if present (graph-based execution), otherwise use hierarchical path
		path := getNodePath(child, basePath, idx)

		// Groups and parallel nodes are transparent containers (recurse into them)
		// Loops are opaque leaf nodes (show the loop itself, not children)
		if child.Type == "group" || child.Type == "parallel" {
			childStates := flattenDefinition(child, path)
			states = append(states, childStates...)
		} else {
			// Track leaf nodes (including loops as single units)
			states = append(states, createNodeState(child, path))
		}
	}

	return states
}

// getNodePath returns the appropriate path for a node.
// Uses node ID for graph-based execution, falls back to hierarchical index.
func getNodePath(child neta.Definition, basePath string, idx int) string {
	if child.ID != "" {
		return child.ID // Use node ID for graph-based execution
	}
	return buildPathForNode(basePath, idx) // Use index-based path for hierarchical execution
}

// createNodeState builds a NodeState from definition and path.
func createNodeState(def neta.Definition, path string) NodeState {
	return NodeState{
		path:     path,
		name:     def.Name,
		nodeType: def.Type,
		status:   NodePending,
		depth:    parseDepth(path),
	}
}

// buildPathForNode constructs node path for tracking.
func buildPathForNode(basePath string, index int) string {
	if basePath == "" {
		return fmt.Sprintf("%d", index)
	}
	return fmt.Sprintf("%s.%d", basePath, index)
}

// parseDepth calculates nesting level from path.
func parseDepth(path string) int {
	if path == "" {
		return 0
	}
	return strings.Count(path, ".") + 1
}
