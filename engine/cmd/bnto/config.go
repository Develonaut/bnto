// Package main implements the config command for managing bnto settings.
package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"

	"github.com/Develonaut/bnto/pkg/paths"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Manage bnto configuration settings",
	Long: `Manage bnto configuration settings.

Examples:
  bnto config home ~/Dropbox/.bnto     # Set bnto home directory
  bnto config home                       # Show current bnto home
  bnto config list                       # List current settings`,
}

var configHomeCmd = &cobra.Command{
	Use:   "home [directory]",
	Short: "Get or set the bnto home directory",
	Long: `Get or set the bnto home directory where bntos, config, and logs are stored.

This allows you to use a custom location (like Google Drive) for syncing your bnto
setup across multiple computers.

Examples:
  bnto config home                                    # Show current home directory
  bnto config home ~/Dropbox/.bnto                   # Set home to Dropbox
  bnto config home "/path/to/Google Drive/.bnto"     # Set home to Google Drive`,
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
		home := paths.LoadBntoHome()
		printInfo(fmt.Sprintf("Current bnto home: %s", home))
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
	if err := paths.SaveBntoHome(newHome); err != nil {
		printError(fmt.Sprintf("Failed to save bnto home: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("Bnto home set to: %s", newHome))
	printInfo("Note: Existing bntos in the old location will not be moved automatically.")
	return nil
}

// runConfigList lists current configuration settings.
func runConfigList(cmd *cobra.Command, args []string) error {
	home := paths.LoadBntoHome()

	fmt.Println("Current Configuration:")
	fmt.Printf("  Home: %s\n", home)

	return nil
}
