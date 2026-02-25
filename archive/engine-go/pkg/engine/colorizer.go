package engine

import (
	"regexp"
	"strings"

	"github.com/charmbracelet/lipgloss"
)

// Color definitions for different parts of log messages
var (
	// Breadcrumb colors
	colorBreadcrumb = lipgloss.NewStyle().Foreground(lipgloss.Color("205")) // Hot Pink

	// NODE type colors
	colorNODELabel = lipgloss.NewStyle().Foreground(lipgloss.Color("51"))  // Cyan
	colorNODEType  = lipgloss.NewStyle().Foreground(lipgloss.Color("141")) // Purple

	// Status word colors
	colorStatusRunning   = lipgloss.NewStyle().Foreground(lipgloss.Color("220")) // Gold (for running)
	colorStatusCompleted = lipgloss.NewStyle().Foreground(lipgloss.Color("46"))  // Bright Green (for completed)
	colorStatusSuccess   = lipgloss.NewStyle().Foreground(lipgloss.Color("46"))  // Bright Green (for success)
	colorStatusFailed    = lipgloss.NewStyle().Foreground(lipgloss.Color("196")) // Red (for failures)

	// Metadata colors
	colorDuration = lipgloss.NewStyle().Foreground(lipgloss.Color("242")) // Gray
	colorProgress = lipgloss.NewStyle().Foreground(lipgloss.Color("208")) // Orange
)

// colorizeMessage applies colors to different parts of a log message.
// Format: "[breadcrumb] StatusWord NODE:type name (duration, progress%)"
func colorizeMessage(text string, isRunning bool, isSuccess bool, isFailed bool) string {
	// Handle breadcrumb at start [Parent:Child]
	breadcrumbRegex := regexp.MustCompile(`^\[([^\]]+)\]\s+`)
	text = breadcrumbRegex.ReplaceAllStringFunc(text, func(match string) string {
		breadcrumb := strings.Trim(strings.TrimPrefix(match, "["), "] ")
		return colorBreadcrumb.Render(breadcrumb) + " "
	})

	// Colorize "NODE:type" pattern
	nodeRegex := regexp.MustCompile(`NODE:([a-z-]+)`)
	text = nodeRegex.ReplaceAllStringFunc(text, func(match string) string {
		parts := strings.Split(match, ":")
		if len(parts) == 2 {
			return colorNODELabel.Render("NODE") + ":" + colorNODEType.Render(parts[1])
		}
		return match
	})

	// Colorize status words (first word in message after breadcrumb)
	statusWords := append(statusWordsRunning, statusWordsCompleted...)
	for _, word := range statusWords {
		if strings.Contains(text, word+" ") || strings.Contains(text, word+"!") {
			var colorStyle lipgloss.Style
			if isRunning {
				colorStyle = colorStatusRunning
			} else {
				colorStyle = colorStatusCompleted
			}
			text = strings.ReplaceAll(text, word+" ", colorStyle.Render(word)+" ")
			text = strings.ReplaceAll(text, word+"!", colorStyle.Render(word)+"!")
		}
	}

	// Colorize success keywords
	successKeywords := []string{"Delicious", "successfully", "success", "Success", "completed", "Completed"}
	for _, keyword := range successKeywords {
		if strings.Contains(text, keyword) {
			text = strings.ReplaceAll(text, keyword, colorStatusSuccess.Render(keyword))
		}
	}

	// Colorize failure keywords
	failureKeywords := []string{"Failed", "failed", "error", "Error", "ERROR"}
	for _, keyword := range failureKeywords {
		if strings.Contains(text, keyword) {
			text = strings.ReplaceAll(text, keyword, colorStatusFailed.Render(keyword))
		}
	}

	// Colorize duration (e.g., "37ms", "2s")
	durationRegex := regexp.MustCompile(`\(([0-9]+(?:\.[0-9]+)?(?:ms|s|m|h))`)
	text = durationRegex.ReplaceAllStringFunc(text, func(match string) string {
		return "(" + colorDuration.Render(strings.TrimPrefix(match, "("))
	})

	// Colorize progress percentage (e.g., "75%")
	progressRegex := regexp.MustCompile(`([0-9]+)%\)`)
	text = progressRegex.ReplaceAllStringFunc(text, func(match string) string {
		pct := strings.TrimSuffix(match, "%)")
		return colorProgress.Render(pct+"%") + ")"
	})

	return text
}
