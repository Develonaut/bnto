// Package main provides output formatting utilities for the bento CLI.
//
// This file contains helpers for creating user-friendly output with:
//   - Duration formatting (ms, seconds, minutes)
//   - Success/error/info messages
//   - Consistent emoji usage
package main

import (
	"fmt"
	"math/rand"
	"strings"
	"time"
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

// printError prints an error message with random error emoji.
func printError(message string) {
	emoji := errorEmojis[rand.Intn(len(errorEmojis))]
	fmt.Printf("\n%s %s\n", emoji, message)
}

// Info emojis for general messages
var infoEmojis = []string{
	"🍣", "🍙", "🥢", "🍥", "🍱", "🍜", "🍡", "🍢",
	"🦐", "🦑", "🐟", "🍤", "🥟", "🥡", "🍶", "🍵", "🥠", "🧋",
}

// Success emojis for completed operations
var successEmojis = []string{
	"🍱", "🍣", "🍜", "🍡", "🍥", "🥢", "🍵", "🍶",
	"🥟", "🍙", "✨", "🎉",
}

// Error emojis for failed operations
var errorEmojis = []string{
	"👹", "👺", "💀", "☠️", "💥", "🔥", "⚠️", "❌", "🚫", "🤢",
}

// Error status words for failed operations
var errorStatusWords = []string{
	"Spoiled", "Burnt", "Dropped", "Ruined", "Failed",
}

// printInfo prints an info message with random emoji.
// Uses bento box emoji for branding on "Running bento:" messages.
func printInfo(message string) {
	if strings.HasPrefix(message, "Running bento:") {
		fmt.Printf("🍱 %s\n", message)
		return
	}

	emoji := infoEmojis[rand.Intn(len(infoEmojis))]
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
