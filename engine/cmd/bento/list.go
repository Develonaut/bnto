// Package main implements the list command for listing bentos.
//
// The list command scans a directory for .bento.json files and displays
// them in a user-friendly format with names and metadata.
package main

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/Develonaut/bento/pkg/storage"
	"github.com/Develonaut/bento/pkg/neta"
	"github.com/spf13/cobra"
)

var (
	recursiveFlag bool
	jsonFlag      bool
)

var listCmd = &cobra.Command{
	Use:   "list [directory]",
	Short: "List available bento workflows",
	Long: `List all available bento workflows.

This command shows you all the bentos in the specified location.
By default, lists bentos from ~/.bento/bentos/

Examples:
  bento list                       # List bentos from ~/.bento/bentos/
  bento list ~/workflows           # List bentos from specific directory
  bento list ~/workflows --recursive`,
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

// runListFromStorage lists bentos from ~/.bento/bentos/ using storage.
func runListFromStorage() error {
	storage := storage.NewDefaultStorage()
	ctx := context.Background()

	names, err := storage.ListBentos(ctx)
	if err != nil {
		printError(fmt.Sprintf("Failed to list bentos: %v", err))
		return err
	}

	if len(names) == 0 {
		fmt.Println("No bentos found in ~/.bento/bentos/")
		fmt.Println("\nCreate your first bento with: bento new my-workflow")
		return nil
	}

	printInfo("Available Bentos (from ~/.bento/bentos/):\n")
	for _, name := range names {
		// Try to load each bento to get its full name and node count
		def, err := storage.LoadBento(ctx, name)
		if err != nil {
			fmt.Printf("  %s.bento.json\n", name)
			fmt.Printf("    (unable to load: %v)\n\n", err)
			continue
		}

		fmt.Printf("  %s.bento.json\n", name)
		if def.Name != "" {
			fmt.Printf("    %s\n", def.Name)
			fmt.Printf("    %d nodes\n", len(def.Nodes))
		}
		fmt.Println()
	}
	fmt.Printf("\n%d bentos found\n", len(names))
	return nil
}

// runListFromDirectory lists bentos from a specific directory (legacy behavior).
func runListFromDirectory(dir string) error {
	bentos, err := findBentos(dir)
	if err != nil {
		printError(fmt.Sprintf("Failed to scan directory: %v", err))
		return err
	}

	displayBentos(bentos)
	return nil
}

// displayBentos displays the list of found bentos.
func displayBentos(bentos []bentoInfo) {
	if len(bentos) == 0 {
		fmt.Println("No bentos found")
		return
	}

	printInfo("Available Bentos:\n")
	for _, bento := range bentos {
		printBento(bento)
	}
	fmt.Printf("\n%d bentos found\n", len(bentos))
}

// bentoInfo contains metadata about a bento file.
type bentoInfo struct {
	Path     string
	FileName string
	Name     string
	NumNodes int
}

// findBentos finds all .bento.json files in the given directory.
func findBentos(dir string) ([]bentoInfo, error) {
	var bentos []bentoInfo

	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if shouldSkipDir(info, path, dir) {
			return filepath.SkipDir
		}

		if isBentoFile(info) {
			bentos = append(bentos, extractBentoInfo(path))
		}

		return nil
	})

	return bentos, err
}

// shouldSkipDir checks if directory should be skipped.
func shouldSkipDir(info os.FileInfo, path, dir string) bool {
	return info.IsDir() && path != dir && !recursiveFlag
}

// isBentoFile checks if file is a bento JSON file.
func isBentoFile(info os.FileInfo) bool {
	return !info.IsDir() && strings.HasSuffix(info.Name(), ".bento.json")
}

// extractBentoInfo extracts metadata from a bento file.
func extractBentoInfo(path string) bentoInfo {
	info := bentoInfo{
		Path:     path,
		FileName: filepath.Base(path),
	}

	// Try to load the bento to get name and node count
	def, err := loadBento(path)
	if err == nil {
		info.Name = def.Name
		info.NumNodes = countNodes(def)
	}

	return info
}

// countNodes counts the number of nodes in a bento.
func countNodes(def *neta.Definition) int {
	return len(def.Nodes)
}

// printBento prints a single bento entry.
func printBento(b bentoInfo) {
	fmt.Printf("  %s\n", b.FileName)
	if b.Name != "" {
		fmt.Printf("    %s\n", b.Name)
		fmt.Printf("    %d nodes\n", b.NumNodes)
	}
	fmt.Println()
}
