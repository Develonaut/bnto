// Package main implements the list command for listing bntos.
//
// The list command scans a directory for .bnto.json files and displays
// them in a user-friendly format with names and metadata.
package main

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/Develonaut/bnto/pkg/storage"
	"github.com/Develonaut/bnto/pkg/node"
	"github.com/spf13/cobra"
)

var (
	recursiveFlag bool
	jsonFlag      bool
)

var listCmd = &cobra.Command{
	Use:   "list [directory]",
	Short: "List available bnto workflows",
	Long: `List all available bnto workflows.

This command shows you all the bntos in the specified location.
By default, lists bntos from ~/.bnto/bntos/

Examples:
  bnto list                       # List bntos from ~/.bnto/bntos/
  bnto list ~/workflows           # List bntos from specific directory
  bnto list ~/workflows --recursive`,
	Args: cobra.MaximumNArgs(1),
	RunE: runList,
}

func init() {
	listCmd.Flags().BoolVarP(&recursiveFlag, "recursive", "r", false, "Search subdirectories")
	listCmd.Flags().BoolVar(&jsonFlag, "json", false, "Output as JSON")
}

// runList executes the list command logic.
func runList(cmd *cobra.Command, args []string) error {
	// If a directory is specified, use the legacy directory scanning
	if len(args) > 0 {
		return runListFromDirectory(args[0])
	}

	// Otherwise, use hangiri storage
	return runListFromStorage()
}

// runListFromStorage lists bntos from ~/.bnto/bntos/ using storage.
func runListFromStorage() error {
	storage := storage.NewDefaultStorage()
	ctx := context.Background()

	names, err := storage.ListBntos(ctx)
	if err != nil {
		printError(fmt.Sprintf("Failed to list bntos: %v", err))
		return err
	}

	if len(names) == 0 {
		fmt.Println("No bntos found in ~/.bnto/bntos/")
		fmt.Println("\nCreate your first bnto with: bnto new my-workflow")
		return nil
	}

	printInfo("Available Bntos (from ~/.bnto/bntos/):\n")
	for _, name := range names {
		// Try to load each bnto to get its full name and node count
		def, err := storage.LoadBnto(ctx, name)
		if err != nil {
			fmt.Printf("  %s.bnto.json\n", name)
			fmt.Printf("    (unable to load: %v)\n\n", err)
			continue
		}

		fmt.Printf("  %s.bnto.json\n", name)
		if def.Name != "" {
			fmt.Printf("    %s\n", def.Name)
			fmt.Printf("    %d nodes\n", len(def.Nodes))
		}
		fmt.Println()
	}
	fmt.Printf("\n%d bntos found\n", len(names))
	return nil
}

// runListFromDirectory lists bntos from a specific directory (legacy behavior).
func runListFromDirectory(dir string) error {
	bntos, err := findBntos(dir)
	if err != nil {
		printError(fmt.Sprintf("Failed to scan directory: %v", err))
		return err
	}

	displayBntos(bntos)
	return nil
}

// displayBntos displays the list of found bntos.
func displayBntos(bntos []bntoInfo) {
	if len(bntos) == 0 {
		fmt.Println("No bntos found")
		return
	}

	printInfo("Available Bntos:\n")
	for _, bnto := range bntos {
		printBnto(bnto)
	}
	fmt.Printf("\n%d bntos found\n", len(bntos))
}

// bntoInfo contains metadata about a bnto file.
type bntoInfo struct {
	Path     string
	FileName string
	Name     string
	NumNodes int
}

// findBntos finds all .bnto.json files in the given directory.
func findBntos(dir string) ([]bntoInfo, error) {
	var bntos []bntoInfo

	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if shouldSkipDir(info, path, dir) {
			return filepath.SkipDir
		}

		if isBntoFile(info) {
			bntos = append(bntos, extractBntoInfo(path))
		}

		return nil
	})

	return bntos, err
}

// shouldSkipDir checks if directory should be skipped.
func shouldSkipDir(info os.FileInfo, path, dir string) bool {
	return info.IsDir() && path != dir && !recursiveFlag
}

// isBntoFile checks if file is a bnto JSON file.
func isBntoFile(info os.FileInfo) bool {
	return !info.IsDir() && strings.HasSuffix(info.Name(), ".bnto.json")
}

// extractBntoInfo extracts metadata from a bnto file.
func extractBntoInfo(path string) bntoInfo {
	info := bntoInfo{
		Path:     path,
		FileName: filepath.Base(path),
	}

	// Try to load the bnto to get name and node count
	def, err := loadBnto(path)
	if err == nil {
		info.Name = def.Name
		info.NumNodes = countNodes(def)
	}

	return info
}

// countNodes counts the number of nodes in a bnto.
func countNodes(def *node.Definition) int {
	return len(def.Nodes)
}

// printBnto prints a single bnto entry.
func printBnto(b bntoInfo) {
	fmt.Printf("  %s\n", b.FileName)
	if b.Name != "" {
		fmt.Printf("    %s\n", b.Name)
		fmt.Printf("    %d nodes\n", b.NumNodes)
	}
	fmt.Println()
}
