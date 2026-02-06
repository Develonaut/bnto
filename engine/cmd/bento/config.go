// Package main implements the config command for managing bento settings.
package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"

	"github.com/Develonaut/bento/pkg/paths"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Manage bento configuration settings",
	Long: `Manage bento configuration settings.

Examples:
  bento config home ~/Dropbox/.bento     # Set bento home directory
  bento config home                       # Show current bento home
  bento config list                       # List current settings`,
}

var configHomeCmd = &cobra.Command{
	Use:   "home [directory]",
	Short: "Get or set the bento home directory",
	Long: `Get or set the bento home directory where bentos, config, and logs are stored.

This allows you to use a custom location (like Google Drive) for syncing your bento
setup across multiple computers.

Examples:
  bento config home                                    # Show current home directory
  bento config home ~/Dropbox/.bento                   # Set home to Dropbox
  bento config home "/path/to/Google Drive/.bento"     # Set home to Google Drive`,
	Args: cobra.MaximumNArgs(1),
	RunE: runConfigHome,
}

var configListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all available configuration options",
	Long:  `List current configuration settings.`,
	RunE:  runConfigList,
}

func init() {
	configCmd.AddCommand(configHomeCmd)
	configCmd.AddCommand(configListCmd)
}

// runConfigHome handles the home subcommand.
func runConfigHome(cmd *cobra.Command, args []string) error {
	if len(args) == 0 {
		// Show current home
		home := paths.LoadBentoHome()
		printInfo(fmt.Sprintf("Current bento home: %s", home))
		return nil
	}

	// Set home
	newHome := args[0]

	// Expand ~ to home directory
	if newHome[0] == '~' {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			printError(fmt.Sprintf("Failed to get home directory: %v", err))
			return err
		}
		newHome = filepath.Join(homeDir, newHome[1:])
	}

	// Save the new home
	if err := paths.SaveBentoHome(newHome); err != nil {
		printError(fmt.Sprintf("Failed to save bento home: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("Bento home set to: %s", newHome))
	printInfo("Note: Existing bentos in the old location will not be moved automatically.")
	return nil
}

// runConfigList lists current configuration settings.
func runConfigList(cmd *cobra.Command, args []string) error {
	home := paths.LoadBentoHome()

	fmt.Println("Current Configuration:")
	fmt.Printf("  Home: %s\n", home)

	return nil
}
