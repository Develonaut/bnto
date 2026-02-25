package api

import (
	"context"

	"github.com/Develonaut/bnto/pkg/node"
)

// ListWorkflows returns a summary of every workflow in storage.
func (s *BntoService) ListWorkflows(ctx context.Context) ([]WorkflowSummary, error) {
	names, err := s.storage.ListBntos(ctx)
	if err != nil {
		return nil, err
	}

	summaries := make([]WorkflowSummary, 0, len(names))
	for _, name := range names {
		summary := WorkflowSummary{Name: name}
		if def, err := s.storage.LoadBnto(ctx, name); err == nil {
			summary.NodeCount = len(def.Nodes)
		}
		summaries = append(summaries, summary)
	}
	return summaries, nil
}

// GetWorkflow loads a workflow definition from storage by name.
func (s *BntoService) GetWorkflow(ctx context.Context, name string) (*node.Definition, error) {
	return s.storage.LoadBnto(ctx, name)
}

// SaveWorkflow persists a workflow definition to storage.
func (s *BntoService) SaveWorkflow(ctx context.Context, name string, def *node.Definition) error {
	return s.storage.SaveBnto(ctx, name, def)
}

// DeleteWorkflow removes a workflow from storage.
func (s *BntoService) DeleteWorkflow(ctx context.Context, name string) error {
	return s.storage.DeleteBnto(ctx, name)
}
