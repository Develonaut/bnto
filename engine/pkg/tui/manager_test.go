// Package tui provides terminal styling and progress display.
//
// Tests for theme manager.
package tui

import (
	"testing"

	"github.com/Develonaut/bento/pkg/paths"
)

// TestNewManager verifies manager initialization with current variant.
func TestNewManager(t *testing.T) {
	m := NewManager()

	if m == nil {
		t.Fatal("NewManager() returned nil")
	}

	// Should have a variant set
	variant := m.GetVariant()
	if variant == "" {
		t.Error("Manager variant is empty")
	}

	// Variant should be valid
	validVariant := false
	for _, v := range AllVariants() {
		if v == variant {
			validVariant = true
			break
		}
	}
	if !validVariant {
		t.Errorf("Manager variant %s is not valid", variant)
	}
}

// TestSetVariant verifies variant switching.
func TestSetVariant(t *testing.T) {
	tmpDir := t.TempDir()
	t.Cleanup(paths.SetConfigDirForTesting(tmpDir))

	m := NewManager()

	// Set each variant
	for _, variant := range AllVariants() {
		m.SetVariant(variant)

		if m.GetVariant() != variant {
			t.Errorf("After SetVariant(%s), GetVariant() = %s", variant, m.GetVariant())
		}

		// Verify palette matches
		expectedPalette := GetPalette(variant)
		actualPalette := m.GetPalette()

		if actualPalette.Primary != expectedPalette.Primary {
			t.Errorf("After SetVariant(%s), palette.Primary = %s, want %s",
				variant, actualPalette.Primary, expectedPalette.Primary)
		}
	}
}

// TestNextVariant verifies cycling through variants.
func TestNextVariant(t *testing.T) {
	tmpDir := t.TempDir()
	t.Cleanup(paths.SetConfigDirForTesting(tmpDir))

	m := NewManager()
	m.SetVariant(VariantNasu) // Start with first variant

	variants := AllVariants()
	expectedNext := []Variant{
		VariantWasabi,   // After Nasu
		VariantToro,     // After Wasabi
		VariantTamago,   // After Toro
		VariantTonkotsu, // After Tamago
		VariantSaba,     // After Tonkotsu
		VariantIka,      // After Saba
		VariantNasu,     // After Ika (wraps around)
	}

	for i, expected := range expectedNext {
		next := m.NextVariant()
		if next != expected {
			t.Errorf("NextVariant() call %d = %s, want %s", i+1, next, expected)
		}
		if m.GetVariant() != expected {
			t.Errorf("After NextVariant() call %d, GetVariant() = %s, want %s",
				i+1, m.GetVariant(), expected)
		}
	}

	// Verify we cycled through all 7 variants
	if len(expectedNext) != len(variants) {
		t.Errorf("Test cycled through %d variants, but there are %d total",
			len(expectedNext), len(variants))
	}
}

// TestCurrentVariant verifies global current variant tracking.
func TestCurrentVariant(t *testing.T) {
	tmpDir := t.TempDir()
	t.Cleanup(paths.SetConfigDirForTesting(tmpDir))

	m := NewManager()
	m.SetVariant(VariantToro)

	// CurrentVariant() should match manager's variant
	if CurrentVariant() != VariantToro {
		t.Errorf("CurrentVariant() = %s, want %s", CurrentVariant(), VariantToro)
	}

	m.SetVariant(VariantSaba)
	if CurrentVariant() != VariantSaba {
		t.Errorf("After SetVariant(Saba), CurrentVariant() = %s, want Saba", CurrentVariant())
	}
}

// TestGetPalette verifies manager returns correct palette.
func TestGetPalette_Manager(t *testing.T) {
	m := NewManager()

	for _, variant := range AllVariants() {
		m.SetVariant(variant)

		expected := GetPalette(variant)
		actual := m.GetPalette()

		if actual.Primary != expected.Primary {
			t.Errorf("Variant %s: Primary = %s, want %s", variant, actual.Primary, expected.Primary)
		}
		if actual.Success != expected.Success {
			t.Errorf("Variant %s: Success = %s, want %s", variant, actual.Success, expected.Success)
		}
	}
}

// TestGetTheme_Manager verifies manager returns theme.
func TestGetTheme_Manager(t *testing.T) {
	m := NewManager()

	theme := m.GetTheme()
	if theme == nil {
		t.Fatal("Manager.GetTheme() returned nil")
	}

	// Verify theme can render
	if theme.Title.Render("test") == "" {
		t.Error("Theme Title style failed to render")
	}

	// Verify theme updates when variant changes
	m.SetVariant(VariantWasabi)
	newTheme := m.GetTheme()
	if newTheme == nil {
		t.Error("Theme is nil after SetVariant")
	}
}
