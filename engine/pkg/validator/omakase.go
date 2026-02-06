// Package validator provides validation for bento files and node definitions.
//
// The package validates:
//   - Core structure (ID, Type, Version required fields)
//   - Node-specific parameters (URL for http-request, mode for loop, etc.)
//   - Nested structures (group node with child nodes and edges)
//   - Edge validity (source and target nodes must exist)
//
// # Usage
//
//	validator := validator.New()
//
//	// Validate a node definition
//	if err := validator.Validate(ctx, nodeDef); err != nil {
//	    log.Fatalf("Invalid node: %v", err)
//	}
//
//	// Pre-flight checks (environment, commands, API keys)
//	if err := validator.PreflightCheck(ctx, nodeDef); err != nil {
//	    log.Fatalf("Pre-flight check failed: %v", err)
//	}
//
// All validation errors include the node ID and clear, actionable error messages.
package validator

import (
	"context"
	"fmt"

	"github.com/Develonaut/bento/pkg/node"
)

// Validator validates node definitions and performs pre-flight checks.
type Validator struct {
	// validators holds node-specific validation functions
	validators map[string]ValidatorFunc
}

// ValidatorFunc is a function that validates node-specific parameters.
type ValidatorFunc func(*node.Definition) error

// New creates a new Validator with all validators registered.
func New() *Validator {
	v := &Validator{
		validators: make(map[string]ValidatorFunc),
	}

	// Register node-specific validators
	v.validators["http-request"] = validateHTTPRequest
	v.validators["file-system"] = validateFileSystem
	v.validators["shell-command"] = validateShellCommand
	v.validators["loop"] = validateLoop
	v.validators["edit-fields"] = validateEditFields
	v.validators["group"] = nil // Handled separately
	v.validators["parallel"] = validateParallel
	v.validators["spreadsheet"] = validateSpreadsheet
	v.validators["image"] = validateImage
	v.validators["transform"] = validateTransform

	return v
}

// Validate validates a node definition.
//
// Returns a clear, actionable error message if validation fails.
// The error message always includes the node ID for debugging.
func (v *Validator) Validate(ctx context.Context, def *node.Definition) error {
	// Check for cancellation
	if ctx.Err() != nil {
		return ctx.Err()
	}

	// Validate core and type
	if err := v.validateCoreAndType(def); err != nil {
		return err
	}

	// Validate type-specific and nested
	return v.validateSpecificAndNested(ctx, def)
}

// validateCoreAndType validates core structure and type.
func (v *Validator) validateCoreAndType(def *node.Definition) error {
	if err := v.validateCore(def); err != nil {
		return err
	}
	return v.validateType(def)
}

// validateSpecificAndNested validates type-specific params and nested nodes.
func (v *Validator) validateSpecificAndNested(ctx context.Context, def *node.Definition) error {
	if err := v.validateTypeSpecific(def); err != nil {
		return err
	}

	if def.Type == "group" {
		return v.validateGroup(ctx, def)
	}
	return nil
}

// validateCore validates core required fields.
func (v *Validator) validateCore(def *node.Definition) error {
	if def.ID == "" {
		return fmt.Errorf("node is missing required field 'id'")
	}

	if def.Type == "" {
		return fmt.Errorf("node '%s' is missing required field 'type'", def.ID)
	}

	if def.Version == "" {
		return fmt.Errorf("node '%s' is missing required field 'version'", def.ID)
	}

	return nil
}

// validateType validates that the node type is known.
func (v *Validator) validateType(def *node.Definition) error {
	if _, exists := v.validators[def.Type]; !exists {
		return fmt.Errorf("node '%s' has unknown type '%s'", def.ID, def.Type)
	}
	return nil
}

// validateTypeSpecific validates node-specific parameters.
func (v *Validator) validateTypeSpecific(def *node.Definition) error {
	validatorFunc := v.validators[def.Type]
	if validatorFunc == nil {
		return nil
	}

	return validatorFunc(def)
}

// validateGroup validates group node (nested nodes and edges).
func (v *Validator) validateGroup(ctx context.Context, def *node.Definition) error {
	// Validate child nodes
	if err := v.validateChildNodes(ctx, def); err != nil {
		return err
	}

	// Validate edges
	return v.validateEdges(def)
}

// validateChildNodes validates all child nodes in a group.
func (v *Validator) validateChildNodes(ctx context.Context, def *node.Definition) error {
	for _, child := range def.Nodes {
		if err := v.Validate(ctx, &child); err != nil {
			return fmt.Errorf("invalid child node in group '%s': %w", def.ID, err)
		}
	}
	return nil
}

// validateEdges validates that all edges reference existing nodes.
func (v *Validator) validateEdges(def *node.Definition) error {
	nodeIDs := buildNodeIDMap(def.Nodes)

	for _, edge := range def.Edges {
		if !nodeIDs[edge.Source] {
			return fmt.Errorf("edge '%s' in group '%s' has invalid source '%s' (node doesn't exist)",
				edge.ID, def.ID, edge.Source)
		}
		if !nodeIDs[edge.Target] {
			return fmt.Errorf("edge '%s' in group '%s' has invalid target '%s' (node doesn't exist)",
				edge.ID, def.ID, edge.Target)
		}
	}
	return nil
}

// buildNodeIDMap creates a map of node IDs for quick lookup.
func buildNodeIDMap(nodes []node.Definition) map[string]bool {
	nodeIDs := make(map[string]bool)
	for _, node := range nodes {
		nodeIDs[node.ID] = true
	}
	return nodeIDs
}

// PreflightCheck performs environment checks before execution.
//
// This includes:
//   - Verifying required commands are installed
//   - Checking environment variables exist
//   - Validating file paths exist
//   - Recursively checking child nodes in groups and loops
//
// Returns a clear error message if any pre-flight check fails.
func (v *Validator) PreflightCheck(ctx context.Context, def *node.Definition) error {
	if ctx.Err() != nil {
		return ctx.Err()
	}

	// Type-specific preflight checks
	if err := v.preflightTypeSpecific(def); err != nil {
		return err
	}

	// Recursive preflight for groups and loops
	return v.preflightRecursive(ctx, def)
}

// preflightTypeSpecific performs type-specific preflight checks.
func (v *Validator) preflightTypeSpecific(def *node.Definition) error {
	switch def.Type {
	case "shell-command":
		return preflightShellCommand(def)
	case "http-request":
		return preflightHTTPRequest(def)
	case "file-system":
		return preflightFileSystem(def)
	case "spreadsheet":
		return preflightSpreadsheet(def)
	}
	return nil
}

// preflightRecursive performs preflight checks on child nodes.
func (v *Validator) preflightRecursive(ctx context.Context, def *node.Definition) error {
	if def.Type != "group" && def.Type != "loop" {
		return nil
	}

	for _, child := range def.Nodes {
		if err := v.PreflightCheck(ctx, &child); err != nil {
			return fmt.Errorf("preflight check failed in '%s': %w", def.ID, err)
		}
	}
	return nil
}
