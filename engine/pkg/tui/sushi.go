//go:build tui

package tui

// Sushi contains all approved sushi-themed emojis for bnto UI.
// These emojis maintain the Japanese food theme throughout the application.
//
// Source of truth: .claude/EMOJIS.md
// This list must stay synchronized with the documentation.
//
// See also: .claude/STATUS_WORDS.md for status word guidelines.
var Sushi = []string{
	"🍣", // sushi
	"🍙", // onigiri
	"🥢", // chopsticks
	"🍥", // fish cake
	"🍱", // bento box - PRIMARY ICON
	"🍜", // ramen
	"🍡", // dango
	"🍢", // oden
	"🦐", // shrimp
	"🦑", // squid
	"🐟", // fish
	"🍤", // fried shrimp
	"🥟", // dumpling
	"🥡", // takeout box
	"🍶", // sake
	"🍵", // teacup
	"🥠", // fortune cookie
	"🧋", // bubble tea
}

// SushiSpinner contains the subset of emojis used for animated spinner.
// Uses first 4 emojis for fast, recognizable rotation.
var SushiSpinner = []string{
	"🍣", // sushi
	"🍙", // onigiri
	"🥢", // chopsticks
	"🍥", // fish cake
}

// StatusWordsRunning contains fun status words for running nodes.
// These are used deterministically based on node name hash.
var StatusWordsRunning = []string{
	"Tasting",
	"Sampling",
	"Trying",
	"Enjoying",
	"Devouring",
	"Nibbling",
	"Savoring",
	"Testing",
}

// StatusWordsCompleted contains fun status words for completed nodes.
var StatusWordsCompleted = []string{
	"Savored",
	"Devoured",
	"Enjoyed",
	"Relished",
	"Finished",
	"Consumed",
	"Completed",
	"Perfected",
}

// StatusWordsFailed contains fun status words for failed nodes.
var StatusWordsFailed = []string{
	"Spoiled",
	"Burnt",
	"Dropped",
	"Ruined",
	"Failed",
	"Overcooked",
	"Undercooked",
}

// StatusWordPending is the status word for pending nodes.
const StatusWordPending = "Preparing"
