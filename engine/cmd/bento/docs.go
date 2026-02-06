// Package main implements the docs command for viewing bento documentation.
//
// The docs command uses charm glow to render markdown documentation
// in a beautiful, readable format directly in the terminal.
package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"

	"github.com/spf13/cobra"
)

var docsCmd = &cobra.Command{
	Use:   "docs [doc-name]",
	Short: "View bento documentation",
	Long: `View bento documentation in beautiful markdown format.

Uses charm glow to render documentation files in the terminal.

Available docs:
  readme          - Project README
  overview        - Project overview and architecture
  packages        - Package naming conventions
  principles      - Bento Box Principle
  emojis          - Approved emoji list
  charm           - Charm stack integration
  nodes           - Complete node inventory
  standards       - Go standards review
  status-words    - Status word guidelines

Examples:
  bento docs readme
  bento docs principles
  bento docs nodes`,
	Args: cobra.MaximumNArgs(1),
	RunE: runDocs,
}

// docMap maps friendly names to file paths.
var docMap = map[string]string{
	"readme":       "README.md",
	"overview":     ".claude/README.md",
	"packages":     ".claude/PACKAGE_NAMING.md",
	"principles":   ".claude/BENTO_BOX_PRINCIPLE.md",
	"emojis":       ".claude/EMOJIS.md",
	"charm":        ".claude/CHARM_STACK.md",
	"nodes":        ".claude/COMPLETE_NODE_INVENTORY.md",
	"standards":    ".claude/GO_STANDARDS_REVIEW.md",
	"status-words": ".claude/STATUS_WORDS.md",
}

// runDocs executes the docs command logic.
func runDocs(cmd *cobra.Command, args []string) error {
	// Default to README if no doc specified
	docName := "readme"
	if len(args) > 0 {
		docName = args[0]
	}

	// Look up the file path
	filePath, ok := docMap[docName]
	if !ok {
		return fmt.Errorf("unknown doc: %s\nRun 'bento docs --help' to see available docs", docName)
	}

	// Check if glow is installed
	if !isGlowInstalled() {
		return fmt.Errorf("%s", getGlowInstallMessage())
	}

	// Get absolute path to the doc
	absPath, err := filepath.Abs(filePath)
	if err != nil {
		return fmt.Errorf("failed to resolve path: %w", err)
	}

	// Check if file exists
	if _, err := os.Stat(absPath); os.IsNotExist(err) {
		return fmt.Errorf("doc file not found: %s", absPath)
	}

	// Run glow
	return viewWithGlow(absPath)
}

// isGlowInstalled checks if glow is available.
func isGlowInstalled() bool {
	_, err := exec.LookPath("glow")
	return err == nil
}

// viewWithGlow renders the markdown file with glow.
func viewWithGlow(path string) error {
	glowCmd := exec.Command("glow", path)
	glowCmd.Stdin = os.Stdin
	glowCmd.Stdout = os.Stdout
	glowCmd.Stderr = os.Stderr
	return glowCmd.Run()
}

// getGlowInstallMessage returns OS-specific installation instructions for glow.
func getGlowInstallMessage() string {
	var installCmd string
	switch runtime.GOOS {
	case "darwin":
		installCmd = "brew install glow"
	case "linux":
		installCmd = "sudo apt install glow  # Debian/Ubuntu\n  sudo yum install glow      # RedHat/CentOS\n  go install github.com/charmbracelet/glow@latest"
	case "windows":
		installCmd = "scoop install glow\n  choco install glow"
	default:
		installCmd = "go install github.com/charmbracelet/glow@latest"
	}

	return fmt.Sprintf(`glow is not installed

Glow is required to view documentation in a beautiful format.

Install with:
  %s

Or visit: https://github.com/charmbracelet/glow`, installCmd)
}
