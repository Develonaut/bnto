package logger

import (
	"github.com/charmbracelet/lipgloss"
	"github.com/charmbracelet/log"
)

// createCustomStyles creates custom styles for log output.
// This configures colors for:
// - Log levels (INFO, DEBUG, WARN, ERROR)
// - Breadcrumb fields (bento name, neta type, neta name)
func createCustomStyles() *log.Styles {
	styles := log.DefaultStyles()

	// Customize log level colors to be more vibrant
	styles.Levels[log.DebugLevel] = lipgloss.NewStyle().
		SetString("DEBUG").
		Padding(0, 1, 0, 1).
		Foreground(lipgloss.Color("63")) // Purple/Magenta

	styles.Levels[log.InfoLevel] = lipgloss.NewStyle().
		SetString("INFO").
		Padding(0, 1, 0, 1).
		Foreground(lipgloss.Color("39")) // Bright Blue

	styles.Levels[log.WarnLevel] = lipgloss.NewStyle().
		SetString("WARN").
		Padding(0, 1, 0, 1).
		Foreground(lipgloss.Color("220")) // Gold/Yellow

	styles.Levels[log.ErrorLevel] = lipgloss.NewStyle().
		SetString("ERROR").
		Padding(0, 1, 0, 1).
		Foreground(lipgloss.Color("196")) // Bright Red

	// Customize key colors for breadcrumb fields
	styles.Keys["bento"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("205")) // Hot Pink (for bento name)

	styles.Keys["neta_type"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("51")) // Cyan (for neta type)

	styles.Keys["neta_name"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("118")) // Green (for neta name)

	styles.Keys["status"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("141")) // Purple (for status words)

	styles.Keys["duration"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("242")) // Gray (for durations)

	styles.Keys["progress"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("208")) // Orange (for progress %)

	// Customize value colors to match keys
	styles.Values["bento"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("205")) // Hot Pink

	styles.Values["neta_type"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("51")) // Cyan

	styles.Values["neta_name"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("118")) // Green

	styles.Values["status"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("141")) // Purple

	styles.Values["duration"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("242")) // Gray

	styles.Values["progress"] = lipgloss.NewStyle().
		Foreground(lipgloss.Color("208")) // Orange

	return styles
}
