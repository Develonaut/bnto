// Package node provides the core node type definitions for the bnto workflow system.
//
// In bnto, workflows are composed of nodes - individual execution units
// that can be connected together to form complex automation workflows.
//
// Every node must implement the Executable interface and can be serialized to/from JSON.
//
// # Core Concepts
//
// Definition: The JSON-serializable structure that describes a node. It contains
// the node's type, parameters, position, and connections to other nodes.
//
// Executable: The interface that all node implementations must satisfy. It provides
// the Execute method that runs the node's logic.
//
// Ports and Edges: Connection points and links between nodes that form the workflow graph.
//
// # Example Usage
//
//	// Create a simple node definition
//	def := node.Definition{
//	    ID:      "node-1",
//	    Type:    "http-request",
//	    Version: "1.0.0",
//	    Name:    "Fetch User Data",
//	    Parameters: map[string]interface{}{
//	        "url":    "https://api.example.com/users",
//	        "method": "GET",
//	    },
//	}
//
//	// Serialize to JSON
//	jsonData, _ := json.Marshal(def)
//
// Learn more about Go packages: https://go.dev/doc/code
package node

// Definition represents a single node in a bnto workflow.
//
// Each node has:
//   - A unique ID within the workflow
//   - A type (e.g., "http-request", "edit-fields")
//   - Parameters specific to that type
//   - Input/output ports for connecting to other nodes
//   - Position (for visual editor, optional for CLI)
//
// Definitions can be nested (for group nodes) and form a tree structure
// representing the workflow hierarchy.
type Definition struct {
	ID          string                 `json:"id"`                 // Unique identifier within workflow
	Type        string                 `json:"type"`               // Node type (http-request, loop, etc.)
	Version     string                 `json:"version"`            // Schema version for compatibility
	ParentID    *string                `json:"parentId,omitempty"` // Parent group ID (if nested)
	Name        string                 `json:"name"`               // Human-readable name
	Position    Position               `json:"position"`           // Visual editor position
	Metadata    Metadata               `json:"metadata"`           // Additional metadata
	Parameters  map[string]interface{} `json:"parameters"`         // Node-specific configuration
	Fields      *FieldsConfig          `json:"fields,omitempty"`   // Field configuration (for edit-fields)
	InputPorts  []Port                 `json:"inputPorts"`         // Input connection points
	OutputPorts []Port                 `json:"outputPorts"`        // Output connection points
	Nodes       []Definition           `json:"nodes,omitempty"`    // Child nodes (for group nodes)
	Edges       []Edge                 `json:"edges,omitempty"`    // Connections between child nodes
}

// Position represents the visual location of a node in the editor.
//
// For CLI-only usage, position can be zero values.
// For visual editor integration, these coordinates place the node on the canvas.
type Position struct {
	X float64 `json:"x"` // X coordinate in pixels
	Y float64 `json:"y"` // Y coordinate in pixels
}

// Metadata contains additional node information.
//
// This structure allows extending nodes with custom data without changing
// the core Definition structure. Useful for tracking creation time,
// user tags, or integration-specific data.
type Metadata struct {
	CreatedAt  string            `json:"createdAt,omitempty"`  // ISO 8601 timestamp
	UpdatedAt  string            `json:"updatedAt,omitempty"`  // ISO 8601 timestamp
	Tags       []string          `json:"tags,omitempty"`       // User-defined tags
	CustomData map[string]string `json:"customData,omitempty"` // Custom key-value pairs
}

// Port represents an input or output connection point on a node.
//
// Ports are the attachment points where edges connect. A node can have
// multiple input and output ports, allowing complex data flow patterns.
//
// Example:
//
//	port := Port{
//	    ID:     "input-1",
//	    Name:   "Data Input",
//	    Handle: "source",
//	}
type Port struct {
	ID     string `json:"id"`               // Unique port identifier within node
	Name   string `json:"name"`             // Human-readable port name
	Handle string `json:"handle,omitempty"` // Handle type (for visual editor)
}

// Edge represents a connection between two nodes.
//
// Edges define the data flow in a workflow. Data flows from the source
// node's output port to the target node's input port.
//
// Example:
//
//	edge := Edge{
//	    ID:           "edge-1",
//	    Source:       "http-node",
//	    Target:       "transform-node",
//	    SourceHandle: "output",
//	    TargetHandle: "input",
//	}
type Edge struct {
	ID           string `json:"id"`                     // Unique edge identifier
	Source       string `json:"source"`                 // Source node ID
	Target       string `json:"target"`                 // Target node ID
	SourceHandle string `json:"sourceHandle,omitempty"` // Source port handle
	TargetHandle string `json:"targetHandle,omitempty"` // Target port handle
}

// FieldsConfig represents field editor configuration for edit-fields nodes.
//
// The edit-fields node uses this configuration to set field values,
// either as static values or using Go templates.
//
// Example:
//
//	fields := FieldsConfig{
//	    Values: map[string]interface{}{
//	        "name":  "Item A",
//	        "label": "REC-001",
//	    },
//	    KeepOnlySet: true,  // Only output fields that are explicitly set
//	}
type FieldsConfig struct {
	Values      map[string]interface{} `json:"values"`                // Field values (static or template)
	KeepOnlySet bool                   `json:"keepOnlySet,omitempty"` // Only output set fields
}
