package engine

import "fmt"

// nodeError represents an error that occurred during node execution.
type nodeError struct {
	nodeID    string // ID of the node that failed
	nodeType  string // Type of the node
	operation string // Operation that failed
	cause     error  // Underlying error
}

// newNodeError creates a new node error.
func newNodeError(nodeID, nodeType, operation string, cause error) error {
	return &nodeError{
		nodeID:    nodeID,
		nodeType:  nodeType,
		operation: operation,
		cause:     cause,
	}
}

// Error returns the error message.
func (e *nodeError) Error() string {
	return fmt.Sprintf(
		"node '%s' (type: %s) failed during %s: %v",
		e.nodeID,
		e.nodeType,
		e.operation,
		e.cause,
	)
}

// Unwrap returns the underlying error.
func (e *nodeError) Unwrap() error {
	return e.cause
}

// NodeID returns the ID of the failed node.
func (e *nodeError) NodeID() string {
	return e.nodeID
}

// NodeType returns the type of the failed node.
func (e *nodeError) NodeType() string {
	return e.nodeType
}

// Operation returns the operation that failed.
func (e *nodeError) Operation() string {
	return e.operation
}
