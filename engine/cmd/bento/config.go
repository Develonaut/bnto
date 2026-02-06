// Package main implements the config command for managing bento settings.
package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"

	"github.com/Develonaut/bento/pkg/tui"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Manage bento configuration settings",
	Long: `Manage bento configuration settings including theme and slowMo delay.

Examples:
  bento config theme wasabi          # Set theme to Wasabi (green)
  bento config theme                 # Show current theme
  bento config slowmo 1000           # Set slowMo delay to 1 second
  bento config slowmo                # Show current slowMo delay
  bento config list                  # List all available themes`,
}

var configThemeCmd = &cobra.Command{
	Use:   "theme [variant]",
	Short: "Get or set the color theme",
	Long: `Get or set the color theme variant.

Available themes:
  Nasu      - Purple (eggplant sushi)
  Wasabi    - Green (wasabi)
  Toro      - Pink (fatty tuna)
  Tamago    - Yellow (egg sushi)
  Tonkotsu  - Creamy white (pork bone broth) - default
  Saba      - Cyan (mackerel)
  Ika       - White (squid)

Examples:
  bento config theme wasabi    # Set theme to Wasabi
  bento config theme           # Show current theme`,
	Args: cobra.MaximumNArgs(1),
	RunE: runConfigTheme,
}

var configSlowMoCmd = &cobra.Command{
	Use:   "slowmo [milliseconds]",
	Short: "Get or set the slowMo execution delay",
	Long: `Get or set the slowMo execution delay in milliseconds.

The slowMo delay adds an artificial pause between node executions,
making animations and progress updates more visible.

Examples:
  bento config slowmo 1000     # Set slowMo to 1 second
  bento config slowmo 250      # Set slowMo to 250ms
  bento config slowmo          # Show current slowMo delay`,
	Args: cobra.MaximumNArgs(1),
	RunE: runConfigSlowMo,
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
	Long:  `List all available themes and current configuration settings.`,
	RunE:  runConfigList,
}

func init() {
	configCmd.AddCommand(configThemeCmd)
	configCmd.AddCommand(configSlowMoCmd)
	configCmd.AddCommand(configHomeCmd)
	configCmd.AddCommand(configListCmd)
}

// runConfigTheme handles the theme subcommand.
func runConfigTheme(cmd *cobra.Command, args []string) error {
	if len(args) == 0 {
		// Show current theme
		variant := tui.LoadSavedTheme()
		palette := tui.GetPalette(variant)

		printInfo(fmt.Sprintf("Current theme: %s", variant))
		printInfo(fmt.Sprintf("  Primary color: %s", palette.Primary))
		printInfo(fmt.Sprintf("  Secondary color: %s", palette.Secondary))
		return nil
	}

	// Set theme
	variantName := args[0]
	variant := tui.Variant(variantName)

	// Validate variant
	validVariants := tui.AllVariants()
	isValid := false
	for _, v := range validVariants {
		if v == variant {
			isValid = true
			break
		}
	}

	if !isValid {
		printError(fmt.Sprintf("Invalid theme: %s", variantName))
		fmt.Println("\nAvailable themes:")
		for _, v := range validVariants {
			p := tui.GetPalette(v)
			fmt.Printf("  %s - %s\n", v, p.Primary)
		}
		return fmt.Errorf("invalid theme variant")
	}

	// Save theme
	if err := tui.SaveTheme(variant); err != nil {
		printError(fmt.Sprintf("Failed to save theme: %v", err))
		return err
	}

	palette := tui.GetPalette(variant)
	printSuccess(fmt.Sprintf("Theme set to %s (%s)", variant, palette.Primary))
	return nil
}

// runConfigSlowMo handles the slowmo subcommand.
func runConfigSlowMo(cmd *cobra.Command, args []string) error {
	if len(args) == 0 {
		// Show current slowMo
		ms := tui.LoadSlowMoDelay()
		printInfo(fmt.Sprintf("Current slowMo delay: %dms", ms))
		return nil
	}

	// Set slowMo
	var ms int
	if _, err := fmt.Sscanf(args[0], "%d", &ms); err != nil {
		printError(fmt.Sprintf("Invalid slowMo value: %s (must be a number)", args[0]))
		return err
	}

	if ms < 0 {
		printError("SlowMo delay cannot be negative")
		return fmt.Errorf("invalid slowMo value")
	}

	if err := tui.SaveSlowMoDelay(ms); err != nil {
		printError(fmt.Sprintf("Failed to save slowMo delay: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("SlowMo delay set to %dms", ms))
	return nil
}

// runConfigHome handles the home subcommand.
func runConfigHome(cmd *cobra.Command, args []string) error {
	if len(args) == 0 {
		// Show current home
		home := tui.LoadBentoHome()
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
	if err := tui.SaveBentoHome(newHome); err != nil {
		printError(fmt.Sprintf("Failed to save bento home: %v", err))
		return err
	}

	printSuccess(fmt.Sprintf("Bento home set to: %s", newHome))
	printInfo("Note: Existing bentos in the old location will not be moved automatically.")
	return nil
}

// runConfigList lists all configuration options.
func runConfigList(cmd *cobra.Command, args []string) error {
	// Show current settings
	variant := tui.LoadSavedTheme()
	slowMo := tui.LoadSlowMoDelay()
	home := tui.LoadBentoHome()

	fmt.Println("Current Configuration:")
	fmt.Printf("  Theme: %s\n", variant)
	fmt.Printf("  SlowMo: %dms\n", slowMo)
	fmt.Printf("  Home: %s\n", home)
	fmt.Println()

	// List all themes
	fmt.Println("Available Themes:")
	for _, v := range tui.AllVariants() {
		p := tui.GetPalette(v)
		current := ""
		if v == variant {
			current = " (current)"
		}
		fmt.Printf("  %s - %s%s\n", v, p.Primary, current)
	}

	return nil
}
