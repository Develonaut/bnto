// Package tui provides terminal styling and progress display.
//
// Sushi-themed color variants for the Bento CLI.
package tui

import "github.com/charmbracelet/lipgloss"

// Variant represents a sushi-themed color variant.
type Variant string

const (
	VariantNasu     Variant = "Nasu"     // Purple (eggplant sushi) - TUI default
	VariantWasabi   Variant = "Wasabi"   // Green (wasabi)
	VariantToro     Variant = "Toro"     // Pink (fatty tuna)
	VariantTamago   Variant = "Tamago"   // Yellow (egg sushi)
	VariantTonkotsu Variant = "Tonkotsu" // Creamy white (pork bone broth) - CLI default
	VariantSaba     Variant = "Saba"     // Cyan (mackerel)
	VariantIka      Variant = "Ika"      // White (squid)
)

// AllVariants returns all available theme variants in order.
func AllVariants() []Variant {
	return []Variant{
		VariantNasu,
		VariantWasabi,
		VariantToro,
		VariantTamago,
		VariantTonkotsu,
		VariantSaba,
		VariantIka,
	}
}

// Palette defines semantic colors for a theme variant.
type Palette struct {
	Primary   lipgloss.Color // Main theme color (brand)
	Secondary lipgloss.Color // Accent color
	Success   lipgloss.Color // Success states (green)
	Error     lipgloss.Color // Error states (red)
	Warning   lipgloss.Color // Warning states (yellow)
	Text      lipgloss.Color // Primary text color
	Muted     lipgloss.Color // Subtle/secondary text
}

// GetPalette returns the color palette for a variant.
// Returns Tonkotsu (creamy white) palette for invalid variants.
func GetPalette(v Variant) Palette {
	switch v {
	case VariantNasu:
		return nasuPalette()
	case VariantWasabi:
		return wasabiPalette()
	case VariantToro:
		return toroPalette()
	case VariantTamago:
		return tamagoPalette()
	case VariantTonkotsu:
		return tonkotsuPalette()
	case VariantSaba:
		return sabaPalette()
	case VariantIka:
		return ikaPalette()
	default:
		return tonkotsuPalette()
	}
}

// nasuPalette returns the Nasu (purple) palette.
func nasuPalette() Palette {
	return Palette{
		Primary:   lipgloss.Color("#BD93F9"), // Purple
		Secondary: lipgloss.Color("#FF79C6"), // Pink
		Success:   lipgloss.Color("#50FA7B"), // Green
		Error:     lipgloss.Color("#f87359"), // Red
		Warning:   lipgloss.Color("#F1FA8C"), // Yellow
		Text:      lipgloss.Color("#F8F8F2"), // White
		Muted:     lipgloss.Color("#6272A4"), // Comment
	}
}

// wasabiPalette returns the Wasabi (green) palette.
func wasabiPalette() Palette {
	return Palette{
		Primary:   lipgloss.Color("#50FA7B"), // Green
		Secondary: lipgloss.Color("#8BE9FD"), // Cyan
		Success:   lipgloss.Color("#50FA7B"), // Green
		Error:     lipgloss.Color("#f87359"), // Red
		Warning:   lipgloss.Color("#F1FA8C"), // Yellow
		Text:      lipgloss.Color("#F8F8F2"), // White
		Muted:     lipgloss.Color("#6272A4"), // Comment
	}
}

// toroPalette returns the Toro (pink) palette.
func toroPalette() Palette {
	return Palette{
		Primary:   lipgloss.Color("#FF79C6"), // Pink
		Secondary: lipgloss.Color("#BD93F9"), // Purple
		Success:   lipgloss.Color("#50FA7B"), // Green
		Error:     lipgloss.Color("#f87359"), // Red
		Warning:   lipgloss.Color("#F1FA8C"), // Yellow
		Text:      lipgloss.Color("#F8F8F2"), // White
		Muted:     lipgloss.Color("#6272A4"), // Comment
	}
}

// tamagoPalette returns the Tamago (yellow) palette.
func tamagoPalette() Palette {
	return Palette{
		Primary:   lipgloss.Color("#F1FA8C"), // Yellow
		Secondary: lipgloss.Color("#FFB86C"), // Orange
		Success:   lipgloss.Color("#50FA7B"), // Green
		Error:     lipgloss.Color("#f87359"), // Red
		Warning:   lipgloss.Color("#F1FA8C"), // Yellow
		Text:      lipgloss.Color("#F8F8F2"), // White
		Muted:     lipgloss.Color("#6272A4"), // Comment
	}
}

// tonkotsuPalette returns the Tonkotsu (pork bone broth) palette - CLI default.
func tonkotsuPalette() Palette {
	return Palette{
		Primary:   lipgloss.Color("#f87359"), // Red (same as old Maguro)
		Secondary: lipgloss.Color("#FFB86C"), // Pink
		Success:   lipgloss.Color("#50FA7B"), // Green
		Error:     lipgloss.Color("#f87359"), // Red
		Warning:   lipgloss.Color("#F1FA8C"), // Yellow
		Text:      lipgloss.Color("#F8F8F2"), // White
		Muted:     lipgloss.Color("#6272A4"), // Comment
	}
}

// sabaPalette returns the Saba (cyan) palette.
func sabaPalette() Palette {
	return Palette{
		Primary:   lipgloss.Color("#8BE9FD"), // Cyan
		Secondary: lipgloss.Color("#BD93F9"), // Purple
		Success:   lipgloss.Color("#50FA7B"), // Green
		Error:     lipgloss.Color("#f87359"), // Red
		Warning:   lipgloss.Color("#F1FA8C"), // Yellow
		Text:      lipgloss.Color("#F8F8F2"), // White
		Muted:     lipgloss.Color("#6272A4"), // Comment
	}
}

// ikaPalette returns the Ika (white) palette.
func ikaPalette() Palette {
	return Palette{
		Primary:   lipgloss.Color("#F8F8F2"), // White
		Secondary: lipgloss.Color("#BFBFBF"), // Light Gray
		Success:   lipgloss.Color("#50FA7B"), // Green
		Error:     lipgloss.Color("#f87359"), // Red
		Warning:   lipgloss.Color("#F1FA8C"), // Yellow
		Text:      lipgloss.Color("#F8F8F2"), // White
		Muted:     lipgloss.Color("#6272A4"), // Comment
	}
}
