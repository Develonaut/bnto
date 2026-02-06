// Package main provides output formatting utilities for the bento CLI.
//
// This file contains helpers for creating user-friendly output with:
//   - Duration formatting (ms, seconds, minutes)
//   - Success/error boxes for visual emphasis
//   - Consistent emoji usage
package main

import (
	"fmt"
	"math/rand"
	"strings"
	"time"

	"github.com/Develonaut/bento/pkg/tui"
)

// formatDuration formats a duration for human readability.
//
// Examples:
//   - 45ms -> "45ms"
//   - 1.5s -> "1.5s"
//   - 3m 45s -> "3m 45s"
func formatDuration(d time.Duration) string {
	if d < time.Second {
		return fmt.Sprintf("%dms", d.Milliseconds())
	}
	if d < time.Minute {
		return fmt.Sprintf("%.1fs", d.Seconds())
	}
	if d < time.Hour {
		mins := int(d.Minutes())
		secs := int(d.Seconds()) % 60
		return fmt.Sprintf("%dm %ds", mins, secs)
	}
	hours := int(d.Hours())
	mins := int(d.Minutes()) % 60
	return fmt.Sprintf("%dh %dm", hours, mins)
}

// printSuccess prints a success message with random success emoji.
func printSuccess(message string) {
	emoji := successEmojis[rand.Intn(len(successEmojis))]
	fmt.Printf("\n%s %s\n", emoji, message)
}

// printError prints an error message with random error emoji and color-coded text.
func printError(message string) {
	emoji := errorEmojis[rand.Intn(len(errorEmojis))]
	manager := tui.NewManager()
	theme := manager.GetTheme()
	fmt.Printf("\n%s %s\n", emoji, theme.Error.Render(message))
}

// Approved sushi emojis for info messages (from .claude/EMOJIS.md)
var sushiEmojis = []string{
	"🍣", "🍙", "🥢", "🍥", "🍱", "🍜", "🍡", "🍢",
	"🦐", "🦑", "🐟", "🍤", "🥟", "🥡", "🍶", "🍵", "🥠", "🧋",
}

// Success emojis for completed operations
var successEmojis = []string{
	"🍱", // bento box
	"🍣", // sushi
	"🍜", // ramen bowl (steaming deliciousness)
	"🍡", // dango (sweet success)
	"🍥", // fish cake with swirl
	"🥢", // chopsticks (completing the meal)
	"🍵", // teacup (relaxing after success)
	"🍶", // sake bottle (celebrating)
	"🥟", // dumpling
	"🍙", // rice ball/onigiri
	"✨", // sparkles
	"🎉", // tada/party popper
}

// Error emojis for failed operations
var errorEmojis = []string{
	"👹",  // oni mask (Japanese demon)
	"👺",  // tengu/goblin mask
	"💀",  // skull
	"☠️", // skull and crossbones
	"💥",  // collision/explosion
	"🔥",  // fire
	"⚠️", // warning
	"❌",  // cross mark
	"🚫",  // no entry
	"🤢",  // nauseated/sick face
}

// Error status words for failed bentos (from .claude/STATUS_WORDS.md)
var errorStatusWords = []string{
	"Spoiled", "Burnt", "Dropped", "Ruined",
	"Failed", "Overcooked", "Undercooked",
}

// printInfo prints an info message with random sushi emoji.
// Randomly picks from approved sushi emoji list for fun variety.
func printInfo(message string) {
	// Use bento box emoji 🍱 for branding on "Running bento:" messages
	if strings.HasPrefix(message, "Running bento:") {
		fmt.Printf("🍱 %s\n", message)
		return
	}

	emoji := sushiEmojis[rand.Intn(len(sushiEmojis))]
	fmt.Printf("%s %s\n", emoji, message)
}

// printCheck prints a check mark for completed items.
func printCheck(message string) {
	fmt.Printf("✓ %s\n", message)
}

// getErrorStatusWord returns a random error status word.
func getErrorStatusWord() string {
	return errorStatusWords[rand.Intn(len(errorStatusWords))]
}
