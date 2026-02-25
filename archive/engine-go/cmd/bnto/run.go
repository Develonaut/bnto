// Package main implements the run command for executing bntos.
//
// The run command loads a bnto definition from a JSON file and executes it
// using the orchestration engine. It provides real-time progress updates
// and displays execution results.
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"time"

	"github.com/mattn/go-isatty"
	"github.com/spf13/cobra"

	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/storage"
	"github.com/Develonaut/bnto/pkg/validator"
)

var (
	verboseFlag bool
	timeoutFlag time.Duration
	dryRunFlag  bool
)

var runCmd = &cobra.Command{
	Use:   "run [file].bnto.json",
	Short: "Execute a bnto workflow",
	Long: `Execute a bnto workflow from start to finish.

This command executes all nodes in the bnto workflow
and reports progress and results.

Examples:
  bnto run workflow.bnto.json
  bnto run workflow.bnto.json --verbose
  bnto run workflow.bnto.json --timeout 30m`,
	Args: cobra.ExactArgs(1),
	RunE: runRun,
}

func init() {
	// Flags are now defined as persistent flags on root command
	// This allows them to work with both:
	// - bnto [file] --verbose
	// - bnto run [file] --verbose
}

// runRun executes the run command logic.
func runRun(cmd *cobra.Command, args []string) error {
	def, err := loadAndValidate(args[0])
	if err != nil {
		return err
	}

	// If dry run, show what would be executed and exit
	if dryRunFlag {
		return showDryRun(def)
	}

	// Detect TTY mode
	if isTTY() {
		return executeTUI(def)
	}
	return executeSimple(def)
}

// loadAndValidate loads and validates a bnto.
func loadAndValidate(bntoPath string) (*node.Definition, error) {
	def, err := loadBnto(bntoPath)
	if err != nil {
		printError(fmt.Sprintf("Failed to load bnto: %v", err))
		return nil, err
	}

	// Don't print "Running bnto:" here - the logger will output it
	// printInfo(fmt.Sprintf("Running bnto: %s", def.Name))
	// fmt.Println() // Add newline for better spacing

	if err := validateBnto(def); err != nil {
		printError(fmt.Sprintf("Validation failed: %v", err))
		return nil, err
	}

	return def, nil
}

// isTTY detects if running in an interactive terminal.
func isTTY() bool {
	return isatty.IsTerminal(os.Stdout.Fd())
}

// loadBnto loads a bnto definition from a file path or from storage.
//
// It tries multiple strategies in order:
// 1. Load from path as-is (e.g., examples/workflow.bnto.json)
// 2. Load from path with .bnto.json added (e.g., examples/workflow → examples/workflow.bnto.json)
// 3. Load from hangiri storage by name (e.g., my-workflow → ~/.bnto/bntos/my-workflow.bnto.json)
//
// This allows flexible usage:
//
//	bnto examples/workflow.bnto.json
//	bnto examples/workflow
//	bnto my-workflow
func loadBnto(path string) (*node.Definition, error) {
	// Strategy 1: Try path as-is
	if isValidFilePath(path) {
		return loadBntoFromFile(path)
	}

	// Strategy 2: Try adding .bnto.json extension
	if !strings.HasSuffix(path, ".bnto.json") {
		pathWithExt := path + ".bnto.json"
		if isValidFilePath(pathWithExt) {
			return loadBntoFromFile(pathWithExt)
		}
	}

	// Strategy 3: Try loading from storage
	return loadBntoFromStorage(path)
}

// isValidFilePath checks if the path exists as a file.
func isValidFilePath(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return !info.IsDir()
}

// loadBntoFromFile loads a bnto from a specific file path.
func loadBntoFromFile(path string) (*node.Definition, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read file: %w", err)
	}

	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		return nil, fmt.Errorf("failed to parse JSON: %w", err)
	}

	return &def, nil
}

// loadBntoFromStorage loads a bnto from storage by name.
func loadBntoFromStorage(name string) (*node.Definition, error) {
	// Strip .bnto.json extension if provided
	name = strings.TrimSuffix(name, ".bnto.json")

	storage := storage.NewDefaultStorage()
	ctx := context.Background()

	def, err := storage.LoadBnto(ctx, name)
	if err != nil {
		return nil, fmt.Errorf("failed to load bnto from storage: %w", err)
	}

	return def, nil
}

// validateBnto validates the bnto definition before execution.
func validateBnto(def *node.Definition) error {
	validator := validator.New()
	ctx := context.Background()
	return validator.Validate(ctx, def)
}

// showDryRun displays what would be executed without running.
func showDryRun(def *node.Definition) error {
	printInfo("DRY RUN MODE - No execution will occur")
	fmt.Printf("\nWould execute bnto: %s\n", def.Name)
	fmt.Printf("Total nodes to execute: %d\n\n", len(def.Nodes))

	if verboseFlag {
		printInfo("Nodes that would be executed:")
		for i, node := range def.Nodes {
			fmt.Printf("  %d. [%s] %s (type: %s)\n", i+1, node.ID, node.Name, node.Type)
		}
		fmt.Println()
	}

	// Perform preflight checks
	fmt.Println("Running preflight checks...")
	validator := createValidator()
	ctx := context.Background()
	if err := validator.PreflightCheck(ctx, def); err != nil {
		printError(fmt.Sprintf("Preflight check failed: %v", err))
		fmt.Println("\n❌ Dry run failed - fix the issues above before running")
		return err
	}

	fmt.Println("Validation: ✓ Passed")
	fmt.Println("Preflight: ✓ Passed")
	printSuccess("\nDry run complete. Use 'bnto run' without --dry-run to execute.")
	return nil
}

// createValidator creates a new validator.
func createValidator() *validator.Validator {
	return validator.New()
}
