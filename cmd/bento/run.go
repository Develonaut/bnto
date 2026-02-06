// Package main implements the run command for executing bentos.
//
// The run command loads a bento definition from a JSON file and executes it
// using the itamae orchestration engine. It provides real-time progress updates
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

	"github.com/Develonaut/bento/pkg/storage"
	"github.com/Develonaut/bento/pkg/neta"
	"github.com/Develonaut/bento/pkg/validator"
	"github.com/Develonaut/bento/pkg/registry"
	"github.com/spf13/cobra"

	editfields "github.com/Develonaut/bento/pkg/neta/library/editfields"
	filesystem "github.com/Develonaut/bento/pkg/neta/library/filesystem"
	group "github.com/Develonaut/bento/pkg/neta/library/group"
	httpneta "github.com/Develonaut/bento/pkg/neta/library/http"
	image "github.com/Develonaut/bento/pkg/neta/library/image"
	loop "github.com/Develonaut/bento/pkg/neta/library/loop"
	parallel "github.com/Develonaut/bento/pkg/neta/library/parallel"
	shellcommand "github.com/Develonaut/bento/pkg/neta/library/shellcommand"
	spreadsheet "github.com/Develonaut/bento/pkg/neta/library/spreadsheet"
	transform "github.com/Develonaut/bento/pkg/neta/library/transform"
)

var (
	verboseFlag bool
	timeoutFlag time.Duration
	dryRunFlag  bool
)

var runCmd = &cobra.Command{
	Use:   "run [file].bento.json",
	Short: "Execute a bento workflow",
	Long: `Execute a bento workflow from start to finish.

This command executes all nodes in the bento workflow
and reports progress and results.

Examples:
  bento run workflow.bento.json
  bento run workflow.bento.json --verbose
  bento run workflow.bento.json --timeout 30m`,
	Args: cobra.ExactArgs(1),
	RunE: runRun,
}

func init() {
	// Flags are now defined as persistent flags on root command
	// This allows them to work with both:
	// - bento [file] --verbose
	// - bento run [file] --verbose
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

// loadAndValidate loads and validates a bento.
func loadAndValidate(bentoPath string) (*neta.Definition, error) {
	def, err := loadBento(bentoPath)
	if err != nil {
		printError(fmt.Sprintf("Failed to load bento: %v", err))
		return nil, err
	}

	// Don't print "Running bento:" here - the logger will output it
	// printInfo(fmt.Sprintf("Running bento: %s", def.Name))
	// fmt.Println() // Add newline for better spacing

	if err := validateBento(def); err != nil {
		printError(fmt.Sprintf("Validation failed: %v", err))
		return nil, err
	}

	return def, nil
}

// isTTY detects if running in an interactive terminal.
func isTTY() bool {
	return isatty.IsTerminal(os.Stdout.Fd())
}

// loadBento loads a bento definition from a file path or from storage.
//
// It tries multiple strategies in order:
// 1. Load from path as-is (e.g., examples/workflow.bento.json)
// 2. Load from path with .bento.json added (e.g., examples/workflow → examples/workflow.bento.json)
// 3. Load from hangiri storage by name (e.g., my-workflow → ~/.bento/bentos/my-workflow.bento.json)
//
// This allows flexible usage:
//
//	bento examples/workflow.bento.json
//	bento examples/workflow
//	bento my-workflow
func loadBento(path string) (*neta.Definition, error) {
	// Strategy 1: Try path as-is
	if isValidFilePath(path) {
		return loadBentoFromFile(path)
	}

	// Strategy 2: Try adding .bento.json extension
	if !strings.HasSuffix(path, ".bento.json") {
		pathWithExt := path + ".bento.json"
		if isValidFilePath(pathWithExt) {
			return loadBentoFromFile(pathWithExt)
		}
	}

	// Strategy 3: Try loading from hangiri storage
	return loadBentoFromStorage(path)
}

// isValidFilePath checks if the path exists as a file.
func isValidFilePath(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return !info.IsDir()
}

// loadBentoFromFile loads a bento from a specific file path.
func loadBentoFromFile(path string) (*neta.Definition, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read file: %w", err)
	}

	var def neta.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		return nil, fmt.Errorf("failed to parse JSON: %w", err)
	}

	return &def, nil
}

// loadBentoFromStorage loads a bento from hangiri storage by name.
func loadBentoFromStorage(name string) (*neta.Definition, error) {
	// Strip .bento.json extension if provided
	name = strings.TrimSuffix(name, ".bento.json")

	storage := storage.NewDefaultStorage()
	ctx := context.Background()

	def, err := storage.LoadBento(ctx, name)
	if err != nil {
		return nil, fmt.Errorf("failed to load bento from storage: %w", err)
	}

	return def, nil
}

// createPantry creates and populates the registry with all neta types.
func createPantry() *registry.Registry {
	p := registry.New()

	// Register all neta types
	p.RegisterFactory("edit-fields", func() neta.Executable { return editfields.New() })
	p.RegisterFactory("file-system", func() neta.Executable { return filesystem.New() })
	p.RegisterFactory("group", func() neta.Executable { return group.New() })
	p.RegisterFactory("http-request", func() neta.Executable { return httpneta.New() })
	p.RegisterFactory("image", func() neta.Executable { return image.New() })
	p.RegisterFactory("loop", func() neta.Executable { return loop.New() })
	p.RegisterFactory("parallel", func() neta.Executable { return parallel.New() })
	p.RegisterFactory("shell-command", func() neta.Executable { return shellcommand.New() })
	p.RegisterFactory("spreadsheet", func() neta.Executable { return spreadsheet.New() })
	p.RegisterFactory("transform", func() neta.Executable { return transform.New() })

	return p
}

// validateBento validates the bento definition before execution.
func validateBento(def *neta.Definition) error {
	validator := validator.New()
	ctx := context.Background()
	return validator.Validate(ctx, def)
}

// showDryRun displays what would be executed without running.
func showDryRun(def *neta.Definition) error {
	printInfo("DRY RUN MODE - No execution will occur")
	fmt.Printf("\nWould execute bento: %s\n", def.Name)
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
	printSuccess("\nDry run complete. Use 'bento run' without --dry-run to execute.")
	return nil
}

// createValidator creates a new validator.
func createValidator() *validator.Validator {
	return validator.New()
}
