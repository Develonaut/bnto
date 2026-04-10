// Shared TUI widgets — small, reusable rendering components.
//
// Each widget is a pure function returning ratatui primitives (Line, Span).
// No Frame dependency — fully testable without a terminal.

pub mod help_bar;
pub mod search_input;
pub mod status_line;
