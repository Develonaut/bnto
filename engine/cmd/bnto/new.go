// Package main implements the new command for creating template bntos.
//
// The new command creates a new bnto template file with a basic structure
// that users can customize. It provides a quick way to start a new workflow.
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"

	"github.com/Develonaut/bnto/pkg/storage"
	"github.com/Develonaut/bnto/pkg/node"
	"github.com/spf13/cobra"
)

var (
	templateType  string
	newDryRunFlag bool
	newLocalFlag  bool
)

var newCmd = &cobra.Command{
	Use:   "new [name]",
	Short: "Create a new bnto workflow template",
	Long: `Create a new bnto workflow template file.

Creates a fresh bnto template with a basic structure that you can customize.
By default, bntos are saved to ~/.bnto/bntos/

Examples:
  bnto new my-workflow                    # Creates ~/.bnto/bntos/my-workflow.bnto.json
  bnto new my-workflow --local            # Creates ./my-workflow.bnto.json
  bnto new my-workflow --type simple      # Specify template type`,
	Args: cobra.ExactArgs(1),
	RunE: runNew,
}

func init() {
	newCmd.Flags().StringVar(&templateType, "type", "simple", "Template type (simple, loop, parallel)")
	newCmd.Flags().BoolVar(&newDryRunFlag, "dry-run", false, "Show what would be created without creating files")
	newCmd.Flags().BoolVar(&newLocalFlag, "local", false, "Create bnto in current directory instead of ~/.bnto/bntos/")
}

// runNew executes the new command logic.
func runNew(cmd *cobra.Command, args []string) error {
	name := args[0]

	// If dry run, show what would be created and exit
	if newDryRunFlag {
		return showNewDryRun(name)
	}

	// Create bnto template
	template := createTemplate(name)

	// Save to storage or local file based on flag
	if newLocalFlag {
		return createLocalBnto(name, template)
	}

	return createStorageBnto(name, template)
}

// createStorageBnto creates a bnto in ~/.bnto/bntos/ using hangiri storage.
func createStorageBnto(name string, template *node.Definition) error {
	printInfo(fmt.Sprintf("Creating new bnto: %s", name))

	storage := storage.NewDefaultStorage()
	ctx := context.Background()

	// Check if bnto already exists
	if storage.BntoExists(ctx, name) {
		printError(fmt.Sprintf("Bnto '%s' already exists in ~/.bnto/bntos/", name))
		return fmt.Errorf("bnto already exists: %s", name)
	}

	// Save to storage
	if err := storage.SaveBnto(ctx, name, template); err != nil {
		printError(fmt.Sprintf("Failed to create bnto: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("Created: ~/.bnto/bntos/%s.bnto.json", name))
	showNextSteps(name, false)
	return nil
}

// createLocalBnto creates a bnto in the current directory.
func createLocalBnto(name string, template *node.Definition) error {
	fileName := name + ".bnto.json"

	printInfo(fmt.Sprintf("Creating new bnto: %s", name))

	// Check if file already exists
	if _, err := os.Stat(fileName); err == nil {
		printError(fmt.Sprintf("File '%s' already exists", fileName))
		return fmt.Errorf("file already exists: %s", fileName)
	}

	// Write template to local file
	if err := writeTemplate(fileName, template); err != nil {
		printError(fmt.Sprintf("Failed to create bnto: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("Created: %s", fileName))
	showNextSteps(fileName, true)
	return nil
}

// showNextSteps displays next steps after creating bnto.
func showNextSteps(nameOrPath string, isLocal bool) {
	fmt.Println("\nNext steps:")
	if isLocal {
		fmt.Printf("  1. Edit %s\n", nameOrPath)
		fmt.Printf("  2. Run: bnto validate %s\n", nameOrPath)
		fmt.Printf("  3. Run: bnto run %s\n", nameOrPath)
	} else {
		fmt.Printf("  1. Edit ~/.bnto/bntos/%s.bnto.json\n", nameOrPath)
		fmt.Printf("  2. Run: bnto validate %s\n", nameOrPath)
		fmt.Printf("  3. Run: bnto run %s\n", nameOrPath)
	}
}

// createTemplate creates a template bnto definition.
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
				"message": "Hello from bnto!",
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

	location := "~/.bnto/bntos/" + name + ".bnto.json"
	if newLocalFlag {
		location = "./" + name + ".bnto.json"
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

	printSuccess("Dry run complete. Use 'bnto new' without --dry-run to create the file.")
	return nil
}
