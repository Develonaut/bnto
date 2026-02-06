// Package tui provides terminal styling and progress display.
//
// Theme configuration persistence to ~/.bento/config/theme.
package tui

import (
	"github.com/Develonaut/bento/pkg/paths"
)

// LoadSavedTheme loads the saved theme variant from disk.
// Returns VariantTonkotsu (creamy white) as default if no saved theme or on error.
func LoadSavedTheme() Variant {
	themeStr := paths.LoadSavedTheme()
	variant := Variant(themeStr)

	// Validate the variant
	for _, v := range AllVariants() {
		if v == variant {
			return variant
		}
	}

	return VariantTonkotsu
}

// SaveTheme saves the theme variant to disk.
// Creates ~/.bento directory if it doesn't exist.
func SaveTheme(variant Variant) error {
	return paths.SaveTheme(string(variant))
}

// LoadSlowMoDelay loads the saved slowMo delay from disk.
// Returns 5000ms as default if no saved value or on error.
// SlowMo adds artificial delays between node executions to make animations visible.
func LoadSlowMoDelay() int {
	return paths.LoadSlowMoDelay()
}

// SaveSlowMoDelay saves the slowMo delay (in milliseconds) to disk.
// Creates ~/.bento directory if it doesn't exist.
func SaveSlowMoDelay(ms int) error {
	return paths.SaveSlowMoDelay(ms)
}

// LoadSaveDirectory loads the saved bentos directory from disk.
// Returns ~/.bento as default if no saved value or on error.
func LoadSaveDirectory() string {
	return paths.LoadSaveDirectory()
}

// SaveSaveDirectory saves the bentos directory to disk.
// Creates ~/.bento directory if it doesn't exist.
func SaveSaveDirectory(dir string) error {
	return paths.SaveSaveDirectory(dir)
}

// LoadBentoHome loads the configured bento home directory from disk.
// Returns the default ~/.bento if no custom home is configured.
// Automatically resolves {{GDRIVE}} and other special markers.
func LoadBentoHome() string {
	return paths.LoadBentoHome()
}

// SaveBentoHome saves the bento home directory to disk.
// Creates ~/.bento directory if it doesn't exist.
func SaveBentoHome(dir string) error {
	return paths.SaveBentoHome(dir)
}

// LoadVerboseLogging loads the verbose logging setting from disk.
// Returns false as default if no saved value or on error.
func LoadVerboseLogging() bool {
	return paths.LoadVerboseLogging()
}

// SaveVerboseLogging saves the verbose logging setting to disk.
// Creates ~/.bento directory if it doesn't exist.
func SaveVerboseLogging(enabled bool) error {
	return paths.SaveVerboseLogging(enabled)
}
