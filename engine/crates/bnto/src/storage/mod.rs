// Persistent storage — paths, config, and atomic writes.
//
// Extracted from `tui/` so all commands (not just TUI) can access
// storage paths and config. The TUI re-imports from here.

pub mod atomic;
pub mod config;
pub mod paths;

pub use paths::BntoPaths;
