package engine

import (
	"fmt"
	"hash/fnv"
	"math/rand"
)

// logMessage provides standardized log messages with optional emojis.
// This ensures all log output uses approved emojis from .claude/EMOJIS.md
// and fun terminology from .claude/STATUS_WORDS.md.
type logMessage struct {
	emoji     string
	text      string
	isRunning bool
	isSuccess bool
	isFailed  bool
}

// format returns the formatted log message with emoji and colors.
func (m logMessage) format() string {
	// Colorize the text based on message type
	coloredText := colorizeMessage(m.text, m.isRunning, m.isSuccess, m.isFailed)

	if m.emoji != "" {
		return m.emoji + " " + coloredText
	}
	return coloredText
}

// errorEmojis contains approved error emojis for failed operations.
var errorEmojis = []string{
	"👹", "👺", "💀", "☠️", "💥", "🔥", "⚠️", "❌", "🚫", "🤢",
}

// statusWordsRunning contains fun status words for running nodes.
// Source: .claude/STATUS_WORDS.md (synchronized with pkg/tui/sushi.go)
var statusWordsRunning = []string{
	"Tasting",
	"Sampling",
	"Trying",
	"Enjoying",
	"Devouring",
	"Nibbling",
	"Savoring",
	"Testing",
}

// statusWordsCompleted contains fun status words for completed nodes.
// Source: .claude/STATUS_WORDS.md (synchronized with pkg/tui/sushi.go)
var statusWordsCompleted = []string{
	"Savored",
	"Devoured",
	"Enjoyed",
	"Relished",
	"Finished",
	"Consumed",
	"Completed",
	"Perfected",
}

// randomErrorEmoji returns a random error emoji from the approved list.
func randomErrorEmoji() string {
	return errorEmojis[rand.Intn(len(errorEmojis))]
}

// getStatusWord returns a fun varied status word based on node name.
// Uses deterministic hash to ensure same node gets same word.
func getStatusWord(name string, isRunning bool) string {
	h := fnv.New32a()
	h.Write([]byte(name))
	hash := h.Sum32()

	if isRunning {
		return statusWordsRunning[hash%uint32(len(statusWordsRunning))]
	}
	return statusWordsCompleted[hash%uint32(len(statusWordsCompleted))]
}

// msgBentoStarted creates a message for bento execution start.
// Format matches CLI output: "🍱 Running Bento: [name]"
func msgBentoStarted(name string) logMessage {
	return logMessage{
		emoji:     "🍱",
		text:      "Running Bento: " + name,
		isRunning: true,
	}
}

// msgBentoCompleted creates a message for bento execution completion.
// Format: "🍱 Delicious! Bento executed successfully in [duration]"
func msgBentoCompleted(duration string) logMessage {
	return logMessage{
		emoji:     "🍱", // Always use bento box emoji for consistency
		text:      "Delicious! Bento executed successfully in " + duration,
		isSuccess: true,
	}
}

// msgBentoFailed creates a message for bento execution failure.
// Format: "❌ Failed! Bento execution failed in [duration]"
func msgBentoFailed(duration string) logMessage {
	return logMessage{
		emoji:    randomErrorEmoji(),
		text:     "Failed! Bento execution failed in " + duration,
		isFailed: true,
	}
}

// msgNodeStarted creates a message for node execution start.
func msgNodeStarted() logMessage {
	return logMessage{
		emoji: "", // No emoji for individual node logs
		text:  "Executing node",
	}
}

// msgGroupStarted creates a message for group execution start.
func msgGroupStarted(breadcrumb, name string) logMessage {
	statusWord := getStatusWord(name, true)
	prefix := ""
	if breadcrumb != "" {
		prefix = "[" + breadcrumb + "]"
	}
	return logMessage{
		emoji:     "",
		text:      prefix + " " + statusWord + " NODE:group " + name,
		isRunning: true,
	}
}

// msgGroupCompleted creates a message for group execution completion.
// Format: "[Parent:Child] Finished NODE:group name (2ms)"
func msgGroupCompleted(breadcrumb, name, duration string) logMessage {
	statusWord := getStatusWord(name, false)
	prefix := ""
	if breadcrumb != "" {
		prefix = "[" + breadcrumb + "]"
	}
	return logMessage{
		emoji:     "",
		text:      prefix + " " + statusWord + " NODE:group " + name + " (" + duration + ")",
		isSuccess: true,
	}
}

// msgLoopStarted creates a message for loop execution start.
// Format: "[Parent:Child] Sampling NODE:loop name"
func msgLoopStarted(breadcrumb, name string) logMessage {
	statusWord := getStatusWord(name, true)
	prefix := ""
	if breadcrumb != "" {
		prefix = "[" + breadcrumb + "]"
	}
	return logMessage{
		emoji:     "",
		text:      prefix + " " + statusWord + " NODE:loop " + name,
		isRunning: true,
	}
}

// msgLoopCompleted creates a message for loop execution completion.
// Format: "[Parent:Child] Perfected NODE:loop name (2ms, 75%)"
func msgLoopCompleted(breadcrumb, name, duration string, progressPct int) logMessage {
	statusWord := getStatusWord(name, false)
	prefix := ""
	if breadcrumb != "" {
		prefix = "[" + breadcrumb + "]"
	}
	return logMessage{
		emoji:     "",
		text:      fmt.Sprintf("%s %s NODE:loop %s (%s, %d%%)", prefix, statusWord, name, duration, progressPct),
		isSuccess: true,
	}
}

// msgChildNodeStarted creates a message for child node execution start.
// Format: "[Parent:Child] Tasting NODE:type name"
func msgChildNodeStarted(breadcrumb, nodeType, name string) logMessage {
	statusWord := getStatusWord(name, true)
	prefix := ""
	if breadcrumb != "" {
		prefix = "[" + breadcrumb + "]"
	}
	return logMessage{
		emoji:     "",
		text:      prefix + " " + statusWord + " NODE:" + nodeType + " " + name,
		isRunning: true,
	}
}

// msgChildNodeCompleted creates a message for child node execution completion.
// Format: "[Parent:Child] Devoured NODE:type name (2ms, 10%)"
func msgChildNodeCompleted(breadcrumb, nodeType, name, duration string, progressPct int) logMessage {
	statusWord := getStatusWord(name, false)
	prefix := ""
	if breadcrumb != "" {
		prefix = "[" + breadcrumb + "]"
	}
	return logMessage{
		emoji:     "",
		text:      fmt.Sprintf("%s %s NODE:%s %s (%s, %d%%)", prefix, statusWord, nodeType, name, duration, progressPct),
		isSuccess: true,
	}
}
