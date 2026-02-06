// Package tui provides terminal styling and progress display.
//
// Theme manager for switching color variants.
package tui

// Manager manages the active theme variant.
type Manager struct {
	variant Variant
	palette Palette
	theme   *Theme
}

// currentVariant tracks the global active variant.
var currentVariant Variant

// NewManager creates a theme manager with the saved or default variant.
func NewManager() *Manager {
	// Load saved theme or use Maguro default
	variant := LoadSavedTheme()
	palette := GetPalette(variant)
	theme := BuildTheme(palette)
	currentVariant = variant

	return &Manager{
		variant: variant,
		palette: palette,
		theme:   theme,
	}
}

// SetVariant changes the active theme variant.
// Updates the manager's palette and saves the preference to disk.
func (m *Manager) SetVariant(v Variant) {
	m.variant = v
	m.palette = GetPalette(v)
	m.theme = BuildTheme(m.palette)
	currentVariant = v

	// Save theme preference (ignore errors - non-critical)
	_ = SaveTheme(v)
}

// GetVariant returns the current variant.
func (m *Manager) GetVariant() Variant {
	return m.variant
}

// GetPalette returns the current color palette.
func (m *Manager) GetPalette() Palette {
	return m.palette
}

// GetTheme returns the current theme styles.
func (m *Manager) GetTheme() *Theme {
	return m.theme
}

// CurrentVariant returns the global current variant.
func CurrentVariant() Variant {
	return currentVariant
}

// NextVariant cycles to the next theme variant.
// Wraps around to the first variant after the last one.
func (m *Manager) NextVariant() Variant {
	variants := AllVariants()

	// Find current variant index
	for i, v := range variants {
		if v == m.variant {
			// Get next variant (wrap around)
			next := variants[(i+1)%len(variants)]
			m.SetVariant(next)
			return next
		}
	}

	// If somehow not found, stay on current
	return m.variant
}
