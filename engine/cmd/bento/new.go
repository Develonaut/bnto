// Package main implements the new command for creating template bentos.
//
// The new command creates a new bento template file with a basic structure
// that users can customize. It provides a quick way to start a new workflow.
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"

	"github.com/Develonaut/bento/pkg/storage"
	"github.com/Develonaut/bento/pkg/node"
	"github.com/spf13/cobra"
)

var (
	templateType  string
	newDryRunFlag bool
	newLocalFlag  bool
)

var newCmd = &cobra.Command{
	Use:   "new [name]",
	Short: "Create a new bento workflow template",
	Long: `Create a new bento workflow template file.

Creates a fresh bento template with a basic structure that you can customize.
By default, bentos are saved to ~/.bento/bentos/

Examples:
  bento new my-workflow                    # Creates ~/.bento/bentos/my-workflow.bento.json
  bento new my-workflow --local            # Creates ./my-workflow.bento.json
  bento new my-workflow --type simple      # Specify template type`,
	Args: cobra.ExactArgs(1),
	RunE: runNew,
}

func init() {
	newCmd.Flags().StringVar(&templateType, "type", "simple", "Template type (simple, loop, parallel)")
	newCmd.Flags().BoolVar(&newDryRunFlag, "dry-run", false, "Show what would be created without creating files")
	newCmd.Flags().BoolVar(&newLocalFlag, "local", false, "Create bento in current directory instead of ~/.bento/bentos/")
}

// runNew executes the new command logic.
func runNew(cmd *cobra.Command, args []string) error {
	name := args[0]

	// If dry run, show what would be created and exit
	if newDryRunFlag {
		return showNewDryRun(name)
	}

	// Create bento template
	template := createTemplate(name)

	// Save to storage or local file based on flag
	if newLocalFlag {
		return createLocalBento(name, template)
	}

	return createStorageBento(name, template)
}

// createStorageBento creates a bento in ~/.bento/bentos/ using hangiri storage.
func createStorageBento(name string, template *node.Definition) error {
	printInfo(fmt.Sprintf("Creating new bento: %s", name))

	storage := storage.NewDefaultStorage()
	ctx := context.Background()

	// Check if bento already exists
	if storage.BentoExists(ctx, name) {
		printError(fmt.Sprintf("Bento '%s' already exists in ~/.bento/bentos/", name))
		return fmt.Errorf("bento already exists: %s", name)
	}

	// Save to storage
	if err := storage.SaveBento(ctx, name, template); err != nil {
		printError(fmt.Sprintf("Failed to create bento: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("Created: ~/.bento/bentos/%s.bento.json", name))
	showNextSteps(name, false)
	return nil
}

// createLocalBento creates a bento in the current directory.
func createLocalBento(name string, template *node.Definition) error {
	fileName := name + ".bento.json"

	printInfo(fmt.Sprintf("Creating new bento: %s", name))

	// Check if file already exists
	if _, err := os.Stat(fileName); err == nil {
		printError(fmt.Sprintf("File '%s' already exists", fileName))
		return fmt.Errorf("file already exists: %s", fileName)
	}

	// Write template to local file
	if err := writeTemplate(fileName, template); err != nil {
		printError(fmt.Sprintf("Failed to create bento: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("Created: %s", fileName))
	showNextSteps(fileName, true)
	return nil
}

// showNextSteps displays next steps after creating bento.
func showNextSteps(nameOrPath string, isLocal bool) {
	fmt.Println("\nNext steps:")
	if isLocal {
		fmt.Printf("  1. Edit %s\n", nameOrPath)
		fmt.Printf("  2. Run: bento validate %s\n", nameOrPath)
		fmt.Printf("  3. Run: bento run %s\n", nameOrPath)
	} else {
		fmt.Printf("  1. Edit ~/.bento/bentos/%s.bento.json\n", nameOrPath)
		fmt.Printf("  2. Run: bento validate %s\n", nameOrPath)
		fmt.Printf("  3. Run: bento run %s\n", nameOrPath)
	}
}

// createTemplate creates a template bento definition.
func createTemplate(name string) *node.Definition {
	return &node.Definition{
		ID:      name,
		Type:    "group",
		Version: "1.0.0",
		Name:    formatName(name),
		Position: node.Position{
			X: 0,
			Y: 0,
		},
		Metadata: node.Metadata{
			Tags: []string{"template"},
		},
		Parameters:  make(map[string]interface{}),
		InputPorts:  []node.Port{},
		OutputPorts: []node.Port{},
		Nodes:       []node.Definition{createSampleNode()},
		Edges:       []node.Edge{},
	}
}

// createSampleNode creates a sample edit-fields node.
func createSampleNode() node.Definition {
	return node.Definition{
		ID:      "sample-1",
		Type:    "edit-fields",
		Version: "1.0.0",
		Name:    "Sample Node",
		Position: node.Position{
			X: 100,
			Y: 100,
		},
		Metadata: node.Metadata{},
		Parameters: map[string]interface{}{
			"values": map[string]interface{}{
				"message": "Hello from bento!",
			},
		},
		InputPorts:  []node.Port{},
		OutputPorts: []node.Port{},
	}
}

// formatName converts a kebab-case or snake_case name to Title Case.
func formatName(name string) string {
	if len(name) == 0 {
		return name
	}
	return strings.ToUpper(string(name[0])) + name[1:]
}

// writeTemplate writes the template to a JSON file.
func writeTemplate(fileName string, template *node.Definition) error {
	data, err := json.MarshalIndent(template, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to serialize template: %w", err)
	}

	if err := os.WriteFile(fileName, data, 0644); err != nil {
		return fmt.Errorf("failed to write file: %w", err)
	}

	return nil
}

// showNewDryRun displays what would be created without creating files.
func showNewDryRun(name string) error {
	printInfo("DRY RUN MODE - No files will be created")

	location := "~/.bento/bentos/" + name + ".bento.json"
	if newLocalFlag {
		location = "./" + name + ".bento.json"
	}
	fmt.Printf("\nWould create file: %s\n", location)

	template := createTemplate(name)
	data, err := json.MarshalIndent(template, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to serialize template: %w", err)
	}

	fmt.Println("\nTemplate preview:")
	fmt.Println(string(data))

	fmt.Println("\nNext steps (after running without --dry-run):")
	showNextSteps(name, newLocalFlag)

	printSuccess("Dry run complete. Use 'bento new' without --dry-run to create the file.")
	return nil
}
