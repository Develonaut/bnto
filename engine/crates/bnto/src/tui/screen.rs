// Screen presentation — display strings and help hints per screen.
//
// Keeps rendering concerns separate from the state machine in app.rs.

use super::app::Screen;

impl Screen {
    /// Display title for the screen's border block.
    pub fn title(&self) -> &'static str {
        match self {
            Self::Browser => " bnto ",
            Self::Detail { .. } => " Recipe Detail ",
            Self::Picker { .. } => " File Picker ",
            Self::Execution { .. } => " Running ",
            Self::Results { .. } => " Results ",
        }
    }

    /// Placeholder label shown in the content area.
    pub fn placeholder_label(&self) -> String {
        match self {
            Self::Browser => "Select a recipe to get started.".to_string(),
            Self::Detail { slug } => format!("Configure {slug}"),
            Self::Picker { slug } => format!("Pick files for {slug}"),
            Self::Execution { slug } => format!("Running {slug}..."),
            Self::Results { slug } => format!("Results for {slug}"),
        }
    }

    /// Contextual key hints for the help bar.
    pub fn help_hints(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            Self::Browser => vec![
                ("↑↓", "navigate"),
                ("/", "search"),
                ("Enter", "select"),
                ("q", "quit"),
            ],
            Self::Detail { .. } => vec![
                ("↑↓", "navigate"),
                ("Enter", "edit/confirm"),
                ("Esc", "back"),
                ("q", "quit"),
            ],
            Self::Picker { .. } => vec![
                ("↑↓", "navigate"),
                ("Space", "select"),
                ("Enter", "confirm"),
                ("Esc", "back"),
            ],
            Self::Execution { .. } => vec![("Esc", "cancel")],
            Self::Results { .. } => vec![
                ("o", "open file"),
                ("O", "open folder"),
                ("r", "run another"),
                ("q", "quit"),
            ],
        }
    }
}
