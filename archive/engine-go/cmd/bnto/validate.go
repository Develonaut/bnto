// Package main implements the validate command for validating bntos.
//
// The validate command loads a bnto definition and validates it without executing.
// It checks structure, node types, parameters, and edges to ensure the bnto
// is properly configured.
package main

import (
	"context"
	"fmt"

	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/validator"
	"github.com/spf13/cobra"
)

var validateVerboseFlag bool

var validateCmd = &cobra.Command{
	Use:   "validate [file].bnto.json",
	Short: "Validate a bnto workflow without executing",
	Long: `Validate a bnto workflow without executing it.

This command validates structure, node types, parameters, and connections
to ensure the workflow is properly configured.

Examples:
  bnto validate workflow.bnto.json
  bnto validate workflow.bnto.json --verbose`,
	Args: cobra.ExactArgs(1),
	RunE: runValidate,
}

func init() {
	validateCmd.Flags().BoolVar(&validateVerboseFlag, "verbose", false, "Show detailed validation results")
}

// runValidate executes the validate command logic.
func runValidate(cmd *cobra.Command, args []string) error {
	def, err := loadBntoForValidate(args[0])
	if err != nil {
		return err
	}

	if err := validateForValidate(def); err != nil {
		return err
	}

	showValidationResults()
	return nil
}

// loadBntoForValidate loads a bnto and prints status.
func loadBntoForValidate(bntoPath string) (*node.Definition, error) {
	def, err := loadBnto(bntoPath)
	if err != nil {
		printError(fmt.Sprintf("Failed to load bnto: %v", err))
		return nil, err
	}

	printInfo(fmt.Sprintf("Validating bnto: %s", def.Name))
	return def, nil
}

// validateForValidate validates the bnto definition.
func validateForValidate(def *node.Definition) error {
	validator := validator.New()
	ctx := context.Background()

	if err := validator.Validate(ctx, def); err != nil {
		printError(fmt.Sprintf("Validation failed: %v", err))
		return err
	}

	return nil
}

// showValidationResults displays validation results.
func showValidationResults() {
	if validateVerboseFlag {
		printCheck("Valid JSON structure")
		printCheck("All node types recognized")
		printCheck("All edges valid")
		printCheck("Required parameters present")
	}

	printSuccess("Validation successful. Workflow is valid and ready to run.")
}
