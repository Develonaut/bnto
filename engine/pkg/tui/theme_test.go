// Package tui provides terminal styling and progress display.
//
// Tests for lipgloss theme styles.
package tui

import "testing"

// TestBuildTheme verifies theme building from palette.
func TestBuildTheme(t *testing.T) {
	palette := GetPalette(VariantNasu)
	theme := BuildTheme(palette)

	if theme == nil {
		t.Fatal("BuildTheme() returned nil")
	}

	// Verify all styles are initialized by rendering
	if theme.Title.Render("test") == "" {
		t.Error("Title style failed to render")
	}
	if theme.Success.Render("test") == "" {
		t.Error("Success style failed to render")
	}
	if theme.Error.Render("test") == "" {
		t.Error("Error style failed to render")
	}
	if theme.Warning.Render("test") == "" {
		t.Error("Warning style failed to render")
	}
}

// TestBuildTheme_AllVariants verifies theme building for all variants.
func TestBuildTheme_AllVariants(t *testing.T) {
	for _, variant := range AllVariants() {
		t.Run(string(variant), func(t *testing.T) {
			palette := GetPalette(variant)
			theme := BuildTheme(palette)

			// Verify styles can render with this palette
			if theme.Title.Render("test") == "" {
				t.Error("Title style failed to render")
			}
			if theme.Success.Render("test") == "" {
				t.Error("Success style failed to render")
			}
			if theme.Error.Render("test") == "" {
				t.Error("Error style failed to render")
			}
			if theme.Warning.Render("test") == "" {
				t.Error("Warning style failed to render")
			}
		})
	}
}

// TestThemeStructure verifies Theme struct has all required styles.
func TestThemeStructure(t *testing.T) {
	palette := GetPalette(VariantTonkotsu)
	theme := BuildTheme(palette)

	// Test that we can use each style (won't panic)
	_ = theme.Title.Render("test")
	_ = theme.Success.Render("test")
	_ = theme.Error.Render("test")
	_ = theme.Warning.Render("test")
	_ = theme.Info.Render("test")
	_ = theme.Subtle.Render("test")
}
