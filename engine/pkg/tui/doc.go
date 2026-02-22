//go:build tui

// Package tui provides an interactive terminal user interface for bnto.
//
// Status: PAUSED - This package is preserved for reference and future development.
// The TUI is not actively maintained but remains functional.
//
// The TUI allows users to interactively select and execute bnto workflows, manage
// secrets and variables, configure settings, and view execution logs in real-time.
//
// Key components:
//   - Workflow browser with list navigation
//   - Variable form builder with file pickers
//   - Secrets management with secure storage
//   - Variables management with persistent config
//   - Real-time execution logs with syntax highlighting
//   - Configurable bnto home directory for cloud sync
//
// The TUI is built on the Bubble Tea framework and follows the Elm architecture
// pattern with a Model-Update-View cycle.
package tui
