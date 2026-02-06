// Package api provides the shared service layer for bento operations.
//
// BentoService is consumed by the CLI, HTTP server (apps/api/), and
// Wails desktop app. It encapsulates workflow execution, validation,
// and storage behind a single, transport-agnostic interface.
//
// # Usage
//
//	svc := api.New(api.DefaultRegistry(), store)
//	result, err := svc.RunWorkflow(ctx, def, api.RunOptions{})
package api

import "time"

// RunResult contains the outcome of a workflow execution.
type RunResult struct {
	Status        string                 `json:"status"`
	NodesExecuted int                    `json:"nodesExecuted"`
	NodeOutputs   map[string]interface{} `json:"nodeOutputs"`
	Duration      time.Duration          `json:"duration"`
	Error         string                 `json:"error,omitempty"`
}

// DryRunResult contains information about what a workflow would do.
type DryRunResult struct {
	Valid     bool       `json:"valid"`
	NodeCount int        `json:"nodeCount"`
	Nodes     []NodeInfo `json:"nodes"`
}

// ValidationResult contains the outcome of a workflow validation.
type ValidationResult struct {
	Valid  bool     `json:"valid"`
	Errors []string `json:"errors,omitempty"`
}

// WorkflowSummary provides a brief overview of a stored workflow.
type WorkflowSummary struct {
	Name      string `json:"name"`
	NodeCount int    `json:"nodeCount"`
}

// NodeInfo describes a single node within a workflow.
type NodeInfo struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Type string `json:"type"`
}
