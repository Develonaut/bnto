package api

import (
	"context"

	"github.com/Develonaut/bento/pkg/node"
)

// ValidateWorkflow validates a workflow definition.
//
// Runs structural validation (required fields, known types, valid edges)
// and preflight checks (commands exist, env vars set, files accessible).
// Returns a structured result with all validation errors collected.
func (s *BentoService) ValidateWorkflow(ctx context.Context, def *node.Definition) (*ValidationResult, error) {
	var errors []string

	if err := s.validator.Validate(ctx, def); err != nil {
		errors = append(errors, err.Error())
	}

	if len(errors) == 0 {
		if err := s.validator.PreflightCheck(ctx, def); err != nil {
			errors = append(errors, err.Error())
		}
	}

	return &ValidationResult{
		Valid:  len(errors) == 0,
		Errors: errors,
	}, nil
}
