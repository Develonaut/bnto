// Package main implements the bnto CLI.
//
// Bnto is a high-performance workflow automation CLI written in Go.
//
// Commands:
//   - run: Execute a bnto workflow
//   - validate: Validate a bnto without executing
//   - list: List available bntos
//   - new: Create a new bnto template
//   - docs: View documentation
//   - secrets: Manage secrets
//
// Learn more: https://github.com/Develonaut/bnto
package main

import (
	"os"
	"strings"
	"time"

	"github.com/spf13/cobra"
)

var version = "dev" // Set by build process

var rootCmd = &cobra.Command{
	Use:   "bnto [file]",
	Short: "High-performance workflow automation",
	Long: `Bnto - High-performance workflow automation

Bnto lets you build powerful automation workflows using composable
nodes that can be connected together.

Usage:
  bnto [file]              Execute a bnto workflow (default)
  bnto run [file]          Execute a bnto workflow (explicit)
  bnto [file] --dry-run    Preview execution without running

File paths support multiple formats:
  bnto examples/workflow.bnto.json   (full path with extension)
  bnto examples/workflow              (full path without extension)
  bnto workflow                       (name from ~/.bnto/bntos/)

Available Commands:
  • run      - Execute a bnto workflow
  • validate - Validate a workflow without executing
  • list     - List available bnto workflows
  • new      - Create a new bnto workflow template
  • docs     - View documentation
  • secrets  - Manage secrets securely
  • logs     - View and tail execution logs
  • version  - Show version information`,
}

func main() {
	// If no args provided, show help
	if len(os.Args) == 1 {
		os.Args = append(os.Args, "help")
	} else if len(os.Args) > 1 {
		// If first arg looks like a file path, inject "run" command
		// This allows: bnto [file] instead of: bnto run [file]
		firstArg := os.Args[1]
		// Check if first arg is not a known subcommand and looks like a file
		if !isKnownSubcommand(firstArg) && looksLikeFile(firstArg) {
			// Inject "run" as the subcommand
			os.Args = append([]string{os.Args[0], "run"}, os.Args[1:]...)
		}
	}

	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

// isKnownSubcommand checks if the arg is a registered subcommand.
func isKnownSubcommand(arg string) bool {
	knownCommands := []string{"run", "validate", "list", "new", "docs", "secrets", "logs", "version", "v", "help", "config"}
	for _, cmd := range knownCommands {
		if arg == cmd {
			return true
		}
	}
	return false
}

// looksLikeFile returns true if the arg looks like a file path or bnto name.
// Checks for path separators, .json extension, or anything that's not a flag.
func looksLikeFile(arg string) bool {
	// If it starts with dash, it's a flag, not a file
	if strings.HasPrefix(arg, "-") {
		return false
	}
	// If it has path separator or .json extension, it's definitely a file
	if strings.Contains(arg, "/") || strings.Contains(arg, "\\") ||
		strings.HasSuffix(arg, ".json") || strings.HasSuffix(arg, ".bnto.json") {
		return true
	}
	// Otherwise, assume it's a bnto name from storage
	// This allows: bnto csv-to-folders, bnto my-workflow, etc.
	return true
}

func init() {
	// Add run command flags to root so they work with both:
	// - bnto [file] --verbose
	// - bnto run [file] --verbose
	rootCmd.PersistentFlags().BoolVar(&verboseFlag, "verbose", false, "Verbose output")
	rootCmd.PersistentFlags().DurationVar(&timeoutFlag, "timeout", 10*time.Minute, "Execution timeout")
	rootCmd.PersistentFlags().BoolVar(&dryRunFlag, "dry-run", false, "Show what would be executed without running")

	rootCmd.AddCommand(runCmd)
	rootCmd.AddCommand(validateCmd)
	rootCmd.AddCommand(listCmd)
	rootCmd.AddCommand(newCmd)
	rootCmd.AddCommand(docsCmd)
	rootCmd.AddCommand(secretsCmd)
	rootCmd.AddCommand(logsCmd)
	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(configCmd)
}
