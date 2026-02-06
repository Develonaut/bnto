// Package main implements the validate command for validating bentos.
//
// The validate command loads a bento definition and validates it without executing.
// It checks structure, node types, parameters, and edges to ensure the bento
// is properly configured.
package main

import (
	"context"
	"fmt"

	"github.com/Develonaut/bento/pkg/neta"
	"github.com/Develonaut/bento/pkg/validator"
	"github.com/spf13/cobra"
)

var validateVerboseFlag bool

var validateCmd = &cobra.Command{
	Use:   "validate [file].bento.json",
	Short: "Validate a bento workflow without executing",
	Long: `Validate a bento workflow without executing it.

This command validates structure, node types, parameters, and connections
to ensure the workflow is properly configured.

Examples:
  bento validate workflow.bento.json
  bento validate workflow.bento.json --verbose`,
	Args: cobra.ExactArgs(1),
	RunE: runValidate,
}

func init() {
	validateCmd.Flags().BoolVar(&validateVerboseFlag, "verbose", false, "Show detailed validation results")
}

// runValidate executes the validate command logic.
func runValidate(cmd *cobra.Command, args []string) error {
	def, err := loadBentoForValidate(args[0])
	if err != nil {
		return err
	}

	if err := validateForValidate(def); err != nil {
		return err
	}

	showValidationResults()
	return nil
}

// loadBentoForValidate loads a bento and prints status.
func loadBentoForValidate(bentoPath string) (*neta.Definition, error) {
	def, err := loadBento(bentoPath)
	if err != nil {
		printError(fmt.Sprintf("Failed to load bento: %v", err))
		return nil, err
	}

	printInfo(fmt.Sprintf("Validating bento: %s", def.Name))
	return def, nil
}

// validateForValidate validates the bento definition.
func validateForValidate(def *neta.Definition) error {
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
