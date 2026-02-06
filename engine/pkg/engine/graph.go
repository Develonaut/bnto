package engine

import (
	"fmt"

	"github.com/Develonaut/bento/pkg/neta"
)

// graph represents a directed graph for node execution order.
type graph struct {
	nodes    map[string]*neta.Definition // Node ID -> Definition
	edges    map[string][]string         // Node ID -> List of target IDs
	incoming map[string]int              // Node ID -> Count of incoming edges
}

// buildGraph creates a graph from a bento definition.
func buildGraph(def *neta.Definition) (*graph, error) {
	g := &graph{
		nodes:    make(map[string]*neta.Definition),
		edges:    make(map[string][]string),
		incoming: make(map[string]int),
	}

	// Add all nodes
	for i := range def.Nodes {
		node := &def.Nodes[i]
		g.nodes[node.ID] = node
		g.incoming[node.ID] = 0
	}

	// Add all edges
	for _, edge := range def.Edges {
		// Validate source and target exist
		if _, ok := g.nodes[edge.Source]; !ok {
			return nil, fmt.Errorf("edge source '%s' not found", edge.Source)
		}
		if _, ok := g.nodes[edge.Target]; !ok {
			return nil, fmt.Errorf("edge target '%s' not found", edge.Target)
		}

		g.edges[edge.Source] = append(g.edges[edge.Source], edge.Target)
		g.incoming[edge.Target]++
	}

	return g, nil
}

// getStartNodes returns all nodes with no incoming edges.
func (g *graph) getStartNodes() []*neta.Definition {
	var starts []*neta.Definition

	for nodeID, count := range g.incoming {
		if count == 0 {
			starts = append(starts, g.nodes[nodeID])
		}
	}

	return starts
}

// getTargets returns all target nodes for a given source node.
func (g *graph) getTargets(nodeID string) []*neta.Definition {
	targetIDs := g.edges[nodeID]
	if len(targetIDs) == 0 {
		return nil
	}

	targets := make([]*neta.Definition, 0, len(targetIDs))
	for _, targetID := range targetIDs {
		if node, ok := g.nodes[targetID]; ok {
			targets = append(targets, node)
		}
	}

	return targets
}

// markExecuted marks a node as executed and decrements incoming edge counts.
func (g *graph) markExecuted(nodeID string) {
	for _, targetID := range g.edges[nodeID] {
		g.incoming[targetID]--
	}
}

// isReady checks if a node is ready to execute (all dependencies met).
func (g *graph) isReady(nodeID string) bool {
	return g.incoming[nodeID] == 0
}

// hasCycle detects cycles in the graph using DFS.
func (g *graph) hasCycle() bool {
	visited := make(map[string]bool)
	recStack := make(map[string]bool)

	for nodeID := range g.nodes {
		if g.hasCycleDFS(nodeID, visited, recStack) {
			return true
		}
	}

	return false
}

// hasCycleDFS performs depth-first search to detect cycles.
func (g *graph) hasCycleDFS(nodeID string, visited, recStack map[string]bool) bool {
	if !visited[nodeID] {
		visited[nodeID] = true
		recStack[nodeID] = true

		for _, targetID := range g.edges[nodeID] {
			if !visited[targetID] {
				if g.hasCycleDFS(targetID, visited, recStack) {
					return true
				}
			} else if recStack[targetID] {
				return true
			}
		}
	}

	recStack[nodeID] = false
	return false
}
