package engine

import (
	"fmt"
	"os"
	"strings"

	"github.com/Develonaut/bento/pkg/secrets"
)

// executionContext holds data passed between nodes during execution.
type executionContext struct {
	nodeData       map[string]interface{} // Data from each executed node
	secretsManager *secrets.Manager        // Secrets manager for {{SECRETS.X}} resolution
	depth          int                    // Nesting depth for logging indentation
	path           []string               // Breadcrumb path of node names
}

// newExecutionContext creates a new execution context.
// Initializes nodeData with environment variables so templates can access them.
func newExecutionContext() *executionContext {
	nodeData := make(map[string]interface{})

	// Load all environment variables into context
	// This allows templates like {{.FIGMA_API_URL}} to work
	for _, env := range os.Environ() {
		// Split on first '=' to handle values that contain '='
		parts := strings.SplitN(env, "=", 2)
		if len(parts) == 2 {
			nodeData[parts[0]] = parts[1]
		}
	}

	// Initialize secrets manager for {{SECRETS.X}} resolution
	// If initialization fails, proceed without secrets (logged as warning)
	secretsMgr, err := secrets.NewManager()
	if err != nil {
		// Note: We don't fail here because secrets might not be needed
		// The error will surface when trying to resolve {{SECRETS.X}} if used
		secretsMgr = nil
	}

	return &executionContext{
		nodeData:       nodeData,
		secretsManager: secretsMgr,
		depth:          0,
		path:           []string{},
	}
}

// set stores output from a node.
func (ec *executionContext) set(nodeID string, data interface{}) {
	ec.nodeData[nodeID] = data
}

// copy creates a shallow copy of the execution context.
// Note: This performs a shallow copy - the nodeData map is copied,
// but the values within the map are not deep-copied. This is intentional
// for performance and works correctly because node outputs are immutable
// after being set. The secrets manager is shared across copies.
func (ec *executionContext) copy() *executionContext {
	newCtx := newExecutionContext()
	for k, v := range ec.nodeData {
		newCtx.nodeData[k] = v
	}
	// Share the same secrets manager (thread-safe)
	newCtx.secretsManager = ec.secretsManager
	// Preserve depth and path
	newCtx.depth = ec.depth
	newCtx.path = make([]string, len(ec.path))
	copy(newCtx.path, ec.path)
	return newCtx
}

// withNode returns a copy of the context with a node added to the path.
func (ec *executionContext) withNode(nodeName string) *executionContext {
	newCtx := ec.copy()
	newCtx.path = append(newCtx.path, nodeName)
	newCtx.depth++
	return newCtx
}

// getBreadcrumb returns the breadcrumb path as a formatted string.
// Format: "Node1:Node2:Node3" (no brackets)
func (ec *executionContext) getBreadcrumb() string {
	if len(ec.path) == 0 {
		return ""
	}
	return strings.Join(ec.path, ":")
}

// toMap converts the context to a map for external use.
func (ec *executionContext) toMap() map[string]interface{} {
	result := make(map[string]interface{})
	for k, v := range ec.nodeData {
		result[k] = v
	}
	return result
}

// String returns a string representation for debugging.
func (ec *executionContext) String() string {
	return fmt.Sprintf("executionContext{nodes: %d}", len(ec.nodeData))
}
